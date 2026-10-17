#!/usr/bin/env bun
// tool/src/codegen/external-reference.h.js
// 机械提取 v8/src/codegen/external-reference.{h,cc} —
// EXTERNAL_REFERENCE_LIST* X-macro 全量展开 (递归拼接宏 + #if 分支登记),
// StackArgumentOrder/Descriptor 枚举, kExternalIsolateIndependent* 计数锚。
// 语义面: ExternalReference 类方法实现登记跳过。
// 产物: v8-codegen-core/src/external_reference_generated.rs
//   (external_reference.rs 为手写产物, 本脚本落 _generated 旁路, 禁覆写)

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/external-reference.h";
const CC_REL = "src/codegen/external-reference.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

// —— 解析所有 #define <NAME>(V) 宏体 (含续行与 #if 分支标记) ——
const hLines = h.split("\n");
const defs = new Map(); // name -> [{cond, body: string[], else: bool}]
for (let i = 0; i < hLines.length; i++) {
  const m = hLines[i].match(/^#define (EXTERNAL_REFERENCE_LIST\w*)\(V\)/);
  if (!m) continue;
  // 向上找最近的条件指令
  let cond = "";
  let inElse = false;
  for (let j = i - 1; j >= 0; j--) {
    const c = hLines[j].match(/^#(ifdef|ifndef|if|else|endif)\b\s*(.*)/);
    if (!c) continue;
    if (c[1] === "else") {
      inElse = true;
      continue;
    }
    if (c[1] === "ifdef") cond = c[2].trim();
    else if (c[1] === "ifndef") cond = "!" + c[2].trim();
    else if (c[1] === "if") cond = c[2].trim();
    break;
  }
  const body = [];
  let j = i;
  do {
    body.push(hLines[j]);
    j++;
  } while (j < hLines.length && hLines[j - 1].trimEnd().endsWith("\\"));
  if (!defs.has(m[1])) defs.set(m[1], []);
  defs.get(m[1]).push({ cond, inElse, body });
}

// —— 递归展开: 返回 [{name, desc, cond}] ——
function expand(listName, seen = new Set()) {
  const variants = defs.get(listName);
  if (!variants) throw new Error(`未找到宏 ${listName}`);
  const out = [];
  for (const v of variants) {
    // 拼接宏体为单串, 逐 token 扫描 V(...) 与嵌套 LIST(V)
    const text = v.body
      .slice(1) // 去掉 #define NAME(V) 头行, 防自引用
      .map((l) => l.replace(/\\\s*$/, "").replace(/\/\/.*$/, ""))
      .join("\n");
    const re = /\b([A-Z][A-Z0-9_]*)\(V\)|\bV\(\s*([a-zA-Z_]\w*)\s*(?:,\s*)?/g;
    let m2;
    while ((m2 = re.exec(text)) !== null) {
      if (m2[1]) {
        // 嵌套列表宏
        if (seen.has(m2[1])) continue; // 防环
        const nested = expand(m2[1], new Set([...seen, listName]));
        for (const e of nested) out.push({ ...e, cond: v.cond || e.cond });
      } else {
        // 条目: 收集紧随其后的字符串字面量片段 (C 串拼接)
        let rest = text.slice(m2.index + m2[0].length, m2.index + m2[0].length + 400);
        const parts = [];
        const strRe = /^\s*"((?:[^"\\]|\\.)*)"/;
        let s;
        while ((s = strRe.exec(rest)) !== null) {
          parts.push(s[1]);
          rest = rest.slice(s[0].length);
        }
        out.push({ name: m2[2], desc: parts.join(""), cond: v.cond });
      }
    }
  }
  return out;
}

// 条件宏存在多份 #define 变体, 嵌套列表会被多路拼接 — 按名去重 (首现优先)
const raw = expand("EXTERNAL_REFERENCE_LIST");
const seenNames = new Set();
const entries = [];
for (const e of raw) {
  if (seenNames.has(e.name)) continue;
  seenNames.add(e.name);
  entries.push(e);
}
if (entries.length < 150) throw new Error(`EXTERNAL_REFERENCE_LIST 展开条数异常: ${entries.length} (raw ${raw.length})`);

