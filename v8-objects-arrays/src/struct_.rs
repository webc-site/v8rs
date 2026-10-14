// 由 tool/src/objects/struct.h.js 从 v8/src/objects/struct.h + struct.tq 生成骨架;
// -inl 语义合并指引由 tool/src/objects/struct-inl.h.js 维护; 修订就地改, 复跑覆盖基线
// Struct 标记类 + Tuple2/AccessorPair/ClassPositions/ForInEnumeratorHolder 全族落地

//! Struct 族: 简单结构类 (固定数量 tagged 字段的轻量堆对象)。
//!
//! 建模 (glossary: 继承 → 包装 + trait; C++ TaggedMember 字段 → repr(C) 逻辑布局):
//! - [`Struct`]: 标记基类, repr(transparent) 包 [`HeapObject`], 无新增字段
//!   (原版 static_assert(sizeof(Struct) == sizeof(HeapObject)), struct.h:32);
//! - 子类直接继承 HeapObject 头 (不经 FixedArrayBase), 字段自偏移
//!   `size_of::<HeapObject>()` (=8, 非压缩 usize 槽) 起按槽序 8/16/24/32 排布;
//! - 运行时语义按 fixed_array 先例挂 `*Ext` trait (跨 crate Tagged 包装禁
//!   inherent impl, E0116), impl 于 `Tagged<X>`: 槽读写走原子 (C++ 普通字段访问
//!   在并发标记默认配置下即 V8_ATOMIC_OBJECT_FIELD_WRITES=Relaxed, 同 core
//!   TaggedMember 先例), 写带屏障桩。
//!
//! 布局双轨制 (同 fixed_array_base 模块头): 本 crate 运行态为非压缩 8 字节 tagged
//! 模型, 快照偏移常量 (K_*) 与运行态 offset_of 合一; Tagged 槽压缩收窄后需按
//! snapshot 4 字节形态复核。
//!
//! 推迟登记 (tool/src/objects/struct-inl.h.js 指引 7/8 + 本席票单):
//! - Struct::BriefPrintDetails / Tuple2::BriefPrintDetails / ClassPositions::BriefPrintDetails
//!   + 全部 DECL_VERIFIER/DECL_PRINTER (8 项, struct.h:62-63/117-118/141-142/164-165)
//!     → L4 诊断层;
//! - `using BodyDescriptor = StructBodyDescriptor` (struct.h:17/30/167) → heap 层
//!   对象尺寸描述;
//! - AccessorPair::Copy (struct.h:84-85) / GetComponent (struct.h:107-110) →
//!   Isolate/Handle/NativeContext 域 (L3 heap/factory), 不落地桩;
//! - AllocationWitness/ReadOnlyRoots 构造族: X::new 的 map 由调用方显式传入
//!   (偏差登记); AccessorPair::new 的 null_value 初值与 IsNull 判定以
//!   [`Tagged::NULL`] 哨兵承接 (roots 落地后替换);
//! - Tuple2 带 RelaxedStoreTag 构造重载 (struct.h:40-41) → new 即 Relaxed 写入,
//!   不另设重载; AcquireLoad/ReleaseStore tag 重载 → 方法后缀 (映射见 -inl 指引 3)。

