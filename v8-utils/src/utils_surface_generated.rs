// 由 tool/src/utils/utils.h.js 从 v8/src/utils/utils.{h,cc,inl.h} + memcopy.h + v8dll-main.cc 生成; 修订就地改, 复跑覆盖
// 通用工具面与内存拷贝转发

/// 内存拷贝转发骨架 (原版 memcopy.h; 底座 v8::base::MemCopy/MemMove)
pub mod memcopy_skel {
  /// 拷贝 (原版 MemCopy; 平台最优实现转发)
  pub fn mem_copy(_dest: *mut u8, _src: *const u8, _size: usize) {
    unimplemented!("骨架")
  }
  /// 可重叠移动 (原版 MemMove)
  pub fn mem_move(_dest: *mut u8, _src: *const u8, _size: usize) {
    unimplemented!("骨架")
  }
  /// 拷贝并翻转字节序 (原版 MemCopyAndSwitchEndianness)
  pub fn mem_copy_switch_endianness(_dst: *mut u8, _src: *const u8, _num: usize, _elem_size: usize) {
    unimplemented!("骨架")
  }
}

/// 页对齐协议 (原版 utils.h RoundUpToPageSize)
pub mod utils_page_skel {
  /// RoundUp 前置: byte_length 加 page_size-1 不回绕 (原版注释)
  pub fn round_up_to_page_size(_byte_length: usize, _page_size: usize) -> usize {
    unimplemented!("骨架")
  }
}

/// 字段填充登记宏 (原版 IS_FIELD_PADDING: V(Name, RoundUp(Offset)-Offset, hacker_name))
pub const IS_FIELD_PADDING_PATTERN: &str = "V(Name, (RoundUp<Alignment>(k##Name##Offset) - k##Name##Offset), hacker_name)";

/// utils.cc 整型常量 (机械登记, 0 项)
pub mod utils_cc_consts {

}

/// DLL 入口 (原版 v8dll-main.cc; 仅 V8_OS_WIN 生效, 当前目标跳过并登记)
pub const DLL_MAIN_PLATFORM: &str = "windows-only; DllMain 空实现返回 1";
