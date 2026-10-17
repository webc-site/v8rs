#!/usr/bin/env bun
// tool/src/codegen/x64/constants-x64.h.js — v8/src/codegen/x64/constants-x64.h → v8-codegen-x64/src/constants.rs
//
// 逐常量提取: `constexpr <type> <name> = <value>;`
//   结构锚: 头文件卫哨 / include globals.h / namespace 包裹; 缺项必 throw。
//   金标准: 常量集合与值以本脚本内置对数表核对 C++ 头 (2 项, 零遗漏)。
// 复跑幂等; 修订就地改 src/constants.rs, 复跑覆盖基线。
import { spawnSync } from "node:child_process";
import { readV8, writeSrc, banner, root } from "../../../lib.js";

const CRATE = "v8-codegen-x64";
const GEN = "src/codegen/x64/constants-x64.h.js";
const SRC_REL = "src/codegen/x64/constants-x64.h";
const H = readV8(SRC_REL);

// 产物入仓前过 nightly rustfmt (固定点, 复跑幂等)
function fmtCrate() {
  const r = spawnSync("cargo", ["+nightly", "fmt", "-p", CRATE], { cwd: root, encoding: "utf8" });
  if (r.status !== 0) throw new Error(`${CRATE} rustfmt 失败: ${r.stderr}`);
}

function must(cond, msg) {
  if (!cond) throw new Error(`constants-x64.h.js 对数核对失败: ${msg}`);
}

// ---- 结构锚 ----
must(H.includes("#ifndef V8_CODEGEN_X64_CONSTANTS_X64_H_"), "头文件卫哨缺失");
must(H.includes('#include "src/common/globals.h"'), "globals.h include 缺失");
must(H.includes("namespace v8 {") && H.includes("namespace internal {"), "namespace 包裹缺失");

// ---- 常量提取 ----
const TYPE_MAP = { int: "i32", size_t: "usize", "constexpr int": "i32" };
const entries = [];
for (const m of H.matchAll(/constexpr\s+(\w+)\s+(\w+)\s*=\s*([^;]+);/g)) {
  entries.push({ cppType: m[1], cppName: m[2], value: m[3].trim() });
}

function rustConst(name) {
  // kRootRegisterBias → K_ROOT_REGISTER_BIAS (k 前缀 + 驼峰/缩写切分)
  const s = name.replace(/^k/, "");
  const snake = s
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
    .toUpperCase();
  return "K_" + snake;
}
function rustType(t) {
  const rt = TYPE_MAP[t];
  if (!rt) throw new Error(`constants-x64.h.js: 未登记 constexpr 类型 ${t}`);
  return rt;
}

// ---- 对数金标准 (与 C++ 头逐条对表) ----
const GOLDEN = [
  { cppName: "kRootRegisterBias", rustConst: "K_ROOT_REGISTER_BIAS", cppType: "int", value: "128" },
  { cppName: "kMaxPCRelativeCodeRangeInMB", rustConst: "K_MAX_PC_RELATIVE_CODE_RANGE_IN_MB", cppType: "size_t", value: "2048" },
];
must(entries.length === GOLDEN.length, `常量数 ${entries.length} != 金标准 ${GOLDEN.length}`);
for (const g of GOLDEN) {
  const e = entries.find((x) => x.cppName === g.cppName);
  must(e, `${g.cppName} 缺失`);
  must(e.value === g.value, `${g.cppName} 值漂移: ${e.value} != ${g.value}`);
  must(e.cppType === g.cppType, `${g.cppName} 类型漂移: ${e.cppType} != ${g.cppType}`);
  const rc = rustConst(e.cppName);
  must(rc === g.rustConst, `${g.cppName} 命名映射漂移: ${rc} != ${g.rustConst}`);
}
for (const e of entries) {
  const g = GOLDEN.find((x) => x.cppName === e.cppName);
  must(g, `未登记常量 ${e.cppName} (新增常量需同步金标准表)`);
}

// ---- 生成 Rust ----
const DOC = {
  kRootRegisterBias:
    "// kRootRegister 实际值相对 IsolateData 起点带偏移, 以利用负位移;\n// x64 最小操作数编码允许 int8 偏移, 故 bias 使 IsolateData 前 32 个 8 字节槽可按此编码。",
  kMaxPCRelativeCodeRangeInMB:
    "// 代码范围最大值, 保证范围内所有 Code 对象之间可用 pc 相对调用。",
};

let out = banner(GEN, SRC_REL, "wave1 纯数据面: 零依赖常量表。");
out += "\n";
for (const g of GOLDEN) {
  out += `${DOC[g.cppName]}\npub const ${g.rustConst}: ${rustType(g.cppType)} = ${g.value};\n\n`;
}
// 静态断言: 与 C++ 侧 constexpr 取值逐条钉死 (头内无 static_assert, 断言由复刻侧补强)
for (const g of GOLDEN) {
  out += `const _: () = assert!(${g.rustConst} == ${g.value}); // 金标准: ${g.cppName} = ${g.value}\n`;
}

writeSrc(CRATE, "constants.rs", out);
fmtCrate();
console.log(`constants-x64.h.js: 通过! 常量 ${entries.length}/${GOLDEN.length}, 金标准对数 ${GOLDEN.length}, 输出 src/constants.rs`);
for (const g of GOLDEN) console.log(`  ${g.rustConst} = ${g.value} (${g.cppType} → ${rustType(g.cppType)})`);
