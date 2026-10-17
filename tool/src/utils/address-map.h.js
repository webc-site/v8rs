#!/usr/bin/env bun
// tool/src/utils/address-map.h.js
// 机械提取 v8/src/utils/address-map.{h,cc} + output-stream.{h,cc} + scoped-list.h:
// 地址映射表 / 输出流接口 / 作用域列表协议

import { readV8, writeSrc, banner } from "../../lib.js";

const amH = readV8("src/utils/address-map.h");
const amCc = readV8("src/utils/address-map.cc");
const osH = readV8("src/utils/output-stream.h");
const osCc = readV8("src/utils/output-stream.cc");
const slH = readV8("src/utils/scoped-list.h");

// 1. AddressMap 面
for (const k of ["Get(", "Set(", "using Entry = base::TemplateHashMapEntry<uintptr_t, uint32_t>"]) {
  if (!amH.includes(k)) throw new Error(`address-map.h 缺 ${k}`);
}
if (!amCc.includes("RootsTable::IsImmortalImmovable") || !amCc.includes("set_root_index_map")) {
  throw new Error("address-map.cc 缺 immortal-immovable 根登记实现");
}

// 2. OutputStream 接口面 (WriteAsciiChunk 双特化: char/uint16_t 载体)
const writeFns = [...osH.matchAll(/WriteResult (Write\w+)\(/g)].map((m) => m[1]);
if (writeFns.length !== 2 || writeFns[0] !== "WriteAsciiChunk") throw new Error(`OutputStream 写面漂移: ${writeFns.join()}`);
// 派生实现面 ( FileOutputStream / StringOutputStream )
for (const k of ["class V8_EXPORT_PRIVATE FileOutputStream", "class V8_EXPORT_PRIVATE StringOutputStream"]) {
  if (!osH.includes(k)) throw new Error(`output-stream.h 缺 ${k}`);
}
// cc 侧
if (!osCc.includes("FileOutputStream::") || !osCc.includes("StringOutputStream::")) throw new Error("output-stream.cc 缺派生实现");

// 3. ScopedList
for (const k of ["class V8_NODISCARD ScopedList", "Rewind(", "MergeInto(", "AddAll("]) {
  if (!slH.includes(k)) throw new Error(`scoped-list.h 缺 ${k}`);
}

let rust = banner(
  "src/utils/address-map.h.js",
  "src/utils/address-map.{h,cc} + output-stream.{h,cc} + scoped-list.h",
  "地址映射/输出流/作用域列表骨架",
);

rust += `
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
`;

const p = writeSrc("v8-utils", "address_map_generated.rs", rust);
console.log(`written ${p}; 写面=${writeFns.length}`);