use std::{
  mem::{offset_of, size_of},
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_core::{
  heap_object::HeapObject,
  instance_type_checker as itc,
  map::Map,
  objects::{AccessorComponent, Cast, WriteBarrierMode, instance_type_of},
  smi::Smi,
  tagged::{Object, Tagged},
  tagged_field::TaggedSlot,
};

use crate::{fixed_array::FixedArray, fixed_array_base::write_barrier_stub};

// ── 槽地址助手 ───────────────────────────────────────────────────────────

/// 读 tagged 槽 (指定内存序; host + offset 指向对象内 8 对齐槽)。
///
/// # Safety
/// `host` 须为有效堆对象对齐基址, `offset` 落在对象内字段槽位, 无未初始化脏数据。
#[inline]
unsafe fn load_slot<T>(host: usize, offset: usize, order: Ordering) -> Tagged<T> {
  // SAFETY: 调用方契约保证 host+offset 为对象内有效对齐 TaggedSlot
  unsafe { Tagged::new(AtomicUsize::from_ptr((host + offset) as *mut TaggedSlot).load(order)) }
}

/// 写 tagged 槽 (指定内存序 + 写屏障桩)。
///
/// # Safety
/// 同 [`load_slot`]; `host` 可写且无并发别名写 (屏障桩语义见 fixed_array_base)。
#[inline]
unsafe fn store_slot<T>(
  host: usize,
  offset: usize,
  value: Tagged<T>,
  order: Ordering,
  mode: WriteBarrierMode,
) {
  // SAFETY: 调用方契约保证槽位有效可写
  unsafe { AtomicUsize::from_ptr((host + offset) as *mut TaggedSlot).store(value.ptr(), order) };
  write_barrier_stub(mode);
}

// ── Struct 标记基类 (struct.h:22-32) ─────────────────────────────────────

/// Struct: 简单结构类的抽象超类 (标记类, struct.h:19-21 注释)。
///
/// 不携带功能, 仅在类型系统中标识 struct 子类 (Tuple2/AccessorPair/
/// ClassPositions/ForInEnumeratorHolder); 谓词见 Cast 实现 (区间
/// FIRST_STRUCT_TYPE..=LAST_STRUCT_TYPE)。
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Struct {
  /// HeapObject 头 (Struct 不加字段)。
  pub header: HeapObject,
}

impl Struct {
  /// 构造头 (原版 explicit Struct(Tagged<ReadOnly<Map>> map), struct-inl.h:23;
  /// ReadOnly 视图归 roots, 以普通 map 承接)。
  #[inline]
  pub const fn new(map: Tagged<Map>) -> Self {
    Self {
      header: HeapObject::new(map),
    }
  }
}

// 原版 static_assert(sizeof(Struct) == sizeof(HeapObject)) (struct.h:32)
const _: () = assert!(size_of::<Struct>() == size_of::<HeapObject>());

// ── Tuple2 (struct.h:34-70) ──────────────────────────────────────────────

/// Tuple2: 2 个 Object 槽的配对结构 (struct.h:34-70)。
///
/// 字段槽偏移 8/16 (自 HeapObject 头起); 构造重载 (RelaxedStoreTag) 归并登记
/// 见模块头推迟清单。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Tuple2 {
  /// Struct 头 (继承建模: 包装)。
  pub struct_: Struct,
  /// value1_ (TaggedMember\<Object\>, struct.h:68, 偏移 8)。
  pub value1: Tagged<Object>,
  /// value2_ (TaggedMember\<Object\>, struct.h:69, 偏移 16)。
  pub value2: Tagged<Object>,
}

impl Tuple2 {
  /// 构造头 (struct-inl.h:25-35; AllocationWitness→map 注入, 模块头登记)。
  #[inline]
  pub const fn new(map: Tagged<Map>, value1: Tagged<Object>, value2: Tagged<Object>) -> Self {
    Self {
      struct_: Struct::new(map),
      value1,
      value2,
    }
  }
}

// 字段槽位布局对数 (struct.h:68-69 声明序 → 偏移 8/16)
const _: () = assert!(offset_of!(Tuple2, value1) == 8);
const _: () = assert!(offset_of!(Tuple2, value2) == 16);
const _: () = assert!(size_of::<Tuple2>() == 24);

/// Tagged&lt;Tuple2&gt; 字段语义 (struct-inl.h:37-66)。
///
/// 普通 load/store → Relaxed 原子 (C++ 普通访问在并发标记配置下即 Relaxed
/// 原子展开, -inl 指引 2); tag 重载 → 方法后缀。
pub trait Tuple2Ext: Copy {
  /// value1() (struct-inl.h:37; C++ 普通读 = Relaxed)。
  ///
  /// # Safety
  /// 指针须指向有效 Tuple2 对象内存。
  unsafe fn value1(&self) -> Tagged<Object>;

