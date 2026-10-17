// 由 tool/src/objects/deoptimization-data-inl.h.js 从 v8/src/objects/deoptimization-data-inl.h (+ deoptimization-data.cc 复对) 生成; 修订就地改, 复跑覆盖
// 访问器 X-macro 展开; 主表面在 deoptimization_data.rs

//! 反优化数据内联访问器 (原版 deoptimization-data-inl.h)
//!
//! DeoptimizationData 是 ProtectedFixedArray 的索引型前端:
//! 头部 12 个固定元素槽 (kFirstDeoptEntryIndex=12) + 每 deopt 出口 3 槽的条目区
//! (DEBUG 增 NodeId 第 4 槽, 基线取 release 3 槽).
//!
//! 元素区/长度槽经 v8-objects-arrays 的 TaggedArrayExt 就位 (fixed-array 席已交付),
//! 本席句柄仅做 DeoptimizationData/DeoptimizationLiteralArray 前端换装。
//!
//! 推迟登记 (见 readme/zh.md 台账):
//! - `get_shared_function_info` 依赖 SharedFunctionInfoWrapper (归 shared-function-info 席)。

use v8_objects_arrays::fixed_array::{ProtectedFixedArray, TaggedArrayExt, TrustedWeakFixedArray};
use v8_objects_core::{
  heap_object::HeapObject,
  instance_type_checker as itc,
  objects::{WriteBarrierMode, instance_type_of},
  smi::Smi,
  tagged::{MaybeObject, Object, Tagged, make_weak},
};

use crate::{
  bytecode_array::BytecodeArray,
  code,
  error::{Error, Result},
};

/// 反优化数据对象占位 (原版 class DeoptimizationData final : public ProtectedFixedArray)
///
/// 单元标记类型: 供 `Tagged<DeoptimizationData>` 句柄形态; 元素本体布局由
/// v8-objects-arrays::fixed_array::ProtectedFixedArray 承载, 访问面见本 trait。
pub struct DeoptimizationData;

/// 反优化字面量数组占位 (原版 class DeoptimizationLiteralArray final : public TrustedWeakFixedArray)
pub struct DeoptimizationLiteralArray;

/// DeoptimizationData 语义面 (挂载于 Tagged&lt;DeoptimizationData&gt;)
pub trait DeoptimizationDataExt: Copy {
  /// v8: src/objects/fixed-array.h ProtectedFixedArray (ulength 继承 TaggedArrayBase)
  fn ulength(&self) -> Result<usize>;
  /// v8: src/objects/deoptimization-data-inl.h DeoptimizationData::literal_array
  /// (get(kLiteralArrayIndex) 后 Cast)
  fn literal_array(&self) -> Tagged<DeoptimizationLiteralArray>;
  /// v8: src/objects/deoptimization-data-inl.h DeoptimizationData::size_including_header
  /// (长度槽求含头字节数; FixedArrayBase 尺寸公式)
  fn size_including_header(&self) -> i32;
  /// v8: src/objects/deoptimization-data-inl.h DeoptimizationData::DeoptCount
  fn deopt_count(&self) -> Result<u32>;
  /// v8: src/objects/deoptimization-data-inl.h
  /// DeoptimizationData::GetBytecodeOffsetOrBuiltinContinuationId
  /// (BytecodeOffset 以 i32 原值表示, 同 code.rs osr_offset 约定)
  fn bytecode_offset_or_builtin_continuation_id(&self, i: usize) -> Result<i32>;
  /// v8: src/objects/deoptimization-data-inl.h DeoptimizationData::SetBytecodeOffset
  fn set_bytecode_offset(&self, i: usize, value: i32) -> Result<()>;
  /// v8: src/objects/deoptimization-data-inl.h DeoptimizationData::GetSharedFunctionInfo
  /// (SharedFunctionInfoWrapper 归 shared-function-info 席 → 台账)
  fn get_shared_function_info(&self);
}

/// 元素区句柄换装 (原版 DeoptimizationData 即 ProtectedFixedArray 子类)
#[inline]
fn protected_front(data: Tagged<DeoptimizationData>) -> Tagged<ProtectedFixedArray> {
  // 基线: 前端类型仅编译期标记, 运行时同指针
  data.cast()
}

impl DeoptimizationDataExt for Tagged<DeoptimizationData> {
  #[inline]
  fn ulength(&self) -> Result<usize> {
    // 原版: FixedArrayBase::ulength() 读长度槽 (SafeHeapObjectSize, uint32 宽)
    // SAFETY: 契约 — Tagged<DeoptimizationData> 指向有效 ProtectedFixedArray
    let len = unsafe { protected_front(*self).ulength() };
    Ok(len as usize)
  }

