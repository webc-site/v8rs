// 由 tool/src/zone/zone-containers.h.js 从 v8/src/zone/zone-{containers,allocator,compact-set,hashmap,list-inl,type-traits,utils}.h 生成; 修订就地改, 复跑覆盖
// Zone 容器七件骨架

/// ZoneCompactSet 标签枚举 (原版 zone-compact-set.h enum Tag; 压缩于指针载荷位)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ZoneCompactSetTag {
  /// 单例: 载荷即句柄
  Singleton = 0,
  /// 空
  Empty = 1,
  /// 列表: 载荷指向 ZoneList
  List = 2,
}

/// Zone 容器类族 (原版 zone-containers.h class 清单, 18 项)
pub const ZONE_CONTAINER_ALIASES: [&str; 18] = ["ZoneVector", "ZoneIntrusiveSet", "ZoneDeque", "ZoneLinkedList", "ZoneForwardList", "ZonePriorityQueue", "ZoneQueue", "ZoneStack", "ZoneSet", "ZoneMultiset", "ZoneMap", "ZoneUnorderedMap", "ZoneUnorderedSet", "ZoneMultimap", "ZoneMapInit", "ZoneAbslFlatHashMap", "ZoneAbslFlatHashSet", "ZoneAbslBTreeMap"];

/// 容器骨架
pub mod zone_containers_skel {
  /// ZoneAllocator: 分配器适配 (原版 value_type 接口 + RecyclingZoneAllocator 复用变体)
  pub fn allocator_rebind() {
    unimplemented!("骨架")
  }

  /// ZoneHashmap: Zone 支撑的映射薄壳 (原版 zone-hashmap.h)
  pub fn hashmap_skel() {
    unimplemented!("骨架")
  }

  /// ZoneList 内联段: Add/clear 热路径 (原版 zone-list-inl.h)
  pub fn zone_list_add(_value: usize) {
    unimplemented!("骨架")
  }

  /// CloneVector 分支协议 (原版 zone-utils.h):
  /// 平凡可拷 → MemCopy; 否则逐元素 std::copy
  pub fn clone_vector_trivially_copyable_branch() -> bool {
    unimplemented!("骨架")
  }

  /// ZoneCompactSet 构造协议: 单例持 HandleToPointer, 列表挂 ZoneList (原版实现)
  pub fn compact_set_from_singleton() {
    unimplemented!("骨架")
  }
}
