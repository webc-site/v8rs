#!/usr/bin/env bun
// tool/src/wasm/wasm-module-sourcemap.h.js
// 机械提取 v8/src/wasm/wasm-module-sourcemap.{h,cc} 的 VLQ 解码状态机与查询语义

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-module-sourcemap.h");
const ccSrc = readV8("src/wasm/wasm-module-sourcemap.cc");

// 1. h: 四个平行数组 + valid
for (const f of [
  "std::vector<size_t> offsets;",
  "std::vector<std::string> filenames;",
  "std::vector<size_t> file_idxs;",
  "std::vector<size_t> source_row;",
  "bool valid_ = false;",
]) {
  if (!hSrc.includes(f)) throw new Error(`成员缺失: ${f}`);
}
if (!hSrc.includes("// As column number in source file is always 0")) {
  throw new Error("未找到 column 不存注释");
}

// 2. h: 规格妥协三条
for (const note of [
  '"names" field is an empty array',
  '";" is treated as illegal symbol in',
  'we only\n// consider "mappings" with 4 fields',
]) {
  if (!hSrc.includes(note)) throw new Error(`规格注释缺失: ${note}`);
}

// 3. cc: version 必须 3
if (!ccSrc.includes("version != 3u")) throw new Error("未找到 version==3 检查");

// 4. cc: JSON 键名
for (const key of ['"version"', '"sources"', '"mappings"', '"length"']) {
  if (!ccSrc.includes(key)) throw new Error(`JSON 键缺失: ${key}`);
}

// 5. cc: DecodeMapping 状态机 — 4 个 VLQ 字段累加
const vlqCount = (ccSrc.match(/VLQBase64Decode\(s\.c_str\(\), s\.size\(\), &pos\)/g) || []).length;
if (vlqCount !== 4) throw new Error(`VLQ 解码调用 ${vlqCount} 次 != 4`);
for (const acc of ["gen_col += qnt;", "file_idx += qnt;", "ori_line += qnt;"]) {
  if (!ccSrc.includes(acc)) throw new Error(`累加缺失: ${acc}`);
}
const i32Min = (ccSrc.match(/std::numeric_limits<int32_t>::min\(\)/g) || []).length;
if (i32Min !== 4) throw new Error(`i32min 哨兵 ${i32Min} 次 != 4`);

// 6. cc: 分隔与终止规则
if (!ccSrc.includes("if (s[pos] == ',') {\n      ++pos;\n      continue;")) {
  throw new Error("未找到跳逗号");
}
if (!ccSrc.includes("if (pos < s.size() && s[pos] != ',') return false;")) {
  throw new Error("未找到段终止检查");
}

// 7. cc: 查询语义
for (const q of [
  "std::upper_bound(offsets.begin(), offsets.end(), wasm_offset)",
  "return start <= *(offsets.end() - 1) && end > *offsets.begin();",
  "if (entry_offset < start) return false;",
]) {
  if (!ccSrc.includes(q)) throw new Error(`查询锚缺失: ${q}`);
}

// 8. cc: 尺寸看板
if (!ccSrc.includes("UPDATE_WHEN_CLASS_CHANGES(WasmModuleSourceMap, 104);")) {
  throw new Error("未找到尺寸看板 104");
}

console.log(`wasm-module-sourcemap: VLQ字段=${vlqCount} i32min=${i32Min} 查询=3`);

// 9. 生成 Rust
let rust = banner(
  "src/wasm/wasm-module-sourcemap.h.js",
  "src/wasm/wasm-module-sourcemap.{h,cc}",
  "wasm source map VLQ 解码与偏移查询",
);