  /// value1(kRelaxedLoad) (struct-inl.h:41-43)。
  ///
  /// # Safety
  /// 同 [`Self::value1`]。
  unsafe fn value1_relaxed(&self) -> Tagged<Object>;

  /// value1(kAcquireLoad) (struct-inl.h:48-50)。
  ///
  /// # Safety
  /// 同 [`Self::value1`]。
  unsafe fn value1_acquire(&self) -> Tagged<Object>;

  /// set_value1(value, mode) (struct-inl.h:38-40; Rust 无默认实参, 屏障模式
  /// 显式传参, 偏差登记)。
  ///
  /// # Safety
  /// 指针有效可写; 无并发别名写。
  unsafe fn set_value1(&self, value: Tagged<Object>, mode: WriteBarrierMode);

  /// set_value1(value, kRelaxedStore, mode) (struct-inl.h:44-47)。
  ///
  /// # Safety
  /// 同 [`Self::set_value1`]。
  unsafe fn set_value1_relaxed(&self, value: Tagged<Object>, mode: WriteBarrierMode);

  /// set_value1(value, kReleaseStore, mode) (struct-inl.h:51-54)。
  ///
  /// # Safety
  /// 同 [`Self::set_value1`]。
  unsafe fn set_value1_release(&self, value: Tagged<Object>, mode: WriteBarrierMode);

  /// value2() (struct-inl.h:56)。
  ///
  /// # Safety
  /// 同 [`Self::value1`]。
  unsafe fn value2(&self) -> Tagged<Object>;

  /// value2(kRelaxedLoad) (struct-inl.h:60-62)。
  ///
  /// # Safety
  /// 同 [`Self::value1`]。
  unsafe fn value2_relaxed(&self) -> Tagged<Object>;

  /// set_value2(value, mode) (struct-inl.h:57-59)。
  ///
  /// # Safety
  /// 同 [`Self::set_value1`]。
  unsafe fn set_value2(&self, value: Tagged<Object>, mode: WriteBarrierMode);

  /// set_value2(value, kRelaxedStore, mode) (struct-inl.h:63-66)。
  ///
  /// # Safety
  /// 同 [`Self::set_value1`]。
  unsafe fn set_value2_relaxed(&self, value: Tagged<Object>, mode: WriteBarrierMode);
}

impl Tuple2Ext for Tagged<Tuple2> {
  #[inline]
  unsafe fn value1(&self) -> Tagged<Object> {
    // SAFETY: 调用方契约; 字段槽恒在对象内
    unsafe {
      load_slot(
        self.address(),
        offset_of!(Tuple2, value1),
        Ordering::Relaxed,
      )
    }
  }

  #[inline]
  unsafe fn value1_relaxed(&self) -> Tagged<Object> {
    // SAFETY: 调用方契约
    unsafe {
      load_slot(
        self.address(),
        offset_of!(Tuple2, value1),
        Ordering::Relaxed,
      )
    }
  }

  #[inline]
  unsafe fn value1_acquire(&self) -> Tagged<Object> {
    // SAFETY: 调用方契约
    unsafe {
      load_slot(
        self.address(),
        offset_of!(Tuple2, value1),
        Ordering::Acquire,
      )
    }
  }

  #[inline]
  unsafe fn set_value1(&self, value: Tagged<Object>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe {
      store_slot(
        self.address(),
        offset_of!(Tuple2, value1),
        value,
        Ordering::Relaxed,
        mode,
      )
    };
  }

  #[inline]
  unsafe fn set_value1_relaxed(&self, value: Tagged<Object>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe {
      store_slot(
        self.address(),
        offset_of!(Tuple2, value1),
        value,
        Ordering::Relaxed,
        mode,
      )
    };
  }

