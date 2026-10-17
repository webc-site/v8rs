#!/usr/bin/env bun
// tool/src/objects/deoptimization-data-inl.h.js — 反优化数据内联访问器生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/deoptimization-data-inl.h + deoptimization-data.cc (布局常量复对)
//   → v8-objects-rest/src/deoptimization_data_inl.rs
//   (deoptimization-data.{h,cc} 主体面归 tool/src/objects/deoptimization-data.h.js → deoptimization_data.rs;
//    本脚本落 -inl 的访问器 X-macro 展开与字面量数组语义)
import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/deoptimization-data-inl.h");
const h = readV8("src/objects/deoptimization-data.h");
const cc = readV8("src/objects/deoptimization-data.cc");

// —— DEFINE_DEOPT_ELEMENT_ACCESSORS 展开 (元素槽, 定义序) ——
const elems = [...inl.matchAll(/^DEFINE_DEOPT_ELEMENT_ACCESSORS\((\w+),\s*([\w<>]+)\)/gm)].map((m) => ({ slot: m[1], type: m[2] }));
const ELEM_EXPECT = [
  ["FrameTranslation", "DeoptimizationFrameTranslation"], ["InlinedFunctionCount", "Smi"],
  ["ProtectedLiteralArray", "ProtectedDeoptimizationLiteralArray"], ["LiteralArray", "DeoptimizationLiteralArray"],
  ["OsrBytecodeOffset", "Smi"], ["OsrPcOffset", "Smi"], ["OptimizationId", "Smi"],
  ["WrappedSharedFunctionInfo", "SharedFunctionInfoWrapperOrSmi"], ["InliningPositions", "TrustedPodArray<InliningPosition>"],
  ["DeoptExitStart", "Smi"], ["EagerDeoptCount", "Smi"], ["LazyDeoptCount", "Smi"],
];
if (elems.length !== ELEM_EXPECT.length || elems.some((e, i) => e.slot !== ELEM_EXPECT[i][0] || e.type !== ELEM_EXPECT[i][1]))
  throw new Error(`元素访问器表漂移 (应 ${ELEM_EXPECT.length}):\n  ${elems.map((e) => `${e.slot}:${e.type}`).join(", ")}`);

