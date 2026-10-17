#!/usr/bin/env bun
// tool/src/maglev/codegen.js — 代码生成: assembler/总装/compiler/入口/并发派发 + 架构族
// 覆盖: src/maglev/maglev-assembler.h + maglev-assembler-inl.h + maglev-assembler.cc +
//       src/maglev/maglev-code-generator.h/.cc +
//       src/maglev/maglev-code-gen-state.h (映射面, 详见 regalloc.js) +
//       src/maglev/maglev-compiler.h/.cc + src/maglev/maglev.h + maglev.cc +
//       src/maglev/maglev-concurrent-dispatcher.h/.cc +
//       src/maglev/maglev-deopt-frame-visitor.h +
//       架构族: arm/arm64/loong64/ppc/riscv/s390/x64 (assembler-inl/cc + ir-<arch>.cc)
// 产出: v8-maglev-backend/src/codegen.rs.skel + v8-maglev-backend/src/arch.rs.skel
// 复跑幂等: 纯文本扫描 + 确定性输出。架构后置: x64+arm64 先行, 其余 feature 隔离。
import { banner, readV8, writeSrc } from "../../lib.js";
import { readdirSync } from "node:fs";

const GEN = "src/maglev/codegen.js";
const die = (m) => { throw new Error(`[codegen.js] ${m}`); };

// —— assembler 锚 (可分配集是 regalloc 真值来源) ————
const asm = readV8("src/maglev/maglev-assembler.h");
readV8("src/maglev/maglev-assembler-inl.h");
readV8("src/maglev/maglev-assembler.cc");
for (const needle of [
  "static constexpr RegList GetAllocatableRegisters() {",
  "static constexpr DoubleRegList GetAllocatableDoubleRegisters() {",
  "static constexpr bool HasZeroRegister() {",
  "enum StoreMode { kField, kFixedArrayElement };",
  "enum ValueIsCompressed { kValueIsDecompressed, kValueIsCompressed };",
  "enum ValueCanBeSmi { kValueCannotBeSmi, kValueCanBeSmi };",
  "enum class CharCodeMaskMode { kValueIsInRange, kMustApplyMask };",
  "void Assert(Condition cond, AbortReason reason);",
]) {
  if (!asm.includes(needle)) die(`assembler 缺针 ${needle}`);
}
// 架构条件分支锚 (修订时 x64+arm64 先行, 其余后置)
for (const needle of ["V8_TARGET_ARCH_ARM", "V8_TARGET_ARCH_RISCV64",
  "kMaglevExtraScratchRegister", "kMaglevFlagsRegister"]) {
  if (!asm.includes(needle)) die(`assembler 缺针 ${needle}`);
}

// —— code-generator 总装锚 ————
const cg = readV8("src/maglev/maglev-code-generator.h");
readV8("src/maglev/maglev-code-generator.cc");
for (const needle of [
  "class MaglevCodeGenerator final {",
  "V8_NODISCARD bool Assemble();",
  "MaybeHandle<Code> Generate(Isolate* isolate);",
  "V8_NODISCARD bool EmitCode();",
  "void EmitDeferredCode();",
  "V8_NODISCARD bool EmitDeopts();",
  "void EmitExceptionHandlerTrampolines();",
  "int stack_slot_count() const { return code_gen_state_.stack_slots(); }",
]) {
  if (!cg.includes(needle)) die(`codegen 缺针 ${needle}`);
}

// —— compiler/入口/派发锚 ————
const comp = readV8("src/maglev/maglev-compiler.h");
readV8("src/maglev/maglev-compiler.cc");
for (const needle of ["class MaglevCompiler : public AllStatic {",
  "static bool Compile(LocalIsolate* local_isolate,",
  "static std::pair<MaybeHandle<Code>, BailoutReason> GenerateCode("]) {
  if (!comp.includes(needle)) die(`compiler 缺针 ${needle}`);
}
const mh = readV8("src/maglev/maglev.h");
readV8("src/maglev/maglev.cc");
if (!mh.includes("class Maglev : public AllStatic {")) die("缺 Maglev 入口类");
const dp = readV8("src/maglev/maglev-concurrent-dispatcher.h");
readV8("src/maglev/maglev-concurrent-dispatcher.cc");
for (const needle of ["class MaglevCompilationJob final : public OptimizedCompilationJob {",
  "class V8_EXPORT_PRIVATE MaglevConcurrentDispatcher final {"]) {
  if (!dp.includes(needle)) die(`dispatcher 缺针 ${needle}`);
}
const dv = readV8("src/maglev/maglev-deopt-frame-visitor.h");
for (const needle of ["DeoptFrame::FrameType::kInterpretedFrame",
  "void VisitSingleFrame(DeoptFrameT& frame, Function&& f) {"]) {
  if (!dv.includes(needle)) die(`deopt-frame-visitor 缺针 ${needle}`);
}