  #[inline]
  unsafe fn set_value1_release(&self, value: Tagged<Object>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe {
      store_slot(
        self.address(),
        offset_of!(Tuple2, value1),
        value,
        Ordering::Release,
        mode,
      )
    };
  }

  #[inline]
  unsafe fn value2(&self) -> Tagged<Object> {
    // SAFETY: 调用方契约
    unsafe {
      load_slot(
        self.address(),
        offset_of!(Tuple2, value2),
        Ordering::Relaxed,
      )
    }
  }

  #[inline]
  unsafe fn value2_relaxed(&self) -> Tagged<Object> {
    // SAFETY: 调用方契约
    unsafe {
      load_slot(
        self.address(),
        offset_of!(Tuple2, value2),
        Ordering::Relaxed,
      )
    }
  }

  #[inline]
  unsafe fn set_value2(&self, value: Tagged<Object>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe {
      store_slot(
        self.address(),
        offset_of!(Tuple2, value2),
        value,
        Ordering::Relaxed,
        mode,
      )
    };
  }

  #[inline]
  unsafe fn set_value2_relaxed(&self, value: Tagged<Object>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe {
      store_slot(
        self.address(),
        offset_of!(Tuple2, value2),
        value,
        Ordering::Relaxed,
        mode,
      )
    };
  }
}

// ── AccessorPair (struct.h:78-127) ───────────────────────────────────────

/// AccessorPair: JS 访问器的 getter/setter 对 (struct.h:72-77 注释)。
///
/// 每个组件可为 JS 函数/proxy、FunctionTemplateInfo (惰性访问器)、undefined
/// (规范也算访问器) 或 null (未设置的访问器)。
/// Copy/GetComponent 依赖 Isolate/NativeContext → 推迟 (模块头登记)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AccessorPair {
  /// Struct 头。
  pub struct_: Struct,
  /// getter_ (TaggedMember\<Object\>, struct.h:125, 偏移 8)。
  pub getter: Tagged<Object>,
  /// setter_ (TaggedMember\<Object\>, struct.h:126, 偏移 16)。
  pub setter: Tagged<Object>,
}

impl AccessorPair {
  /// 构造头 (struct-inl.h:68-71; roots().null_value() → Tagged::NULL 哨兵,
  /// 模块头登记)。
  #[inline]
  pub const fn new(map: Tagged<Map>) -> Self {
    Self {
      struct_: Struct::new(map),
      getter: Tagged::NULL,
      setter: Tagged::NULL,
    }
  }
}

const _: () = assert!(offset_of!(AccessorPair, getter) == 8);
const _: () = assert!(offset_of!(AccessorPair, setter) == 16);
const _: () = assert!(size_of::<AccessorPair>() == 24);

/// Tagged&lt;AccessorPair&gt; 组件语义 (struct-inl.h:73-128)。
///
/// get/set 按 [`AccessorComponent`] 分派 (core objects 枚举复用, -inl 指引 4);
/// 原版 set/set_components 默认 UPDATE_WRITE_BARRIER → 体内承接默认值。
pub trait AccessorPairExt: Copy {
  /// getter() (struct-inl.h:94)。
  ///
  /// # Safety
  /// 指针须指向有效 AccessorPair 对象内存。
  unsafe fn getter(&self) -> Tagged<Object>;

  /// getter(kAcquireLoad) (struct-inl.h:99-101)。
  ///
  /// # Safety
  /// 同 [`Self::getter`]。
  unsafe fn getter_acquire(&self) -> Tagged<Object>;

  /// set_getter(value, mode) (struct-inl.h:95-97)。
  ///
  /// # Safety
  /// 指针有效可写; 无并发别名写。
  unsafe fn set_getter(&self, value: Tagged<Object>, mode: WriteBarrierMode);

