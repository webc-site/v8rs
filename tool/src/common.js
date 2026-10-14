#!/usr/bin/env bun
// v8/src/common → v8-common 组脚本
// 规则代码的确定性转换: 枚举 / 常量表 / 位标志 / 类型别名 / X-macro 展开 / 签名骨架
// 语义逻辑留给修订阶段; 直出 v8-common/src/<模块>.rs (修订就地改, 复跑覆盖)
import { readV8, writeSrc, banner } from "../lib.js";

const CRATE = "v8-common";

// ---------------------------------------------------------------------------
// 公共工具
// ---------------------------------------------------------------------------

// 去 // 行注释与 /* */ 块注释 (字符串字面量内的 / 与引号需先保护)
export function stripComments(src) {
  let out = "";
  let i = 0;
  let inStr = false;
  let inChar = false;
  while (i < src.length) {
    const c = src[i];
    const n = src[i + 1];
    if (inStr || inChar) {
      out += c;
      if (c === "\\") {
        out += n ?? "";
        i += 2;
        continue;
      }
      if (inStr && c === '"') inStr = false;
      if (inChar && c === "'") inChar = false;
      i += 1;
      continue;
    }
    if (c === '"') {
      inStr = true;
      out += c;
      i += 1;
      continue;
    }
    if (c === "'") {
      inChar = true;
      out += c;
      i += 1;
      continue;
    }
    if (c === "/" && n === "/") {
      while (i < src.length && src[i] !== "\n") i += 1;
      continue;
    }
    if (c === "/" && n === "*") {
      i += 2;
      while (i < src.length && !(src[i] === "*" && src[i + 1] === "/")) {
        if (src[i] === "\n") out += "\n";
        i += 1;
      }
      i += 2;
      continue;
    }
    out += c;
    i += 1;
  }
  return out;
}

// 平衡括号扫描: 返回 open 后第一个平衡闭括号的索引 (含)
function matchBrace(text, open) {
  let depth = 0;
  let inStr = false;
  for (let i = open; i < text.length; i += 1) {
    const c = text[i];
    if (inStr) {
      if (c === "\\") i += 1;
      else if (c === '"') inStr = false;
      continue;
    }
    if (c === '"') inStr = true;
    else if (c === "{") depth += 1;
    else if (c === "}") {
      depth -= 1;
      if (depth === 0) return i;
    }
  }
  return -1;
}

// 提取所有 enum 块
// 返回 { name, underlying|null, isClass, body, line }
export function extractEnums(src) {
  const res = [];
  const re = /enum(?:\s+(?:class|struct))?\s+(\w+)\s*(?::\s*([\w:]+))?\s*\{/g;
  for (let m = re.exec(src); m; m = re.exec(src)) {
    const open = m.index + m[0].length - 1;
    const close = matchBrace(src, open);
    if (close < 0) continue;
    const isClass = /enum\s+(?:class|struct)/.test(m[0]);
    const line = src.slice(0, m.index).split("\n").length;
    res.push({
      name: m[1],
      underlying: m[2] ?? null,
      isClass,
      body: src.slice(open + 1, close),
      line,
    });
    re.lastIndex = close + 1;
  }
  return res;
}

// enum 体内拆顶层逗号条目 (字符串感知)
function splitTop(body, sep = ",") {
  const parts = [];
  let depth = 0;
  let inStr = false;
  let cur = "";
  for (let i = 0; i < body.length; i += 1) {
    const c = body[i];
    if (inStr) {
      cur += c;
      if (c === "\\") {
        cur += body[i + 1] ?? "";
        i += 1;
      } else if (c === '"') inStr = false;
      continue;
    }
    if (c === '"') {
      inStr = true;
      cur += c;
    } else if ("([{".includes(c)) {
      depth += 1;
      cur += c;
    } else if (")]}".includes(c)) {
      depth -= 1;
      cur += c;
    } else if (c === sep && depth === 0) {
      parts.push(cur);
      cur = "";
    } else cur += c;
  }
  if (cur.trim()) parts.push(cur);
  return parts.map((p) => p.trim()).filter(Boolean);
}

// enum 条目 → { name, value|null }
// 别名条目 (FIRST_SPACE = RO_SPACE / kLast = kAny) 归入 aliases
export function parseEnumEntries(body) {
  const variants = [];
  const aliases = [];
  for (const raw of splitTop(body)) {
    const item = raw.split("\n")[0] === raw ? raw : raw; // 条目内换行留给 value
    const m = /^(\w+)\s*(?:=\s*(.+))?$/s.exec(item);
    if (!m) continue;
    const [, name, value] = m;
    if (!value) {
      variants.push({ name, value: null });
      continue;
    }
    // 值为单一标识符 (引用其它变体) → 别名 const; true/false 是字面量非引用
    const word = /^\(?\s*([A-Za-z_]\w*)\s*\)?$/.exec(value);
    if (word && word[1] !== "true" && word[1] !== "false") {
      aliases.push({ name, expr: word[1] });
    } else {
      variants.push({ name, value: value.trim() });
    }
  }
  return { variants, aliases };
}

const RUST_PRIM = {
  int: "i32",
  bool: "bool",
  double: "f64",
  float: "f32",
  size_t: "usize",
  intptr_t: "isize",
  uintptr_t: "usize",
  Address: "usize",
  uint8_t: "u8",
  int8_t: "i8",
  uint16_t: "u16",
  int16_t: "i16",
  uint32_t: "u32",
  int32_t: "i32",
  uint64_t: "u64",
  int64_t: "i64",
};

// C++ 表达式 → Rust 表达式的机械改写
export function rewriteExpr(expr) {
  let e = expr.trim();
  e = e.replace(/size_t\{([^}]+)\}/g, "$1usize");
  e = e.replace(/uint64_t\{([^}]+)\}/g, "$1u64");
  e = e.replace(/uint32_t\{([^}]+)\}/g, "$1u32");
  e = e.replace(/int64_t\{([^}]+)\}/g, "$1i64");
  e = e.replace(/double\{([^}]+)\}/g, "($1) as f64");
  e = e.replace(/\bstatic_cast<double>\(([^)]+)\)/g, "($1) as f64");
  e = e.replace(/\bstatic_cast<intptr_t>\(([^)]+)\)/g, "($1) as isize");
  e = e.replace(/\bstd::numeric_limits<double>::infinity\(\)/g, "f64::INFINITY");
  e = e.replace(/\b0x([0-9a-fA-F]+)'([0-9a-fA-F]+)/g, "0x$1$2");
  e = e.replace(/\bu\b/g, "");
  return e;
}

export function toRustType(t) {
  if (RUST_PRIM[t]) return RUST_PRIM[t];
  return t; // 未知类型留给修订
}

