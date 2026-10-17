#!/usr/bin/env bun
// tool/src/maglev/maglev-basic-block.js — BasicBlock 纯枚举/位旗
// 覆盖: v8/src/maglev/maglev-basic-block.h (匿名 `enum : uint8_t {kMerge,kEdgeSplit,kOther}`
//       + 三个 1 位旗位 deferred_/is_start_block_of_switch_case_/is_dead_)
// 产出: v8-maglev-ir/src/basic_block.rs
// 对数: 枚举 3 变体 + 旗位 3; 原版无 BasicBlockPredecessorInfo(旧版快照符号, 已漂移) → 登记台账。
// 复跑幂等: 纯文本扫描 + 原版声明序。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/maglev/maglev-basic-block.js";
const SRC = "src/maglev/maglev-basic-block.h";
const txt = readV8(SRC);
const die = (m) => {
  throw new Error(`[maglev-basic-block.js] ${m}`);
};

// —— 匿名枚举锚 (整行原文) ——
const ENUM_LINE = "  enum : uint8_t { kMerge, kEdgeSplit, kOther } type_;";
if (!txt.includes(ENUM_LINE)) die("缺匿名枚举声明行");
const names = [...ENUM_LINE.matchAll(/k(\w+)/g)].map((m) => m[1]);
if (names.join(",") !== "Merge,EdgeSplit,Other") die(`枚举变体漂移: ${names.join(",")}`);
if (names.length !== 3) die(`枚举变体数=${names.length}, 期望 3`);

// —— 1 位旗位锚 ——
const FLAGS = [
  "bool deferred_ : 1 = false;",
  "bool is_start_block_of_switch_case_ : 1 = false;",
  "bool is_dead_ : 1 = false;",
];
for (const f of FLAGS) if (!txt.includes(f)) die(`缺旗位 ${f}`);
// 旗位声明序锚 (禁漂移)
const flagAt = FLAGS.map((f) => txt.indexOf(f));
if (flagAt.some((x) => x < 0)) die("旗位锚缺失");
if (!(flagAt[0] < flagAt[1] && flagAt[1] < flagAt[2])) die("旗位声明序漂移");

// —— 依赖面锚 (不转, 仅证明零依赖枚举已全取) ——
if (!txt.includes("class BasicBlock {")) die("缺 class BasicBlock");
if (!txt.includes("using NodeIterator = ZoneVector<Node*>::iterator;")) die("缺 NodeIterator 别名锚");
// 本快照无旧版 BasicBlockPredecessorInfo/Kind; 缺失即登记, 不静默
const hasPredInfo = txt.includes("BasicBlockPredecessorInfo");
const otherEnums = [...txt.matchAll(/\benum\b[^;{]*\{/g)].length;
if (otherEnums !== 1) die(`本头文件 enum 数=${otherEnums}, 期望 1 (仅匿名类型枚举)`);

// —— 产出 ———
const L = [];
const push = (s = "") => L.push(s);
push(banner(GEN, SRC, `BasicBlock 类型枚举 3 变体 + 旗位 3; BasicBlockPredecessorInfo=${hasPredInfo ? "存在" : "本快照不存在(已登记)"}`));
push("//! Maglev 基本块的零依赖数据面: 块类型枚举 + 位旗。");
push("//! 原版 `class BasicBlock` 本体依赖 Graph/FrameState/MaglevAssembler, 属后续波次;");
push("//! 见 readme/zh.md skip 登记。");
push();
push("/// 原版匿名 `enum : uint8_t { kMerge, kEdgeSplit, kOther } type_`。");
push("#[derive(Clone, Copy, Debug, PartialEq, Eq)]");
push("#[repr(u8)]");
push("pub enum BasicBlockType {");
names.forEach((n, i) => push(`  ${n} = ${i},`));
push("}");
push();
push("/// 原版 `type_` 存储宽度 (uint8_t)。");
push("pub const BASIC_BLOCK_TYPE_BITS: u32 = 8;");
push();
push("/// 原版 `bool xxx_ : 1` 旗位序号 (声明序即位序)。");
push(`pub const BLOCK_FLAG_COUNT: usize = ${FLAGS.length};`);
const flagNames = FLAGS.map((f) => /bool (\w+)_ : 1/.exec(f)[1] + "_");
flagNames.forEach((n, i) => {
  push(`/// 原版 \`bool ${n} : 1\` 的旗位序号。`);
  push(`pub const BLOCK_FLAG_${n.slice(0, -1).toUpperCase()}: usize = ${i};`);
});
push();
push("/// 原版三个 1 位旗的位名 (顺序即声明序)。");
push(`pub const BLOCK_FLAG_NAMES: [&str; BLOCK_FLAG_COUNT] = [`);
for (const n of flagNames) push(`  ${JSON.stringify(n)},`);
push("];");
push();
push("// 对数: 枚举 3 (变体名逐个核) + 旗位 3 (原文整行锚 + 声明序锚) +");
push("//   本头 enum 总数 1 反证锚 = 全过; BasicBlockPredecessorInfo 缺失已登台账。");

writeSrc("v8-maglev-ir", "basic_block.rs", L.join("\n") + "\n");
console.log(`OK maglev-basic-block.js: ${names.length} 枚举变体 + ${FLAGS.length} 旗位 → v8-maglev-ir/src/basic_block.rs`);