rust += `
//! Emscripten 风格 wasm source map: 单行 mappings, 每段 4 个 VLQ 字段。
/// v8: src/wasm/wasm-module-sourcemap.h WasmModuleSourceMap

/// 仅接受的 source map 版本
/// v8: src/wasm/wasm-module-sourcemap.cc version != 3u
pub const REQUIRED_VERSION: u32 = 3;

/// mappings 每段的 VLQ 字段数 (gen_col, file_idx, ori_line, ori_col)
/// v8: src/wasm/wasm-module-sourcemap.cc DecodeMapping
pub const FIELDS_PER_SEGMENT: usize = 4;

/// ori_col 解码后弃用 (工具链恒 0)
/// v8: src/wasm/wasm-module-sourcemap.cc "without further usage of it"
pub const SOURCE_COLUMN_ALWAYS_ZERO: bool = true;

/// 解码哨兵: VLQBase64Decode 失败返回 i32::MIN
/// v8: src/wasm/wasm-module-sourcemap.cc DecodeMapping
pub const VLQ_DECODE_ERROR: i32 = i32::MIN;

/// 解码后的平行数组
/// v8: src/wasm/wasm-module-sourcemap.h 私有成员
#[derive(Debug, Default)]
pub struct WasmModuleSourceMap {
    /// v8: std::vector<size_t> offsets — 生成的 wasm 偏移 (gen_col 累加)
    pub offsets: Vec<usize>,
    /// v8: std::vector<std::string> filenames — sources 数组
    pub filenames: Vec<String>,
    /// v8: std::vector<size_t> file_idxs — 每段的源文件下标
    pub file_idxs: Vec<usize>,
    /// v8: std::vector<size_t> source_row — 每段的源行号
    pub source_row: Vec<usize>,
    /// v8: bool valid_
    pub valid: bool,
}

impl WasmModuleSourceMap {
    /// 段解码状态机: 跳冗余逗号, 4 字段相对累加, 段尾必须是 ',' 或 EOF
    /// v8: src/wasm/wasm-module-sourcemap.cc DecodeMapping
    pub fn decode_mapping(&mut self, s: &str, mut vlq: impl FnMut(&[u8], &mut usize) -> i32) -> bool {
        let bytes = s.as_bytes();
        let mut pos = 0usize;
        let (mut gen_col, mut file_idx, mut ori_line) = (0i64, 0i64, 0i64);
        while pos < bytes.len() {
            if bytes[pos] == b','' {
                pos += 1;
                continue;
            }
            let mut qnt = vlq(bytes, &mut pos);
            if qnt == VLQ_DECODE_ERROR { return false; }
            gen_col += qnt as i64;
            qnt = vlq(bytes, &mut pos);
            if qnt == VLQ_DECODE_ERROR { return false; }
            file_idx += qnt as i64;
            qnt = vlq(bytes, &mut pos);
            if qnt == VLQ_DECODE_ERROR { return false; }
            ori_line += qnt as i64;
            qnt = vlq(bytes, &mut pos);
            if qnt == VLQ_DECODE_ERROR { return false; }
            if pos < bytes.len() && bytes[pos] != b','' { return false; }
            pos += 1;
            self.file_idxs.push(file_idx as usize);
            self.source_row.push(ori_line as usize);
            self.offsets.push(gen_col as usize);
        }
        true
    }

    /// 行号查询: upper_bound 后退一位
    /// v8: src/wasm/wasm-module-sourcemap.cc GetSourceLine
    pub fn get_source_line(&self, wasm_offset: usize) -> usize {
        let up = self.offsets.partition_point(|&o| o <= wasm_offset);
        self.source_row[up - 1]
    }

    /// 文件名查询
    /// v8: src/wasm/wasm-module-sourcemap.cc GetFilename
    pub fn get_filename(&self, wasm_offset: usize) -> &str {
        let up = self.offsets.partition_point(|&o| o <= wasm_offset);
        &self.filenames[self.file_idxs[up - 1]]
    }

    /// 函数区间 [start, end) 是否被任何映射覆盖
    /// v8: src/wasm/wasm-module-sourcemap.cc HasSource
    pub fn has_source(&self, start: usize, end: usize) -> bool {
        start <= *self.offsets.last().unwrap() && end > self.offsets[0]
    }

    /// addr 的最近低偏移是否仍在函数内 (跨函数映射判无效)
    /// v8: src/wasm/wasm-module-sourcemap.cc HasValidEntry
    pub fn has_valid_entry(&self, start: usize, addr: usize) -> bool {
        let up = self.offsets.partition_point(|&o| o <= addr);
        if up == 0 { return false; }
        self.offsets[up - 1] >= start
    }
}

/// 原版尺寸看板基准
/// v8: src/wasm/wasm-module-sourcemap.cc UPDATE_WHEN_CLASS_CHANGES
pub const SOURCEMAP_SIZE_BASELINE: usize = 104;

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(REQUIRED_VERSION, 3);
    assert_eq!(FIELDS_PER_SEGMENT, 4);
    assert_eq!(VLQ_DECODE_ERROR, i32::MIN);
    assert_eq!(SOURCEMAP_SIZE_BASELINE, 104);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_module_sourcemap.rs.skel", rust);
console.log(`written ${p}`);
