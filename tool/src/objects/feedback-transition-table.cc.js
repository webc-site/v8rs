#!/usr/bin/env bun
// tool/src/objects/feedback-transition-table.cc.js — 覆盖 v8/src/objects/feedback-transition-table.cc
//   (类声明位于 src/common/globals.h, 一并由 -E oracle 提取)
//   → v8-objects-js-func/src/feedback_transition_table.rs
// 规则代码: BinaryOperationFeedback / CompareOperationFeedback
//   Type 值表 + TypeIndex 顺序表 + transition map (编译期真实现) + feedback encode lut。

import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { root, writeSrc, banner } from "../../lib.js";

// ── clang -E oracle (编译器产物, 非脚本自身解析) ──────────────────────────
// stub 头 (absl 等三方缺失) 自动补空文件迭代; 结果缓存 target/agent_R3O6/clang-E/。
const STUB = join(root, "target", "agent_R3O6", "clang-stub");
const CACHE = join(root, "target", "agent_R3O6", "clang-E");
function clangE(rel) {
  const cache = join(CACHE, rel.replaceAll("/", "_") + ".i");
  if (existsSync(cache)) return readFileSync(cache, "utf8");
  mkdirSync(STUB, { recursive: true });
  mkdirSync(CACHE, { recursive: true });
  for (let i = 0; i < 96; i++) {
    const r = spawnSync(
      "clang",
      ["-E", "-x", "c++", "-std=c++20", "-I", "v8", "-I", "v8/include", "-I", STUB, "-P", join("v8", rel)],
      { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 },
    );
    const miss = r.stderr?.match(/fatal error: '([^']+)' file not found/);
    if (!miss) {
      if (r.status !== 0) throw new Error(`clang -E ${rel} 失败: ${r.stderr?.slice(0, 400)}`);
      writeFileSync(cache, r.stdout);
      return r.stdout;
    }
    const p = join(STUB, miss[1]);
    mkdirSync(dirname(p), { recursive: true });
    writeFileSync(p, "");
  }
  throw new Error(`clang -E ${rel}: stub 迭代超限`);
}

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

const iSrc = clangE("src/objects/feedback-transition-table.cc");

// ── 1. 从 -E 产物提取 Type 枚举值 (宏已展开, 值为字面量或 flag 表达式) ────
function enumBody(src, cls) {
  const at = src.indexOf(`class ${cls} : public AllStatic`);
  must(at >= 0, `-E 产物缺 class ${cls}`);
  const e = src.indexOf("enum Type {", at);
  must(e >= 0, `${cls} 缺 enum Type`);
  const depthEnd = src.indexOf("};", e);
  return src.slice(e + "enum Type {".length, depthEnd);
}

