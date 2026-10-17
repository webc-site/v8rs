use crate::{map::Map, tagged::Tagged, tagged_constants::HEAP_OBJECT_MAP_OFFSET};

/// 堆分配对象的通用基类头
///
/// 在 V8 内存模型中，所有分配在堆上的对象首部固定为指向其形状描述元对象 (Map) 的指针。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HeapObject {
  /// 对象的 Map 指针，首字段偏移固定为 0
  pub map: Tagged<Map>,
}

impl HeapObject {
  /// Map 字段相对于对象首地址的字节偏移量 (0)
  pub const MAP_OFFSET: usize = HEAP_OBJECT_MAP_OFFSET;

  /// 构造 HeapObject 头字段
  #[inline]
  pub const fn new(map: Tagged<Map>) -> Self {
    Self { map }
  }

  /// 获取对象的 Map 指针
  #[inline]
  pub const fn map(&self) -> Tagged<Map> {
    self.map
  }

  /// 更新对象的 Map 指针
  #[inline]
  pub fn set_map(&mut self, map: Tagged<Map>) {
    self.map = map;
  }
}

impl Tagged<HeapObject> {
  /// 从 Tagged 指针读取所指对象的 Map
  ///
  /// # Safety
  /// 调用者必须确保当前 Tagged 指针指向有效初始化的 HeapObject 堆内存。
  #[inline]
  pub unsafe fn map(&self) -> Tagged<Map> {
    // SAFETY: 调用者保证 self 指向有效初始化的 HeapObject 堆内存
    unsafe {
      let raw = self.as_ptr();
      (*raw).map
    }
  }

  /// 设置当前 Tagged 堆对象的 Map
  ///
  /// # Safety
  /// 调用者必须确保当前 Tagged 指针指向有效 HeapObject 内存且具备写权限。
  #[inline]
  pub unsafe fn set_map(&self, map: Tagged<Map>) {
    // SAFETY: 调用者保证 self 指向有效 HeapObject 内存且具备写权限
    unsafe {
      let raw = self.as_mut_ptr();
      (*raw).map = map;
    }
  }
}
