// 由 tool/src/utils/address-map.h.js 从 v8/src/utils/address-map.{h,cc} + output-stream.{h,cc} + scoped-list.h 生成; 修订就地改, 复跑覆盖
// 地址映射/输出流/作用域列表骨架

/// AddressMap — 指针→下标双向映射 (原版 address-map.h; 底座 TemplateHashMapEntry<uintptr_t, uint32_t>)
pub mod address_map_skel {
  /// 条目类型对数
  pub const ENTRY_TYPE: &str = "TemplateHashMapEntry<uintptr_t, uint32_t>";

  /// 查值 (原版 Get → Maybe<uint32_t>)
  pub fn get(_obj: usize) -> Option<u32> {
    unimplemented!("骨架")
  }
  /// 写值 (原版 Set)
  pub fn set(_obj: usize, _index: u32) {
    unimplemented!("骨架")
  }
  /// 根索引表预填 (原版 cc:  immortal-immovable 只读根按 RootIndex 预登记,
  /// 可移动根跳过; 末了 set_root_index_map)
  pub fn prefill_root_index_map() {
    unimplemented!("骨架")
  }
}

/// OutputStream — 抽象输出流 (原版 output-stream.h)
pub mod output_stream_skel {
  /// 写结果 (原版 WriteResult: ok/abort)
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum WriteResult {
    Ok,
    Abort,
  }

  /// 分块写 ×2 (原版 WriteAsciiChunk 双载体特化; std::ostream / iostream 派生)
  pub fn write_ascii_chunk(_data: &[u8], _size: i32) -> WriteResult {
    unimplemented!("骨架")
  }
  /// 文件输出实现 (原版 FileOutputStream)
  pub fn file_output_stream_skel() {
    unimplemented!("骨架")
  }
  /// 字符串输出实现 (原版 StringOutputStream)
  pub fn string_output_stream_skel() {
    unimplemented!("骨架")
  }
}

/// ScopedList — Zone 上作用域限定的列表 (原版 scoped-list.h 模板:
/// Start 收集不定长, End 定容收缩; kInitialSize 起步)
pub mod scoped_list_skel {
  /// 开始收集 (原版 Start)
  pub fn start() {
    unimplemented!("骨架")
  }
  /// 结束收集并定容 (原版 End)
  pub fn end() {
    unimplemented!("骨架")
  }
  /// 回退清空 (原版 Rewind)
  pub fn rewind() {
    unimplemented!("骨架")
  }
  /// 归并到父列表 (原版 MergeInto)
  pub fn merge_into(_parent: usize) {
    unimplemented!("骨架")
  }
}