// 求值器: 0x.. | 1 << n | 标识符 | a | b (标识符查 env; 先剥 static_cast<..>(..))
function evalConst(expr, env) {
  const s = expr
    .replace(/static_cast<[^>]+>\(/g, "(")
    .replace(/\s+/g, "")
    .replace(/\(/g, "(").replace(/\)/g, ")");
  const toks = s.match(/[0-9a-zA-Z_]+|<<|\||\(|\)/g) ?? [];
  const vals = [];
  const ops = [];
  const apply = (b, a, op) => (op === "|" ? a | b : a << b);
  for (const t of toks) {
    if (t === "|" || t === "<<") ops.push(t);
    else if (t === "(") ops.push(t);
    else if (t === ")") {
      while (ops.length && ops.at(-1) !== "(") {
        const op = ops.pop();
        vals.push(apply(vals.pop(), vals.pop(), op));
      }
      ops.pop();
    } else if (/^0x/i.test(t)) vals.push(parseInt(t, 16));
    else if (/^\d+$/.test(t)) vals.push(parseInt(t, 10));
    else {
      if (!(t in env)) throw new Error(`求值缺符号 ${t} (表达式 "${expr}")`);
      vals.push(env[t]);
    }
  }
  while (ops.length) {
    const op = ops.pop();
    vals.push(apply(vals.pop(), vals.pop(), op));
  }
  if (vals.length !== 1) throw new Error(`求值栈异常: "${expr}" → ${vals}`);
  return vals[0] >>> 0;
}

function parseTypes(cls) {
  const body = enumBody(iSrc, cls);
  // 条目: kName = expr, (允许尾逗号/空行)
  const entries = body
    .split(",")
    .map((x) => x.trim())
    .filter(Boolean)
    .map((x) => {
      const m = x.match(/^(\w+)\s*=\s*(.+)$/);
      if (!m) throw new Error(`${cls} Type 条目异常: "${x}"`);
      return [m[1], m[2].trim()];
    });
  // 匿名 flag enum (Compare 侧): kXxxFlag = 1 << n / kAnyMask
  const env = {};
  const flagAt = iSrc.indexOf(`class ${cls} : public AllStatic`);
  const fEnum = iSrc.slice(flagAt, flagAt + 4000).match(/enum \{([\s\S]*?)\};/);
  if (fEnum) {
    for (const [name, expr] of fEnum[1].split(",").map((x) => x.trim()).filter(Boolean)
      .map((x) => [x.match(/^(\w+)\s*=/)?.[1], x.match(/=\s*(.+)$/)?.[1]])) {
      if (name && expr) env[name] = evalConst(expr, env);
    }
  }
  // Type 条目可引用先前条目 (kOddball = kBoolean | kNullOrUndefined) → 增量入 env
  const values = entries.map(([name, expr]) => {
    const v = evalConst(expr, env);
    env[name] = v;
    return [name, v];
  });
  // TypeIndex 顺序 (X-macro 展开后的明文)
  const tiAt = iSrc.indexOf("enum class TypeIndex : uint8_t {", flagAt);
  must(tiAt >= 0, `${cls} 缺 TypeIndex`);
  const tiBody = iSrc.slice(tiAt, iSrc.indexOf("};", tiAt)).split("kFirstTypeIndex")[0];
  const order = tiBody.match(/k\w+(?=[,\s])/g) ?? [];
  return { values, order };
}

const bin = parseTypes("BinaryOperationFeedback");
const cmp = parseTypes("CompareOperationFeedback");
must(bin.values.length === 12, `Binary Type 条数 ${bin.values.length} != 12`);
must(cmp.values.length === 17, `Compare Type 条数 ${cmp.values.length} != 17`);
must(bin.order.length === 12 && cmp.order.length === 17, `TypeIndex 顺序表 ${bin.order.length}/${cmp.order.length} != 12/17`);
must(JSON.stringify(bin.order) === JSON.stringify(bin.values.map(([n]) => n)), "Binary TypeIndex 顺序与 Type 声明序不一致");
must(JSON.stringify(cmp.order) === JSON.stringify(cmp.values.map(([n]) => n)), "Compare TypeIndex 顺序与 Type 声明序不一致");
must(bin.values[0][0] === "kNone" && bin.values.at(-1)[0] === "kAny", "Binary 首尾条目应 kNone/kAny");
must(bin.values.at(-1)[1] === 0x1ff, `Binary kAny ${bin.values.at(-1)[1]} != 0x1FF`);
must(cmp.values.at(-1)[1] === 0x3ff, `Compare kAny ${cmp.values.at(-1)[1]} != 0x3FF`);
must(bin.values[4][1] === 0xf && bin.values[10][1] === 0x180, "Binary 关键值 kNumber=0xF/kStringOrStringWrapper=0x180 失配");
must(cmp.values[7][1] === 0xf && cmp.values[9][1] === 0x30 && cmp.values[13][1] === 0x80 && cmp.values[14][1] === 0x180 && cmp.values[15][1] === 0x40, "Compare 关键值失配");

// kTransitionMapStride = bit_ceil(N): 12→16, 16→16
const bitCeil = (n) => {
  let c = 1;
  while (c < n) c <<= 1;
  return c;
};
must(bitCeil(12) === 16 && bitCeil(17) === 32, "bit_ceil 失配");

// ── 2. Rust 产物 ──────────────────────────────────────────────────────────
const snake = (k) => k.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

function emitFamily(mod, docName, fam, prefix) {
  const [names, vals] = [fam.values.map(([n]) => n), fam.values.map(([, v]) => v)];
  const n = vals.length;
  const stride = bitCeil(n);
  const lutLen = vals.at(-1) + 1;
  return `// ── ${docName} ── ${n} 类型索引, stride ${stride}, lut ${lutLen} 项 ──
pub const ${prefix}_TYPE_NAMES: [&str; ${n}] = [
${names.map((k) => `  "${k.slice(1)}",`).join("\n")}
];
pub const ${prefix}_TYPE_VALUES: [u32; ${n}] = [
  ${vals.map((v) => `0x${v.toString(16)}`).join(", ")},
];

/// TypeIndex → Type 值 (原版 DecodeTypeIndex)
#[inline]
#[must_use]
pub const fn ${mod}_decode_type_index(index: usize) -> u32 {
  ${prefix}_TYPE_VALUES[index]
}

/// feedback 值 → 最小覆盖 TypeIndex (原版 CalculateTypeIndex, X-macro 序首个覆盖者)
#[inline]
#[must_use]
pub const fn ${mod}_calculate_type_index(feedback_value: u32) -> u8 {
  let mut i = 0;
  while i < ${prefix}_TYPE_VALUES.len() {
    if ${prefix}_TYPE_VALUES[i] & feedback_value == feedback_value {
      return i as u8;
    }
    i += 1;
  }
  (${prefix}_TYPE_VALUES.len() - 1) as u8
}

/// 双反馈合成 (原版 CombineTypeIndex = CalculateTypeIndex(a|b))
#[inline]
#[must_use]
pub const fn ${mod}_combine_type_index(a: usize, b: usize) -> u8 {
  ${mod}_calculate_type_index(${prefix}_TYPE_VALUES[a] | ${prefix}_TYPE_VALUES[b])
}

const fn ${mod}_build_transition_map() -> [[u8; ${stride}]; ${n}] {
  let mut m = [[0u8; ${stride}]; ${n}];
  let mut i = 0;
  while i < ${n} {
    let mut j = 0;
    while j < ${n} {
      m[i][j] = ${mod}_combine_type_index(i, j);
      j += 1;
    }
    i += 1;
  }
  m
}

/// 原版 kTransitionMap (u8 表, 编译期与原版 constexpr BuildTransitionMap 同算法)
pub static ${prefix}_TRANSITION_MAP: [[u8; ${stride}]; ${n}] = ${mod}_build_transition_map();

const fn ${mod}_build_encode_table() -> [u8; ${lutLen}] {
  let mut lut = [0u8; ${lutLen}];
  let mut v = 0;
  while v < ${lutLen} {
    lut[v] = ${mod}_calculate_type_index(v as u32);
    v += 1;
  }
  lut
}

/// 原版 kFeedbackEncodeTable (0..=kAny 全值反查表)
pub static ${prefix}_FEEDBACK_ENCODE_TABLE: [u8; ${lutLen}] = ${mod}_build_encode_table();

/// v8: src/objects/feedback-transition-table.cc ${docName}::GetTransitionMapAddress
///   — 表首地址供生成代码侧读取
#[inline]
#[must_use]
pub fn ${mod}_transition_map_address() -> usize {
  ${prefix}_TRANSITION_MAP.as_ptr() as usize
}

/// v8: src/objects/feedback-transition-table.cc ${docName}::GetFeedbackEncodeTableAddress
#[inline]
#[must_use]
pub fn ${mod}_feedback_encode_table_address() -> usize {
  ${prefix}_FEEDBACK_ENCODE_TABLE.as_ptr() as usize
}

`;
}

const asserts = `// ── 对数不变式 (oracle: clang -E 展开后 globals.h 明文) ──
const _: () = {
  assert!(BINARY_TYPE_VALUES[0] == 0x0);
  assert!(BINARY_TYPE_VALUES[2] == 0x3); // kSignedSmallInputs
  assert!(BINARY_TYPE_VALUES[5] == 0x1f); // kNumberOrOddball
  assert!(BINARY_TYPE_VALUES[8] == 0x80); // kString
  assert!(BINARY_TYPE_VALUES[BINARY_TYPE_VALUES.len() - 1] == 0x1ff); // kAny
  assert!(COMPARE_TYPE_VALUES[0] == 0x0);
  assert!(COMPARE_TYPE_VALUES[1] == 0x4); // kBoolean
  assert!(COMPARE_TYPE_VALUES[2] == 0x8); // kNullOrUndefined
  assert!(COMPARE_TYPE_VALUES[3] == 0xc); // kOddball = Boolean|NullOrUndefined
  assert!(COMPARE_TYPE_VALUES[4] == 0x1); // kSignedSmall
  assert!(COMPARE_TYPE_VALUES[5] == 0x3); // kNumber
  assert!(COMPARE_TYPE_VALUES[6] == 0x7); // kNumberOrBoolean
  assert!(COMPARE_TYPE_VALUES[9] == 0x30); // kString = Internalized|OtherString
  assert!(COMPARE_TYPE_VALUES[13] == 0x80); // kBigInt64
  assert!(COMPARE_TYPE_VALUES[14] == 0x180); // kBigInt
  assert!(COMPARE_TYPE_VALUES[15] == 0x40); // kSymbol
  assert!(COMPARE_TYPE_VALUES[COMPARE_TYPE_VALUES.len() - 1] == 0x3ff); // kAny
  // transition map 抽样: combine(kSignedSmall,kNumber)=kNumber(4/5), combine(kNone,x)=x
  assert!(binary_combine_type_index(4, 5) == 5);
  assert!(binary_combine_type_index(0, 11) == 11);
  assert!(cmp_combine_type_index(0, 16) == 16);
  assert!(cmp_combine_type_index(4, 6) == 6); // SignedSmall|NumberOrBoolean → NumberOrBoolean
  assert!(cmp_combine_type_index(2, 9) == 10); // NullOrUndefined|String → StringOrOddball
  assert!(BINARY_FEEDBACK_ENCODE_TABLE[0x1ff] == 11);
  assert!(BINARY_FEEDBACK_ENCODE_TABLE[0x3] == 2);
  assert!(COMPARE_FEEDBACK_ENCODE_TABLE[0x3ff] == 16);
  assert!(COMPARE_FEEDBACK_ENCODE_TABLE[0x7] == 6);
  assert!(BINARY_TRANSITION_MAP.len() == 12 && BINARY_TRANSITION_MAP[0].len() == 16);
  assert!(COMPARE_TRANSITION_MAP.len() == 17 && COMPARE_TRANSITION_MAP[0].len() == 32);
};
`;

const body = `${banner("src/objects/feedback-transition-table.cc.js", "src/objects/feedback-transition-table.cc + src/common/globals.h (Binary/CompareOperationFeedback)", "反馈类型转移表与编码表, 编译期真实现")}
//! BinaryOperationFeedback / CompareOperationFeedback: 类型反馈格转移表。
//! 反馈按位或合成, 走向更泛化类型; 表与原版 constexpr BuildTransitionMap 同构。

${emitFamily("binary", "BinaryOperationFeedback", bin, "BINARY")}${emitFamily("cmp", "CompareOperationFeedback", cmp, "COMPARE")}${asserts}`;

const p = writeSrc("v8-objects-js-func", "feedback_transition_table.rs", body);
console.log(
  `feedback-transition-table.cc.js: ${p} — Binary Type ${bin.values.length}/12 (kAny=0x${bin.values.at(-1)[1].toString(16)})` +
    `, Compare Type ${cmp.values.length}/17 (kAny=0x${cmp.values.at(-1)[1].toString(16)}), stride 16/32, lut ${bin.values.at(-1)[1] + 1}/${cmp.values.at(-1)[1] + 1}, 断言 ${checks} 项通过`,
);
