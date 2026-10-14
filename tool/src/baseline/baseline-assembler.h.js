#!/usr/bin/env bun
// tool/src/baseline/baseline-assembler.h.js
// 机械提取 src/baseline/baseline-assembler.{h,-inl.h} 的Sparkplug 汇编器: 压缩模式枚举/ scratch 栈骨架/ 其他架构跳过登记
// → v8-interpreter/src/baseline_assembler.rs
//
// 架构展开策略: x64/arm64 由 baseline_x64_assembler / baseline_arm64_assembler 模块承载;
// arm/ia32/loong64/mips64/ppc/riscv/s390 七架构的 baseline-{assembler,compiler}-<arch>-inl.h
// 按目标配置跳过 (仅 x64+arm64), 于 SKIP_REGISTRATION 登记。
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/baseline/baseline-assembler.h");
const inlSrc = readV8("src/baseline/baseline-assembler-inl.h");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "enum class CompressionMode {", "CompressionMode \u679a\u4e3e"],
  [hSrc, "class BaselineAssembler {", "BaselineAssembler"],
  [hSrc, "class ScratchRegisterScope;", "ScratchRegisterScope \u524d\u7f6e"],
  [inlSrc, "BaselineAssembler::ScratchRegisterScope scratch(assembler_);", "inl ScratchRegisterScope \u4f7f\u7528\u951a"],
]) {
  if (!src.includes(needle)) throw new Error(`结构锚缺失: ${name}`);
}

// —— 机械提取: 去注释 + 类块 + 成员声明 (对数守卫内嵌) ——
function stripComments(src) {
  return src.replace(/\/\*[\s\S]*?\*\//g, "").replace(/^\s*\/\/.*$/gm, "").replace(/^\s*#.*$/gm, "");
}
function classBlocks(src) {
  const out = [];
  const re = /\b(class|struct)\s+(?:[A-Z][A-Z_0-9]*\s+)?(\w+)[^{;]*\{/g;
  let m;
  while ((m = re.exec(src)) !== null) {
    let depth = 1, i = re.lastIndex;
    while (depth > 0 && i < src.length) {
      if (src[i] === "{") depth++;
      else if (src[i] === "}") depth--;
      i++;
    }
    out.push({ name: m[2], kind: m[1], body: src.slice(re.lastIndex, i - 1) });
  }
  return out;
}
function members(body) {
  const marked = body
    .replace(/\bpublic\s*:/g, "\x01public:")
    .replace(/\bprivate\s*:/g, "\x01private:")
    .replace(/\bprotected\s*:/g, "\x01protected:");
  const out = [];
  let vis = "private";
  let paren = 0, start = 0, i = 0;
  const emit = (seg) => {
    const visM = [...seg.matchAll(/\x01(public|private|protected):/g)];
    if (visM.length) vis = visM[visM.length - 1][1];
    seg = seg.replace(/\x01(?:public|private|protected):/g, " ").trim().replace(/\s+/g, " ");
    if (!seg) return;
    if (/^(enum|template|typedef|using|static_assert|friend)\b/.test(seg)) return;
    if (/DISALLOW|NON_EXPORTED|^#/.test(seg)) return;
    out.push({ sig: seg, vis, kind: /constexpr|static const/.test(seg) ? "const" : (/\(/.test(seg) ? "method" : "field") });
  };
  while (i < marked.length) {
    const c = marked[i];
    if (c === "(") paren++;
    else if (c === ")") paren = Math.max(0, paren - 1);
    else if (paren === 0 && c === ";") {
      emit(marked.slice(start, i));
      start = i + 1;
    } else if (paren === 0 && c === "{") {
      const sigEnd = i;
      let d = 1;
      i++;
      while (d > 0 && i < marked.length) {
        if (marked[i] === "{") d++;
        else if (marked[i] === "}") d--;
        i++;
      }
      emit(marked.slice(start, sigEnd));
      start = i;
      continue;
    }
    i++;
  }
  emit(marked.slice(start));
  return out;
}

const stripped = stripComments(hSrc);
const blocks = classBlocks(stripped);
const CLASS_TABLES = [["BaselineAssembler", "BASELINE_ASSEMBLER_DECLARATIONS", 99, 0], ["EnsureAccumulatorPreservedScope", "ENSURE_ACCUMULATOR_PRESERVED_SCOPE_DECLARATIONS", 5, 0]];
const emitTables = [];
for (const [className, constName, expected, occurrence] of CLASS_TABLES) {
  const hits = blocks.filter((b) => b.name === className);
  const idx = occurrence ?? 0;
  if (hits.length <= idx) throw new Error(`类缺失: ${className} (#${idx})`);
  const decls = members(hits[idx].body);
  if (decls.length !== expected) throw new Error(`${className} 对数不符: 期望 ${expected} 实得 ${decls.length}`);
  emitTables.push({ constName, decls, className });
}

const RUST_HEAD = `
/// 指针压缩模式 (原版 enum class CompressionMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum CompressionMode {
  /// 默认 (按构建配置)
  Default = 0,
  /// 强制解压
  ForceDecompression = 1,
}

impl CompressionMode {
  #[inline]
  #[must_use]
  pub const fn is_default(self) -> bool {
    matches!(self, Self::Default)
  }

  #[inline]
  #[must_use]
  pub const fn is_force_decompression(self) -> bool {
    matches!(self, Self::ForceDecompression)
  }
}

/// 其他架构 inl 文件跳过登记 (目标仅 x64 + arm64)
pub const SKIP_REGISTRATION: [&str; 14] = [
  "src/baseline/arm/baseline-assembler-arm-inl.h",
  "src/baseline/arm/baseline-compiler-arm-inl.h",
  "src/baseline/ia32/baseline-assembler-ia32-inl.h",
  "src/baseline/ia32/baseline-compiler-ia32-inl.h",
  "src/baseline/loong64/baseline-assembler-loong64-inl.h",
  "src/baseline/loong64/baseline-compiler-loong64-inl.h",
  "src/baseline/mips64/baseline-assembler-mips64-inl.h",
  "src/baseline/mips64/baseline-compiler-mips64-inl.h",
  "src/baseline/ppc/baseline-assembler-ppc-inl.h",
  "src/baseline/ppc/baseline-compiler-ppc-inl.h",
  "src/baseline/riscv/baseline-assembler-riscv-inl.h",
  "src/baseline/riscv/baseline-compiler-riscv-inl.h",
  "src/baseline/s390/baseline-assembler-s390-inl.h",
  "src/baseline/s390/baseline-compiler-s390-inl.h",
];
`;

let rust = banner("src/baseline/baseline-assembler.h.js", "src/baseline/baseline-assembler.{h,-inl.h}", "");

rust += RUST_HEAD;

for (const { constName, decls, className } of emitTables) {
  rust += `
/// 原版 class/struct ${className} 成员声明 (机械 1:1 提取, ${decls.length} 条)
pub const ${constName}: &[&str] = &[
`;
  for (const d of decls) rust += `  "[${d.vis}/${d.kind}] ${d.sig}",
`;
  rust += `];
`;
}

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(BASELINE_ASSEMBLER_DECLARATIONS.len() == 99);
  assert!(ENSURE_ACCUMULATOR_PRESERVED_SCOPE_DECLARATIONS.len() == 5);
  assert!(SKIP_REGISTRATION.len() == 14);
};
`;

const p = writeSrc("v8-baseline", "baseline_assembler.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