// 生成一个 enum 的 Rust 草稿
export function emitEnum(en, extra = "") {
  const { variants, aliases } = parseEnumEntries(en.body);
  const repr = en.underlying ? toRustType(en.underlying) : "u8";
  const lines = [];
  lines.push(`// 原版 v8/src/common/globals.h:${en.line} enum ${en.name}`);
  if (en.underlying === "bool") {
    lines.push(`// C++ 底层类型 bool, Rust 以 u8 表示 (登记偏差)`);
  }
  lines.push(
    `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]`,
  );
  lines.push(`#[repr(${en.underlying === "bool" ? "u8" : repr})]`);
  lines.push(`pub enum ${en.name} {`);
  let idx = 0;
  const boolRepr = en.underlying === "bool";
  for (const v of variants) {
    if (v.value != null) {
      let val = rewriteExpr(v.value);
      if (boolRepr) val = val === "true" ? "1" : "0";
      lines.push(`    ${v.name} = ${val},`);
    } else if (aliases.length || variants.some((x) => x.value != null)) {
      // 前面出现显式值时 Rust 不会自动续位 — 补显式序号
      lines.push(`    ${v.name} = ${idx},`);
    } else {
      lines.push(`    ${v.name},`);
    }
    idx += 1;
  }
  lines.push(`}`);
  for (const a of aliases) {
    lines.push(`pub const ${a.name}: ${en.name} = ${en.name}::${a.expr};`);
  }
  lines.push(...extra.split("\n").filter(Boolean));
  return lines.join("\n");
}

// 提取 constexpr / const 常量声明
// 返回 { type, name, expr, line }
export function extractConsts(src) {
  const res = [];
  const re =
    /^(?:static\s+)?(?:constexpr|const)\s+(?:inline\s+)?([\w:]+(?:\s+const)?)\s+(\w+)\s*=\s*(.+?);/gm;
  for (let m = re.exec(src); m; m = re.exec(src)) {
    const line = src.slice(0, m.index).split("\n").length;
    res.push({ type: m[1], name: m[2], expr: m[3], line });
  }
  return res;
}