  #[inline]
  fn literal_array(&self) -> Tagged<DeoptimizationLiteralArray> {
    // 原版: Cast<DeoptimizationLiteralArray>(get(kLiteralArrayIndex))
    // kLiteralArrayIndex = 3 (DEOPT_ELEMENT_SLOTS 第 4 槽)
    // SAFETY: 契约同上; kLiteralArrayIndex < kFirstDeoptEntryIndex 恒在界内
    let element: Tagged<Object> = unsafe { protected_front(*self).get(3) }.cast();
    // 原版 Cast 的 instance type CHECK 归句柄信任链 (基线裸转换)
    element.cast()
  }

  #[inline]
  fn size_including_header(&self) -> i32 {
    // 原版: FixedArrayBase 尺寸公式 SizeFor(ulength())
    // = OBJECTS_OFFSET + len * kElementSize (TaggedArrayExt::size_for)
    // SAFETY: 契约同上
    let len = unsafe { protected_front(*self).ulength() };
    <Tagged<ProtectedFixedArray> as TaggedArrayExt>::size_for(len) as i32
  }

  #[inline]
  fn deopt_count(&self) -> Result<u32> {
    // 原版: (ulength - kFirstDeoptEntryIndex) / kDeoptEntrySize
    let len = self.ulength()?;
    if len == 0 {
      return Ok(0);
    }
    if len < K_FIRST_DEOPT_ENTRY_INDEX {
      return Err(Error::OutOfBounds {
        index: K_FIRST_DEOPT_ENTRY_INDEX,
        length: len,
      });
    }
    Ok(((len - K_FIRST_DEOPT_ENTRY_INDEX) / K_DEOPT_ENTRY_SIZE) as u32)
  }

  #[inline]
  fn bytecode_offset_or_builtin_continuation_id(&self, i: usize) -> Result<i32> {
    // 原版: BytecodeOffset(BytecodeOffsetRaw(i).value());
    // BytecodeOffsetRaw(i) = get(kFirstDeoptEntryIndex + i*kDeoptEntrySize + 0)
    let index = K_FIRST_DEOPT_ENTRY_INDEX + i * K_DEOPT_ENTRY_SIZE;
    let len = self.ulength()?;
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    // SAFETY: 上方界检
    let element: Tagged<Smi> = unsafe { protected_front(*self).get(index as u32) }.cast();
    Ok(element.value())
  }

  #[inline]
  fn set_bytecode_offset(&self, i: usize, value: i32) -> Result<()> {
    // 原版: SetBytecodeOffsetRaw(i, Smi::FromInt(value.ToInt()))
    let index = K_FIRST_DEOPT_ENTRY_INDEX + i * K_DEOPT_ENTRY_SIZE;
    let len = self.ulength()?;
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    // SAFETY: 上方界检
    unsafe { protected_front(*self).set_smi(index as u32, Smi::from_int(value)) };
    Ok(())
  }

  #[inline]
  fn get_shared_function_info(&self) {
    // 原版: TrustedCast<SharedFunctionInfoWrapper>(WrappedSharedFunctionInfo())
    //       ->shared_info(); 元素读面 (index 7) 已就位, 缺
    //       SharedFunctionInfoWrapper 类本体 (shared-function-info-wrappers.h)
    // TODO(台账): 归 shared-function-info 席, 就位即填
    todo!(
      "推迟登记(readme/zh.md): SharedFunctionInfoWrapper::shared_info 归 shared-function-info 席"
    )
  }
}

/// DeoptimizationLiteralArray 语义面 (挂载于 Tagged&lt;DeoptimizationLiteralArray&gt;)
pub trait DeoptimizationLiteralArrayExt: Copy {
  /// v8: src/objects/fixed-array.h TrustedWeakFixedArray (ulength 继承 TaggedArrayBase)
  fn ulength(&self) -> Result<usize>;
  /// v8: src/objects/deoptimization-data-inl.h DeoptimizationLiteralArray::get_raw
  /// (直转 TrustedWeakFixedArray::get, 返回弱槽原值 MaybeObject)
  fn get_raw(&self, index: usize) -> Result<Tagged<MaybeObject>>;
  /// v8: src/objects/deoptimization-data-inl.h DeoptimizationLiteralArray::get
  /// (弱槽取强值; 已清除 = 不可达路径, 原版 CHECK 失败)
  fn get(&self, index: usize) -> Result<Tagged<Object>>;
  /// v8: src/objects/deoptimization-data-inl.h DeoptimizationLiteralArray::set
  /// (三分支换装: BytecodeArray→wrapper / 沙箱开 RegExpData→wrapper /
  /// 可弱对象→MakeWeak; 基线沙箱关仅两支)
  fn set(&self, index: usize, value: Tagged<Object>) -> Result<()>;
}