// —— DEFINE_DEOPT_ENTRY_ACCESSORS (DEBUG 下 4: 含 NodeId) ——
const entries = [...inl.matchAll(/^DEFINE_DEOPT_ENTRY_ACCESSORS\((\w+),\s*(\w+)\)/gm)].map((m) => m[2]);
if (entries.join() !== "Smi,Smi,Smi,Smi") throw new Error(`条目访问器应 4 (DEBUG 含 NodeId), 实得 ${entries}`);
const entrySlots = [...inl.matchAll(/^DEFINE_DEOPT_ENTRY_ACCESSORS\((\w+),/gm)].map((m) => m[1]);
if (entrySlots.join() !== "BytecodeOffsetRaw,TranslationIndex,Pc,NodeId") throw new Error(`条目槽序漂移: ${entrySlots}`);

// —— 布局常量复对 (h 侧权威值, -inl 语义依赖) ——
const kFirstDeoptEntryIndex = h.match(/kFirstDeoptEntryIndex = (\d+)/)?.[1];
if (!kFirstDeoptEntryIndex) throw new Error("kFirstDeoptEntryIndex 缺失");
const entrySizes = [...h.matchAll(/kDeoptEntrySize = (\d+)/g)].map((m) => m[1]);
if (kFirstDeoptEntryIndex !== "12") throw new Error(`kFirstDeoptEntryIndex 应 12, 实得 ${kFirstDeoptEntryIndex}`);
if (entrySizes.join() !== "4,3") throw new Error(`kDeoptEntrySize 应 DEBUG 4 / release 3, 实得 ${entrySizes}`);
const kDeoptEntrySize = "3"; // 基线取 release 3 槽 (DEBUG 4 槽含 NodeId)

// —— 语义锚 ——
if (!/return \(len - kFirstDeoptEntryIndex\) \/ kDeoptEntrySize;/.test(inl)) throw new Error("DeoptCount 公式缺失");
if (!/BytecodeOffset\(BytecodeOffsetRaw\(i\)\.value\(\)\)/.test(inl)) throw new Error("GetBytecodeOffsetOrBuiltinContinuationId 缺失");
if (!/TrustedCast<i::SharedFunctionInfoWrapper>\(WrappedSharedFunctionInfo\(\)\)/.test(inl)) throw new Error("GetSharedFunctionInfo 信任链缺失");
if (!/CHECK\(!maybe\.IsCleared\(\)\);/.test(inl)) throw new Error("字面量数组不得取到已清除弱项断言缺失");
if (!/maybe = TrustedCast<BytecodeArray>\(value\)->wrapper\(\);/.test(inl)) throw new Error("BytecodeArray 字面量换 wrapper 语义缺失");
if (/Is<RegExpData>\(value\)/.test(inl) && !/TrustedCast<RegExpData>\(value\)->wrapper\(\);/.test(inl)) throw new Error("RegExpData 换 wrapper 语义缺失");
if (!/maybe = MakeWeak\(Cast<HeapObject>\(maybe\)\);/.test(inl)) throw new Error("弱化存储语义缺失");

// —— cc 侧复对 (主表已在 deoptimization_data.rs; 此处只对计数) ——
const ccStaticAsserts = (cc.match(/static_assert\(/g) || []).length;
if (ccStaticAsserts < 3) throw new Error(`deoptimization-data.cc static_assert 应 ≥3, 实得 ${ccStaticAsserts}`);

const body = `${banner("src/objects/deoptimization-data-inl.h.js", "src/objects/deoptimization-data-inl.h (+ deoptimization-data.cc 复对)", "访问器 X-macro 展开; 主表面在 deoptimization_data.rs")}
//! 反优化数据内联访问器 (原版 deoptimization-data-inl.h)
//!
//! DeoptimizationData 是 ProtectedFixedArray 的索引型前端:
//! 头部 ${elems.length} 个固定元素槽 (kFirstDeoptEntryIndex=${kFirstDeoptEntryIndex}) + 每 deopt 出口 3 槽的条目区
//! (DEBUG 增 NodeId 第 4 槽).

/// 元素槽表 (DEFINE_DEOPT_ELEMENT_ACCESSORS 展开, ${elems.length} 槽, 索引即数组下标)
pub const DEOPT_ELEMENT_SLOTS: [(&str, &str); ${elems.length}] = [
${elems.map((e) => `  ("${e.slot}", "${e.type}"),`).join("\n")}
];

/// 条目槽 (DEFINE_DEOPT_ENTRY_ACCESSORS 展开, DEBUG 全集 4 槽)
pub const DEOPT_ENTRY_SLOTS: [(&str, &str); ${entrySlots.length}] = [
${entrySlots.map((s, i) => `  ("${s}", "${entries[i]}"),`).join("\n")}
];

/// 首个 deopt 条目的数组下标 (原版 kFirstDeoptEntryIndex)
pub const K_FIRST_DEOPT_ENTRY_INDEX: usize = ${kFirstDeoptEntryIndex};

/// 单条目槽数 (原版 kDeoptEntrySize: release 3 槽; DEBUG 4 槽含 NodeId, 基线取 release)
pub const K_DEOPT_ENTRY_SIZE: usize = ${kDeoptEntrySize};

/// 反优化出口计数 (原版 DeoptCount: (len - kFirstDeoptEntryIndex) / kDeoptEntrySize)
pub fn deopt_count(/* this */) -> u32 {
  todo!("就地修订: (ulength - K_FIRST_DEOPT_ENTRY_INDEX) / K_DEOPT_ENTRY_SIZE")
}

/// 取共享函数信息 (经 SharedFunctionInfoWrapper 信任链)
pub fn get_shared_function_info(/* this */) /* -> Tagged<SharedFunctionInfo> */ {
  todo!("就地修订: TrustedCast<SharedFunctionInfoWrapper>(WrappedSharedFunctionInfo)->shared_info()")
}

/// 出口 i 的字节码偏移 (原版 GetBytecodeOffsetOrBuiltinContinuationId: BytecodeOffset 原值)
pub fn get_bytecode_offset_or_builtin_continuation_id(/* this, i */) /* -> BytecodeOffset */ {
  todo!("就地修订: BytecodeOffset(BytecodeOffsetRaw(i).value())")
}

// ===== DeoptimizationLiteralArray (弱持有字面量) =====
/// 字面量读 (原版 get: 弱槽取强值; 已清除 = 不可达, CHECK 失败)
pub fn literal_get(/* this, index */) /* -> Tagged<Object> */ {
  todo!("就地修订: TrustedWeakFixedArray::get 后 CHECK(!IsCleared) 再 GetHeapObjectOrSmi")
}

/// 字面量原始弱读 (原版 get_raw: 不做清除检查)
pub fn literal_get_raw(/* this, index */) /* -> Tagged<MaybeObject> */ {
  todo!("就地修订: 直转 TrustedWeakFixedArray::get")
}

/// 字面量写 (三重换装规则: BytecodeArray→wrapper / 沙箱开 RegExpData→wrapper /
/// 可弱对象→MakeWeak; 其余强存)
pub fn literal_set(/* this, index, value */) {
  todo!("就地修订: 三分支换装后 TrustedWeakFixedArray::set")
}
`;

const p = writeSrc("v8-objects-rest", "deoptimization_data_inl.rs", body);
console.log(`deoptimization-data-inl.h.js: 已生成 ${p} (元素槽 ${elems.length}, 条目槽 ${entrySlots.length}, cc static_assert ${ccStaticAsserts})`);
