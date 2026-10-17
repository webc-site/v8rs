// 由 tool/src/builtins/wasm.tq.js 从 v8/src/builtins/wasm.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 83 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "runtime", "namespace runtime"),
  ("namespace", "unsafe", "namespace unsafe"),
  ("namespace", "wasm", "namespace wasm"),
  (
    "const",
    "trustedData",
    "const trustedData: WasmTrustedInstanceData = LoadInstanceDataFromFrame()",
  ),
  (
    "const",
    "dst",
    "const dst: Smi = Convert<PositiveSmi>(dstRaw) otherwise TableOutOfBounds",
  ),
  (
    "const",
    "src",
    "const src: Smi = Convert<PositiveSmi>(srcRaw) otherwise TableOutOfBounds",
  ),
  (
    "const",
    "size",
    "const size: Smi = Convert<PositiveSmi>(sizeRaw) otherwise TableOutOfBounds",
  ),
  (
    "const",
    "trustedData",
    "const trustedData: WasmTrustedInstanceData = LoadInstanceDataFromFrame()",
  ),
  (
    "const",
    "dst",
    "const dst: Smi = Convert<PositiveSmi>(dstRaw) otherwise TableOutOfBounds",
  ),
  (
    "const",
    "src",
    "const src: Smi = Convert<PositiveSmi>(srcRaw) otherwise TableOutOfBounds",
  ),
  (
    "const",
    "size",
    "const size: Smi = Convert<PositiveSmi>(sizeRaw) otherwise TableOutOfBounds",
  ),
  (
    "const",
    "trustedData",
    "const trustedData: WasmTrustedInstanceData = LoadInstanceDataFromFrame()",
  ),
  (
    "const",
    "start",
    "const start: Smi = Convert<PositiveSmi>(startRaw) otherwise TableOutOfBounds",
  ),
  (
    "const",
    "count",
    "const count: Smi = Convert<PositiveSmi>(countRaw) otherwise TableOutOfBounds",
  ),
  (
    "const",
    "trustedData",
    "const trustedData: WasmTrustedInstanceData = LoadInstanceDataFromFrame()",
  ),
  (
    "const",
    "delta",
    "const delta: Smi = Convert<PositiveSmi>(deltaRaw) otherwise TableOutOfBounds",
  ),
  (
    "const",
    "tables",
    "const tables: FixedArray = LoadTablesFromInstanceData(trustedData)",
  ),
  (
    "const",
    "table",
    "const table: WasmTableObject = %RawDownCast<WasmTableObject>( LoadFixedArrayElement(tables, tableIndex))",
  ),
  (
    "const",
    "entriesCount",
    "const entriesCount: uintptr = Unsigned(ChangeInt32ToIntPtr(SmiToInt32(table.current_length)))",
  ),
  (
    "const",
    "entries",
    "const entries: FixedArray = table.entries",
  ),
  (
    "const",
    "entry",
    "const entry: Object = LoadFixedArrayElement(entries, index)",
  ),
  (
    "const",
    "tables",
    "const tables: FixedArray = LoadTablesFromInstanceData(trustedData)",
  ),
  (
    "const",
    "table",
    "const table: WasmTableObject = %RawDownCast<WasmTableObject>( LoadFixedArrayElement(tables, tableIndex))",
  ),
  (
    "const",
    "entriesCount",
    "const entriesCount: uintptr = Unsigned(ChangeInt32ToIntPtr(SmiToInt32(table.current_length)))",
  ),
  (
    "const",
    "entries",
    "const entries: FixedArray = table.entries",
  ),
  (
    "const",
    "tables",
    "const tables: FixedArray = LoadTablesFromInstanceData(trustedData)",
  ),
  (
    "const",
    "table",
    "const table: WasmTableObject = %RawDownCast<WasmTableObject>( LoadFixedArrayElement(tables, tableIndex))",
  ),
  (
    "const",
    "entriesCount",
    "const entriesCount: uintptr = Unsigned(ChangeInt32ToIntPtr(SmiToInt32(table.current_length)))",
  ),
  (
    "const",
    "entries",
    "const entries: FixedArray = table.entries",
  ),
  (
    "const",
    "entry",
    "const entry: HeapObject = UnsafeCast<HeapObject>(LoadFixedArrayElement(entries, index))",
  ),
  (
    "const",
    "tables",
    "const tables: FixedArray = LoadTablesFromInstanceData(trustedData)",
  ),
  (
    "const",
    "table",
    "const table: WasmTableObject = %RawDownCast<WasmTableObject>( LoadFixedArrayElement(tables, tableIndex))",
  ),
  (
    "const",
    "entriesCount",
    "const entriesCount: uintptr = Unsigned(ChangeInt32ToIntPtr(SmiToInt32(table.current_length)))",
  ),
  (
    "const",
    "funcRefs",
    "const funcRefs: FixedArray = LoadFuncRefsFromInstanceData(instance)",
  ),
  (
    "const",
    "funcref",
    "const funcref: Object = funcRefs.objects[index]",
  ),
  (
    "const",
    "smiOffset",
    "const smiOffset = Convert<PositiveSmi>(offset) otherwise SegmentOutOfBounds",
  ),
  (
    "const",
    "smiLength",
    "const smiLength = Convert<PositiveSmi>(length) otherwise ArrayTooLarge",
  ),
  (
    "const",
    "smiArrayIndex",
    "const smiArrayIndex = Convert<PositiveSmi>(arrayIndex) otherwise ArrayOutOfBounds",
  ),
  (
    "const",
    "smiOffset",
    "const smiOffset = Convert<PositiveSmi>(segmentOffset) otherwise SegmentOutOfBounds",
  ),
  (
    "const",
    "smiLength",
    "const smiLength = Convert<PositiveSmi>(length) otherwise ArrayOutOfBounds",
  ),
  (
    "const",
    "count",
    "const count = UnsafeCast<Smi>(secondSlot) + SmiConstant(1)",
  ),
  (
    "const",
    "entries",
    "const entries = UnsafeCast<FixedArray>(firstSlot)",
  ),
  (
    "const",
    "entries",
    "const entries = UnsafeCast<FixedArray>(firstSlot)",
  ),
  (
    "const",
    "kMaxSlots",
    "const kMaxSlots = kMaxPolymorphism * 2",
  ),
  (
    "const",
    "newEntries",
    "const newEntries = UnsafeCast<FixedArray>(AllocateFixedArray( ElementsKind::PACKED_ELEMENTS, 4, AllocationFlag::kNone))",
  ),
  (
    "const",
    "error",
    "const error = MessageTemplate::kWasmTrapArrayOutOfBounds",
  ),
  (
    "const",
    "raw",
    "const raw: char16 = *torque_internal::unsafe::NewReference<char16>( this.object, this.start)",
  ),
  (
    "const",
    "result",
    "const result: char8 = %RawDownCast<char8>(raw & 0xFF)",
  ),
  ("const", "intptrLength", "const intptrLength = slice.length"),
  (
    "const",
    "eightElementLoopEnd",
    "const eightElementLoopEnd = intptrLength - 8",
  ),
  ("const", "length", "const length: uint32 = end - start"),
  (
    "const",
    "intptrLength",
    "const intptrLength = Convert<intptr>(length)",
  ),
  (
    "const",
    "arrayContent",
    "const arrayContent = torque_internal::unsafe::NewConstSlice<char16>( array, kWasmUnsharedArrayHeaderSize, Convert<intptr>(array.length))",
  ),
  (
    "const",
    "substring",
    "const substring = Subslice(arrayContent, Convert<intptr>(start), intptrLength) otherwise goto OffsetOutOfRange",
  ),
  (
    "const",
    "context",
    "const context = LoadContextFromWasmOrJsFrame()",
  ),
  (
    "const",
    "error",
    "const error = MessageTemplate::kWasmTrapArrayOutOfBounds",
  ),
  (
    "const",
    "context",
    "const context = LoadContextFromWasmOrJsFrame()",
  ),
  (
    "const",
    "segmentOffsetU",
    "const segmentOffsetU: uint32 = Unsigned(SmiToInt32(segmentOffset))",
  ),
  (
    "const",
    "arrayLength",
    "const arrayLength = arrayEnd - arrayStart",
  ),
  (
    "const",
    "smiOffset",
    "const smiOffset = Convert<PositiveSmi>(segmentOffsetU + arrayStart) otherwise SegmentOOB",
  ),
  (
    "const",
    "smiLength",
    "const smiLength = Convert<PositiveSmi>(arrayLength) otherwise SegmentOOB",
  ),
  (
    "const",
    "headerSize",
    "const headerSize: intptr = InSharedSpace(array) ? Convert<intptr>(kWasmSharedArrayHeaderSize) : Convert<intptr>(kWasmUnsharedArrayHeaderSize)",
  ),
  (
    "const",
    "byteOffset",
    "const byteOffset: intptr = headerSize + torque_internal::TimesSizeOf<char16>(Convert<intptr>(start))",
  ),
  (
    "const",
    "arrayContent",
    "const arrayContent = torque_internal::unsafe::NewMutableSlice<char16>( array, byteOffset, string.length_intptr)",
  ),
  (
    "const",
    "error",
    "const error = MessageTemplate::kWasmTrapArrayOutOfBounds",
  ),
  (
    "const",
    "code",
    "const code: char16 = StringCharCodeAt(string, Convert<uintptr>(offset))",
  ),
  (
    "const",
    "error",
    "const error = MessageTemplate::kWasmTrapStringOffsetOutOfBounds",
  ),
  (
    "const",
    "lead",
    "const lead: char16 = StringCharCodeAt(string, Convert<uintptr>(offset))",
  ),
  ("const", "trailOffset", "const trailOffset = offset + 1"),
  (
    "const",
    "trail",
    "const trail: char16 = StringCharCodeAt(string, Convert<uintptr>(trailOffset))",
  ),
  (
    "const",
    "error",
    "const error = MessageTemplate::kWasmTrapStringOffsetOutOfBounds",
  ),
  (
    "const",
    "code2",
    "const code2: char16 = StringCharCodeAt(string, Convert<uintptr>(offset + 1))",
  ),
  ("const", "smi", "const smi = SmiFromInt32(x)"),
  ("const", "untagged", "const untagged = SmiToInt32(smi)"),
  (
    "const",
    "arrayIndex",
    "const arrayIndex: uint32 = DecodeArrayIndexFromHashField(hash)",
  ),
  (
    "const",
    "rec",
    "const rec = Cast<JSReceiver>(receiver) otherwise goto IllegalCast",
  ),
  (
    "const",
    "error",
    "const error = MessageTemplate::kIllegalInvocation",
  ),
  (
    "const",
    "heapObject",
    "const heapObject: HeapObject = Cast<HeapObject>(obj) otherwise ReturnFalse",
  ),
  (
    "const",
    "heapObject",
    "const heapObject: HeapObject = Cast<HeapObject>(obj) otherwise ReturnFalse",
  ),
  (
    "const",
    "heapObject",
    "const heapObject: HeapObject = Cast<HeapObject>(obj) otherwise ReturnFalse",
  ),
  (
    "const",
    "heapObject",
    "const heapObject: HeapObject = UnsafeCast<HeapObject>(obj)",
  ),
  (
    "const",
    "heapObject",
    "const heapObject: HeapObject = Cast<HeapObject>(obj) otherwise IllegalCast",
  ),
  (
    "const",
    "heapObject",
    "const heapObject: HeapObject = Cast<HeapObject>(obj) otherwise IllegalCast",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 83);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 3);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 80);
};