  /// set_getter(value, kReleaseStore, mode) (struct-inl.h:102-105)。
  ///
  /// # Safety
  /// 同 [`Self::set_getter`]。
  unsafe fn set_getter_release(&self, value: Tagged<Object>, mode: WriteBarrierMode);

  /// setter() (struct-inl.h:107)。
  ///
  /// # Safety
  /// 同 [`Self::getter`]。
  unsafe fn setter(&self) -> Tagged<Object>;

  /// setter(kAcquireLoad) (struct-inl.h:112-114)。
  ///
  /// # Safety
  /// 同 [`Self::getter`]。
  unsafe fn setter_acquire(&self) -> Tagged<Object>;

  /// set_setter(value, mode) (struct-inl.h:108-110)。
  ///
  /// # Safety
  /// 同 [`Self::set_getter`]。
  unsafe fn set_setter(&self, value: Tagged<Object>, mode: WriteBarrierMode);

  /// set_setter(value, kReleaseStore, mode) (struct-inl.h:115-118)。
  ///
  /// # Safety
  /// 同 [`Self::set_getter`]。
  unsafe fn set_setter_release(&self, value: Tagged<Object>, mode: WriteBarrierMode);

  /// get(component) (struct-inl.h:73-75)。
  ///
  /// # Safety
  /// 同 [`Self::getter`]。
  unsafe fn get(&self, component: AccessorComponent) -> Tagged<Object>;

  /// set(component, value) (struct-inl.h:77-83; 默认屏障模式承原版 UPDATE)。
  ///
  /// # Safety
  /// 同 [`Self::set_getter`]。
  unsafe fn set(&self, component: AccessorComponent, value: Tagged<Object>);

  /// set(component, value, kReleaseStore) (struct-inl.h:85-92)。
  ///
  /// # Safety
  /// 同 [`Self::set_getter`]。
  unsafe fn set_release(&self, component: AccessorComponent, value: Tagged<Object>);

  /// SetComponents(getter, setter) (struct-inl.h:120-123): 跳过 null 实参
  /// (IsNull → Tagged::NULL 哨兵, 模块头登记)。
  ///
  /// # Safety
  /// 同 [`Self::set_getter`]。
  unsafe fn set_components(&self, getter: Tagged<Object>, setter: Tagged<Object>);

  /// Equals(getter_value, setter_value) (struct-inl.h:125-128)。
  ///
  /// # Safety
  /// 同 [`Self::getter`]。
  unsafe fn equals(&self, getter_value: Tagged<Object>, setter_value: Tagged<Object>) -> bool;
}

impl AccessorPairExt for Tagged<AccessorPair> {
  #[inline]
  unsafe fn getter(&self) -> Tagged<Object> {
    // SAFETY: 调用方契约
    unsafe {
      load_slot(
        self.address(),
        offset_of!(AccessorPair, getter),
        Ordering::Relaxed,
      )
    }
  }

  #[inline]
  unsafe fn getter_acquire(&self) -> Tagged<Object> {
    // SAFETY: 调用方契约
    unsafe {
      load_slot(
        self.address(),
        offset_of!(AccessorPair, getter),
        Ordering::Acquire,
      )
    }
  }

  #[inline]
  unsafe fn set_getter(&self, value: Tagged<Object>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe {
      store_slot(
        self.address(),
        offset_of!(AccessorPair, getter),
        value,
        Ordering::Relaxed,
        mode,
      )
    };
  }

  #[inline]
  unsafe fn set_getter_release(&self, value: Tagged<Object>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe {
      store_slot(
        self.address(),
        offset_of!(AccessorPair, getter),
        value,
        Ordering::Release,
        mode,
      )
    };
  }

  #[inline]
  unsafe fn setter(&self) -> Tagged<Object> {
    // SAFETY: 调用方契约
    unsafe {
      load_slot(
        self.address(),
        offset_of!(AccessorPair, setter),
        Ordering::Relaxed,
      )
    }
  }

