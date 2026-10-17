#!/usr/bin/env bun
// tool/src/objects/fixed-primitive-array.h.js — FixedPrimitiveArray 族签名骨架生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/fixed-primitive-array.h → v8-objects-arrays/src/primitive_array.rs
//   语义实现 (get/set/Allocate/New 族) 由 fixed-primitive-array-inl.h.js 出合并指引, 就地修订
// 产物: v8-objects-arrays/src/primitive_array.rs.skel (差集骨架; 最终 src 以 Developer 修订为准)
// 跨席依赖申报: FixedArrayBase/TrustedFixedArrayBase 实现归 AA-1 席 (fixed_array_base.rs);
//   本脚本仅读取 v8/src/objects/fixed-array-base.h 锚定 length_/optional_padding_ 字段布局,
//   不 import 其未落地符号, Rust 侧以本地最小头结构 FixedArrayHeader 表达继承形态。
// 配置登记: 64 位非压缩 (objects-core tagged_field.rs 落地形态 TaggedSlot=usize) →
//   kTaggedSize=8, TAGGED_SIZE_8_BYTES=true → FixedArrayBase 头 = map(8)+length_(4)+padding_(4)=16。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/fixed-primitive-array.h");
const base = readV8("src/objects/fixed-array-base.h"); // 只读锚点 (AA-1 管辖文件, 不写)
const globals = readV8("src/common/globals.h");