// .cc 侧计数锚 (kExternalReferenceCountIsolateIndependent/Dependent)
const cc = readV8(CC_REL);
const indM = h.match(/static constexpr int kExternalReferenceCountIsolateIndependent =\s*EXTERNAL_REFERENCE_LIST\(COUNT_EXTERNAL_REFERENCE\);/);
const depM = h.match(/static constexpr int kExternalReferenceCountIsolateDependent =\s*EXTERNAL_REFERENCE_LIST_WITH_ISOLATE\(COUNT_EXTERNAL_REFERENCE\);/);
if (!indM || !depM) throw new Error("缺 COUNT_EXTERNAL_REFERENCE 计数宏锚");

// —— 生成 Rust ——
let rust = banner("src/codegen/external-reference.h.js", "src/codegen/external-reference.{h,cc}", "EXTERNAL_REFERENCE_LIST X-macro 全量展开");
rust += `
// 展开规则: 递归拼接 EXTERNAL_REFERENCE_LIST* 宏; 条件宏 (#ifdef 分支) 全部
// 并入并以 doc 注明触发条件, 运行时门控由 Developer 接入时裁定。
// 语义面: ExternalReference 类方法实现登记跳过。

/// 外部引用名册 (原 EXTERNAL_REFERENCE_LIST(V) 展开, ${entries.length} 条)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExternalReferenceName {
`;
for (const e of entries) {
  rust += `  /// ${e.desc || e.name}${e.cond ? ` (原 #if ${e.cond})` : ""}\n  ${e.name.replace(/(^|_)([a-z])/g, (_, p, c) => (p ? p + c.toUpperCase() : c.toUpperCase()))},\n`;
}
rust += `}

impl ExternalReferenceName {
  /// 总条数
  pub const COUNT: usize = ${entries.length};

  /// 原版描述串 (原 V(name, "desc") 的 desc, C 串拼接已合并)
  #[must_use]
  pub fn desc(self) -> &'static str {
    match self {
`;
for (let i = 0; i < entries.length; i++) {
  rust += `      Self::_Variant${i} if false => "",\n`;
}
rust += `      _ => "",
    }
  }
}

`;
// desc 表改用定长数组, 避免巨型 match
rust += `/// 原版描述串表 (与 ExternalReferenceName 判别值同序)
#[rustfmt::skip]
pub static EXTERNAL_REFERENCE_DESCS: [&str; ${entries.length}] = [
`;
for (const e of entries) {
  rust += `  ${JSON.stringify(e.desc || e.name)},\n`;
}
rust += `];

/// 条件分支登记表 (None = 无条件; Some(条件) = 原 #if 门控)
#[rustfmt::skip]
pub static EXTERNAL_REFERENCE_CONDITIONS: [Option<&str>; ${entries.length}] = [
`;
for (const e of entries) {
  rust += `  ${e.cond ? `Some(${JSON.stringify("#if " + e.cond)})` : "None"},\n`;
}
rust += `];

// 原 .cc 计数锚:
/// 独立 (共享) 外部引用数: ${indM[1]}
pub const EXTERNAL_REFERENCE_COUNT_ISOLATE_INDEPENDENT: usize = ${indM[1]};
/// 依赖 isolate 的外部引用数: ${depM[1]}
pub const EXTERNAL_REFERENCE_COUNT_ISOLATE_DEPENDENT: usize = ${depM[1]};

// —— 提取不变式断言 ——
const _: () = {
  assert!(ExternalReferenceName::COUNT == ${entries.length});
  assert!(EXTERNAL_REFERENCE_DESCS.len() == ${entries.length});
  assert!(EXTERNAL_REFERENCE_CONDITIONS.len() == ${entries.length});
  assert!(EXTERNAL_REFERENCE_DESCS[0].len() > 0);
};
`;

// 清掉占位 match (避免未用变体警告语义) — 直接以 desc 表为准
rust = rust.replace(/impl ExternalReferenceName \{[\s\S]*?\n\}\n\n/, "");

const p = writeSrc("v8-codegen-core", "external_reference_generated.rs", rust);
console.log(`written ${p} (列表 ${entries.length} 条, 条件 ${entries.filter((e) => e.cond).length})`);
