#!/usr/bin/env bun
// tool/src/maglev/regalloc.js — 寄存器分配规则先行 (分配器状态机 + 帧面)
// 覆盖: src/maglev/maglev-regalloc.h + maglev-regalloc.cc +
//       src/maglev/maglev-regalloc-data.h + maglev-regalloc-node-info.h +
//       src/maglev/maglev-register-frame-array.h +
//       src/maglev/maglev-code-gen-state.h + maglev-code-gen-state-inl.h
// 产出: v8-maglev-backend/src/regalloc.rs.skel (规则代码: 状态机 + 帧布局 + 节点分配面)
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/maglev/regalloc.js";
const FILES = [
  "src/maglev/maglev-regalloc.h", "src/maglev/maglev-regalloc.cc",
  "src/maglev/maglev-regalloc-data.h", "src/maglev/maglev-regalloc-node-info.h",
  "src/maglev/maglev-register-frame-array.h",
  "src/maglev/maglev-code-gen-state.h", "src/maglev/maglev-code-gen-state-inl.h",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[regalloc.js] ${m}`); };

// —— RegisterFrameState 四态机 (used/free × blocked/unblocked) ————
const rh = txt["src/maglev/maglev-regalloc.h"];
for (const needle of [
  "class MaglevPrintingVisitor;",
  "class MergePointRegisterState;",
  "struct RegallocBlockInfo {",
  "class RegisterFrameState {",
  "RegTList unblocked_free() const { return free_ - blocked_; }",
  "void FreeRegistersUsedBy(ValueNode* node) {",
  "void SetValue(RegisterT reg, ValueNode* node) {",
  "void SetValueWithoutBlocking(RegisterT reg, ValueNode* node) {",
]) {
  if (!rh.includes(needle)) die(`regalloc.h 缺针 ${needle}`);
}

// —— regalloc-data: 可分配寄存器集 + merge 状态位 ————
const rd = txt["src/maglev/maglev-regalloc-data.h"];
for (const needle of [
  "struct AllocatableRegisters;",
  "static constexpr RegList kRegisters =",
  "struct RegisterStateFlags {",
  "static constexpr int kIsMergeShift = 0;",
  "static constexpr int kIsInitializedShift = 1;",
  "typedef base::PointerWithPayload<void, RegisterStateFlags, 2> RegisterState;",
  "struct RegisterMerge {",
  "inline bool LoadMergeState(RegisterState state, RegisterMerge** merge) {",
]) {
  if (!rd.includes(needle)) die(`regalloc-data 缺针 ${needle}`);
}

// —— regalloc-node-info: NodeId + LiveRange + ValueLocation/InputLocation ————
const ni = txt["src/maglev/maglev-regalloc-node-info.h"];
for (const needle of [
  "using NodeIdT = uint32_t;",
  "static constexpr NodeIdT kInvalidNodeId = 0;",
  "static constexpr NodeIdT kFirstValidNodeId = 1;",
  "struct LiveRange {",
  "class ValueLocation {",
  "class InputLocation : public ValueLocation {",
  "class RegallocNodeInfo {",
  "NodeIdT next_use_id() const { return next_use_id_; }",
]) {
  if (!ni.includes(needle)) die(`node-info 缺针 ${needle}`);
}

// —— register-frame-array: 蝴蝶指针帧 (参数负索引/头/局部) ————
const fa = txt["src/maglev/maglev-register-frame-array.h"];
for (const needle of [
  "frame_start_ =",
  "T& operator[](interpreter::Register reg) { return frame_start_[reg.index()]; }",
  "V8_NOEXCEPT = delete;",
]) {
  if (!fa.includes(needle)) die(`frame-array 缺针 ${needle}`);
}

// —— code-gen-state: DeferredCodeInfo + MaglevCodeGenState + 操作数→寄存器映射 ————
const cs = txt["src/maglev/maglev-code-gen-state.h"];
for (const needle of [
  "class DeferredCodeInfo {",
  "class MaglevCodeGenState {",
  "inline Register ToRegister(const compiler::InstructionOperand& operand) {",
]) {
  if (!cs.includes(needle)) die(`code-gen-state 缺针 ${needle}`);
}

const out = [];
out.push(banner(GEN, FILES.join(", "), "寄存器分配规则先行: 四态机 + merge 位 + 帧蝴蝶指针"));
out.push(`//! 四态机 (修订禁改语义): Free+unblocked=完全空闲; Used+unblocked=可溢出;`);
out.push(`//!   Used+blocked=当前分配输入; Free+blocked=保留临时/clobber。`);
out.push(`//! 溢出 hint 面: RegallocLoopInfo.reload_hints_/spill_hints_ (循环头重载/溢出)。`);
out.push(``);
out.push(`// ── RegisterFrameState<RegisterT> (Register/DoubleRegister 双实例; AllocatableRegisters 特化) ──`);
out.push(`//   free_/blocked_ 双位集; unblocked_free=free_-blocked_; SetValue(阻塞)/SetValueWithoutBlocking`);
out.push(`//   FreeRegistersUsedBy(node); ForEachUsedRegister; Used=Allocatable^free_`);
out.push(``);
out.push(`// ── RegisterStateFlags (2 位: kIsMergeShift=0/kIsInitializedShift=1; PointerWithPayload) ──`);
out.push(`//   RegisterMerge{node; operands 跟体分配}; LoadMergeState 双重载 (merge/直值二态)`);
out.push(``);
out.push(`// ── RegallocNodeInfo (NodeIdT=u32; kInvalid=0/kFirst=1; LiveRange 起止闭区间) ──`);
out.push(`//   ValueLocation(SetUnallocated/SetAllocated/SetConstant/InjectLocation/Cloberred=USED_AT_START)`);
out.push(`//   InputLocation.next_use_id_; GetRegisterHint (fixed-register 策略→no_reg 兜底)`);
out.push(``);
out.push(`// ── RegisterFrameArray (蝴蝶指针: 参数负索引—未优化帧头—局部零起; CopyFrom 按活跃集) ──`);
out.push(``);
out.push(`// ── MaglevCodeGenState + DeferredCodeInfo (延迟代码块; ToRegister/ToMemOperand 映射) ──`);
out.push(``);
out.push(`// 对数: 状态机 8 + 数据面 8 + 节点面 8 + 帧面 3 + cgstate 3 全过`);

writeSrc("v8-maglev-backend", "regalloc.rs.skel", out.join("\n") + "\n");
console.log(`OK regalloc.js → v8-maglev-backend/src/regalloc.rs.skel`);