  #[inline]
  unsafe fn setter_acquire(&self) -> Tagged<Object> {
    // SAFETY: 调用方契约
    unsafe {
      load_slot(
        self.address(),
        offset_of!(AccessorPair, setter),
        Ordering::Acquire,
      )
    }
  }

  #[inline]
  unsafe fn set_setter(&self, value: Tagged<Object>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe {
      store_slot(
        self.address(),
        offset_of!(AccessorPair, setter),
        value,
        Ordering::Relaxed,
        mode,
      )
    };
  }

  #[inline]
  unsafe fn set_setter_release(&self, value: Tagged<Object>, mode: WriteBarrierMode) {
    // SAFETY: 调用方契约
    unsafe {
      store_slot(
        self.address(),
        offset_of!(AccessorPair, setter),
        value,
        Ordering::Release,
        mode,
      )
    };
  }

  #[inline]
  unsafe fn get(&self, component: AccessorComponent) -> Tagged<Object> {
    // SAFETY: 调用方契约 (分派至同对象槽读)
    unsafe {
      if component == AccessorComponent::AccessorGetter {
        self.getter()
      } else {
        self.setter()
      }
    }
  }

  #[inline]
  unsafe fn set(&self, component: AccessorComponent, value: Tagged<Object>) {
    // SAFETY: 分派至槽写; 原版默认屏障 UPDATE_WRITE_BARRIER
    unsafe {
      if component == AccessorComponent::AccessorGetter {
        self.set_getter(value, WriteBarrierMode::UpdateWriteBarrier);
      } else {
        self.set_setter(value, WriteBarrierMode::UpdateWriteBarrier);
      }
    }
  }

  #[inline]
  unsafe fn set_release(&self, component: AccessorComponent, value: Tagged<Object>) {
    // SAFETY: 分派; 原版默认屏障 UPDATE_WRITE_BARRIER
    unsafe {
      if component == AccessorComponent::AccessorGetter {
        self.set_getter_release(value, WriteBarrierMode::UpdateWriteBarrier);
      } else {
        self.set_setter_release(value, WriteBarrierMode::UpdateWriteBarrier);
      }
    }
  }

  #[inline]
  unsafe fn set_components(&self, getter: Tagged<Object>, setter: Tagged<Object>) {
    // SAFETY: 条件分派至槽写 (null 实参跳过, struct-inl.h:121-122)
    unsafe {
      if !getter.is_null() {
        self.set_getter(getter, WriteBarrierMode::UpdateWriteBarrier);
      }
      if !setter.is_null() {
        self.set_setter(setter, WriteBarrierMode::UpdateWriteBarrier);
      }
    }
  }

  #[inline]
  unsafe fn equals(&self, getter_value: Tagged<Object>, setter_value: Tagged<Object>) -> bool {
    // SAFETY: 调用方契约 (两次槽读)
    unsafe { self.getter() == getter_value && self.setter() == setter_value }
  }
}

// ── ClassPositions (struct.h:129-149) ────────────────────────────────────

/// ClassPositions: 类声明起止位置 (const Smi 字段, 偏移 8/16)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ClassPositions {
  /// Struct 头。
  pub struct_: Struct,
  /// start_ (const TaggedMember\<Smi\>, struct.h:147, 偏移 8)。
  pub start: Tagged<Smi>,
  /// end_ (const TaggedMember\<Smi\>, struct.h:148, 偏移 16)。
  pub end: Tagged<Smi>,
}

impl ClassPositions {
  /// 构造头 (struct-inl.h:130-134; int → Smi::from_int 编码写入)。
  #[inline]
  pub const fn new(map: Tagged<Map>, start: i32, end: i32) -> Self {
    Self {
      struct_: Struct::new(map),
      start: Smi::from_int(start),
      end: Smi::from_int(end),
    }
  }
}

