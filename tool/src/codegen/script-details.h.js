#!/usr/bin/env bun
// tool/src/codegen/script-details.h.js
// 机械提取 v8/src/codegen/script-details.h: ScriptDetails 字段布局 + 签名骨架
// → v8-codegen-core/src/script_details.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/codegen/script-details.h");

const m = (re, what) => {
  const r = src.match(re);
  if (!r) throw new Error(`未找到 ${what}`);
  return r;
};

m(/struct ScriptDetails \{/, "ScriptDetails");
// 字段清点 (对数): 8 个数据字段
const FIELDS = [
  "int line_offset;",
  "int column_offset;",
  "MaybeHandle<Object> name_obj;",
  "MaybeHandle<Object> source_map_url;",
  "MaybeHandle<Object> host_defined_options;",
  "MaybeHandle<FixedArray> wrapped_arguments;",
  "REPLMode repl_mode;",
  "const ScriptOriginOptions origin_options;",
];
for (const f of FIELDS) {
  if (!src.includes(f)) throw new Error(`缺字段 ${f}`);
}
m(/void SetScriptFieldsFromDetails\(/, "SetScriptFieldsFromDetails");
// 默认 repl_mode = REPLMode::kNo
m(/repl_mode\(REPLMode::kNo\)/, "默认 kNo", src);

let rust = banner(
  "src/codegen/script-details.h.js",
  "src/codegen/script-details.h",
  "ScriptDetails 字段布局 + 签名骨架",
);

rust += `
/// 原版 REPLMode::kNo 哨兵编码 (枚举本体见 parsing/interpreter 侧)
pub const REPL_MODE_NO: u8 = 0;

/// 原版 ScriptDetails: 脚本编译入参。
/// MaybeHandle 以 Option<u32> 占位 (对象句柄待堆侧接线)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScriptDetails {
  /// 行偏移
  pub line_offset: i32,
  /// 列偏移
  pub column_offset: i32,
  /// 脚本名 (原版 name_obj)
  pub name_obj: Option<u32>,
  /// source_map_url
  pub source_map_url: Option<u32>,
  /// host_defined_options
  pub host_defined_options: Option<u32>,
  /// 包装参数 (原版 wrapped_arguments)
  pub wrapped_arguments: Option<u32>,
  /// REPL 模式
  pub repl_mode: u8,
  /// 来源选项 (ScriptOriginOptions 位域编码占位)
  pub origin_options: u32,
}

impl Default for ScriptDetails {
  /// 原版默认构造: line=0, column=0, repl_mode=kNo
  fn default() -> Self {
    Self {
      line_offset: 0,
      column_offset: 0,
      name_obj: None,
      source_map_url: None,
      host_defined_options: None,
      wrapped_arguments: None,
      repl_mode: REPL_MODE_NO,
      origin_options: 0,
    }
  }
}

impl ScriptDetails {
  /// 原版显式构造: (script_name, origin_options = 默认)
  #[must_use]
  pub const fn with_name(script_name: Option<u32>, origin_options: u32) -> Self {
    Self {
      line_offset: 0,
      column_offset: 0,
      name_obj: script_name,
      source_map_url: None,
      host_defined_options: None,
      wrapped_arguments: None,
      repl_mode: REPL_MODE_NO,
      origin_options,
    }
  }
}

/// 原版 SetScriptFieldsFromDetails(isolate, script, details, no_gc) — 签名骨架
pub fn set_script_fields_from_details(
  isolate: *mut (),
  script: *mut (),
  script_details: &ScriptDetails,
  no_gc: *mut (),
) {
  let _ = (isolate, script, script_details, no_gc);
  todo!("待 Developer 修订: 对齐 objects 侧 SetScriptFieldsFromDetails")
}

// —— 提取不变式断言 ——
const _: () = {
  let d = ScriptDetails::default();
  assert!(d.line_offset == 0 && d.column_offset == 0 && d.repl_mode == REPL_MODE_NO);
  assert!(d.name_obj.is_none());
};
`;

const p = writeSrc("v8-codegen-core", "script_details.rs", rust);
console.log(`script-details: 字段 ${FIELDS.length} / 构造 2 / 自由函数 1 → ${p}`);
