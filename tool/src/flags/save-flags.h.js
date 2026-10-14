#!/usr/bin/env bun
// v8/src/flags/{save-flags.h, save-flags.cc} → v8-flags/src/save.rs (快照/恢复基线)
// 提取规则:
//   - SaveFlags 构造: FLAG_MODE_APPLY (不带 INCLUDE_READONLY/INCLUDE_ALIASES) → 仅非只读主 flag 存值
//   - 析构: 值变化才写回 (TSAN 友好); 写回直写值, 不走冲突检查, set_by 不动
//   - RAII → Rust 显式 save/restore 对 (drop 时机由调用方掌控)
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-flags";
const GEN = "src/flags/save-flags.h.js";
const h = readV8("src/flags/save-flags.h");
const cc = readV8("src/flags/save-flags.cc");

// ---- 提取: 两个 FLAG_MODE_APPLY 用法 (存/回) ----
if (!/SAVED_##nam = v8_flags\.nam\.value\(\);/.test(cc)) throw new Error("save-flags.cc 存值宏已变");
if (!/if \(SAVED_##nam != v8_flags\.nam\.value\(\)\) \{[\s\\]*v8_flags\.nam = SAVED_##nam;/.test(cc)) throw new Error("save-flags.cc 回写宏已变");
// APPLY 无 INCLUDE_READONLY → 只覆盖非只读主 flag (flag-definitions.h 模式定义)
if (/INCLUDE_READONLY/.test(h) || /INCLUDE_ALIASES/.test(h)) throw new Error("save-flags.h 引入了 readonly/别名, 需人工核对");

const out = [
  banner(GEN, "src/flags/{save-flags.h, save-flags.cc}", "快照层: SaveFlags 的 save/restore 对 (RAII → 显式调用)"),
  `use crate::{defs, value::{FlagValue, Flags}};`,
  ``,
  `/// flag 值快照 (原版 SaveFlags; 仅非只读主 flag)`,
  `///`,
  `/// 原版为 RAII (构造存/析构回); Rust 侧由调用方掌控恢复时机:`,
  `/// \`let saved = flags.save(); ... ; flags.restore(&saved);\``,
  `#[derive(Debug)]`,
  `pub struct SavedFlags {`,
  `  /// 快照值 (索引 = defs::FLAGS 主 flag 序)`,
  `  values: Box<[Option<FlagValue>]>,`,
  `}`,
  ``,
  `impl Flags {`,
  `  /// 存当前值 (原版 SaveFlags 构造; 只读 flag 不存 — 其值恒为默认)`,
  `  pub fn save(&self) -> SavedFlags {`,
  `    SavedFlags {`,
  `      values: defs::FLAGS`,
  `        .iter()`,
  `        .enumerate()`,
  `        .map(|(i, d)| if d.readonly { None } else { Some(self.value(i as u32).clone()) })`,
  `        .collect(),`,
  `    }`,
  `  }`,
  ``,
  `  /// 恢复快照值 (原版 SaveFlags 析构; 直写值不走门禁, set_by/implied_by 保持现状)`,
  `  pub fn restore(&mut self, saved: &SavedFlags) {`,
  `    for (i, v) in saved.values.iter().enumerate() {`,
  `      // 变化才写 (原版为 TSAN 友好; 此处语义同)`,
  `      if let Some(v) = v`,
  `        && self.value(i as u32) != v {`,
  `          self.write_value(i as u32, v.clone());`,
  `      }`,
  `    }`,
  `  }`,
  `}`,
  ``,
];

writeSrc(CRATE, "save.rs", out.join("\n") + "\n");
console.log("save.rs: 非只读主 flag 快照/恢复");
