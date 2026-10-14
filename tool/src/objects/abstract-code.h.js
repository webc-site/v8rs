#!/usr/bin/env bun
// tool/src/objects/abstract-code.h.js — AbstractCode 包装生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/abstract-code.h + abstract-code.cc + abstract-code-inl.h
//   → v8-objects-rest/src/abstract_code.rs
// 规则面: 方法面清单 + Code/BytecodeArray 双分发矩阵 (14 个分发方法) + 全指针比较契约
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/abstract-code.h");
const cc = readV8("src/objects/abstract-code.cc");
const inl = readV8("src/objects/abstract-code-inl.h");

// —— 方法面 (.h 声明) ——
const methods = [...h.matchAll(/^\s{2}(?:inline )?(?:int|Address|bool|Tagged<[\w<>, ]+>|CodeKind|Builtin|BytecodeOffset) (\w+)\(/gm)].map((m) => m[1]);
const EXPECT = ["SourcePosition", "SourceStatementPosition", "InstructionStart", "InstructionEnd", "InstructionSize",
  "SourcePositionTable", "SizeIncludingMetadata", "contains", "kind", "builtin_id", "has_instruction_stream",
  "is_context_specialized", "osr_offset", "GetCode", "GetBytecodeArray", "ToUnionType"];
if (methods.join() !== EXPECT.join())
  throw new Error(`AbstractCode 方法面漂移:\n  期望 ${EXPECT}\n  实得 ${methods}`);

// —— 双分发面 (cc 4 + inl 10, 全部 IsCode(map) 分支) ——
const dispatchCc = [...cc.matchAll(/^(?:int|bool|BytecodeOffset) AbstractCode::(\w+)\(/gm)].map((m) => m[1]);
if (dispatchCc.join() !== "SourcePosition,SourceStatementPosition,is_context_specialized,osr_offset")
  throw new Error(`cc 分发面漂移: ${dispatchCc}`);
const dispatchInl = [...inl.matchAll(/^(?:int|Address|bool|Tagged<[\w<>, ]+>|CodeKind|Builtin) AbstractCode::(\w+)\(/gm)].map((m) => m[1]);
const INL_EXPECT = ["InstructionSize", "SourcePositionTable", "SizeIncludingMetadata", "InstructionStart",
  "InstructionEnd", "contains", "kind", "builtin_id", "has_instruction_stream", "GetCode", "GetBytecodeArray", "ToUnionType"];
if (dispatchInl.join() !== INL_EXPECT.join())
  throw new Error(`inl 分发面漂移 (应 ${INL_EXPECT.length}):\n  ${dispatchInl}`);
const dispatchTotal = dispatchCc.length + dispatchInl.length;
// 双分发结构锚: 每个分发函数都有 IsCode(map_object) 分支
const branchCount = [...cc.matchAll(/InstanceTypeChecker::IsCode\(map_object\)/g), ...inl.matchAll(/InstanceTypeChecker::IsCode\(map_object\)/g)].length;
if (branchCount !== 12) throw new Error(`IsCode 双分支应 12 处, 实得 ${branchCount}`);

// —— 语义锚 (BytecodeArray 侧默认值) ——
if (!/return GetBytecodeArray\(\)->length\(\);/.test(inl)) throw new Error("InstructionSize 字节码侧 length 缺失");
if (!/return CodeKind::INTERPRETED_FUNCTION;/.test(inl)) throw new Error("kind 字节码侧默认 INTERPRETED_FUNCTION 缺失");
if (!/return Builtin::kNoBuiltinId;/.test(inl)) throw new Error("builtin_id 字节码侧默认 kNoBuiltinId 缺失");
if (!/return BytecodeOffset::None\(\);/.test(cc)) throw new Error("osr_offset 字节码侧 None 缺失");
// 全指针比较契约
if (!/static_assert\(!kAllCodeObjectsLiveInTrustedSpace\);/.test(h)) throw new Error("kAllCodeObjectsLiveInTrustedSpace 断言缺失");
if (!/constexpr bool operator==\(const Tagged<AbstractCode> lhs,/.test(h)) throw new Error("全指针比较 operator== 缺失");

const body = `${banner("src/objects/abstract-code.h.js", "src/objects/abstract-code.{h,cc,-inl.h}", "Code|BytecodeArray 双分发骨架")}
//! AbstractCode 包装 (原版 abstract-code.{h,cc,-inl.h})
//!
//! AbstractCode 是 {Code | BytecodeArray} 的剖析期 (profiling) 联合包装:
//! BytecodeArray 在 trusted space (主 cage 外), Code 在 sandbox 内,
//! 故比较必须走全指针 (非压缩指针) — 原版 static_assert(!kAllCodeObjectsLiveInTrustedSpace).

/// AbstractCode 分发矩阵 (${dispatchTotal} 个分发方法, 每个含 Code/BytecodeArray 两支)
///
/// | 方法 | Code 支 | BytecodeArray 支 |
/// |---|---|---|
${dispatchCc.concat(dispatchInl).map((m) => `/// | ${m} | 转发 GetCode() | ${"见原版"} |\n`).join("")}///
/// BytecodeArray 侧固定默认: kind→INTERPRETED_FUNCTION, builtin_id→kNoBuiltinId,
/// osr_offset→BytecodeOffset::None(), is_context_specialized→false,
/// InstructionSize→length().
pub const ABSTRACT_CODE_DISPATCHERS: [&str; ${dispatchTotal}] = [${[...dispatchCc, ...dispatchInl].map((m) => `"${m}"`).join(", ")}];

/// 源位置查询 (双分发: Code::SourcePosition / BytecodeArray::SourcePosition)
pub fn source_position(/* this, isolate, offset: i32 */) -> i32 {
  todo!("就地修订: IsCode(map) ? GetCode()->SourcePosition(offset) : GetBytecodeArray()->SourcePosition(offset)")
}

/// 指令起点 (Code: instruction_start; BytecodeArray: GetFirstBytecodeAddress)
pub fn instruction_start(/* this */) -> usize {
  todo!("就地修订: 双分发")
}

/// 指令终点 (Code: instruction_end; BytecodeArray: 首址 + length)
pub fn instruction_end(/* this */) -> usize {
  todo!("就地修订: 双分发")
}

/// 指令体大小 (Code: instruction_size; BytecodeArray: length)
pub fn instruction_size(/* this */) -> i32 {
  todo!("就地修订: 双分发")
}

/// 种类 (BytecodeArray 恒 INTERPRETED_FUNCTION)
pub fn kind(/* this */) /* -> CodeKind */ {
  todo!("就地修订: 双分发")
}

/// 内建号 (BytecodeArray 恒 kNoBuiltinId = -1, int16 符号扩展)
pub fn builtin_id(/* this */) /* -> Builtin */ {
  todo!("就地修订: 双分发")
}

/// 取 Code 支 (原版 GetCode: SbxCast<Code>(TrustedCast<TrustedObject>(this)))
pub fn get_code(/* this */) /* -> Tagged<Code> */ {
  todo!("就地修订: TrustedCast + SbxCast 信任链转换")
}

/// 取 BytecodeArray 支 (同上信任链)
pub fn get_bytecode_array(/* this */) /* -> Tagged<BytecodeArray> */ {
  todo!("就地修订: TrustedCast + SbxCast")
}

/// 联合类型视图 (原版 ToUnionType: TrustedCast<Union<Code, BytecodeArray>>)
pub fn to_union_type(/* this */) /* -> Tagged<Union<Code, BytecodeArray>> */ {
  todo!("就地修订: TrustedCast 直转")
}
`;

const p = writeSrc("v8-objects-rest", "abstract_code.rs", body);
console.log(`abstract-code.h.js: 已生成 ${p} (方法 ${methods.length}, 分发 ${dispatchTotal}, IsCode 分支 ${branchCount})`);