const _: () = assert!(offset_of!(ClassPositions, start) == 8);
const _: () = assert!(offset_of!(ClassPositions, end) == 16);
const _: () = assert!(size_of::<ClassPositions>() == 24);

/// Tagged&lt;ClassPositions&gt; 只读语义 (struct-inl.h:136-138; const 字段无 setter)。
pub trait ClassPositionsExt: Copy {
  /// start() (= start_.load().value(), Smi 解包)。
  ///
  /// # Safety
  /// 指针须指向有效 ClassPositions 对象内存 (字段槽构造期即 Smi)。
  unsafe fn start(&self) -> i32;

  /// end() (struct-inl.h:138)。
  ///
  /// # Safety
  /// 同 [`Self::start`]。
  unsafe fn end(&self) -> i32;
}

impl ClassPositionsExt for Tagged<ClassPositions> {
  #[inline]
  unsafe fn start(&self) -> i32 {
    // SAFETY: 调用方契约; Smi 构造期保证槽值
    unsafe {
      load_slot(
        self.address(),
        offset_of!(ClassPositions, start),
        Ordering::Relaxed,
      )
      .value()
    }
  }

  #[inline]
  unsafe fn end(&self) -> i32 {
    // SAFETY: 调用方契约
    unsafe {
      load_slot(
        self.address(),
        offset_of!(ClassPositions, end),
        Ordering::Relaxed,
      )
      .value()
    }
  }
}

// ── ForInEnumeratorHolder (struct.h:151-177) ─────────────────────────────

/// ForInEnumeratorHolder: for-in 枚举器缓存持有者 (4 槽, 偏移 8/16/24/32)。
///
/// 字段全部 const (只读访问器); named_keys_ 原版 Tagged\<FixedArray\> —
/// AA-1 fixed_array::FixedArray 已落地, 直接引用标记类型。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ForInEnumeratorHolder {
  /// Struct 头。
  pub struct_: Struct,
  /// enum_cache_map_ (const TaggedMember\<Map\>, struct.h:173, 偏移 8)。
  pub enum_cache_map: Tagged<Map>,
  /// named_keys_ (const TaggedMember\<FixedArray\>, struct.h:174, 偏移 16)。
  pub named_keys: Tagged<FixedArray>,
  /// elements_length_ (const TaggedMember\<Smi\>, struct.h:175, 偏移 24)。
  pub elements_length: Tagged<Smi>,
  /// cache_length_ (const TaggedMember\<Smi\>, struct.h:176, 偏移 32)。
  pub cache_length: Tagged<Smi>,
}

impl ForInEnumeratorHolder {
  /// 构造头 (struct-inl.h:140-149; witness 初始化 → 头值直落, Smi 免屏障)。
  #[inline]
  pub const fn new(
    map: Tagged<Map>,
    enum_cache_map: Tagged<Map>,
    named_keys: Tagged<FixedArray>,
    elements_length: Tagged<Smi>,
    cache_length: Tagged<Smi>,
  ) -> Self {
    Self {
      struct_: Struct::new(map),
      enum_cache_map,
      named_keys,
      elements_length,
      cache_length,
    }
  }
}

// ObjectTraits<ForInEnumeratorHolder> offsetof 常量 (struct.h:179-189)
/// kEnumCacheMapOffset (struct.h:181-182)。
pub const K_ENUM_CACHE_MAP_OFFSET: usize = offset_of!(ForInEnumeratorHolder, enum_cache_map);
/// kNamedKeysOffset (struct.h:183-184)。
pub const K_NAMED_KEYS_OFFSET: usize = offset_of!(ForInEnumeratorHolder, named_keys);
/// kElementsLengthOffset (struct.h:185-186)。
pub const K_ELEMENTS_LENGTH_OFFSET: usize = offset_of!(ForInEnumeratorHolder, elements_length);
/// kCacheLengthOffset (struct.h:187-188)。
pub const K_CACHE_LENGTH_OFFSET: usize = offset_of!(ForInEnumeratorHolder, cache_length);