// ── 锚点断言计数器 ────────────────────────────────────────────────────────
let checks = 0;
const must = (cond, msg) => {
  checks++;
  if (!cond) throw new Error(`锚点失配: ${msg}`);
};
const has = (s, sub, file, line) => {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${file ?? line} 无 "${sub.slice(0, 64)}"`);
  checks++;
};

// ── 1. 类全集对数 (6 类) ─────────────────────────────────────────────────
const classNames = [...h.matchAll(/class (\w+)/g)].map((m) => m[1]);
for (const need of [
  "PrimitiveArrayBase",
  "FixedDoubleArray",
  "ByteArray",
  "TrustedByteArray",
  "FixedIntegerArrayBase",
  "TrustedFixedAddressArray",
])
  must(classNames.includes(need), `fixed-primitive-array.h 缺 class ${need}`);

// ── 2. PrimitiveArrayBase 方法全集对数 (15) ──────────────────────────────
const pab = h.slice(h.indexOf("class PrimitiveArrayBase"), h.indexOf("// FixedDoubleArray"));
const pabMethods = [
  "capacity", "set_capacity", "clear_optional_padding", "get", "set",
  "AllocatedSize", "SizeFor", "OffsetOfElementAt", "begin", "end",
  "DataSize", "FromAddressOfFirstElement", "MaxRegularLength", "Allocate", "IsInBounds",
];
for (const m of pabMethods)
  must(new RegExp(`\\b${m}\\b\\s*\\(`).test(pab), `PrimitiveArrayBase.${m} 缺失`);

// ── 3. 元素类型与 static_assert 锚点 ─────────────────────────────────────
has(pab, "static_assert(std::is_base_of_v<HeapObject, Super>)", "fixed-primitive-array.h");
has(pab, "static_assert(!is_subtype_v<ElementT_, Object>)", "fixed-primitive-array.h");
has(pab, "static_assert(alignof(ElementMemberT) <= alignof(Tagged_t))", "fixed-primitive-array.h");
has(pab, "static constexpr int kElementSize = sizeof(ElementMemberT);", "fixed-primitive-array.h");
has(pab, "using ElementMemberT = std::conditional_t<std::is_same_v<ElementT_, double>,\n" +
  "                                            UnalignedDoubleMember, ElementT_>;", "fixed-primitive-array.h");
has(pab, "static constexpr uint32_t kMaxLength = kMaxFixedArrayCapacity;", "fixed-primitive-array.h");

// ── 4. 子类成员全集对数 ──────────────────────────────────────────────────
const fda = h.slice(h.indexOf("class FixedDoubleArray"), h.indexOf("// ByteArray represents"));
for (const m of ["New", "get_scalar", "get_representation", "get", "set", "set_raw",
  "set_undefined", "is_undefined", "set_the_hole", "is_the_hole", "MoveElements", "FillWithHoles"])
  must(new RegExp(`\\b${m}\\b\\s*\\(`).test(fda), `FixedDoubleArray.${m} 缺失`);
has(fda, "FLEXIBLE_ARRAY_MEMBER(ElementMemberT, values,", "FixedDoubleArray");

const ba = h.slice(h.indexOf("class ByteArray :"), h.indexOf("// A ByteArray in trusted space"));
for (const m of ["New", "get_int", "set_int", "LengthFor"])
  must(new RegExp(`\\b${m}\\b\\s*\\(`).test(ba), `ByteArray.${m} 缺失`);
has(ba, "FLEXIBLE_ARRAY_MEMBER(uint8_t, values);", "ByteArray");
has(ba, "return size_in_bytes - OFFSET_OF_DATA_START(ByteArray);", "ByteArray::LengthFor");

const tba = h.slice(h.indexOf("class TrustedByteArray"), h.indexOf("// Convenience class"));
for (const m of ["New", "get_int", "set_int", "LengthFor"])
  must(new RegExp(`\\b${m}\\b\\s*\\(`).test(tba), `TrustedByteArray.${m} 缺失`);
has(tba, "PrimitiveArrayBase<TrustedByteArray, uint8_t,\n                                                   TrustedFixedArrayBase>", "TrustedByteArray 基类");

const fib = h.slice(h.indexOf("class FixedIntegerArrayBase"), h.indexOf("using FixedInt8Array"));
for (const m of ["New", "get", "set", "OffsetOfElementAt", "length", "get_element_address"])
  must(new RegExp(`\\b${m}\\b\\s*\\(`).test(fib), `FixedIntegerArrayBase.${m} 缺失`);
has(fib, "return OFFSET_OF_DATA_START(Base) + index * sizeof(T);", "FixedIntegerArrayBase::OffsetOfElementAt");

// ── 5. 整数视图别名全集对数 (8 个 + TrustedFixedAddressArray 特化) ──────
const aliases = [...h.matchAll(/using (\w+) = FixedIntegerArrayBase<(int8_t|uint8_t|int16_t|uint16_t|int32_t|uint32_t|int64_t|uint64_t), (\w+)>;/g)]
  .map((m) => [m[1], m[2], m[3]]);
must(aliases.length === 8, `FixedIntegerArray 别名应 8 个, 实得 ${aliases.length}`);
{
  const expect = [
    ["FixedInt8Array", "int8_t"], ["FixedUInt8Array", "uint8_t"],
    ["FixedInt16Array", "int16_t"], ["FixedUInt16Array", "uint16_t"],
    ["FixedInt32Array", "int32_t"], ["FixedUInt32Array", "uint32_t"],
    ["FixedInt64Array", "int64_t"], ["FixedUInt64Array", "uint64_t"],
  ];
  for (const [name, ty] of expect) {
    const hit = aliases.find((a) => a[0] === name);
    must(hit && hit[1] === ty && hit[2] === "ByteArray", `别名 ${name}=${ty} ByteArray 失配`);
  }
}
has(h, "class TrustedFixedAddressArray\n    : public FixedIntegerArrayBase<Address, TrustedByteArray>", "TrustedFixedAddressArray");

// ── 6. 布局常量实测 (fixed-array-base.h 锚点, AA-1 依赖登记) ─────────────
has(base, "static constexpr int kLengthOffset = sizeof(HeapObject);", "fixed-array-base.h");
has(base, "static constexpr uint32_t kPaddingOffset = kLengthOffset + kUInt32Size;", "fixed-array-base.h (TAGGED_SIZE_8_BYTES 分支)");
has(base, "static constexpr uint32_t kHeaderSize = kPaddingOffset + kUInt32Size;", "fixed-array-base.h (TAGGED_SIZE_8_BYTES 分支)");
has(base, "V8_TQ_CONST uint32_t length_;", "FixedArrayBase 字段");
has(base, "uint32_t optional_padding_;", "FixedArrayBase padding 字段");
has(base, "class TrustedFixedArrayBase : public TrustedObject", "TrustedFixedArrayBase");
// TrustedObject 不加字段 (trusted-object.h: kHeaderSize = sizeof(HeapObject) 注释)
has(base, "static constexpr int kLengthOffset = sizeof(TrustedObject);", "TrustedFixedArrayBase");
// kMaxFixedArrayCapacity 实测: LOWER_LIMITS 关 (globals.h 默认分支) → 128Mi
has(base, "V8_LOWER_LIMITS_MODE_BOOL ? (1 * 1024 * 1024) : (128 * 1024 * 1024);", "kMaxFixedArrayCapacity");
has(globals, "#define V8_LOWER_LIMITS_MODE_BOOL false", "globals.h LOWER_LIMITS 默认关");

// ── 7. SizeFor/MaxRegularLength 尾定义 (h:297-313) ───────────────────────
has(h, "return OBJECT_POINTER_ALIGN(OffsetOfElementAt(length));", "SizeFor");
has(h, "return OFFSET_OF_DATA_START(Derived) + index * kElementSize;", "OffsetOfElementAt");
has(h, "return (kMaxRegularHeapObjectSize - OFFSET_OF_DATA_START(Derived)) /\n         kElementSize;", "MaxRegularLength");

// ── 布局推导 (非压缩 8 字节 tagged): sizeof(HeapObject)=8 → 头 16 ────────
const TAGGED = 8;
const HEADER = TAGGED + 4 + 4; // map + length_ + optional_padding_
const LAYOUT = [
  ["FixedDoubleArray", HEADER, 8],
  ["ByteArray", HEADER, 1],
  ["TrustedByteArray", HEADER, 1],
];
must(HEADER === 16, `头大小应为 16, 推导 ${HEADER}`);

const sig = (l) => `impl ${l[0]} { /* get/set/capacity/… 由 PrimitiveArray trait 提供; 详见 primitive_array.rs */ }`;

const body = `${banner("src/objects/fixed-primitive-array.h.js", "src/objects/fixed-primitive-array.h (+fixed-array-base.h 只读锚点)", "FixedPrimitiveArray 族骨架; 语义以 Developer 修订后的 primitive_array.rs 为准")}
//! 差集骨架: FixedPrimitiveArray 族 (PrimitiveArrayBase CRTP 建模)
//!
//! 继承建模: C++ PrimitiveArrayBase<Derived, ElementT_, Super> (CRTP) → Rust trait PrimitiveArray,
//!   关联常量 DATA_OFFSET (OFFSET_OF_DATA_START) / ELEMENT_SIZE (kElementSize) + 关联类型 Element。
//!   FixedArrayBase 基类归 AA-1 席; 本席以本地 FixedArrayHeader (repr(C), 16 字节) 表达继承字段。

/// kMaxFixedArrayCapacity (fixed-array-base.h:22, V8_LOWER_LIMITS_MODE 关): 128 * 1024 * 1024
pub const K_MAX_FIXED_ARRAY_CAPACITY: u32 = 128 * 1024 * 1024;
/// FixedArrayBase::kLengthOffset = sizeof(HeapObject) (非压缩 = 8)
pub const K_LENGTH_OFFSET: usize = ${TAGGED};
/// FixedArrayBase::kPaddingOffset = kLengthOffset + kUInt32Size (TAGGED_SIZE_8_BYTES)
pub const K_PADDING_OFFSET: usize = ${HEADER - 4};
/// FixedArrayBase::kHeaderSize = kPaddingOffset + kUInt32Size
pub const K_FIXED_ARRAY_BASE_HEADER_SIZE: usize = ${HEADER};

/// (Trusted)FixedArrayBase 头字段建模 (AA-1 落地后替换为 crate::fixed_array_base 定义)
struct FixedArrayHeader {
  map: Tagged<Map>,
  length_: u32,
  optional_padding_: u32,
}

/// trait PrimitiveArray: PrimitiveArrayBase 公共成员 (capacity 族 / get / set /
/// AllocatedSize / SizeFor / OffsetOfElementAt / begin / end / DataSize /
/// FromAddressOfFirstElement / MaxRegularLength / IsInBounds / clear_optional_padding)
trait PrimitiveArray { ... }

${LAYOUT.map((l) => `/// ${l[0]}: DATA_OFFSET=${l[1]}, kElementSize=${l[2]}`).join("\n")}
${LAYOUT.map((l) => sig(l)).join("\n")}

/// FixedIntegerArrayBase<T, B>: ByteArray/TrustedByteArray 字节的定宽整数视图
/// 别名 8 组 (h:275-282 机械展开):
${aliases.map((a) => `/// pub type ${a[0]} = FixedIntegerArrayBase<${a[1].replace(/int(\d+)_t/, "i$1").replace("uint", "u")}, ${a[2]}>;`).join("\n")}
/// pub type TrustedFixedAddressArray = FixedIntegerArrayBase<Address, TrustedByteArray>;
`;

const p = writeSrc("v8-objects-arrays", "primitive_array.rs.skel", body);
console.log(
  `fixed-primitive-array.h.js: 骨架 ${p} — 类 ${classNames.length >= 6 ? classNames.length : "?"}/6 组、方法对数 ${pabMethods.length}+12+4+4+6、` +
    `别名 ${aliases.length}/8、布局 ${TAGGED}+4+4=${HEADER} (kMaxLength=${128 * 1024 * 1024})、断言 ${checks} 项全部通过`,
);
