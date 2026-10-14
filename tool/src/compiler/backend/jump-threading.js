#!/usr/bin/env bun
// tool/src/compiler/backend/jump-threading.js
// v8/src/compiler/backend/jump-threading.{h,cc} → v8-compiler-backend/src/jump_threading.rs.skel
// JumpThreading: 空块跳转转发 (传递闭包), ComputeForwarding/ApplyForwarding 两静态入口
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/jump-threading.h");
const cc = readV8("src/compiler/backend/jump-threading.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 静态类两入口
must(h.includes("class V8_EXPORT_PRIVATE JumpThreading {"), "h JumpThreading 静态类");
must(h.includes("static bool ComputeForwarding(Zone* local_zone, ZoneVector<RpoNumber>* result,"), "h ComputeForwarding");
must(h.includes("static void ApplyForwarding(Zone* local_zone,"), "h ApplyForwarding");
must(h.includes("May also negate some branches"), "h 分支取反注释锚");

// cc: JumpThreadingState 三态标记 (-1 unvisited / -2 onstack)
must(cc.includes("struct JumpThreadingState {"), "cc JumpThreadingState");
must(cc.includes("RpoNumber unvisited() { return RpoNumber::FromInt(-1); }"), "cc unvisited=-1");
must(cc.includes("RpoNumber onstack() { return RpoNumber::FromInt(-2); }"), "cc onstack=-2");
// cc: Forward 四分支 (自环/递归/破环/转发)
for (const anchor of ["if (to == from) {", "else if (to_to == unvisited()) {",
  "else if (to_to == onstack()) {", "// break the cycle.", "// forward the block."])
  must(cc.includes(anchor), `cc Forward 分支 ${anchor}`);
// cc: GapJumpRecord 缓存 (ZoneUnorderedMap + RpoNumberHash)
must(cc.includes("struct GapJumpRecord {"), "cc GapJumpRecord");
must(cc.includes("struct RpoNumberHash"), "cc RpoNumberHash");
must(cc.includes("ZoneUnorderedMap<RpoNumber, ZoneVector<Record>, RpoNumberHash>"), "cc 记录表");
must(cc.includes("bool CanForwardGapJump(Instruction* instr, RpoNumber instr_block,"), "cc CanForwardGapJump");
// cc: 空返回块双缓存槽 (有/无析构帧)
must(cc.includes("RpoNumber empty_deconstruct_frame_return_block = RpoNumber::Invalid();"), "cc 析构帧空返回块");
must(cc.includes("RpoNumber empty_no_deconstruct_frame_return_block = RpoNumber::Invalid();"), "cc 非析构帧空返回块");
// cc: 两静态实现 + 末尾 ao 号重排
must(cc.includes("bool JumpThreading::ComputeForwarding(Zone* local_zone,"), "cc ComputeForwarding 实现");
must(cc.includes("void JumpThreading::ApplyForwarding(Zone* local_zone,"), "cc ApplyForwarding 实现");
must(cc.includes("block->set_ao_number(RpoNumber::FromInt(ao));"), "cc ao 号重排");
// cc: 转发闭包二轮扫描 (自洽化)
must(count(cc, /for \(int i = 0; i < static_cast<int>\(result->size\(\)\); i\+\+\)/g) === 1, "cc 二轮扫描");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/jump-threading.js",
  "src/compiler/backend/jump-threading.h + jump-threading.cc",
  "JumpThreading: 空块跳转转发传递闭包 + 指令重写/分支取反")}
//! 台账: ComputeForwarding — 栈式传递闭包 (state 三态: -1 unvisited/-2 onstack;
//!   Forward 四分支: 自环终止/递归入栈/破环/转发; 空返回块双缓存槽按帧析构分桶;
//!   GapJumpRecord (CanForwardGapJump 逐 gap 比对) 缓存防重扫; 末轮二扫自洽化)。
//!   ApplyForwarding — 跳转/分支目标重写 (可取反分支), ao 号重排, rpo 立即数重定向。

/// v8: JumpThreading::ComputeForwarding — 转发映射 (是否有转发发生)
// pub fn compute_forwarding(code: &InstructionSequence, frame_at_start: bool) -> (bool, Vec<RpoNumber>)
/// v8: JumpThreading::ApplyForwarding — 按映射重写跳转/分支
// pub fn apply_forwarding(forwarding: &[RpoNumber], code: &mut InstructionSequence)
/// v8: JumpThreadingState::unvisited / onstack — 三态哨兵
pub const UNVISITED: i32 = -1;
pub const ON_STACK: i32 = -2;

// —— 提取不变式断言 ——
const _: () = {
  assert!(UNVISITED == -1);
  assert!(ON_STACK == -2);
};
`;

writeSrc("v8-compiler-backend", "jump_threading.rs.skel", skel);
console.log(`jump-threading.js: 对数 ${checks} 条全过; 静态入口 2; 状态三态哨兵 -1/-2; ` +
  `Forward 四分支; GapJumpRecord 缓存; 产物 jump_threading.rs.skel`);
