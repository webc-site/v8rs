//! 原版 zone-containers.h / zone-list.h / zone-allocator.h 模块汇总

pub mod allocator;
pub mod compact_set;
pub mod deque;
pub mod linked;
pub mod list;
pub mod vector;

/// 原版 using IntVector = ZoneVector<int>
pub type IntVector<'z, 'a> = vector::ZoneVector<'z, 'a, i32>;

/// 原版 zone-type-traits.h:19 using ZonePtrList = ZoneList<T*>
/// (原版 T 为 ZoneObject; Rust 以裸指针元素等价表达)
pub type ZonePtrList<'z, 'a, T> = list::ZoneList<'z, 'a, *mut T>;
