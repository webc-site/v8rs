#!/usr/bin/env bun
// tool/src/objects/code.h.js — Code 对象族生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/code.h + code.cc + code-inl.h → v8-objects-rest/src/code.rs
//   (CodeKind 枚举本体归 code-kind.h.js → code_kind.rs; builtin 表归 builtins 席)
// 基准配置: 非压缩 (kTaggedSize=8) / 沙箱关 / 恒常池关 (V8_EMBEDDED_CONSTANT_POOL_BOOL=false)
//   / 跳转表信息关 (V8_JUMP_TABLE_INFO_BOOL=false) / gearbox 关
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/code.h");
const inl = readV8("src/objects/code-inl.h");
const cc = readV8("src/objects/code.cc");

let checks = 0;
const need = (cond, msg) => { checks++; if (!cond) throw new Error(msg); };

// —— CODE_DATA_FIELDS X-macro 提取 (V(Name, Size)) ——
const xma = h.match(/#define CODE_DATA_FIELDS\(V\)([\s\S]*?)DEFINE_FIELD_OFFSET_CONSTANTS/);
need(!!xma, "CODE_DATA_FIELDS 未找到");
const clean = xma[1].replace(/\\\n/g, " ").replace(/\/\*[\s\S]*?\*\//g, " ").replace(/\/\/[^\n]*/g, " ");
const SIZEOF = { kTaggedSize: 8, kSystemPointerSize: 8, kUInt32Size: 4, kIntSize: 4, kInt32Size: 4, kUInt16Size: 2, kInt16Size: 2, kJSDispatchHandleSize: 4 };
const fields = [...clean.matchAll(/V\((\w+),\s*((?:[^()]|\([^()]*\))+?)\)/g)].map((m) => {
  let sizeExpr = m[2].trim();
  let size = null;
  const direct = sizeExpr.match(/^(k\w+Size)$/);
  if (direct && SIZEOF[direct[1]] !== undefined) size = SIZEOF[direct[1]];
  else if (sizeExpr === "0") size = 0;
  else if (/V8_EMBEDDED_CONSTANT_POOL_BOOL \? kIntSize : 0/.test(sizeExpr)) size = 0; // 基线关
  else if (/V8_JUMP_TABLE_INFO_BOOL \? kInt32Size : 0/.test(sizeExpr)) size = 0;      // 基线关
  else if (/OBJECT_POINTER_PADDING\(kUnalignedSize\)/.test(sizeExpr)) size = "PAD";
  if (size === null) throw new Error(`CODE_DATA_FIELDS 尺寸不可解析: ${m[1]} = ${sizeExpr}`);
  return { name: m[1], sizeExpr: sizeExpr.replace(/\s+/g, " "), size };
});
need(fields.length === 23, `CODE_DATA_FIELDS 应 23 项, 实得 ${fields.length}`);

// —— 布局求值 (DEFINE_FIELD_OFFSET_CONSTANTS 语义: enum 链隐式 +1) ——
let cursor = 8; // sizeof(ExposedTrustedObject) — 沙箱关 = HeapObject = 8
const layout = [];
let unaligned = 0;
for (const f of fields) {
  if (f.size === "PAD") {
    const pad = ((cursor + 7) & ~7) - cursor;
    layout.push({ ...f, offset: cursor, size: pad });
    cursor += pad;
    unaligned = layout[layout.length - 2].offset + (layout[layout.length - 2].size ?? 0);
    continue;
  }
  layout.push({ ...f, offset: cursor });
  cursor += f.size;
}
const kSize = layout[layout.length - 1].offset;
const kUnaligned = layout.find((f) => f.name === "kUnalignedSize").offset;
// 手数对数 (独立推演): Wrapper@24, InstructionStream@32, InstructionStart@40, Flags@52, BuiltinId@86, kSize=88
const byName = Object.fromEntries(layout.map((f) => [f.name, f]));
need(byName.kWrapperOffset.offset === 24, `kWrapperOffset 应 24, 实得 ${byName.kWrapperOffset.offset}`);
need(byName.kInstructionStreamOffset.offset === 32, `kInstructionStreamOffset 应 32, 实得 ${byName.kInstructionStreamOffset.offset}`);
need(byName.kInstructionStartOffset.offset === 40, `kInstructionStartOffset 应 40, 实得 ${byName.kInstructionStartOffset.offset}`);
need(byName.kFlagsOffset.offset === 52, `kFlagsOffset 应 52, 实得 ${byName.kFlagsOffset.offset}`);
need(byName.kBuiltinIdOffset.offset === 86, `kBuiltinIdOffset 应 86, 实得 ${byName.kBuiltinIdOffset.offset}`);
need(kSize === 88, `kSize 应 88, 实得 ${kSize}`);
need(kUnaligned === 88, `kUnalignedSize 应 88, 实得 ${kUnaligned}`);

// —— FLAGS_BIT_FIELDS (gearbox 关 → 7 段 10 位) ——
const fx = h.match(/#define FLAGS_BIT_FIELDS\(V, _\)([\s\S]*?)DEFINE_BIT_FIELDS/);
need(!!fx, "FLAGS_BIT_FIELDS 未找到");
const fClean = fx[1].replace(/\\\n/g, " ");
const flagBits = [...fClean.matchAll(/V\((\w+), (\w+), (\d+), _\)/g)].map((m) => ({ name: m[1], type: m[2], width: Number(m[3]) }));
need(flagBits.length === 7, `FLAGS_BIT_FIELDS 应 7 段 (gearbox 关), 实得 ${flagBits.length}`);
let shift = 0;
const flagLayout = flagBits.map((b) => { const r = { ...b, shift }; shift += b.width; return r; });
need(shift === 10, `位宽合计应 10, 实得 ${shift}`);
const kMarkedBit = flagLayout.find((b) => b.name === "MarkedForDeoptimizationField").shift;
const kTurbofanBit = flagLayout.find((b) => b.name === "IsTurbofannedField").shift;
need(kMarkedBit === 7, `MarkedForDeoptimization shift 应 7, 实得 ${kMarkedBit}`);
need(kTurbofanBit === 5, `IsTurbofanned shift 应 5, 实得 ${kTurbofanBit}`);
need(/static const int kMarkedForDeoptimizationBit =\s*\n\s*MarkedForDeoptimizationField::kShift;/.test(h), "kMarkedForDeoptimizationBit 定义缺失");

// —— 参数常量 ——
need(/static const int kArgumentsBits = 16;/.test(h), "kArgumentsBits=16 缺失");
need(/static const int kMaxArguments = \(1 << kArgumentsBits\) - 10;/.test(h), "kMaxArguments 公式缺失");
const K_MAX_ARGUMENTS = (1 << 16) - 10;

// —— GcSafeCode 转发面 (code-inl.h X-macro) ——
const gcFwd = [...inl.matchAll(/^GCSAFE_CODE_FWD_ACCESSOR\([\w<>]+, (\w+)\)/gm)].map((m) => m[1]);
need(gcFwd.length === 21, `GcSafeCode 转发面应 21, 实得 ${gcFwd.length}`);

// —— 入口点标签 switch (kind → tag; case 组共享返回值, UNREACHABLE 支不计) ——
const entryCases = [...inl.matchAll(/^((?:    case CodeKind::\w+:\s*\n)+)\s*(?:return (k\w+);|UNREACHABLE\(\);)/gm)]
  .flatMap((m) => [...m[1].matchAll(/case CodeKind::(\w+):/g)].map((k) => ({ kind: k[1], tag: m[2] })))
  .filter((c) => c.tag);
need(entryCases.length >= 12, `entrypoint_tag case 应 ≥12, 实得 ${entryCases.length}`);
need(entryCases.some((c) => c.kind === "BYTECODE_HANDLER" && c.tag === "kBytecodeHandlerEntrypointTag"), "BYTECODE_HANDLER→kBytecodeHandlerEntrypointTag 缺失");
need(entryCases.some((c) => c.kind === "REGEXP" && c.tag === "kRegExpEntrypointTag"), "REGEXP→kRegExpEntrypointTag 缺失");
need(entryCases.some((c) => c.kind === "BASELINE" && c.tag === "kJSEntrypointTag"), "BASELINE→kJSEntrypointTag 缺失");

// —— CodeKindHasTaggedOutgoingParams ——
need(/kind != CodeKind::JS_TO_WASM_FUNCTION &&\s*\n\s*kind != CodeKind::C_WASM_ENTRY && kind != CodeKind::WASM_FUNCTION;/.test(inl),
  "CodeKindHasTaggedOutgoingParams 三排除缺失");

// —— CodeWrapper / 元数据偏移家族 (inl 面计数) ——
const metaFns = ["safepoint_table_address", "safepoint_table_size", "handler_table_address", "handler_table_size",
  "constant_pool", "constant_pool_size", "code_comments", "code_comments_size", "jump_table_info", "jump_table_info_size",
  "unwinding_info_start", "unwinding_info_end", "unwinding_info_size", "metadata_start", "metadata_end"];
for (const fn of metaFns) need(new RegExp(`Code::${fn}\\(`).test(inl), `元数据访问 ${fn} 缺失`);
const codeDefs = (inl.match(/^(?:inline )?(?:void|bool|int|unsigned|uint\w+|Address|Tagged<[\w<>]+>|CodeKind|Builtin|BytecodeOffset|uintptr_t|CodeSandboxingMode|uint8_t\*) Code::\w+\(/gm) || []).length;
need(codeDefs >= 60, `code-inl.h Code:: 定义应 ≥60, 实得 ${codeDefs}`);
const ccDefs = (cc.match(/^(?:void|int|bool|Tagged<[\w<>]+>) Code::\w+\(/gm) || []).length;
need(ccDefs >= 10, `code.cc Code:: 定义应 ≥10, 实得 ${ccDefs}`);

const body = `${banner("src/objects/code.h.js", "src/objects/code.{h,cc,-inl.h}", "基准: 非压缩/沙箱关/恒常池关/跳转表关/gearbox 关")}
//! Code 对象族 (原版 code.{h,cc,-inl.h})
//!
//! Code 是 InstructionStream 的元数据容器 (InstructionStream 在写保护页上,
//! 头部不可变); 内建 Code 无 InstructionStream, 指令与元数据在离堆嵌入区.

// ===== 布局 (CODE_DATA_FIELDS × DEFINE_FIELD_OFFSET_CONSTANTS, ${fields.length} 项) =====
/// Code 字段偏移表 (基准配置求值; Size 列为字节, PAD 项 = OBJECT_POINTER_PADDING)
pub const CODE_FIELDS: [(&str, usize, usize, &str); ${layout.length}] = [
  // (字段, 偏移, 尺寸, 原版尺寸表达式)
${layout.map((f) => `  ("${f.name}", ${f.offset}, ${f.size}, "${f.sizeExpr.replace(/"/g, "'")}"),`).join("\n")}
];

/// 对象总大小 (基准配置 kSize)
pub const K_SIZE: usize = ${kSize};

/// 尾部对齐前大小 (kUnalignedSize; 基准下恰已对齐, PAD=0)
pub const K_UNALIGNED_SIZE: usize = ${kUnaligned};

// ===== 标志位 (FLAGS_BIT_FIELDS × DEFINE_BIT_FIELDS, ${flagLayout.length} 段共 ${shift} 位) =====
/// flags 位域布局 (LSB 起)
pub const CODE_FLAG_BITS: [(&str, &str, u32, u32); ${flagLayout.length}] = [
  // (段名, 值类型, shift, width)
${flagLayout.map((b) => `  ("${b.name}", "${b.type}", ${b.shift}, ${b.width}),`).join("\n")}
];

/// 标记反优化位 (生成代码直访, 原版 kMarkedForDeoptimizationBit = MarkedForDeoptimizationField::kShift)
pub const K_MARKED_FOR_DEOPTIMIZATION_BIT: u32 = ${kMarkedBit};

/// TurboFan 位 (原版 kIsTurbofannedBit)
pub const K_IS_TURBOFANNED_BIT: u32 = ${kTurbofanBit};

/// 参数计数字宽 (原版 kArgumentsBits)
pub const K_ARGUMENTS_BITS: u32 = 16;

/// 最大参数数 (原版 kMaxArguments = 2^16 - 10, 预留隐式实参余量)
pub const K_MAX_ARGUMENTS: u32 = ${K_MAX_ARGUMENTS};

// ===== GcSafeCode (GC 中安全访问面, 原样转发 ${gcFwd.length} 法) =====
/// GcSafeCode 转发清单 (GCSAFE_CODE_FWD_ACCESSOR X-macro 展开)
pub const GCSAFE_CODE_FORWARDERS: [&str; ${gcFwd.length}] = [${gcFwd.map((n) => `"${n}"`).join(", ")}];

// ===== 方法面 =====
/// flags 松弛读 (原版 RELAXED_UINT32_ACCESSORS)
pub fn flags(/* this */) -> u32 {
  todo!("就地修订: Relaxed_ReadField(kFlagsOffset)")
}

/// 种类 (原版 kind: KindField::decode(flags(kRelaxedLoad)), 低 4 位)
pub fn kind(/* this */) /* -> CodeKind */ {
  todo!("就地修订: (flags() >> 0) & 0xf")
}

/// 内建号 (int16 存储依赖符号扩展保 -1/kNoBuiltinId)
pub fn builtin_id(/* this */) /* -> Builtin */ {
  todo!("就地修订: ReadField<int16_t>(kBuiltinIdOffset) 符号扩展")
}

/// 是否内建 (builtin_id != kNoBuiltinId)
pub fn is_builtin(/* this */) -> bool {
  todo!("就地修订: builtin_id() != -1")
}

/// 入口点标签 (原版 entrypoint_tag switch; 沙箱开时入口地址存 tag 异或值)
///
/// 映射表 (CodeKind → CodeEntrypointTag, BUILTIN 支按禁用位/内建表细分):
pub const ENTRYPOINT_TAGS: [(&str, &str); ${entryCases.length}] = [
${entryCases.map((c) => `  ("${c.kind}", "${c.tag}"),`).join("\n")}
];

/// 指令起点 (原版 instruction_start: 沙箱开读 tag 异或值, 关直读)
pub fn instruction_start(/* this */) -> usize {
  todo!("就地修订: ReadField<Address>(K_INSTRUCTION_START_OFFSET)")
}

/// 指令终点 (instruction_start + instruction_size)
pub fn instruction_end(/* this */) -> usize {
  todo!("就地修订: instruction_start() + instruction_size()")
}

/// 元数据基址 (原版 metadata_start: 有指令流 = 指令尾; 内建 = 嵌入 blob 对应段)
pub fn metadata_start(/* this */) -> usize {
  todo!("就地修订: has_instruction_stream ? instruction_end : EmbeddedData::MetadataStartOf(builtin)")
}

/// safepoint 表区间 (offset 0 起; size = handler_table_offset - 0)
pub fn safepoint_table_span(/* this */) -> (usize, i32) {
  todo!("就地修订: (metadata_start + 0, handler_table_offset - safepoint_table_offset)")
}

/// handler 表区间 (size = constant_pool_offset - handler_table_offset)
pub fn handler_table_span(/* this */) -> (usize, i32) {
  todo!("就地修订: 恒常池关时 constant_pool_offset 重定向 code_comments_offset")
}

/// 恒常池 (恒常池关配置: size 恒 0, offset 重定向 code_comments_offset)
pub fn constant_pool(/* this */) -> usize {
  todo!("就地修订: !V8_EMBEDDED_CONSTANT_POOL_BOOL → has_constant_pool 恒 false")
}

/// 栈槽数 (原版 stack_slots: safepoint 表首字段 SafepointTableStackSlotsField)
pub fn stack_slots(/* this */) -> u32 {
  todo!("就地修订: safepoint_table_size()==0 → 0; 否则读表首字段")
}

/// 参数计数 (无接收者版 = -1)
pub fn parameter_count_without_receiver(/* this */) -> u16 {
  todo!("就地修订: parameter_count() - 1")
}

/// 出参是否 tagged (原版 CodeKindHasTaggedOutgoingParams: 排除 JS_TO_WASM/C_WASM_ENTRY/WASM_FUNCTION)
pub fn code_kind_has_tagged_outgoing_params(kind: &str) -> bool {
  !matches!(kind, "JS_TO_WASM_FUNCTION" | "C_WASM_ENTRY" | "WASM_FUNCTION")
}

/// wasm-js 包装参数计数 (复用 InlinedBytecodeSize 32 位字段, 两个 16 位半区)
pub fn wasm_js_tagged_params(/* this */) -> (u16, u16) {
  todo!("就地修订: (read_u16(kInlinedBytecodeSizeOffset), read_u16(+2))")
}

/// 弱对象资格 (原版 IsWeakObjectInOptimizedCode: Map(可迁移)/PropertyCell/JSReceiver/Context)
pub fn is_weak_object_in_optimized_code(/* object: Tagged<HeapObject> */) -> bool {
  todo!("就地修订: IsMap→CanTransition; 否则 PropertyCell||JSReceiver||Context")
}

/// 去优字面量弱资格 (Map 必须强持有 — 物化描述可能引用)
pub fn is_weak_object_in_deoptimization_literal_array(/* object */) -> bool {
  todo!("就地修订: IsHeapObject && !IsMap && is_weak_object_in_optimized_code")
}

/// 反优化字面量遍历 (原版 IterateDeoptimizationLiterals: 逐弱项 VisitRootPointer)
pub fn iterate_deoptimization_literals(/* this, v: &mut RootVisitor */) {
  todo!("就地修订: uses_deoptimization_data 空转出; 逐 get_raw 弱项派发")
}

/// 设置指令流与起点 (原版 SetInstructionStreamAndInstructionStart)
pub fn set_instruction_stream_and_instruction_start(/* this, isolate, istream, mode */) {
  todo!("就地修订: set_raw_instruction_stream + set_instruction_start(istream->instruction_start())")
}

/// 标志初始化 (原版 initialize_flags: Kind|IsContextSpecialized|IsTurbofanned 三段装配)
pub fn initialize_flags(/* this, kind, is_context_specialized, is_turbofanned */) {
  todo!("就地修订: KindField::encode(kind) | IsContextSpecializedField | IsTurbofannedField")
}

/// 序列化前清入口 (原版 ClearInstructionStartForSerialization: 沙箱下清自间接句柄 + 入口置 0 + 指令流槽置 Smi::zero())
pub fn clear_instruction_start_for_serialization(/* this, isolate */) {
  todo!("就地修订: 按沙箱开关双路径")
}

/// 清尾填充 (快照确定性: kSize - kUnalignedSize 区间清零; 基线为 0 直接返回)
pub fn clear_padding(/* this */) {
  todo!("就地修订: fill_n(address + K_UNALIGNED_SIZE, K_SIZE - K_UNALIGNED_SIZE, 0)")
}

/// 源位置查询 (原版 code.cc SourcePosition: 起始 PC 差 1 修正 + 表迭代)
pub fn source_position(/* this, offset: i32 */) -> i32 {
  todo!("就地修订: CHECK_NE(kind, BASELINE); offset-1; SourcePositionTableIterator 扫描")
}

/// 是否 isolate 无关 (原版 IsIsolateIndependent: 重定位模式掩码校验)
pub fn is_isolate_independent(/* this, isolate */) -> bool {
  todo!("就地修订: WritableRelocIterator 全扫, 命中需重定位模式即 false")
}

/// 清嵌入对象与派发句柄 (原版 ClearEmbeddedObjectsAndJSDispatchHandles: 模式掩码双家族)
pub fn clear_embedded_objects_and_js_dispatch_handles(/* this, heap */) {
  todo!("就地修订: EmbeddedObjectModeMask | JSDispatchHandleModeMask 迭代置空")
}

// ===== CodeWrapper =====
/// Code 受信指针槽访问 (原版 CodeWrapper::code ×2 读序 + set ×2 + has/clear)
pub fn code_wrapper_accessors() {
  todo!("就地修订: TrustedPointerMember<kCodeIndirectPointerTag> 五法")
}

/// 沙箱模式 (原版 sandboxing_mode: 内建查 Builtins::SandboxingModeOf, 其余 kSandboxed)
pub fn sandboxing_mode(/* this */) /* -> CodeSandboxingMode */ {
  todo!("就地修订: 双分支")
}
`;

const p = writeSrc("v8-objects-rest", "code.rs", body);
console.log(`code.h.js: 已生成 ${p} (must ${checks}: 布局 ${layout.length} 项 K_SIZE=${kSize}, 位段 ${flagLayout.length}/${shift}位, GcSafe 转发 ${gcFwd.length}, 入口 tag ${entryCases.length}, inl 定义 ${codeDefs}, cc 定义 ${ccDefs})`);