// 提取 X-macro #define 列表: 返回条目名数组 (V(Name) 形式)
export function extractXList(src, macroName) {
  const re = new RegExp(`#define\\s+${macroName}\\b`);
  const m = re.exec(src);
  if (!m) return null;
  const start = m.index;
  const bodyStart = src.indexOf("\n", start) + 1;
  // 宏体: 连续的以 \ 续行的文本, 直到无续行符
  let body = "";
  let i = bodyStart;
  for (;;) {
    const nl = src.indexOf("\n", i);
    const line = src.slice(i, nl < 0 ? src.length : nl);
    const cont = /\\\s*$/.test(line);
    body += `${line.replace(/\\\s*$/, "")}\n`;
    if (!cont) break;
    i = nl + 1;
  }
  const names = [];
  const vre = /\bV\((\w+)/g;
  for (let vm = vre.exec(body); vm; vm = vre.exec(body)) {
    names.push(vm[1]);
  }
  return names;
}

const hdr = (gen, v8rel, note) => banner(gen, v8rel, note) + "\n";

// ---------------------------------------------------------------------------
// checks.h → checks.h.rs
// ---------------------------------------------------------------------------
function genChecks() {
  const rel = "src/common/checks.h";
  const out = [];
  out.push(hdr("src/common.js", rel, "规则区: SLOW_DCHECK / DCHECK_* / TAG_ALIGNED 宏"));
  out.push(`// 宏 → Rust 函数/断言映射 (目标配置: 非 ENABLE_SLOW_DCHECK 分支)`);
  out.push(`// SLOW_DCHECK(c)  → 无操作 (ENABLE_SLOW_DCHECK 未定义)`);
  out.push(`// DCHECK_WITH_SANDBOX_ACCESS(c) → debug_assert!(c)`);
  out.push(`// DCHECK_TAG_ALIGNED / DCHECK_SIZE_TAG_ALIGNED → debug_assert`);
  out.push(`use crate::globals::constants::{HEAP_OBJECT_TAG_MASK, K_HEAP_OBJECT_TAG_MASK};`);
  out.push(``);
  out.push(`/// 地址按堆对象 2 字节 tag 对齐 (原 DCHECK_TAG_ALIGNED)`);
  out.push(`#[inline]`);
  out.push(`pub fn dcheck_tag_aligned(address: usize) {`);
  out.push(`  debug_assert!(address & K_HEAP_OBJECT_TAG_MASK == 0);`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 尺寸按堆对象 tag 对齐 (原 DCHECK_SIZE_TAG_ALIGNED)`);
  out.push(`#[inline]`);
  out.push(`pub fn dcheck_size_tag_aligned(size: usize) {`);
  out.push(`  debug_assert!(size & K_HEAP_OBJECT_TAG_MASK == 0);`);
  out.push(`}`);
  writeSrc(CRATE, "checks.rs", out.join("\n") + "\n");
}

// ---------------------------------------------------------------------------
// operation.h → operation.h.rs
// ---------------------------------------------------------------------------
function genOperation() {
  const rel = "src/common/operation.h";
  const src = readV8(rel);
  const clean = stripComments(src);
  const bitwise = extractXList(clean, "ARITHMETIC_BITWISE_OPERATION_LIST");
  const unary = extractXList(clean, "UNARY_OPERATION_LIST");
  const compare = extractXList(clean, "COMPARISON_OPERATION_LIST");
  const arithmetic = [
    ...extractXList(clean, "ARITHMETIC_OPERATION_LIST").filter((n) => !bitwise.includes(n)),
    ...bitwise,
  ];
  const all = [...arithmetic, ...unary, ...compare];

  const out = [];
  out.push(hdr("src/common.js", rel, "OPERATION_LIST X-macro 全量展开"));
  out.push(`/// 原版 enum class Operation : uint8_t (${all.length} 变体, 顺序 = OPERATION_LIST)`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`);
  out.push(`#[repr(u8)]`);
  out.push(`pub enum Operation {`);
  for (const n of all) out.push(`  K${n.charAt(0).toUpperCase()}${n.slice(1)},`);
  out.push(`}`);
  out.push(``);
  out.push(`impl Operation {`);
  out.push(`  /// 原版 operator<< 输出名 (C++ 拼接 #name)`);
  out.push(`  pub fn as_str(self) -> &'static str {`);
  out.push(`    match self {`);
  for (const n of all) {
    out.push(`      Self::K${n.charAt(0).toUpperCase()}${n.slice(1)} => "${n}",`);
  }
  out.push(`    }`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 IsBitwiseBinaryOperation<op>()`);
  out.push(`  pub const fn is_bitwise_binary(self) -> bool {`);
  out.push(`    matches!(self,`);
  const bw = bitwise.map((n) => `Self::K${n.charAt(0).toUpperCase()}${n.slice(1)}`);
  out.push(`      ${bw.join(" | ")})`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 IsUnaryOperation(op)`);
  out.push(`  pub const fn is_unary(self) -> bool {`);
  const un = unary.map((n) => `Self::K${n.charAt(0).toUpperCase()}${n.slice(1)}`);
  out.push(`    matches!(self, ${un.join(" | ")})`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 IsBinaryOperation(op)`);
  out.push(`  pub const fn is_binary(self) -> bool {`);
  out.push(`    !self.is_unary()`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 IsComparisonOperation(op)`);
  out.push(`  pub const fn is_comparison(self) -> bool {`);
  const cmp = compare.map((n) => `Self::K${n.charAt(0).toUpperCase()}${n.slice(1)}`);
  out.push(`    matches!(self, ${cmp.join(" | ")})`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`impl core::fmt::Display for Operation {`);
  out.push(`  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {`);
  out.push(`    f.write_str(self.as_str())`);
  out.push(`  }`);
  out.push(`}`);
  writeSrc(CRATE, "operation.rs", out.join("\n") + "\n");

  return { bitwise, unary, compare, arithmetic, all };
}

// ---------------------------------------------------------------------------
// simd128.h → simd128.h.rs
// ---------------------------------------------------------------------------
function genSimd128() {
  const rel = "src/common/simd128.h";
  const out = [];
  out.push(hdr("src/common.js", rel, "FOREACH_SIMD_TYPE X-macro 展开 + Simd128 骨架"));
  out.push(`/// FOREACH_SIMD_TYPE 展开: (Rust 类型, 转换名, lane 数)`);
  out.push(`/// C++: V(double,float64x2,f64x2,2) V(float,float32x4,f32x4,4)`);
  out.push(`///      V(int64_t,int64x2,i64x2,2) V(int32_t,int32x4,i32x4,4)`);
  out.push(`///      V(int16_t,int16x8,i16x8,8) V(int8_t,int8x16,i8x16,16)`);
  out.push(`pub type Float64x2 = [f64; 2];`);
  out.push(`pub type Float32x4 = [f32; 4];`);
  out.push(`pub type Int64x2 = [i64; 2];`);
  out.push(`pub type Int32x4 = [i32; 4];`);
  out.push(`pub type Int16x8 = [i16; 8];`);
  out.push(`pub type Int8x16 = [i8; 16];`);
  out.push(``);
  out.push(`/// 原版 class alignas(double) Simd128 — 16 字节按 f64 对齐的原始值容器`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq)]`);
  out.push(`#[repr(align(8))]`);
  out.push(`pub struct Simd128(pub [u8; 16]);`);
  out.push(``);
  out.push(`impl Simd128 {`);
  out.push(`  /// 原版 constexpr Simd128() = default → 全零`);
  out.push(`  pub const ZERO: Self = Self([0; 16]);`);
  out.push(``);
  out.push(`  /// 原版 Splat<T>(value) — 各类型 splat, 由宏生成 6 组`);
  out.push(`  pub const fn splat_f64x2(v: f64) -> Self {`);
  out.push(`    let a = v.to_bits();`);
  out.push(`    Self(bits_to_bytes64([[a, a]]))`);
  out.push(`  }`);
  out.push(`  // TODO(修订): 补齐 f32x4/i64x2/i32x4/i16x8/i8x16 的 splat 与 to_* 系列`);
  out.push(`  // TODO(修订): bytes 构造 (原 explicit Simd128(uint8_t*))`);
  out.push(`}`);
  out.push(``);
  out.push(`const fn bits_to_bytes64(v: [[u64; 2]; 1]) -> [u8; 16] {`);
  out.push(`  let mut out = [0u8; 16];`);
  out.push(`  let mut i = 0;`);
  out.push(`  while i < 2 {`);
  out.push(`    let mut b = 0;`);
  out.push(`    while b < 8 {`);
  out.push(`      out[i * 8 + b] = (v[0][i] >> (b * 8)) as u8;`);
  out.push(`      b += 1;`);
  out.push(`    }`);
  out.push(`    i += 1;`);
  out.push(`  }`);
  out.push(`  out`);
  out.push(`}`);
  writeSrc(CRATE, "simd128.rs", out.join("\n") + "\n");
}

// ---------------------------------------------------------------------------
// thread-local-storage.h → thread-local-storage.h.rs
// ---------------------------------------------------------------------------
function genThreadLocalStorage() {
  const rel = "src/common/thread-local-storage.h";
  const out = [];
  out.push(hdr("src/common.js", rel, "TLS 模型宏展开 (非 COMPONENT_BUILD 分支)"));
  out.push(`// V8_TLS_LIBRARY_MODE: COMPONENT_BUILD/V8_TLS_USED_IN_LIBRARY 时为 1, 其余 0`);
  out.push(`// Rust crate 为静态链接目标 → V8_TLS_LIBRARY_MODE = 0`);
  out.push(`// V8_TLS_MODEL 展开表:`);
  out.push(`//   library 模式            → "local-dynamic"`);
  out.push(`//   非 library + Win        → "initial-exec"`);
  out.push(`//   非 library + Android    → "local-dynamic"`);
  out.push(`//   非 library + 其它(Linux/Mac) → "local-exec"`);
  out.push(`// V8_TLS_DECLARE_GETTER / V8_TLS_DEFINE_GETTER → Rust 直接 thread_local! + 访问函数,`);
  out.push(`// library 模式差异由调用方 (L3+) 承担, 本 crate 不展开`);
  writeSrc(CRATE, "thread_local_storage.rs", out.join("\n") + "\n");
}

// ---------------------------------------------------------------------------
// high-allocation-throughput-scope.h / scoped-modification.h → 骨架
// ---------------------------------------------------------------------------
function genScopes() {
  let rel = "src/common/high-allocation-throughput-scope.h";
  let out = [
    hdr("src/common.js", rel, "签名骨架"),
    `/// 原版 HighAllocationThroughputScope — 通知 embedder 进入高吞吐 malloc/free 区段`,
    `/// 依赖 v8::Platform (include/v8-platform.h), Rust 侧以 trait 抽象`,
    `pub trait HighAllocationThroughputObserver {`,
    `  fn enter_section(&self);`,
    `  fn leave_section(&self);`,
    `}`,
    ``,
    `pub struct HighAllocationThroughputScope<'a> {`,
    `  observer: &'a dyn HighAllocationThroughputObserver,`,
    `}`,
    ``,
    `impl<'a> HighAllocationThroughputScope<'a> {`,
    `  // 原版: 构造时 LeaveSection, 析构时 EnterSection`,
    `  pub fn new(observer: &'a dyn HighAllocationThroughputObserver) -> Self {`,
    `    observer.leave_section();`,
    `    Self { observer }`,
    `  }`,
    `}`,
    ``,
    `impl Drop for HighAllocationThroughputScope<'_> {`,
    `  fn drop(&mut self) {`,
    `    self.observer.enter_section();`,
    `  }`,
    `}`,
  ].join("\n");
  writeSrc(CRATE, "high_allocation_throughput_scope.rs", out + "\n");

  rel = "src/common/scoped-modification.h";
  out = [
    hdr("src/common.js", rel, "模板类骨架"),
    `/// 原版 template ScopedModification<T> — 作用域内 *ptr = new_value, 退出时恢复`,
    `pub struct ScopedModification<'a, T> {`,
    `  ptr: &'a mut T,`,
    `  old_value: T,`,
    `}`,
    ``,
    `impl<'a, T> ScopedModification<'a, T> {`,
    `  pub fn new(ptr: &'a mut T, new_value: T) -> Self {`,
    `    let old_value = std::mem::replace(ptr, new_value);`,
    `    Self { ptr, old_value }`,
    `  }`,
    ``,
    `  pub fn old_value(&self) -> &T {`,
    `    &self.old_value`,
    `  }`,
    `}`,
    ``,
    `impl<T> Drop for ScopedModification<'_, T> {`,
    `  fn drop(&mut self) {`,
    `    *self.ptr = std::mem::take_old(&mut self.old_value);`,
    `  }`,
    `}`,
    `// TODO(修订): Drop 里的 take_old 需改为 std::mem::replace(&mut self.old_value, 语义占位)`,
    `// 或用 ManuallyDrop / MaybeUninit 实现 T: 无 Default 的恢复`,
  ].join("\n");
  writeSrc(CRATE, "scoped_modification.rs", out + "\n");
}

// ---------------------------------------------------------------------------
// assert-scope.h / .cc → assert_scope.rs
// ---------------------------------------------------------------------------
function parsePerIsolateList(clean) {
  // PER_ISOLATE_DCHECK_TYPE / PER_ISOLATE_CHECK_TYPE → (Allow, Disallow, field) 三元组
  const triples = [];
  const re = /V\((\w+),\s*(\w+),\s*(\w+),\s*enable\)/g;
  for (const name of ["PER_ISOLATE_DCHECK_TYPE", "PER_ISOLATE_CHECK_TYPE"]) {
    const m = new RegExp(`#define\\s+${name}\\b`).exec(clean);
    if (!m) continue;
    // 宏体: 反斜杠续行拼接
    let body = "";
    let i = clean.indexOf("\n", m.index) + 1;
    for (;;) {
      const nl = clean.indexOf("\n", i);
      const line = clean.slice(i, nl < 0 ? clean.length : nl);
      const cont = /\\\s*$/.test(line);
      body += `${line.replace(/\\\s*$/, "")}\n`;
      if (!cont) break;
      i = nl + 1;
    }
    for (let vm = re.exec(body); vm; vm = re.exec(body)) {
      triples.push({ allow: vm[1], disallow: vm[2], field: vm[3] });
    }
  }
  return triples;
}

function genAssertScope() {
  const relH = "src/common/assert-scope.h";
  const relCc = "src/common/assert-scope.cc";
  const src = readV8(relH);
  const clean = stripComments(src);

  // PerThreadAssertType 枚举
  const enums = extractEnums(clean);
  const perThread = enums.find((e) => e.name === "PerThreadAssertType");
  const pt = parseEnumEntries(perThread.body);
  const triples = parsePerIsolateList(clean);

  const out = [];
  out.push(hdr("src/common.js", relH, "PerThreadAssertType 枚举 + PER_ISOLATE_* X-macro 展开"));
  out.push(`/// 原版 enum PerThreadAssertType (位集下标 = 变体声明顺序)`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`);
  out.push(`#[repr(u32)]`);
  out.push(`pub enum PerThreadAssertType {`);
  for (const v of pt.variants) out.push(`  ${v.name},`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 using PerThreadAsserts = base::EnumSet<PerThreadAssertType, uint32_t>`);
  out.push(`pub type PerThreadAsserts = u32;`);
  out.push(``);
  out.push(`/// PER_ISOLATE_DCHECK_TYPE / PER_ISOLATE_CHECK_TYPE 展开`);
  out.push(`/// (Allow 类型, Disallow 类型, isolate 字段名)`);
  out.push(`pub const PER_ISOLATE_ASSERT_TYPES: [(&str, &str, &str); ${triples.length}] = [`);
  for (const t of triples) out.push(`  ("${t.allow}", "${t.disallow}", "${t.field}"),`);
  out.push(`];`);
  out.push(``);
  out.push(`/// Per-isolate 断言类别 (字段名 → 枚举, 供泛型 scope 使用)`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`);
  out.push(`#[repr(u8)]`);
  out.push(`pub enum PerIsolateAssertType {`);
  for (const t of triples) out.push(`  ${t.field.split("_").map((s) => s.charAt(0).toUpperCase() + s.slice(1)).join("")},`);
  out.push(`}`);
  out.push(``);
  out.push(`// 原版 using 清单 (PerThreadAssertScopeDebugOnly<allow, types>):`);
  const usings = [
    ["DisallowHandleAllocation", "false", ["HANDLE_ALLOCATION_ASSERT"]],
    ["AllowHandleAllocation", "true", ["HANDLE_ALLOCATION_ASSERT"]],
    ["DisallowSafepoints", "false", ["SAFEPOINTS_ASSERT"]],
    ["AllowSafepoints", "true", ["SAFEPOINTS_ASSERT"]],
    ["DisallowHeapAllocation", "false", ["HEAP_ALLOCATION_ASSERT"]],
    ["AllowHeapAllocation", "true", ["HEAP_ALLOCATION_ASSERT"]],
    ["AllowHeapAllocationInRelease", "true", ["HEAP_ALLOCATION_ASSERT"]],
    ["DisallowHandleDereference", "false", ["HANDLE_DEREFERENCE_ASSERT"]],
    ["AllowHandleDereference", "true", ["HANDLE_DEREFERENCE_ASSERT"]],
    ["AllowHandleUsageOnAllThreads", "true", ["HANDLE_USAGE_ON_ALL_THREADS_ASSERT"]],
    ["DisallowCodeDependencyChange", "false", ["CODE_DEPENDENCY_CHANGE_ASSERT"]],
    ["AllowCodeDependencyChange", "true", ["CODE_DEPENDENCY_CHANGE_ASSERT"]],
    ["DisallowCodeAllocation", "false", ["CODE_ALLOCATION_ASSERT"]],
    ["AllowCodeAllocation", "true", ["CODE_ALLOCATION_ASSERT"]],
    ["DisallowGarbageCollection", "false", ["SAFEPOINTS_ASSERT", "HEAP_ALLOCATION_ASSERT"]],
    ["DisallowGarbageCollectionInRelease", "false", ["SAFEPOINTS_ASSERT", "HEAP_ALLOCATION_ASSERT"]],
    ["DisableGCMole", "false", ["GC_MOLE"]],
    ["DisallowPositionInfoSlow", "false", ["POSITION_INFO_SLOW_ASSERT"]],
    ["AllowPositionInfoSlow", "true", ["POSITION_INFO_SLOW_ASSERT"]],
    ["AllowGarbageCollection", "true", ["SAFEPOINTS_ASSERT", "HEAP_ALLOCATION_ASSERT"]],
    ["AllowGarbageCollectionInRelease", "true", ["SAFEPOINTS_ASSERT", "HEAP_ALLOCATION_ASSERT"]],
    ["DisallowHeapAccess", "false", ["CODE_DEPENDENCY_CHANGE_ASSERT", "HANDLE_DEREFERENCE_ASSERT", "HANDLE_ALLOCATION_ASSERT", "HEAP_ALLOCATION_ASSERT"]],
    ["AllowHeapAccess", "true", ["CODE_DEPENDENCY_CHANGE_ASSERT", "HANDLE_DEREFERENCE_ASSERT", "HANDLE_ALLOCATION_ASSERT", "HEAP_ALLOCATION_ASSERT"]],
  ];
  for (const [name, allow, types] of usings) {
    const mask = types.join(" | ");
    out.push(`pub type ${name} = PerThreadAssertScope<${allow}, { ${mask} as u32 }>;`);
  }
  out.push(``);
  out.push(`// 原版 assert-scope.cc 显式实例化清单 (${triples.length} 条 per-isolate + 21 条 per-thread)`);
  out.push(`// Rust 用 const 泛型 PerThreadAssertScope<ALLOW, TYPES> 统一实现, 无需逐实例化`);
  out.push("");
  out.push("// ── 原版 assert-scope.cc ──");
  out.push("");

  // assert-scope.cc 的规则部分: kInitialValue / kClearedValue + static_assert
  const out2 = [];
  out2.push(hdr("src/common.js", relCc, "初始化常量与 static_assert 登记"));
  out2.push(`/// 原版 kInitialValue = ~PerThreadAsserts{HANDLE_USAGE_ON_ALL_THREADS_ASSERT}`);
  out2.push(`pub const K_INITIAL_VALUE: PerThreadAsserts =`);
  out2.push(`  !(1 << PerThreadAssertType::HANDLE_USAGE_ON_ALL_THREADS_ASSERT as u32);`);
  out2.push(`/// 原版 kClearedValue = PerThreadAsserts{}`);
  out2.push(`pub const K_CLEARED_VALUE: PerThreadAsserts = 0;`);
  out2.push(``);
  out2.push(`// 原版 static_assert:`);
  out2.push(`//   kInitialValue.contains(ASSERT_TYPE_IS_VALID_MARKER)`);
  out2.push(`//   !kClearedValue.contains(ASSERT_TYPE_IS_VALID_MARKER)`);
  out2.push(`//   Internals::kDisallowGarbageCollectionAlign == alignof(DisallowGarbageCollectionInRelease)`);
  out2.push(`//   Internals::kDisallowGarbageCollectionSize == sizeof(DisallowGarbageCollectionInRelease)`);
  out2.push(`// TODO(修订): 语义部分 — PerThreadAssertScope/PerIsolate scope/NoGarbageCollectionMutexGuard`);
  writeSrc(
    CRATE,
    "assert_scope.rs",
    `${out.join("\n")}\n\n${out2.join("\n")}\n`,
  );

  return { perThreadVariants: pt.variants.length, triples: triples.length, usings: usings.length };
}

// ---------------------------------------------------------------------------
// ptr-compr.h / .cc / -inl.h → 草稿
// ---------------------------------------------------------------------------
function genPtrCompr() {
  const relH = "src/common/ptr-compr.h";
  const relCc = "src/common/ptr-compr.cc";
  const relInl = "src/common/ptr-compr-inl.h";

  const out = [];
  out.push(hdr("src/common.js", relH, "签名骨架 + 配置展开"));
  out.push(`// 目标配置: V8_COMPRESS_POINTERS + V8_COMPRESS_POINTERS_IN_SHARED_CAGE`);
  out.push(`//          + V8_ENABLE_SANDBOX + V8_EXTERNAL_CODE_SPACE 开`);
  out.push(`//          V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES 关 → base_ 为进程级存储`);
  out.push(``);
  out.push(`/// 原版 V8HeapCompressionSchemeImpl<Cage> — 泛型改为 trait CompressionCage`);
  out.push(`pub trait CompressionCage {`);
  out.push(`  fn base_() -> usize;`);
  out.push(`  fn set_base_(base: usize);`);
  out.push(`}`);
  out.push(``);
  out.push(`// 原版签名清单 (V8HeapCompressionSchemeImpl<Cage>):`);
  out.push(`//   GetPtrComprCageBaseAddress(on_heap_addr) -> Address`);
  out.push(`//   GetPtrComprCageBaseAddress(cage_base: PtrComprCageBase) -> Address`);
  out.push(`//   CompressObject(tagged) -> Tagged_t`);
  out.push(`//   CompressAny(tagged) -> Tagged_t (const)`);
  out.push(`//   DecompressTaggedSigned(raw_value) -> Address`);
  out.push(`//   DecompressTagged(raw_value) -> Address`);
  out.push(`//   ProcessIntermediatePointers(raw_value, callback)`);
  out.push(`//   InitBase(base) / base() -> Address`);
  out.push(`// MainCage / TrustedCage (sandbox) / SmiCompressionScheme / ExternalCodeCompressionScheme`);
  out.push(`// ReadMaybeUnalignedValue<V> / WriteMaybeUnalignedValue<V>`);
  out.push(`// PtrComprCageAccessScope (multiple cages 配置, 目标配置 no-op)`);
  out.push(`// GetPtrComprCageBase()`);
  out.push("");
  out.push("// ── 原版 ptr-compr.cc ──");
  out.push("");

  const out2 = [];
  out2.push(hdr("src/common.js", relCc, "base_ 存储定义展开"));
  out2.push(`// THREAD_LOCAL_IF_MULTICAGE 宏: shared cage → 空 (进程级), 否则 thread_local`);
  out2.push(`// 目标配置为 shared cage: MainCage::base_ / TrustedCage::base_ /`);
  out2.push(`// ExternalCodeCompressionScheme::base_ 均为进程级, 初始 kNullAddress = 0`);
  out2.push(`// Rust: static AtomicUsize, 语义见 ptr_compr.rs`);
  out2.push("");
  out2.push("// ── 原版 ptr-compr-inl.h ──");
  out2.push("");

  const srcInl = stripComments(readV8(relInl));
  const consts = [];
  const cm = /constexpr\s+Address\s+(\w+)\s*=\s*~\(k(\w+)\s*-\s*1\)/g;
  for (let m = cm.exec(srcInl); m; m = cm.exec(srcInl)) {
    consts.push({ name: m[1], of: m[2] });
  }
  const out3 = [];
  out3.push(hdr("src/common.js", relInl, "常量 + 语义骨架"));
  out3.push(`/// 原版 ptr-compr-inl.h:${35} kPtrComprCageBaseMask = ~(kPtrComprCageBaseAlignment - 1)`);
  for (const c of consts) {
    out3.push(`pub const ${c.name}: usize = !(K_${c.of} - 1);`);
  }
  out3.push(``);
  out3.push(`// TODO(修订): 语义逻辑 — Compress/Decompress 族, ExternalCode 的 4GB 回绕,`);
  out3.push(`// ProcessIntermediatePointers 双半字展开`);
  writeSrc(
    CRATE,
    "ptr_compr.rs",
    [out, out2, out3].map((x) => x.join("\n")).join("\n\n") + "\n",
  );

  return consts;
}

// ---------------------------------------------------------------------------
// code-memory-access.h / .cc / -inl.h → 草稿
// ---------------------------------------------------------------------------
function genCodeMemoryAccess() {
  const relH = "src/common/code-memory-access.h";
  const relCc = "src/common/code-memory-access.cc";
  const relInl = "src/common/code-memory-access-inl.h";
  const src = stripComments(readV8(relH));

  const enums = extractEnums(src);
  const jitType = enums.find((e) => e.name === "JitAllocationType");
  const jt = parseEnumEntries(jitType.body);
  const srcInl = stripComments(readV8(relInl));
  const alignSz = /#define\s+THREAD_ISOLATION_ALIGN_SZ\s+(0x[0-9a-fA-F]+)/.exec(srcInl);

  const out = [];
  out.push(hdr("src/common.js", relH, "JitAllocationType 枚举 + 对齐常量 + 签名骨架"));
  out.push(`/// 原版 ThreadIsolation::JitAllocationType`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`);
  out.push(`#[repr(u8)]`);
  out.push(`pub enum JitAllocationType {`);
  for (const v of jt.variants) out.push(`  ${v.name},`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 V8_HAS_PKU_JIT_WRITE_PROTECT 分支 THREAD_ISOLATION_ALIGN_SZ`);
  out.push(`/// 非分支值为 0; PAGE 填充公式 THREAD_ISOLATION_FILL_PAGE_SZ(size) =`);
  out.push(`///   (ALIGN_SZ - (size & (ALIGN_SZ-1))) % ALIGN_SZ`);
  out.push(`pub const THREAD_ISOLATION_ALIGN_SZ: usize = ${alignSz ? alignSz[1] : "0"};`);
  out.push(``);
  out.push(`/// 原版 WritableJitAllocation::JitAllocationSource`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq)]`);
  out.push(`pub enum JitAllocationSource {`);
  out.push(`  KRegister,`);
  out.push(`  KLookup,`);
  out.push(`}`);
  out.push(``);
  out.push(`// 签名清单:`);
  out.push(`//   ThreadIsolation: Enabled/Initialize/TearDown/RegisterJitPage/UnregisterJitPage/`);
  out.push(`//     MakeExecutable/RegisterJitAllocation/RegisterInstructionStreamAllocation/`);
  out.push(`//     RegisterJitAllocations/LookupJitAllocation/LookupJumpTablePair(wasm)/`);
  out.push(`//     LookupWritableJitPage/UnregisterWasmAllocation/CanLookupStartOfJitAllocationAt/`);
  out.push(`//     StartOfJitAllocationAt/WriteProtectMemory/RegisterJitAllocationForTesting/`);
  out.push(`//     UnregisterJitAllocationForTesting/pkey/PkeyIsAvailable/initialized`);
  out.push(`//   JitPage/JitPageReference: Size/End/RegisterAllocation/LookupAllocation/Contains/`);
  out.push(`//     UnregisterAllocation/UnregisterAllocationsExcept/UnregisterRange/`);
  out.push(`//     StartOfAllocationAt/AllocationContaining/EndOfLastAllocation/Empty/Shrink/Expand/Merge`);
  out.push(`//   WritableJitAllocation: ForInstructionStream/ForNonExecutableMemory/WriteHeaderSlot*/`);
  out.push(`//     CopyCode/CopyData/WriteUnalignedValue/WriteValue/ClearBytes/UpdateWasmCodePointer(wasm)`);
  out.push(`//   WritableJitPage: LookupAllocationContaining/FreeRange/Empty`);
  out.push(`//   WritableFreeSpace: ForNonExecutableMemory/WriteHeaderSlot/ClearTagged/Address/Size/Executable`);
  out.push(`//   WritableJumpTablePair (wasm): jump_table/far_jump_table/ForTesting`);
  out.push(`//   RwxMemoryWriteScope: IsSupported/SetWritable/SetExecutable/memory_protection_key/IsPKUWritable`);
  out.push(`//   NopRwxMemoryWriteScope / RwxMemoryWriteScopeForTesting / CFIMetadataWriteScope /`);
  out.push(`//   DiscardSealedMemoryScope`);
  out.push("");
  out.push("// ── 原版 code-memory-access.cc ──");
  out.push("");

  const out2 = [];
  out2.push(hdr("src/common.js", relCc, "TrustedData 布局 + 常量"));
  out2.push(`/// 原版 ThreadIsolation::TrustedData (THREAD_ISOLATION_ALIGN 对齐)`);
  out2.push(`/// 字段: allocator_ / pkey_(pku) / jit_pages_mutex_ / jit_pages_ / initialized_`);
  out2.push(`/// Rust: Mutex<BTreeMap> 合并 mutex_+map 两字段, 语义等价 (登记)`);
  out2.push(`pub const K_SPLIT_THRESHOLD: usize = 0x40000; // RegisterJitAllocations 拆页阈值`);
  out2.push(``);
  out2.push(`// TODO(修订): 语义逻辑 — Lookup/Split/UnregisterJitPage 的页合并拆分, CheckForRegionOverlap`);
  out2.push("");
  out2.push("// ── 原版 code-memory-access-inl.h ──");
  out2.push("");

  const out3 = [];
  out3.push(hdr("src/common.js", relInl, "平台分支登记"));
  out3.push(`// RwxMemoryWriteScope 三平台分支:`);
  out3.push(`//   V8_HAS_PTHREAD_JIT_WRITE_PROTECT (macOS arm64): SetJitWriteProtected(0/1)`);
  out3.push(`//   V8_HAS_BECORE_JIT_WRITE_PROTECT (iOS 17.4+): be_memory_inline_jit_*`);
  out3.push(`//   V8_HAS_PKU_JIT_WRITE_PROTECT (Linux x64): pkey 权限切换`);
  out3.push(`//   其余: no-op`);
  out3.push(`// Rust: cfg(target_os/target_arch) + 运行时 pkey 探测, 见 code_memory_access.rs`);
  out3.push(`// WritableJitAllocation::WriteHeaderSlot 族依赖 TaggedField/HeapObject → L3 延后`);
  writeSrc(
    CRATE,
    "code_memory_access.rs",
    [out, out2, out3].map((x) => x.join("\n")).join("\n\n") + "\n",
  );

  return { jitTypeVariants: jt.variants.length };
}

// ---------------------------------------------------------------------------
// synchronization-point-support.h / .cc → 草稿
// ---------------------------------------------------------------------------
function genSyncPoint() {
  let rel = "src/common/synchronization-point-support.h";
  let out = [
    hdr("src/common.js", rel, "签名骨架"),
    `/// 原版 SynchronizationPointSupport 单例 — 测试/fuzzer 用的同步点阻塞`,
    `/// 字段: any_requested_(atomic bool) / mutex_ / states_(map<string, SyncPointState>)`,
    `/// SyncPointState: cv / block_requested / blocked_threads / block_requester_thread / block_timeout`,
    `/// 方法: Initialize/Get/RequestBlockAt/Resume/WaitUntilBlocked/BlockIfRequested(Slow)`,
    `/// SYNCHRONIZATION_POINT 宏 → block_if_requested(name)`,
    `/// SYNCHRONIZATION_POINT_TEST_ONLY: V8_ENABLE_TEST_ONLY_SYNC_POINTS 开才生效,`,
    `///   目标配置关 → no-op (feature 登记: test_only_sync_points)`,
  ].join("\n");
  const hOut = `${out}\n\n// ── 原版 synchronization-point-support.cc ──\n`;

  rel = "src/common/synchronization-point-support.cc";
  out = [
    hdr("src/common.js", rel, "规则部分: 无常量/枚举, 全为语义逻辑"),
    `// TODO(修订): 全部语义 — RequestBlockAt/Resume/WaitUntilBlocked/BlockIfRequestedSlow`,
    `// 原版 base::TimeDelta/TimeTicks → std::time::Instant/Duration; OS::PrintError → log::error!`,
    `// LocalHeap::ExecuteWhileParked 分支依赖 heap 层 → L3 接入点 (本 crate 走直等路径)`,
  ].join("\n");
  writeSrc(CRATE, "synchronization_point_support.rs", hOut + out + "\n");
}

// ---------------------------------------------------------------------------
// message-template.h → message_template.rs
// ---------------------------------------------------------------------------
// 解析 MESSAGE_TEMPLATES(T) 宏: T(Name, "..." "...") 序列
export function parseMessageTemplates(src) {
  const clean = stripComments(src);
  const start = clean.indexOf("#define MESSAGE_TEMPLATES(T)");
  const end = clean.indexOf("enum class MessageTemplate");
  const body = clean.slice(start, end);

  const entries = [];
  let i = body.indexOf("T(");
  while (i >= 0) {
    const nameM = /^T\((\w+),/.exec(body.slice(i));
    if (!nameM) {
      i = body.indexOf("T(", i + 2);
      continue;
    }
    const name = nameM[1];
    let j = i + nameM[0].length;
    // 扫描到平衡右括号, 字符串感知
    let depth = 1;
    let inStr = false;
    const argStart = j;
    while (j < body.length && depth > 0) {
      const c = body[j];
      if (inStr) {
        if (c === "\\") j += 1;
        else if (c === '"') inStr = false;
      } else if (c === '"') inStr = true;
      else if (c === "(") depth += 1;
      else if (c === ")") depth -= 1;
      j += 1;
    }
    const arg = body.slice(argStart, j - 1).trim();
    let template;
    if (arg.includes("v8_flags")) {
      // T(TypedArrayValidateErrorOperation, (cond ? "A" : "B")) — flag 默认 false → 取 B
      const segs = [...arg.matchAll(/"((?:[^"\\]|\\.)*)"/g)].map((m) => m[1]);
      template = segs[segs.length - 1];
    } else {
      template = [...arg.matchAll(/"((?:[^"\\]|\\.)*)"/g)].map((m) => m[1]).join("");
    }
    entries.push({ name, template });
    i = body.indexOf("T(", j);
  }
  return entries;
}

function genMessageTemplate() {
  const rel = "src/common/message-template.h";
  const entries = parseMessageTemplates(readV8(rel));
  const out = [];
  out.push(hdr("src/common.js", rel, `MESSAGE_TEMPLATES 宏全量展开: ${entries.length} 个模板`));
  out.push(`/// 原版 enum class MessageTemplate (k##NAME, 含 kMessageCount 哨兵)`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]`);
  out.push(`#[repr(u16)]`);
  out.push(`pub enum MessageTemplate {`);
  for (const e of entries) out.push(`  K${e.name},`);
  out.push(`}`);
  out.push(``);
  out.push(`/// kMessageCount 哨兵值`);
  out.push(`pub const K_MESSAGE_COUNT: u16 = ${entries.length};`);
  out.push(``);
  out.push(`/// 模板字符串表, 下标 = 判别值 (原版 message-template 由 % 填充)`);
  out.push(`pub const MESSAGE_TEMPLATE_STRINGS: [&str; ${entries.length}] = [`);
  for (const e of entries) {
    // C++ 转义与 Rust 兼容性检查: \\ \" \n 均一致
    out.push(`  "${e.template}",`);
  }
  out.push(`];`);
  out.push(``);
  out.push(`/// 原版 MessageTemplateFromInt(message_id) — DCHECK 越界改 Option (登记)`);
  out.push(`pub fn message_template_from_int(id: i32) -> Option<MessageTemplate> {`);
  out.push(`  if id < 0 || id >= K_MESSAGE_COUNT as i32 {`);
  out.push(`    return None;`);
  out.push(`  }`);
  out.push(`  // SAFETY: u16 判别值已在上方校验`);
  out.push(`  Some(unsafe { std::mem::transmute::<u16, MessageTemplate>(id as u16) })`);
  out.push(`}`);
  out.push(``);
  out.push(`impl core::fmt::Display for MessageTemplate {`);
  out.push(`  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {`);
  out.push(`    f.write_str(MESSAGE_TEMPLATE_STRINGS[*self as usize])`);
  out.push(`  }`);
  out.push(`}`);
  writeSrc(CRATE, "message_template.rs", out.join("\n") + "\n");
  return entries;
}

// ---------------------------------------------------------------------------
// globals.h → globals.h.rs
// ---------------------------------------------------------------------------
// 目标配置表: x64+arm64, COMPRESS_POINTERS + SHARED_CAGE + SANDBOX + EXTERNAL_CODE_SPACE 开
const CONFIG_BOOLS = [
  ["USE_SIMULATOR", false, "目标架构 = 宿主架构, 不模拟"],
  ["V8_EMBEDDED_CONSTANT_POOL", false, "仅 PPC64"],
  ["DEBUG", false, "release 构建"],
  ["V8_MAP_PACKING", false, "实验特性"],
  ["COMPRESS_POINTERS", true, "目标配置开"],
  ["V8_ENABLE_GEARBOX", false, "V8_BUILTINS_GEARBOX 未定义"],
  ["DECOMPRESS_POINTER_BY_ADDRESSING_MODE", true, "COMPRESS_POINTERS && x64 (arm64 false, 见 cfg)"],
  ["COMPRESS_POINTERS_IN_SHARED_CAGE", true, "sandbox 要求 shared cage"],
  ["CONTIGUOUS_COMPRESSED_READ_ONLY_SPACE", false, "实验特性"],
  ["COMPRESS_POINTERS_IN_MULTIPLE_CAGES", false, "shared cage 的补集"],
  ["V8_CAN_CREATE_SHARED_HEAP", true, "V8_DISABLE_WRITE_BARRIERS 未定义"],
  ["V8_LOWER_LIMITS_MODE", false, "默认关"],
  ["V8_STATIC_ROOTS_GENERATION", false, "仅构建期"],
  ["V8_STATIC_DISPATCH_HANDLES", true, "COMPRESS_POINTERS 开"],
  ["V8_ENABLE_SANDBOX", true, "目标配置开"],
  ["V8_JS_LINKAGE_INCLUDES_DISPATCH_HANDLE", true, "随 sandbox"],
  ["ENABLE_CONTROL_FLOW_INTEGRITY", false, "仅 arm64 + 开关, 默认关"],
  ["V8_ENABLE_NEAR_CODE_RANGE", true, "EXTERNAL_CODE_SPACE 开"],
  ["V8_ENABLE_SWISS_NAME_DICTIONARY", true, "默认开 (v8_enable_swiss_name_dictionary)"],
  ["V8_DICT_PROPERTY_CONST_TRACKING", false, "实验特性"],
  ["V8_EXTERNAL_CODE_SPACE", true, "sandbox 要求"],
  ["V8_JUMP_TABLE_INFO", false, "ENABLE_DISASSEMBLER 未开"],
  ["V8_HEAP_USE_PTHREAD_JIT_WRITE_PROTECT", null, "cfg: macOS arm64 宿主 (HAS && !(CP && !ECS))"],
  ["V8_HEAP_USE_BECORE_JIT_WRITE_PROTECT", null, "cfg: iOS arm64 宿主"],
  ["V8_HEAP_USE_PKU_JIT_WRITE_PROTECT", null, "cfg: Linux x64 宿主 (HAS && !(CP && !ECS))"],
  ["TAGGED_SIZE_8_BYTES", false, "COMPRESS_POINTERS 开"],
  ["V8_OS_WIN_X64", null, "cfg: 目标 Windows x64"],
  ["V8_OS_WIN_ARM64", null, "cfg: 目标 Windows arm64"],
  ["V8_OS_WIN64", null, "cfg: 上两者之一"],
  ["V8_ENABLE_FP_PARAMS_IN_C_LINKAGE", true, "x64/arm64 均在列表"],
  ["V8_UNDEFINED_DOUBLE", false, "实验特性"],
  ["V8_EXPERIMENTAL_TSA_BUILTINS", false, "实验特性"],
  ["V8_EXPERIMENTAL_TQ_TO_TSA", false, "实验特性"],
  ["V8_X64_16BYTE_STACK_ALIGNMENT", false, "默认关"],
  ["V8_COMPRESS_POINTERS_8GB", false, "默认关"],
  ["USES_FUNCTION_DESCRIPTORS", false, "仅 PPC64/AIX"],
];

function genGlobals() {
  const rel = "src/common/globals.h";
  const src = readV8(rel);
  const clean = stripComments(src);
  const enums = extractEnums(clean);
  const out = [];

  out.push(hdr("src/common.js", rel, "配置布尔表 + 枚举全量 + 常量清单"));

  out.push(`// ── 配置布尔 (目标配置: x64+arm64, COMPRESS_POINTERS/SANDBOX/SHARED_CAGE/EXTERNAL_CODE_SPACE 开)`);
  out.push(`// cfg 条目由修订阶段展开为 cfg! 或 const; null = 平台相关`);
  for (const [name, val, why] of CONFIG_BOOLS) {
    out.push(`pub const ${name}_BOOL: bool = ${val === null ? "/* cfg */ false" : val}; // ${why}`);
  }
  out.push("");

  // 枚举全量
  const skipped = [];
  for (const en of enums) {
    // 引用外部符号的枚举先跳过, 修订阶段补
    const external = /Internals|kSmiValueSize|kSmiShiftSize|kSmiTagSize|Tagged_t|kTaggedSize|JSDispatchHandle|kJSDispatchHandleShift|MB|GB/.test(en.body);
    if (external) {
      skipped.push(en.name);
      continue;
    }
    // StackFrameId 值为 kMinInt/kMaxInt — 非 enum 语义, 转 i32 类型别名 + 常量
    if (en.name === "StackFrameId") {
      out.push(`// 原版 v8/src/common/globals.h:${en.line} enum StackFrameId — int 型 ID 空间, 转 i32`);
      out.push(`pub type StackFrameId = i32;`);
      out.push(`pub const ID_MIN_VALUE: StackFrameId = i32::MIN;`);
      out.push(`pub const ID_MAX_VALUE: StackFrameId = i32::MAX;`);
      out.push(`pub const NO_ID: StackFrameId = 0;`);
      out.push("");
      continue;
    }
    // enum 前注释行保留原位引用
    out.push(emitEnum(en));
    out.push("");
  }

  out.push(`// ── 依赖 include/v8-internal.h 的枚举 (修订阶段结合 internal 常量展开): ${skipped.join(", ")}`);
  out.push("");

  // 常量清单 (顶层 constexpr, 无条件编译区内的)
  const consts = extractConsts(clean);
  out.push(`// ── constexpr 常量清单 (${consts.length} 条, 机械转写; 依赖 KB/MB/GB 由 internal 模块提供)`);
  for (const c of consts) {
    out.push(`// ${rel}:${c.line} ${c.type} ${c.name} = ${c.expr}`);
  }
  out.push("");

  // X-macro 清单
  out.push(`// ── X-macro 清单`);
  const lists = ["FOREACH_WASM_TRAPREASON", "BINARY_OPERATION_FEEDBACK_TYPES", "COMPARE_OPERATION_FEEDBACK_TYPES"];
  for (const l of lists) {
    const names = extractXList(clean, l);
    out.push(`// ${l}: ${names ? names.join(", ") : "未找到"}`);
  }

  writeSrc(CRATE, "globals.rs", out.join("\n") + "\n");
  return { enums: enums.length, skipped, consts };
}

// ---------------------------------------------------------------------------
// 主流程
// ---------------------------------------------------------------------------
const stats = {};
genChecks();
stats.operation = genOperation();
genSimd128();
genThreadLocalStorage();
genScopes();
stats.assert = genAssertScope();
stats.ptrCompr = genPtrCompr();
stats.codeMemory = genCodeMemoryAccess();
genSyncPoint();
stats.templates = genMessageTemplate();
stats.globals = genGlobals();

console.log("== v8-common src 直出完毕 ==");
console.log(
  `operation: ${stats.operation.all.length} 变体 (bitwise=${stats.operation.bitwise.length} unary=${stats.operation.unary.length} compare=${stats.operation.compare.length})`,
);
console.log(`message-template: ${stats.templates.length} 模板`);
console.log(`assert-scope: ${stats.assert.perThreadVariants} per-thread 类型, ${stats.assert.triples} per-isolate 三元组, ${stats.assert.usings} using 别名`);
console.log(`ptr-compr mask 常量: ${stats.ptrCompr.length}`);
console.log(`code-memory: JitAllocationType ${stats.codeMemory.jitTypeVariants} 变体`);
console.log(`globals: ${stats.globals.enums} 枚举 (跳过依赖内部的 ${stats.globals.skipped.length}: ${stats.globals.skipped.join(",")}), ${stats.globals.consts.length} 常量`);
