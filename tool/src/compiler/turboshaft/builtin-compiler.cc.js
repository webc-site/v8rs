#!/usr/bin/env bun
// tool/src/compiler/turboshaft/builtin-compiler.cc.js
// 机械提取 v8/src/compiler/turboshaft/builtin-compiler.cc
// → v8-turboshaft/src/builtin_compiler.rs.skel
//
// 提取面: BuildWithTurboshaftAssemblerImpl 单入口 (builtin 编译编排:
// ZoneStats → OptimizedCompilationInfo → PipelineData(kTSABuiltin) →
// GenerateCodeForTurboshaftBuiltin) + kBuiltinCompilationZoneName 常量。
import { readV8, writeSrc, banner } from "../../../lib.js";

const cc = readV8("src/compiler/turboshaft/builtin-compiler.cc");

// 1. 常量: 编译 zone 名
const zoneM = cc.match(/inline constexpr char kBuiltinCompilationZoneName\[\] =\s*\n?\s*"([^"]+)";/);
if (!zoneM) throw new Error("未找到 kBuiltinCompilationZoneName");
if (zoneM[1] !== "builtin-compilation-zone") throw new Error(`zone 名 ${zoneM[1]} != builtin-compilation-zone`);

// 2. 入口函数签名 (7 参数逐一断言)
const sigM = cc.match(
  /DirectHandle<Code> (BuildWithTurboshaftAssemblerImpl)\(\s*Isolate\* isolate, Builtin builtin, TurboshaftAssemblerGenerator generator,\s*std::function<compiler::CallDescriptor\*\(Zone\*\)> call_descriptor_builder,\s*const char\* name, const AssemblerOptions& options, CodeKind code_kind,\s*std::optional<interpreter::BytecodeHandlerData> bytecode_handler_data\)/,
);
if (!sigM) throw new Error("BuildWithTurboshaftAssemblerImpl 签名不匹配 (7 参数)");

// 3. 编排流锚
for (const [re, label] of [
  [/DCHECK_EQ\(code_kind == CodeKind::BYTECODE_HANDLER,\s*bytecode_handler_data\.has_value\(\)\);/, "BYTECODE_HANDLER ↔ bytecode_handler_data 一致断言"],
  [/compiler::ZoneStats zone_stats\(isolate->allocator\(\)\);/, "ZoneStats"],
  [/ZoneWithName<kBuiltinCompilationZoneName> zone\(/, "ZoneWithName"],
  [/OptimizedCompilationInfo info\(base::CStrVector\(name\), zone, code_kind,\s*builtin\);/, "OptimizedCompilationInfo"],
  [/PipelineData data\(&zone_stats, TurboshaftPipelineKind::kTSABuiltin, isolate,\s*&info, options\);/, "PipelineData(kTSABuiltin)"],
  [/data\.InitializeBuiltinComponent\(call_descriptor,\s*std::move\(bytecode_handler_data\)\);/, "InitializeBuiltinComponent"],
  [/data\.InitializeGraphComponent\(nullptr, Graph::Origin::kPureTurboshaft\);/, "InitializeGraphComponent(kPureTurboshaft)"],
  [/ZoneWithName<kTempZoneName> temp_zone\(/, "kTempZoneName"],
  [/generator\(&data, isolate, data\.graph\(\), temp_zone\);/, "generator 回调"],
  [/compiler::Pipeline::GenerateCodeForTurboshaftBuiltin\(/, "GenerateCodeForTurboshaftBuiltin"],
  [/ProfileDataFromFile::TryRead\(name\)/, "ProfileDataFromFile::TryRead"],
  [/ToHandleChecked\(\)/, "ToHandleChecked"],
]) {
  if (!re.test(cc)) throw new Error(`编排锚缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/builtin-compiler.cc.js",
  "src/compiler/turboshaft/builtin-compiler.cc",
  "builtin 编译编排: TSA 生成器 → 指令码单入口骨架",
);
rust += `
/// 原版 kBuiltinCompilationZoneName
pub const BUILTIN_COMPILATION_ZONE_NAME: &'static str = "${zoneM[1]}";

/// v8: src/compiler/turboshaft/builtin-compiler.cc BuildWithTurboshaftAssemblerImpl
///
/// 编排流 (原版 11 锚):
/// 1. DCHECK: BYTECODE_HANDLER ↔ bytecode_handler_data 成对
/// 2. ZoneStats + ZoneWithName(builtin-compilation-zone)
/// 3. OptimizedCompilationInfo(name, zone, code_kind, builtin)
/// 4. PipelineData(kTSABuiltin) + InitializeBuiltinComponent(call_descriptor)
/// 5. InitializeGraphComponent(nullptr, kPureTurboshaft)
/// 6. temp_zone (kTempZoneName) → generator 回调填图
/// 7. Pipeline::GenerateCodeForTurboshaftBuiltin(..., ProfileDataFromFile::TryRead(name))
///    → ToHandleChecked
pub fn build_with_turboshaft_assembler_impl() -> () {
  unimplemented!("Developer 修订: 按编排流组装")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BUILTIN_COMPILATION_ZONE_NAME == "builtin-compilation-zone");
};
`;

const p = writeSrc("v8-turboshaft", "builtin_compiler.rs.skel", rust);
console.log(`written ${p}`);
console.log(`入口=1 (7参数) zone名=1 编排锚=11`);
