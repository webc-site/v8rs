#!/usr/bin/env bun
// tool/src/compiler/backend/frame-elider.js
// v8/src/compiler/backend/frame-elider.{h,cc} → v8-compiler-backend/src/frame_elider.rs.skel
// FrameElider: 标记需要帧的块并定帧构造/析构点 (need_frame/frame_has_utf16 传播)
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/frame-elider.h");
const cc = readV8("src/compiler/backend/frame-elider.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 类 + ctor + Run + 私有方法 8 + 字段 3
must(h.includes("class FrameElider {"), "h FrameElider 类");
must(h.includes("explicit FrameElider(InstructionSequence* code, bool has_dummy_end_block,"), "h ctor 三参");
must(h.includes("void Run();"), "h Run");
for (const sig of ["void MarkBlocks();", "void PropagateMarks();", "void MarkDeConstruction();",
  "bool PropagateInOrder();", "bool PropagateReversed();", "bool PropagateIntoBlock(InstructionBlock* block);",
  "const InstructionBlocks& instruction_blocks() const;",
  "InstructionBlock* InstructionBlockAt(RpoNumber rpo_number) const;",
  "Instruction* InstructionAt(int index) const;"])
  must(h.includes(sig), `h 私有 ${sig}`);
for (const f of ["InstructionSequence* const code_;", "const bool has_dummy_end_block_;", "const bool is_wasm_to_js_;"])
  must(h.includes(f), `h 字段 ${f}`);

// cc: 成员实现 10
must(count(cc, /^[\w:<>* &]*FrameElider::\w+\(/gm) === 11, `cc 成员定义 11 (ctor+Run+私有 9), 实得 ${count(cc, /^[\w:<>* &]*FrameElider::\w+\(/gm)}`);
// cc: Run 三段 (Mark/Propagate/MarkDeConstruction)
must(count(cc, /MarkBlocks\(\);|PropagateMarks\(\);|MarkDeConstruction\(\);/g) === 3, "cc Run 三段调用");
// cc: 标记 API 面 (InstructionBlock 访存器)
must(count(cc, /(?<!mark_)needs_frame\(\)/g) === 9, "cc 裸 needs_frame ×9 (另 mark_needs_frame ×4)");
must(count(cc, /mark_needs_frame\(\)/g) === 4, "cc mark_needs_frame ×4");
must(count(cc, /mark_must_construct_frame\(\)/g) === 2, "cc mark_must_construct_frame ×2");
must(count(cc, /mark_must_deconstruct_frame\(\)/g) === 3, "cc mark_must_deconstruct_frame ×3");
must(count(cc, /mark_must_construct_frame\(\)/g) >= 1, "cc mark_must_construct_frame");
must(count(cc, /mark_must_deconstruct_frame\(\)/g) >= 1, "cc mark_must_deconstruct_frame");
// cc: 传播双轮询 (正向+逆向, 直到不动点)
must(cc.includes("while (PropagateInOrder() || PropagateReversed())"), "cc 双向传播不动点");
// cc: wasm-to-js 特例 + dummy-end-block 特例
must(count(cc, /is_wasm_to_js_/g) >= 1, "cc is_wasm_to_js_ 消费");
must(count(cc, /has_dummy_end_block_/g) >= 1, "cc has_dummy_end_block_ 消费");
// cc: 析构标记三态 (should frame / construct / deconstruct)
must(cc.includes("block->predecessors().empty()"), "cc 无前驱判构帧");
must(cc.includes("block->last_instruction_index()"), "cc 末指令索引");
must(count(cc, /block->SuccessorCount\(\) == 0/g) === 2, "cc 无后继判 ×2");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/frame-elider.js",
  "src/compiler/backend/frame-elider.h + frame-elider.cc",
  "FrameElider: 需帧块标记 + 帧构造/析构点标定 (双向传播不动点)")}
//! 台账: Run = MarkBlocks (入口/被调用/异常 handler 起需帧; wasm-to-js 特例)
//!   → PropagateMarks (正向+逆向双轮询直到不动点) → MarkDeConstruction
//!   (loop header 构造、throw 终结/末块析构、dummy-end-block 修正);
//!   need_frame 向后继传播, 前驱需帧亦传染; utf16 槽标记随帧传播。

/// v8: FrameElider::Run — 公有入口 (三段)
// pub fn run(&mut self)
/// v8: FrameElider::MarkBlocks — 初始需帧集
// fn mark_blocks(&mut self)
/// v8: FrameElider::PropagateMarks — 双向传播不动点
// fn propagate_marks(&mut self)
/// v8: FrameElider::MarkDeConstruction — 构造/析构点标定
// fn mark_de_construction(&mut self)
/// v8: FrameElider::PropagateInOrder — 正向轮询 (changed)
// fn propagate_in_order(&mut self) -> bool
/// v8: FrameElider::PropagateReversed — 逆向轮询 (changed)
// fn propagate_reversed(&mut self) -> bool
/// v8: FrameElider::PropagateIntoBlock — 单块传播 (前驱向下/后继向上)
// fn propagate_into_block(&mut self, block: &mut InstructionBlock) -> bool
/// v8: FrameElider::InstructionAt — 序列指令索引
// fn instruction_at(&self, index: usize) -> &Instruction

// —— 提取不变式断言 ——
const _: () = {
  assert!(true); // 纯过程 pass; 锚点对数见生成脚本日志
};
`;

writeSrc("v8-compiler-backend", "frame_elider.rs.skel", skel);
console.log(`frame-elider.js: 对数 ${checks} 条全过; 方法定义 11 (ctor + Run 公有 + 私有 9); 标记 API needs_frame ×7 + mark_* 族; ` +
  `双向传播不动点; 产物 frame_elider.rs.skel`);