// —— 架构族: 7  arch × 3 文件 = 21 (顶层 74 另计; 合计 95, 任务票面 74 指顶层) ————
const ARCHES = ["arm", "arm64", "loong64", "ppc", "riscv", "s390", "x64"];
const archFiles = [];
for (const a of ARCHES) {
  const names = readdirSync(new URL(`../../../v8/src/maglev/${a}`, import.meta.url)).sort();
  if (names.length !== 3) die(`${a} 文件数=${names.length}, 期望 3`);
  for (const n of names) archFiles.push(`src/maglev/${a}/${n}`);
}
if (archFiles.length !== 21) die(`架构文件=${archFiles.length}, 期望 21`);
// x64/arm64 代表针
const x64ir = readV8("src/maglev/x64/maglev-ir-x64.cc");
const a64ir = readV8("src/maglev/arm64/maglev-ir-arm64.cc");
if (!x64ir.length || !a64ir.length) die("x64/arm64 ir 侧空");
const x64asm = readV8("src/maglev/x64/maglev-assembler-x64.cc");
if (!x64asm.length) die("x64 assembler 侧空");

const out = [];
out.push(banner(GEN, "src/maglev/maglev-assembler.h, maglev-code-generator.h, maglev-compiler.h, maglev.h, maglev-concurrent-dispatcher.h, maglev-deopt-frame-visitor.h (+.cc/-inl.h 同族)", "代码生成总装 + 架构先行表"));
out.push(`//! 依赖: v8-maglev-frontend (Graph), v8-codegen-{x64,arm64} (MacroAssembler 底);`);
out.push(`//! 架构范围 (todo.md 五): x64+arm64 先行, 其余后置 feature 隔离默认关。`);
out.push(``);
out.push(`// ── MaglevAssembler (MacroAssembler 子类; 可分配集=regalloc 真值来源) ──`);
out.push(`//   GetAllocatableRegisters (ARM/RISCV64/LOONG64 减 scratch/flags 特化); GetAllocatableDoubleRegisters`);
out.push(`//   HasZeroRegister/ZeroRegister (arm64 xzr); StoreMode/ValueIsCompressed/ValueCanBeSmi/CharCodeMaskMode`);
out.push(`//   MoveRepr/MemOperand 映射; Assert(cond, AbortReason) 系列`);
out.push(``);
out.push(`// ── MaglevCodeGenerator (Assemble→Generate; EmitCode/DeferredCode/Deopts/异常桩/retained/元数据) ──`);
out.push(`//   stack_slot_count(+fixed_frame)/parameter_count; retained_maps_; BuildCodeObject`);
out.push(``);
out.push(`// ── MaglevCompiler (AllStatic; Compile 任意线程 / GenerateCode 主线程; BailoutReason) ──`);
out.push(`//   Maglev::Compile (V8_ENABLE_MAGLEV 门下测试入口); MaglevCompilationJob + ConcurrentDispatcher`);
out.push(`//   DeoptFrameVisitor (反优化帧重建; frame-translation 底)`);
out.push(``);
out.push(`// 对数: asm 12 + codegen 8 + compiler 5 + 架构 21 全过`);

const arch = [];
arch.push(banner(GEN, archFiles.join(", "), "架构族 7×3=21; x64+arm64 先行"));
arch.push(`//! 修订: x64/arm64 全量; arm/loong64/ppc/riscv/s390 后置可选 crate, 不阻塞里程碑。`);
for (const f of archFiles) arch.push(`//   ARCH ${f}`);
arch.push(``);
arch.push(`// 对数: 21 文件存在性全过 (各 arch 3 文件: assembler-inl.h/assembler.cc/ir.cc)`);

writeSrc("v8-maglev-backend", "codegen.rs.skel", out.join("\n") + "\n");
writeSrc("v8-maglev-backend", "arch.rs.skel", arch.join("\n") + "\n");
console.log(`OK codegen.js: codegen + arch 21 → v8-maglev-backend/src/codegen.rs.skel, arch.rs.skel`);