// 槽偏移对数: HeapObject 头 8 + Tagged 槽 8 → 8/16/24/32 (脚本 struct.h.js 同步锚)
const _: () = assert!(K_ENUM_CACHE_MAP_OFFSET == 8);
const _: () = assert!(K_NAMED_KEYS_OFFSET == 16);
const _: () = assert!(K_ELEMENTS_LENGTH_OFFSET == 24);
const _: () = assert!(K_CACHE_LENGTH_OFFSET == 32);
const _: () = assert!(size_of::<ForInEnumeratorHolder>() == 40);

/// Tagged&lt;ForInEnumeratorHolder&gt; 只读访问器族 (struct-inl.h:151-165)。
pub trait ForInEnumeratorHolderExt: Copy {
  /// enum_cache_map() (struct-inl.h:151-153)。
  ///
  /// # Safety
  /// 指针须指向有效 ForInEnumeratorHolder 对象内存。
  unsafe fn enum_cache_map(&self) -> Tagged<Map>;

  /// named_keys() (struct-inl.h:155-157)。
  ///
  /// # Safety
  /// 同 [`Self::enum_cache_map`]。
  unsafe fn named_keys(&self) -> Tagged<FixedArray>;

  /// elements_length() (struct-inl.h:159-161)。
  ///
  /// # Safety
  /// 同 [`Self::enum_cache_map`]。
  unsafe fn elements_length(&self) -> Tagged<Smi>;

  /// cache_length() (struct-inl.h:163-165)。
  ///
  /// # Safety
  /// 同 [`Self::enum_cache_map`]。
  unsafe fn cache_length(&self) -> Tagged<Smi>;
}

impl ForInEnumeratorHolderExt for Tagged<ForInEnumeratorHolder> {
  #[inline]
  unsafe fn enum_cache_map(&self) -> Tagged<Map> {
    // SAFETY: 调用方契约
    unsafe { load_slot(self.address(), K_ENUM_CACHE_MAP_OFFSET, Ordering::Relaxed) }
  }

  #[inline]
  unsafe fn named_keys(&self) -> Tagged<FixedArray> {
    // SAFETY: 调用方契约
    unsafe { load_slot(self.address(), K_NAMED_KEYS_OFFSET, Ordering::Relaxed) }
  }

  #[inline]
  unsafe fn elements_length(&self) -> Tagged<Smi> {
    // SAFETY: 调用方契约
    unsafe { load_slot(self.address(), K_ELEMENTS_LENGTH_OFFSET, Ordering::Relaxed) }
  }

  #[inline]
  unsafe fn cache_length(&self) -> Tagged<Smi> {
    // SAFETY: 调用方契约
    unsafe { load_slot(self.address(), K_CACHE_LENGTH_OFFSET, Ordering::Relaxed) }
  }
}

// ── Cast 谓词 (casting.h DEF_CAST_TRAITS; 判别锚点 instance_type_checker) ──

impl Cast for Struct {
  /// IsStruct: FIRST_STRUCT_TYPE..=LAST_STRUCT_TYPE 区间 (struct.tq:7 @abstract)。
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_struct)
  }
}

impl Cast for Tuple2 {
  /// IsTuple2: == TUPLE2_TYPE (判别值 173)。
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_tuple2)
  }
}

impl Cast for AccessorPair {
  /// IsAccessorPair: == ACCESSOR_PAIR_TYPE (判别值 141)。
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_accessor_pair)
  }
}

impl Cast for ClassPositions {
  /// IsClassPositions: == CLASS_POSITIONS_TYPE (判别值 151)。
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_class_positions)
  }
}

impl Cast for ForInEnumeratorHolder {
  /// IsForInEnumeratorHolder: == FOR_IN_ENUMERATOR_HOLDER_TYPE (判别值 157)。
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_for_in_enumerator_holder)
  }
}