/// 弱元素区句柄换装 (原版 DeoptimizationLiteralArray 即 TrustedWeakFixedArray 子类)
#[inline]
fn weak_front(array: Tagged<DeoptimizationLiteralArray>) -> Tagged<TrustedWeakFixedArray> {
  array.cast()
}

impl DeoptimizationLiteralArrayExt for Tagged<DeoptimizationLiteralArray> {
  #[inline]
  fn ulength(&self) -> Result<usize> {
    // 原版: TrustedWeakFixedArray::ulength() 读长度槽
    // SAFETY: 契约 — Tagged<DeoptimizationLiteralArray> 指向有效 TrustedWeakFixedArray
    let len = unsafe { weak_front(*self).ulength() };
    Ok(len as usize)
  }

  #[inline]
  fn get_raw(&self, index: usize) -> Result<Tagged<MaybeObject>> {
    // 原版: TrustedWeakFixedArray::get(index) 弱全宽槽读
    let len = self.ulength()?;
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    // SAFETY: 上方界检
    Ok(unsafe { weak_front(*self).get(index as u32) })
  }

  #[inline]
  fn get(&self, index: usize) -> Result<Tagged<Object>> {
    // 原版: get → CHECK(!maybe.IsCleared()) → maybe.GetHeapObjectOrSmi()
    // (字面量槽仅在无可达代码路径时清除; 运行中 InstructionStream 由强标记兜底)
    let maybe = self.get_raw(index)?;
    if maybe.is_cleared() {
      // 原版 CHECK 失败 = 不可达; 转不变式错误 (禁 panic 红线)
      return Err(Error::Invariant(
        "DeoptimizationLiteralArray 弱槽已清除 (原版 CHECK(!IsCleared))",
      ));
    }
    Ok(maybe.get_heap_object_or_smi())
  }

  #[inline]
  fn set(&self, index: usize, value: Tagged<Object>) -> Result<()> {
    // 原版: 三分支换装后 TrustedWeakFixedArray::set(index, maybe)
    let mut maybe: Tagged<MaybeObject> = value.cast();
    if let Some(ty) = instance_type_of(value) {
      if itc::is_bytecode_array(ty) {
        // BytecodeArray 在 trusted space, 改存其 wrapper (专为此起)
        let ba: Tagged<BytecodeArray> = value.cast();
        // SAFETY: 契约 — value 为有效 BytecodeArray (instance type 判定)
        let wrapper = unsafe { (*ba.as_ptr()).wrapper };
        maybe = wrapper.cast();
      } else if code::is_weak_object_in_deoptimization_literal_array(value) {
        // Map 恒强持有 (上游已排除), 其余可弱对象 MakeWeak
        maybe = make_weak(value.cast::<HeapObject>()).cast();
      }
    }
    // 沙箱开支 (RegExpData→wrapper) 基线不存在, #ifdef 剔除
    let len = self.ulength()?;
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    // SAFETY: 上方界检
    unsafe { weak_front(*self).set(index as u32, maybe, WriteBarrierMode::UpdateWriteBarrier) };
    Ok(())
  }
}

/// 元素槽表 (DEFINE_DEOPT_ELEMENT_ACCESSORS 展开, 12 槽, 索引即数组下标)
pub const DEOPT_ELEMENT_SLOTS: [(&str, &str); 12] = [
  ("FrameTranslation", "DeoptimizationFrameTranslation"),
  ("InlinedFunctionCount", "Smi"),
  (
    "ProtectedLiteralArray",
    "ProtectedDeoptimizationLiteralArray",
  ),
  ("LiteralArray", "DeoptimizationLiteralArray"),
  ("OsrBytecodeOffset", "Smi"),
  ("OsrPcOffset", "Smi"),
  ("OptimizationId", "Smi"),
  (
    "WrappedSharedFunctionInfo",
    "SharedFunctionInfoWrapperOrSmi",
  ),
  ("InliningPositions", "TrustedPodArray<InliningPosition>"),
  ("DeoptExitStart", "Smi"),
  ("EagerDeoptCount", "Smi"),
  ("LazyDeoptCount", "Smi"),
];

/// 条目槽 (DEFINE_DEOPT_ENTRY_ACCESSORS 展开, DEBUG 全集 4 槽)
pub const DEOPT_ENTRY_SLOTS: [(&str, &str); 4] = [
  ("BytecodeOffsetRaw", "Smi"),
  ("TranslationIndex", "Smi"),
  ("Pc", "Smi"),
  ("NodeId", "Smi"),
];

/// 首个 deopt 条目的数组下标 (原版 kFirstDeoptEntryIndex)
pub const K_FIRST_DEOPT_ENTRY_INDEX: usize = 12;

/// 单条目槽数 (原版 kDeoptEntrySize: release 3 槽; DEBUG 4 槽含 NodeId, 基线取 release)
pub const K_DEOPT_ENTRY_SIZE: usize = 3;
