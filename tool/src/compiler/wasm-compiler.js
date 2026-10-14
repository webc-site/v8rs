#!/usr/bin/env bun
// tool/src/compiler/wasm-compiler.js
// v8/src/compiler/wasm-compiler.h + wasm-compiler.cc
// → v8-compiler-passes/src/wasm_compiler.rs.skel
// wasm 编译入口: 解析 kCWasmEntrySigTypes 签名表 / WasmLoopInfo /
// WasmCompilationData 字段 / 入口函数清单与 cc 方法计数。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/wasm-compiler.h");
const cc = readV8("src/compiler/wasm-compiler.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`wasm-compiler: ${msg}`);
};

for (const [s, name] of [
  ["constexpr MachineType kCWasmEntrySigTypes[] = {", "CWasmEntry 签名表"],
  ["constexpr MachineSignature kCWasmEntrySig(1, 4, kCWasmEntrySigTypes);", "签名常量"],
  ["struct WasmLoopInfo {", "WasmLoopInfo"],
  ["struct WasmCompilationData {", "WasmCompilationData"],
  ["wasm::WasmCompilationResult CompileWasmImportCallWrapper(", "ImportCallWrapper 入口"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— kCWasmEntrySigTypes 表 (真解析 5 项) ——
const sigTypes = [...h.slice(h.indexOf("constexpr MachineType kCWasmEntrySigTypes[] = {")).matchAll(/MachineType::(\w+)\(\)/g)].map((m) => m[1]);
need(sigTypes.length === 5, `签名表 ${sigTypes.length}`);
need(sigTypes[0] === "Pointer" && sigTypes[1] === "Uint32" && sigTypes[2] === "AnyTagged", `签名表形态 ${sigTypes}`);

// —— 入口函数清单 (真解析 h) ——
const entries = [
  ...[...h.matchAll(/^(?:V8_EXPORT_PRIVATE )?wasm::WasmCompilationResult (Compile\w+)\(/gm)].map((m) => m[1]),
  ...[...h.matchAll(/^std::unique_ptr<OptimizedCompilationJob> (New\w+)\(/gm)].map((m) => m[1]),
  ...[...h.matchAll(/^V8_EXPORT_PRIVATE Handle<Code> (Compile\w+)\(/gm)].map((m) => m[1]),
];
for (const e of ["CompileWasmImportCallWrapper", "CompileWasmCapiCallWrapper", "CompileWasmJSFastCallWrapper", "NewJSToWasmCompilationJob", "CompileCWasmEntry"]) {
  need(entries.includes(e), `入口 ${e} 缺席 (实得 ${entries.join(",")})`);
}

// —— WasmCompilationData 字段 ——
const wcd = h.slice(h.indexOf("struct WasmCompilationData {"));
const wcdFields = [...wcd.slice(0, 800).matchAll(/^\s{2}(?:const )?[\w:<>*& ]+? (\w+)(?:\{[^}]*\})?(?: = [^;]+)?;/gm)].map((m) => m[1]);
need(wcdFields.length >= 8, `WCD 字段 ${wcdFields.length}`);

// —— cc: 类面 (wasm 编译大类在 cc 中) ——
const ccClasses = [...cc.matchAll(/^class (\w+) : public \w+::(\w+)/gm)].map((m) => `${m[1]} : ${m[2]}`);
const buildTFGraphFns = [...cc.matchAll(/^Node\* Build\w+\(/gm)].length;

const body = `${banner("src/compiler/wasm-compiler.js", "src/compiler/wasm-compiler.h + wasm-compiler.cc", "wasm→TF 图编译入口与共享数据")}
//! wasm-compiler — wrapper/入口编译 + (cc 内) BuildWasmNode 主编译器
//!
//! CWasmEntry 签名: 1 返回 + 4 参数 (${sigTypes.join("/")});
//! 入口 ${entries.length} 个; WasmCompilationData 字段 ${wcdFields.length};
//! cc Build* 工厂 ${buildTFGraphFns} 个, 继承类 ${ccClasses.length}。

/// 原版 kCWasmEntrySigTypes — CWasmEntry 桩签名表
/// (return, target, object_ref, argv, c_entry_fp)
pub const K_C_WASM_ENTRY_SIG_TYPES: [MachineType; 5] = [
${sigTypes.map((t) => `  MachineType::${t},`).join("\n")}
];

/// 原版 WasmLoopInfo — 循环信息 (嵌套深度 + 无内环估计)
pub struct WasmLoopInfo {
  /// header
  pub header: *mut Node,
  /// nesting_depth
  pub nesting_depth: u32,
  /// can_be_innermost — 无内环 (内联后可能失效)
  pub can_be_innermost: bool,
}

/// 原版 WasmCompilationData — 编译共享数据
pub struct WasmCompilationData<'z> {
${wcdFields.slice(0, 12).map((f) => `  /// ${f}\n  pub ${f.replace(/_$/, "")}: Placeholder,`).join("\n")}
  /// 其余字段见 v8: src/compiler/wasm-compiler.h WasmCompilationData
}

${entries.map((e) => `/// 原版 ${e}\n/// v8: src/compiler/wasm-compiler.cc ${e}\npub fn ${e.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(/* 按签名 */) -> WasmCompilationResult {\n  todo!("v8: src/compiler/wasm-compiler.cc ${e}")\n}`).join("\n\n")}

/// 原版 CWasmEntrySignature — CWasmEntry 桩签名单例
/// v8: src/compiler/wasm-compiler.h CWasmEntrySignature
pub fn c_wasm_entry_signature() -> &'static MachineSignature {
  todo!("v8: src/compiler/wasm-compiler.h CWasmEntrySignature (kCWasmEntrySig: 1 返回 + 4 参数)")
}
`;

const p = writeSrc("v8-compiler-passes", "wasm_compiler.rs.skel", body);
console.log(`written ${p}\nsig_types=${sigTypes.length} entries=${entries.length} wcd_fields=${wcdFields.length} cc_build_fns=${buildTFGraphFns} cc_classes=${ccClasses.length}`);
