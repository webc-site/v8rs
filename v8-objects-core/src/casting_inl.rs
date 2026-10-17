// 由 tool/src/objects/casting-inl.h.js 从 v8/src/objects/casting-inl.h (+ casting.h 家族登记) 生成; 修订就地改, 复跑覆盖
// GCAwareObjectTypeCheck 语义与 TryCast / Is<T> 向下转型实现

//! GC 感知对象类型检查与类型安全向下转型 (原版 casting.h / casting-inl.h)

use crate::{
  heap_number::HeapNumber,
  heap_object::HeapObject,
  instance_type_checker::{is_heap_number, is_map, is_oddball, is_symbol},
  map::Map,
  oddball::Oddball,
  smi::Smi,
  symbol::Symbol,
  tagged::{Object, Tagged},
};

/// 转型特征约束 (原版 CastTraits<To>)
pub trait CastTraits<T> {
  /// 判定来源对象是否属于目标类型 T
  fn allow_from(obj: Tagged<Object>) -> bool;
}

impl CastTraits<Object> for Object {
  #[inline]
  fn allow_from(_obj: Tagged<Object>) -> bool {
    true
  }
}

impl CastTraits<Smi> for Smi {
  #[inline]
  fn allow_from(obj: Tagged<Object>) -> bool {
    obj.is_smi()
  }
}

impl CastTraits<HeapObject> for HeapObject {
  #[inline]
  fn allow_from(obj: Tagged<Object>) -> bool {
    obj.is_heap_object()
  }
}

macro_rules! impl_cast_traits_via_instance_type {
  ($ty:ident, $pred:ident) => {
    impl CastTraits<$ty> for $ty {
      #[inline]
      fn allow_from(obj: Tagged<Object>) -> bool {
        match obj.to_heap_object() {
          Some(ho) => unsafe { $pred(ho.map().as_ref().instance_type()) },
          None => false,
        }
      }
    }
  };
}

impl_cast_traits_via_instance_type!(Map, is_map);
impl_cast_traits_via_instance_type!(HeapNumber, is_heap_number);
impl_cast_traits_via_instance_type!(Symbol, is_symbol);
impl_cast_traits_via_instance_type!(Oddball, is_oddball);

/// 类型测试 (原版 Is<T>)
#[inline]
pub fn is<T: CastTraits<T>>(obj: Tagged<Object>) -> bool {
  T::allow_from(obj)
}

/// 尝试安全向下转型 (原版 TryCast<T>)
#[inline]
pub fn try_cast<T: CastTraits<T>>(obj: Tagged<Object>) -> Option<Tagged<T>> {
  is::<T>(obj).then(|| Tagged::new(obj.ptr()))
}

/// 调试期检查向下转型 (原版 Cast<T>)
#[inline]
pub fn cast<T: CastTraits<T>>(obj: Tagged<Object>) -> Tagged<T> {
  debug_assert!(is::<T>(obj), "Cast type check failed");
  Tagged::new(obj.ptr())
}

/// 无检查原始转型 (原版 UncheckedCast<T>)
#[inline]
pub fn unchecked_cast<T>(obj: Tagged<Object>) -> Tagged<T> {
  Tagged::new(obj.ptr())
}

/// GC 运行状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcState {
  NotRunning,
  Scavenge,
  MarkCompact,
}

/// GC 场景判定枚举 (原版函数内 4 个编号分支, 4 种)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcObjectTypeCheckCase {
  /// 1) Scavenge 进行中且对象为已提升的大对象页 (map 检查跳过)
  PromotedLargeObjectInScavenge,
  /// 2) 保守栈扫描下被钉住的年轻代对象, map 字已是自转发地址
  PinnedByConservativeStackScanning,
  /// 3) GC 进行中且对象已被疏散, 转发目标是 T
  EvacuatedToTargetOfT,
  /// 4) 常规情形: 退回 Is<T>(object)
  RegularIsCheck,
}

/// GC 感知类型检查环境
#[derive(Debug, Clone, Copy)]
pub struct GcCheckContext {
  pub gc_state: GcState,
  pub is_large_page: bool,
  pub in_young_generation: bool,
  pub is_conservative_pinning: bool,
}

/// GC 感知类型检查 (原版 GCAwareObjectTypeCheck<T>, DEBUG 构建)
#[inline]
pub fn gc_aware_object_type_check<T: CastTraits<T>>(
  object: Tagged<Object>,
  ctx: Option<&GcCheckContext>,
  map_word_forwarding: Option<Tagged<Object>>,
) -> bool {
  if !object.is_heap_object() {
    return is::<T>(object);
  }

  if let Some(ctx) = ctx {
    match ctx.gc_state {
      // 1) Scavenge 进行中且对象为已提升的大对象页
      GcState::Scavenge if ctx.is_large_page => return true,
      // 2) 保守栈扫描下被钉住的年轻代对象
      GcState::Scavenge
        if ctx.in_young_generation
          && ctx.is_conservative_pinning
          && map_word_forwarding.is_some() =>
      {
        return true;
      }
      // 3) GC 进行中且对象已被疏散, 转发目标是 T
      state if state != GcState::NotRunning => {
        if let Some(target) = map_word_forwarding
          && is::<T>(target)
        {
          return true;
        }
      }
      _ => {}
    }
  }

  // 4) 常规情形: 退回 Is<T>(object)
  is::<T>(object)
}
