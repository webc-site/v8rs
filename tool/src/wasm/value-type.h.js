#!/usr/bin/env bun
// v8/src/wasm/value-type.h (+ value-type.cc 名字表) → v8-wasm-core/src/value_type.rs
// 解析 FOREACH_* X-macro 表 → 计算 StandardType 序 / GenericKind / NumericKind
// 判别值 (位域编码: payload<<8 | extra bits) / from_code 码表 / 尺寸与名字表 /
// kWasm* 预定义常量 / LoadType/StoreType 表。
// 位布局 (与原版 1:1, wasm 二进制互操作前提, 禁改):
//   bits 0-1 TypeKind | bit 2 nullable | bit 3 exact | bit 4 shared
//   bits 5-7 RefTypeKind | bits 8-28 payload (index 或 StandardType) | bits 29-31 reserved
import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/value-type.h");
const flat = src.split("\n").map((l) => l.replace(/\/\/.*$/, "")).join("\n");

// ---- X-macro 表提取 -------------------------------------------------------
// 取宏定义体 (跨行到未转义行尾反斜杠结束)
function macroBody(name) {
  const re = new RegExp(`#define ${name}\\(V\\)[^\\n]*\\n((?:.*\\\\\\n)*.*)`, "g");
  const m = re.exec(flat);
  if (!m) throw new Error(`宏 ${name} 未找到`);
  return m[1];
}
// 提取 V(...) 调用参数列表 (顶层逗号分割; 排除 FOREACH_XXX(V) 形态的转发调用)
function macroCalls(body) {
  const calls = [];
  const re = /(?<![A-Za-z_])V\((.*?)\)(?=\s*(?:\\|$))/g;
  let m;
  while ((m = re.exec(body)) !== null) calls.push(m[1]);
  return calls;
}
// 顶层逗号分割 (无嵌套括号场景)
const splitTop = (s) => {
  const out = [];
  let depth = 0, cur = "";
  for (const ch of s) {
    if (ch === "(") depth++;
    if (ch === ")") depth--;
    if (ch === "," && depth === 0) { out.push(cur.trim()); cur = ""; }
    else cur += ch;
  }
  if (cur.trim()) out.push(cur.trim());
  return out;
};

// FOREACH_NUMERIC_VALUE_TYPE: kind, log2Size, code, machineType, shortName, typeName
const numeric = macroCalls(macroBody("FOREACH_NUMERIC_VALUE_TYPE")).map((c) => {
  const p = splitTop(c);
  return { kind: p[0], log2: +p[1], code: p[2], mtype: p[3], shortName: p[4].slice(1, -1), typeName: p[5].replace(/"/g, "") };
});
// FOREACH_VALUE_TYPE 完整 kind 序 (Ref/RefNull 的 log2 = kTaggedSizeLog2 = 2, 64 位压缩指针)
const TAGGED_LOG2 = 2;
const valueKindSeq = [
  { kind: "Void", log2: -1 }, ...numeric.map((n) => ({ kind: n.kind, log2: n.log2 })),
  { kind: "Ref", log2: TAGGED_LOG2 }, { kind: "RefNull", log2: TAGGED_LOG2 },
  { kind: "Top", log2: -1 }, { kind: "Bottom", log2: -1 },
];

// extra bits 编码值
const REF = 0b01, SENTINEL = 0b10;
const REFTYPE_SHIFT = 5;
const eb = { REF, SENTINEL, FUNC: 1 << REFTYPE_SHIFT, STRUCT: 2 << REFTYPE_SHIFT, ARRAY: 3 << REFTYPE_SHIFT, CONT: 4 << REFTYPE_SHIFT };
function evalExtraBits(expr) {
  let v = 0;
  for (const t of expr.split("|").map((s) => s.trim())) {
    if (t in eb) v |= eb[t];
    else throw new Error(`未知 extra bits 项: ${t}`);
  }
  return v;
}
// V(name, code, extra_bits, printable)
function parseTypeTable(name) {
  return macroCalls(macroBody(name)).map((c) => {
    const p = splitTop(c);
    return { name: p[0], code: p[1], extra: evalExtraBits(p[2]), printable: p[3].replace(/"/g, "") };
  });
}
const noneTypes = parseTypeTable("FOREACH_NONE_TYPE");
const topTypes = parseTypeTable("FOREACH_TOP_TYPE");
const abstractTypes = [...noneTypes, ...topTypes,
  { name: "Eq", code: "EqRef", extra: REF, printable: "eq" },
  { name: "I31", code: "I31Ref", extra: REF, printable: "i31" },
  { name: "Struct", code: "StructRef", extra: REF | eb.STRUCT, printable: "struct" },
  { name: "Array", code: "ArrayRef", extra: REF | eb.ARRAY, printable: "array" },
  { name: "String", code: "StringRef", extra: REF, printable: "string" },
  { name: "StringViewWtf8", code: "StringViewWtf8", extra: REF, printable: "stringview_wtf8" },
  { name: "StringViewWtf16", code: "StringViewWtf16", extra: REF, printable: "stringview_wtf16" },
  { name: "StringViewIter", code: "StringViewIter", extra: REF, printable: "stringview_iter" }];
// 对数: 手工展开的尾部 8 项须与原版 FOREACH_ABSTRACT_TYPE 显式项逐项一致
{
  const orig = parseTypeTable("FOREACH_ABSTRACT_TYPE");
  const tail = abstractTypes.slice(noneTypes.length + topTypes.length);
  if (orig.length !== tail.length || orig.some((o, i) =>
    o.name !== tail[i].name || o.code !== tail[i].code || o.printable !== tail[i].printable)) {
    throw new Error(`FOREACH_ABSTRACT_TYPE 尾部对数不符: ${orig.length} vs ${tail.length}`);
  }
}
const internalTypes = [
  { name: "Void", code: "Void", extra: SENTINEL, printable: "<void>" },
  { name: "Top", code: "Void", extra: SENTINEL, printable: "<top>" },
  { name: "Bottom", code: "Void", extra: SENTINEL, printable: "<bot>" },
  { name: "ExternString", code: "Void", extra: REF, printable: "<extern_string>" },
];
// FOREACH_INTERNAL_TYPE 显式项对数校验 (原版 4 项)
{
  const orig = parseTypeTable("FOREACH_INTERNAL_TYPE");
  if (orig.length !== 4 || orig.some((o, i) => o.name !== internalTypes[i].name)) {
    throw new Error(`FOREACH_INTERNAL_TYPE 对数不符: ${orig.length} 项`);
  }
}
const genericTypes = [...internalTypes, ...abstractTypes]; // StandardType 序
const NUM_GENERIC = genericTypes.length, NUM_NUMERIC = numeric.length, NUM_STANDARD = NUM_GENERIC + NUM_NUMERIC;

// ---- 位域判别值计算 -------------------------------------------------------
const PAYLOAD_SHIFT = 8;
const stdOrd = new Map([
  ...genericTypes.map((t, i) => [t.name, i]),
  ...numeric.map((t, i) => [t.kind, NUM_GENERIC + i]),
]);
// GenericKind = payload(StandardType) | extra_bits; NumericKind = payload | kNumericExtraBits(shared 位)
const NUMERIC_EXTRA_BITS = 1 << 4;
const genericKinds = genericTypes.map((t) => ({ ...t, v: (stdOrd.get(t.name) << PAYLOAD_SHIFT) | t.extra }));
const numericKinds = numeric.map((t) => ({ ...t, v: (stdOrd.get(t.kind) << PAYLOAD_SHIFT) | NUMERIC_EXTRA_BITS }));

const GENERIC_KIND_MASK = (0x1fffff << PAYLOAD_SHIFT) | (0b111 << REFTYPE_SHIFT) | 0b11;
const NUMERIC_KIND_MASK = GENERIC_KIND_MASK | (1 << 4);

// 校验 (对应 value-type.cc CHECK_MASK): 判别值不得超出各自 mask
for (const g of genericKinds) if ((g.v & GENERIC_KIND_MASK) !== g.v) throw new Error(`GenericKind::${g.name} 超出 kGenericKindMask`);
for (const n of numericKinds) if ((n.v & NUMERIC_KIND_MASK) !== n.v) throw new Error(`NumericKind::${n.name} 超出 kNumericKindMask`);

// ---- from_code 码表 (HeapType::from_code, 区间 [0x5b..0x75]) ----
const FIRST_CODE = 0x5b, LAST_CODE = 0x75;
const codeToKind = new Map(abstractTypes.map((t) => [t.code, t.name]));
const lookup = [];
for (let c = FIRST_CODE; c <= LAST_CODE; c++) lookup.push(codeToKind.get(String.fromCharCode(c)) ?? null);
// 数字码: code 是 ValueTypeCode 变体名 (如 "NoCont"); 需用名字映射
// 上面 fromCharCode 是错的——code 列是标识符。重建:
lookup.length = 0;
{
  // 码值表: 从 wasm-constants.h ValueTypeCode 名 (k + code + Code) → 数值
  const constantsH = readV8("src/wasm/wasm-constants.h").split("\n").map((l) => l.replace(/\/\/.*$/, "")).join(" ");
  const codeVal = new Map();
  for (const m of constantsH.matchAll(/k(\w+)Code = (0x[0-9a-fA-F]+|\d+),/g)) codeVal.set(m[1], parseInt(m[2]));
  for (let c = FIRST_CODE; c <= LAST_CODE; c++) {
    const entry = abstractTypes.find((t) => codeVal.get(t.code) === c);
    lookup.push(entry ? entry.name : "Bottom");
  }
}

// ---- kWasm* 预定义常量表 --------------------------------------------------
// (rustName, GenericKind/NumericKind, nullable, shared)
const predefined = [
  ...numeric.map((n) => [`K_WASM_${n.kind.toUpperCase()}`, `NumericKind::${n.kind}`, null, false]),
  ["K_WASM_VOID", "GenericKind::Void", false, false],
  ["K_WASM_TOP", "GenericKind::Top", null, false],
  ["K_WASM_BOTTOM", "GenericKind::Bottom", false, false],
  ["K_WASM_FUNC_REF", "GenericKind::Func", null, false],
  ["K_WASM_ANY_REF", "GenericKind::Any", null, false],
  ["K_WASM_SHARED_ANY_REF", "GenericKind::Any", null, true],
  ["K_WASM_EXTERN_REF", "GenericKind::Extern", null, false],
  ["K_WASM_REF_EXTERN", "GenericKind::Extern", false, false],
  ["K_WASM_SHARED_EXTERN_REF", "GenericKind::Extern", null, true],
  ["K_WASM_EXN_REF", "GenericKind::Exn", null, false],
  ["K_WASM_SHARED_EXN_REF", "GenericKind::Exn", null, true],
  ["K_WASM_EQ_REF", "GenericKind::Eq", null, false],
  ["K_WASM_SHARED_EQ_REF", "GenericKind::Eq", null, true],
  ["K_WASM_I31_REF", "GenericKind::I31", null, false],
  ["K_WASM_REF_I31", "GenericKind::I31", false, false],
  ["K_WASM_STRUCT_REF", "GenericKind::Struct", null, false],
  ["K_WASM_ARRAY_REF", "GenericKind::Array", null, false],
  ["K_WASM_WAITQUEUE_REF", "GenericKind::Waitqueue", null, true],
  ["K_WASM_NULL_WAITQUEUE_REF", "GenericKind::NoWaitqueue", null, true],
  ["K_WASM_STRING_REF", "GenericKind::String", null, false],
  ["K_WASM_REF_STRING", "GenericKind::String", false, false],
  ["K_WASM_REF_NULL_EXTERN_STRING", "GenericKind::ExternString", null, false],
  ["K_WASM_REF_EXTERN_STRING", "GenericKind::ExternString", false, false],
  ["K_WASM_REF_NULL_SHARED_EXTERN_STRING", "GenericKind::ExternString", null, true],
  ["K_WASM_REF_SHARED_EXTERN_STRING", "GenericKind::ExternString", false, true],
  ["K_WASM_STRING_VIEW_WTF8", "GenericKind::StringViewWtf8", false, false],
  ["K_WASM_STRING_VIEW_WTF16", "GenericKind::StringViewWtf16", false, false],
  ["K_WASM_STRING_VIEW_ITER", "GenericKind::StringViewIter", false, false],
  ["K_WASM_NULL_REF", "GenericKind::None", null, false],
  ["K_WASM_SHARED_NULL_REF", "GenericKind::None", null, true],
  ["K_WASM_REF_NONE", "GenericKind::None", false, false],
  ["K_WASM_NULL_EXTERN_REF", "GenericKind::NoExtern", null, false],
  ["K_WASM_NULL_EXN_REF", "GenericKind::NoExn", null, false],
  ["K_WASM_NULL_FUNC_REF", "GenericKind::NoFunc", null, false],
  ["K_WASM_CONT_REF", "GenericKind::Cont", null, false],
  ["K_WASM_NULL_CONT_REF", "GenericKind::NoCont", null, false],
];
// nullable: null → 默认 kNullable (HeapType 构造内无所谓), false → kNonNullable

// ---- LoadType / StoreType 表 ---------------------------------------------
const loadTypes = [ // (variantSuffix, typeKind, memtype)
  ["", "I32", "Int32"], ["8S", "I32", "Int8"], ["8U", "I32", "Uint8"], ["16S", "I32", "Int16"], ["16U", "I32", "Uint16"],
  ["", "I64", "Int64"], ["8S", "I64", "Int8"], ["8U", "I64", "Uint8"], ["16S", "I64", "Int16"], ["16U", "I64", "Uint16"],
  ["32S", "I64", "Int32"], ["32U", "I64", "Uint32"], ["F16", "F32", "Float16"], ["", "F32", "Float32"],
  ["", "F64", "Float64"], ["", "S128", "Simd128"],
];
const storeTypes = [
  ["", "I32", "Word32"], ["8", "I32", "Word8"], ["16", "I32", "Word16"], ["", "I64", "Word64"],
  ["8", "I64", "Word8"], ["16", "I64", "Word16"], ["32", "I64", "Word32"], ["F16", "F32", "Float16"],
  ["", "F32", "Float32"], ["", "F64", "Float64"], ["", "S128", "Simd128"],
];
const MT_SIZE_LOG2 = { Word8: 0, Int8: 0, Uint8: 0, Word16: 1, Int16: 1, Uint16: 1, Float16: 1,
  Word32: 2, Int32: 2, Uint32: 2, Float32: 2, Word64: 3, Int64: 3, Float64: 3, Simd128: 4 };
const MT_SIGNED = { Int8: true, Int16: true, Int32: true, Int64: true };

// ---- 对数输出 -------------------------------------------------------------
console.log("value-type.h 对数:");
console.log(`  FOREACH_NUMERIC_VALUE_TYPE ${numeric.length} 项; FOREACH_VALUE_TYPE ${valueKindSeq.length} kind`);
console.log(`  INTERNAL ${internalTypes.length} + NONE ${noneTypes.length} + TOP ${topTypes.length} + 其余抽象 ${abstractTypes.length - noneTypes.length - topTypes.length} → GENERIC ${NUM_GENERIC}, STANDARD ${NUM_STANDARD}`);
console.log(`  GENERIC_KIND_MASK=0x${GENERIC_KIND_MASK.toString(16)} NUMERIC_KIND_MASK=0x${NUMERIC_KIND_MASK.toString(16)}`);
for (const g of genericKinds) console.log(`  GenericKind::${g.name} = 0x${g.v.toString(16).padStart(5, "0")} (std=${stdOrd.get(g.name)}, printable=${g.printable})`);
for (const n of numericKinds) console.log(`  NumericKind::${n.kind} = 0x${n.v.toString(16).padStart(5, "0")}`);
console.log(`  from_code 表 [0x${FIRST_CODE.toString(16)}..0x${LAST_CODE.toString(16)}] ${lookup.length} 项, 命中 ${lookup.filter((x) => x !== "Bottom").length}`);
console.log(`  预定义常量 ${predefined.length} 项; LoadType ${loadTypes.length}; StoreType ${storeTypes.length}`);

// ---- 生成 -----------------------------------------------------------------
const hx = (v) => `0x${v.toString(16)}`;
const vk = (k) => k; // ValueKind 变体与 kind 同名

let out = `//! v8/src/wasm/value-type.h 的 1:1 转写 (名字表来自 value-type.cc)
//! ValueType/HeapType/CanonicalValueType/IndependentValueType 为 uint32 位域编码:
//!   bits 0-1 TypeKind | bit 2 nullable | bit 3 exact | bit 4 shared
//!   bits 5-7 RefTypeKind | bits 8-28 payload(索引或 StandardType) | bits 29-31 保留
//! 禁改位布局 — wasm 二进制互操作前提。
//! 登记差异:
//! - C++ 继承体系 (ValueTypeBase + 3 子类) → 共享查询方法的 trait ValueTypeBits
//!   + 4 个 newtype; Module/Canonical 索引的类型区分保留 (防混用)。
//! - MachineType/MachineRepresentation 取 codegen/machine-type.h 的最小子集
//!   (仅 value-type 所需), 后续迁移至 codegen crate。
//! - FunctionSig/CanonicalSig/Signature 依赖 codegen/signature.h → 推迟。
//! - AsExactIfEnabled 依赖 WasmEnabledFeatures::has_custom_descriptors → 推迟
//!   (flags 梯队); WasmReturnTypeFromSignature/GetI32Sig 依赖 Signature → 推迟。
`;
out += banner("src/wasm/value-type.h.js", "src/wasm/value-type.h", "X-macro 展开计算判别值; 对数见脚本运行输出");
out += `use crate::wasm_constants::ValueTypeCode;
use v8_base::bit_field::{BitField32, BitFieldCodec};
use v8_common::{K_SYSTEM_POINTER_SIZE, K_TAGGED_SIZE, K_TAGGED_SIZE_LOG2};

/// 值类型字节表示上限 (kMaxValueTypeSize)
pub const K_MAX_VALUE_TYPE_SIZE: usize = 16;

// ---------------------------------------------------------------------------
// 类型索引: Module 级与 Canonical 级禁止互比 (原版注释), 故各自独立类型
// ---------------------------------------------------------------------------

macro_rules! type_index {
  ($name:ident, $doc:expr) => {
    #[doc = $doc]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct $name {
      pub index: u32,
    }

    impl $name {
      /// kInvalid = ~0u
      pub const INVALID: Self = Self { index: u32::MAX };

      #[inline]
      pub const fn new(index: u32) -> Self {
        Self { index }
      }

      /// constexpr bool valid()
      #[inline]
      pub const fn is_valid(self) -> bool {
        self.index != u32::MAX
      }
    }
  };
}

type_index!(ModuleTypeIndex, "模块内类型索引 (module-specific)");
type_index!(CanonicalTypeIndex, "规范化类型索引 (canonical)");

/// kNoSuperType: 类型层级顶端的父类型
pub const K_NO_SUPER_TYPE: ModuleTypeIndex = ModuleTypeIndex::INVALID;
/// kNoType
pub const K_NO_TYPE: ModuleTypeIndex = ModuleTypeIndex::INVALID;

/// 可空性 (enum Nullability : bool)
pub type Nullability = bool;
pub const K_NULLABLE: Nullability = true;
pub const K_NON_NULLABLE: Nullability = false;

/// 精确性 (enum Exactness : bool)
pub type Exactness = bool;
pub const K_EXACT: Exactness = true;
pub const K_ANY_SUBTYPE: Exactness = false;

// ---------------------------------------------------------------------------
// ValueKind: FOREACH_VALUE_TYPE 枚举 (序号与原版一致)
// ---------------------------------------------------------------------------

/// 粗粒度 kind (FOREACH_VALUE_TYPE 展开序)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValueKind {
`;
for (const [i, k] of valueKindSeq.entries()) out += `  ${vk(k.kind)} = ${i},\n`;
out += `}

impl ValueKind {
  /// 枚举序 → ValueKind (越界 → None)
  pub const fn from_u8(v: u8) -> Option<Self> {
    match v {
`;
for (const [i, k] of valueKindSeq.entries()) out += `      ${i} => Some(Self::${vk(k.kind)}),\n`;
out += `      _ => None,
    }
  }
}

// ---------------------------------------------------------------------------
// TypeKind / RefTypeKind: 位域低 8 位的两个子域
// ---------------------------------------------------------------------------

/// 2 位粗分类 (bits 0-1); bit0=是否引用, bit1=引用是否有索引/是否哨兵
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeKind {
  Numeric = 0b00,     // i32 等数值 (FOREACH_NUMERIC_VALUE_TYPE)
  AbstractRef = 0b01, // anyref/eqref/funcref 等抽象引用
  Sentinel = 0b10,    // top/bottom/void 哨兵
  IndexedRef = 0b11,  // 带索引的用户定义类型引用
}

/// 引用类别 (bits 5-7); 非引用类型存 Other
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RefTypeKind {
  Other = 0,
  Function = 1,
  Struct = 2,
  Array = 3,
  Cont = 4,
  /// 解码越界哨兵 (原版 static_cast UB; is_valid 以 > LastValue 判非法)
  Invalid = 5,
}

impl RefTypeKind {
  /// kLastValue = kCont
  pub const LAST_VALUE: Self = Self::Cont;
}

impl BitFieldCodec for TypeKind {
  #[inline]
  fn to_u64(self) -> u64 {
    self as u64
  }
  #[inline]
  fn from_u64(v: u64) -> Self {
    match v & 0b11 {
      0 => Self::Numeric,
      1 => Self::AbstractRef,
      2 => Self::Sentinel,
      _ => Self::IndexedRef,
    }
  }
}

impl BitFieldCodec for RefTypeKind {
  #[inline]
  fn to_u64(self) -> u64 {
    self as u64
  }
  #[inline]
  fn from_u64(v: u64) -> Self {
    match v & 0b111 {
      0 => Self::Other,
      1 => Self::Function,
      2 => Self::Struct,
      3 => Self::Array,
      4 => Self::Cont,
      _ => Self::Invalid,
    }
  }
}

// ---------------------------------------------------------------------------
// 位域布局 (value_type_impl namespace)
// ---------------------------------------------------------------------------

/// ValueType 位域字段族 (对应 value_type_impl 的 BitField 别名)
pub mod fields {
  use super::{BitField32, RefTypeKind, TypeKind};

  /// bit 0: 是否某种引用
  pub type IsRef = BitField32<bool, 0, 1>;
  /// bit 1: 引用是否带索引 / 非引用是否内部哨兵
  pub type HasIndexOrSentinel = BitField32<bool, 1, 1>;
  /// bits 0-1: TypeKind (IsRef | HasIndexOrSentinel 的联合视图)
  pub type TypeKindF = BitField32<TypeKind, 0, 2>;
  /// bit 2: 引用可空性
  pub type IsNullable = BitField32<bool, 2, 1>;
  /// bit 3: 引用精确性
  pub type IsExact = BitField32<bool, 3, 1>;
  /// bit 4: 共享标志 (数值类型视为共享)
  pub type IsShared = BitField32<bool, 4, 1>;
  /// bits 5-7: 引用类别
  pub type RefTypeKindF = BitField32<RefTypeKind, 5, 3>;
  /// bits 8-28: 索引 (has_index) 或 StandardType
  pub type Payload = BitField32<u32, 8, 21>;
  /// bits 29-31: 保留
  pub type Reserved = BitField32<u32, 29, 3>;

  /// kGenericKindMask: HeapType 相关位 (payload | refKind | typeKind)
  pub const GENERIC_KIND_MASK: u32 = ${hx(GENERIC_KIND_MASK)};
  /// kNumericKindMask: 数值类型恒共享 → 额外含 shared 位
  pub const NUMERIC_KIND_MASK: u32 = ${hx(NUMERIC_KIND_MASK)};
  /// kNumericExtraBits: IsSharedField::encode(true)
  pub const NUMERIC_EXTRA_BITS: u32 = ${hx(NUMERIC_EXTRA_BITS)};

  /// 关联位断言 (原版 static_assert 组; k_mask() 非 const → 位运算常量复核)
  const _: () = {
    assert!(TypeKindF::K_SHIFT == 0 && TypeKindF::K_SIZE == 2);
    assert!(IsNullable::K_SHIFT == 2 && IsExact::K_SHIFT == 3 && IsShared::K_SHIFT == 4);
    assert!(RefTypeKindF::K_SHIFT == 5 && RefTypeKindF::K_SIZE == 3);
    assert!(Payload::K_SHIFT == 8 && Payload::K_SIZE == 21);
    assert!(Reserved::K_SHIFT == 29 && Reserved::K_SIZE == 3);
    assert!(Reserved::K_SHIFT + Reserved::K_SIZE == 32);
    // kGenericKindMask == Payload | RefTypeKindF | TypeKindF 位联合
    assert!(GENERIC_KIND_MASK == ((1 << 21) - 1) << 8 | 0b111 << 5 | 0b11);
    assert!(NUMERIC_KIND_MASK == GENERIC_KIND_MASK | 1 << 4);
    assert!(NUMERIC_EXTRA_BITS == 1 << 4);
  };
}

// ---------------------------------------------------------------------------
// StandardType / GenericKind / NumericKind
// ---------------------------------------------------------------------------

/// 非索引类型的紧凑枚举, 值 = PayloadField 内容 (FOREACH_GENERIC_TYPE + NUMERIC 序)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StandardType {
`;
for (const [i, t] of [...genericTypes.map((g) => g.name), ...numeric.map((n) => n.kind)].entries()) out += `  ${t} = ${i},\n`;
out += `}

impl StandardType {
  /// 序号 → 枚举 (越界 → None)
  pub const fn from_u32(v: u32) -> Option<Self> {
    match v {
`;
for (const [i, t] of [...genericTypes.map((g) => g.name), ...numeric.map((n) => n.kind)].entries()) out += `      ${i} => Some(Self::${t}),\n`;
out += `      _ => None,
    }
  }
}

/// 抽象引用 + 内部哨兵 (FOREACH_GENERIC_TYPE); 判别值 = Payload | extra bits
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenericKind {
`;
for (const g of genericKinds) out += `  ${g.name} = ${hx(g.v)}, // std=${stdOrd.get(g.name)} "${g.printable}"\n`;
out += `}

impl GenericKind {
  /// 位域值 → 枚举 (未定义值 → None)
  pub const fn from_bits(v: u32) -> Option<Self> {
    match v & ${hx(GENERIC_KIND_MASK)} {
`;
for (const g of genericKinds) out += `      ${hx(g.v)} => Some(Self::${g.name}),\n`;
out += `      _ => None,
    }
  }

  /// 底层位域值
  #[inline]
  pub const fn bits(self) -> u32 {
    self as u32
  }
}

/// 数值类型 (FOREACH_NUMERIC_VALUE_TYPE); 判别值 = Payload | kNumericExtraBits
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericKind {
`;
for (const n of numericKinds) out += `  ${n.kind} = ${hx(n.v)}, // log2size=${n.log2} "${n.typeName}"\n`;
out += `}

impl NumericKind {
  /// 位域值 → 枚举 (未定义值 → None)
  pub const fn from_bits(v: u32) -> Option<Self> {
    match v & ${hx(NUMERIC_KIND_MASK)} {
`;
for (const n of numericKinds) out += `      ${hx(n.v)} => Some(Self::${n.kind}),\n`;
out += `      _ => None,
    }
  }

  /// 底层位域值
  #[inline]
  pub const fn bits(self) -> u32 {
    self as u32
  }

  /// ToZeroBasedIndex: 稠密表下标 (StandardType 中 generic 在前)
  #[inline]
  pub const fn zero_based_index(self) -> u32 {
    (self as u32 >> 8) - ${NUM_GENERIC}
  }
}

/// 数值 kind 的稠密查找表下标界
pub const NUMERIC_KINDS: usize = ${NUM_NUMERIC};
/// 泛型 kind (抽象引用 + 哨兵) 数
pub const NUMERIC_LOOKUP_OFFSET: u32 = ${NUM_GENERIC};

/// IsNullKind: FOREACH_NONE_TYPE (null 堆类型的引用无居民)
pub const fn is_null_kind(kind: GenericKind) -> bool {
  matches!(kind, ${noneTypes.map((t) => `GenericKind::${t.name}`).join(" | ")})
}
`;

writeSrc("v8-wasm-core", "value_type.rs", out);
console.log("→ v8-wasm-core/src/value_type.rs (第一部分: 枚举与位域)");
// ===========================================================================
// 第二部分: ValueTypeBits trait + 4 newtype + 方法族 + 表 + 预定义常量
// ===========================================================================
let out2 = `
// ---------------------------------------------------------------------------
// ValueTypeBits: 共享查询方法 (原版 ValueTypeBase 公共接口)
// ---------------------------------------------------------------------------

/// 类型无关查询方法族 (原版 ValueTypeBase); 4 个 newtype 共用
pub trait ValueTypeBits: Copy {
  /// 原始位域 (bit_field_)
  fn bit_field(self) -> u32;

  // --- 分类 ---
  #[inline]
  fn type_kind(self) -> TypeKind {
    fields::TypeKindF::decode(self.bit_field())
  }
  #[inline]
  fn has_index(self) -> bool {
    self.type_kind() == TypeKind::IndexedRef
  }
  #[inline]
  fn is_sentinel(self) -> bool {
    self.type_kind() == TypeKind::Sentinel
  }
  #[inline]
  fn is_numeric(self) -> bool {
    self.type_kind() == TypeKind::Numeric
  }
  #[inline]
  fn is_abstract_ref(self) -> bool {
    self.type_kind() == TypeKind::AbstractRef
  }
  /// 任意引用: 抽象或索引
  #[inline]
  fn is_ref(self) -> bool {
    fields::IsRef::decode(self.bit_field())
  }
  /// 任意 GenericKind: 抽象引用或内部哨兵
  #[inline]
  fn is_generic(self) -> bool {
    matches!(self.type_kind(), TypeKind::AbstractRef | TypeKind::Sentinel)
  }

  // --- 属性位 ---
  #[inline]
  fn nullability(self) -> Nullability {
    debug_assert!(self.is_ref());
    fields::IsNullable::decode(self.bit_field())
  }
  #[inline]
  fn is_nullable(self) -> bool {
    self.is_ref() && self.nullability()
  }
  #[inline]
  fn is_non_nullable(self) -> bool {
    self.is_ref() && !self.nullability()
  }
  #[inline]
  fn exactness(self) -> Exactness {
    fields::IsExact::decode(self.bit_field())
  }
  #[inline]
  fn is_exact(self) -> bool {
    self.exactness()
  }
  #[inline]
  fn is_shared(self) -> bool {
    fields::IsShared::decode(self.bit_field())
  }
  #[inline]
  fn ref_type_kind(self) -> RefTypeKind {
    fields::RefTypeKindF::decode(self.bit_field())
  }

  // --- 判别值 ---
  /// 未索引时 payload 即 StandardType (非法 payload → None)
  #[inline]
  fn standard_type(self) -> Option<StandardType> {
    debug_assert!(!self.has_index());
    match fields::Payload::decode(self.bit_field()) {
      v if v < ${NUM_STANDARD} => StandardType::from_u32(v),
      _ => None,
    }
  }
  /// 数值判别值 (非数值 → None)
  #[inline]
  fn numeric_kind(self) -> Option<NumericKind> {
    debug_assert!(self.is_numeric());
    NumericKind::from_bits(self.bit_field())
  }
  /// 泛型判别值 (非泛型 → None)
  #[inline]
  fn generic_kind(self) -> Option<GenericKind> {
    debug_assert!(self.is_generic());
    GenericKind::from_bits(self.bit_field())
  }

  // --- 特定类型判断 (位比较, 与原版一致) ---
  #[inline]
  fn is_bottom(self) -> bool {
    self.bit_field() & fields::GENERIC_KIND_MASK == GenericKind::Bottom.bits()
  }
  #[inline]
  fn is_top(self) -> bool {
    self.bit_field() & fields::GENERIC_KIND_MASK == GenericKind::Top.bits()
  }
  #[inline]
  fn is_void(self) -> bool {
    self.bit_field() & fields::GENERIC_KIND_MASK == GenericKind::Void.bits()
  }
  /// none 或 bottom: (exact $t) 的子类型集
  #[inline]
  fn is_none_or_bottom(self) -> bool {
    let bits = self.bit_field() & fields::GENERIC_KIND_MASK;
    bits == GenericKind::None.bits() || bits == GenericKind::Bottom.bits()
  }
  #[inline]
  fn is_string_view(self) -> bool {
    let bits = self.bit_field() & fields::GENERIC_KIND_MASK;
    bits == GenericKind::StringViewWtf8.bits()
      || bits == GenericKind::StringViewWtf16.bits()
      || bits == GenericKind::StringViewIter.bits()
  }
  /// 打包类型 (i8/i16/f16)
  #[inline]
  fn is_packed(self) -> bool {
    matches!(self.numeric_kind(), Some(NumericKind::I8 | NumericKind::I16 | NumericKind::F16))
  }
  #[inline]
  fn is_reference_to(self, kind: GenericKind) -> bool {
    self.is_abstract_ref() && self.generic_kind() == Some(kind)
  }
  /// 数值与可空引用可默认初始化
  #[inline]
  fn is_defaultable(self) -> bool {
    self.is_numeric() || self.is_nullable()
  }

  /// kind(): 位域 → 粗粒度 ValueKind (原版 ValueTypeBase::kind)
  #[inline]
  fn kind(self) -> ValueKind {
    if self.is_numeric() {
      return match self.numeric_kind() {
        Some(NumericKind::I32) => ValueKind::I32,
        Some(NumericKind::I64) => ValueKind::I64,
        Some(NumericKind::F32) => ValueKind::F32,
        Some(NumericKind::F64) => ValueKind::F64,
        Some(NumericKind::S128) => ValueKind::S128,
        Some(NumericKind::I8) => ValueKind::I8,
        Some(NumericKind::I16) => ValueKind::I16,
        Some(NumericKind::F16) => ValueKind::F16,
        // 原版 UNREACHABLE; 禁 panic → I32 兜底 (调用方错误输入)
        None => ValueKind::I32,
      };
    }
    if self.is_top() {
      return ValueKind::Top;
    }
    if self.is_bottom() {
      return ValueKind::Bottom;
    }
    if self.is_void() {
      return ValueKind::Void;
    }
    if self.is_nullable() {
      ValueKind::RefNull
    } else {
      ValueKind::Ref
    }
  }

  /// 无居民类型: bottom 或 非空 null 类型引用
  #[inline]
  fn is_uninhabited(self) -> bool {
    if self.is_bottom() {
      return true;
    }
    if self.is_defaultable() {
      return false; // 含全部可空引用
    }
    if self.has_index() {
      return false;
    }
    // 非空 null 类型引用无居民
    self.generic_kind().is_some_and(is_null_kind)
  }
  /// 是否使用 wasm null 表示 (extern 系例外)
  #[inline]
  fn use_wasm_null(self) -> bool {
    debug_assert!(self.is_ref());
    if self.has_index() {
      return true;
    }
    !matches!(self.generic_kind(), Some(GenericKind::Extern | GenericKind::ExternString | GenericKind::NoExtern))
  }
  #[inline]
  fn is_none_type(self) -> bool {
    debug_assert!(!self.is_numeric());
    self.is_abstract_ref() && self.generic_kind().is_some_and(is_null_kind)
  }

  // --- 尺寸 ---
  /// log2(存储字节数); 引用 → kTaggedSizeLog2
  #[inline]
  fn value_kind_size_log2(self) -> u8 {
    debug_assert!(!self.is_sentinel());
    if self.is_ref() {
      return K_TAGGED_SIZE_LOG2 as u8;
    }
    K_VALUE_KIND_SIZE_LOG2[self.numeric_kind().map(NumericKind::zero_based_index).unwrap_or(0) as usize]
  }
  /// 存储字节数; 引用 → kTaggedSize
  #[inline]
  fn value_kind_size(self) -> u8 {
    debug_assert!(!self.is_sentinel());
    if self.is_ref() {
      return K_TAGGED_SIZE as u8;
    }
    K_VALUE_KIND_SIZE[self.numeric_kind().map(NumericKind::zero_based_index).unwrap_or(0) as usize]
  }
  /// 未压缩指针尺寸 (引用 → kSystemPointerSize)
  #[inline]
  fn value_kind_full_size(self) -> usize {
    if self.is_ref() {
      K_SYSTEM_POINTER_SIZE
    } else {
      self.value_kind_size() as usize
    }
  }

  // --- wire 编码 ---
  /// wasm 二进制首字节; 优先短编码 (如 kFuncRefCode)
  fn value_type_code(self) -> ValueTypeCode {
    if self.is_numeric() {
      return self.value_type_code_numeric();
    }
    if self.is_ref() && self.encoding_needs_heap_type() {
      return if self.is_nullable() { ValueTypeCode::RefNull } else { ValueTypeCode::Ref };
    }
    debug_assert!(!self.has_index()); // encoding_needs_heap_type 已处理
    self.value_type_code_generic()
  }
  /// 数值判别 → wire 码
  fn value_type_code_numeric(self) -> ValueTypeCode {
    match self.numeric_kind() {
`;
for (const n of numeric) out2 += `      Some(NumericKind::${n.kind}) => ValueTypeCode::${n.code},\n`;
out2 += `      None => ValueTypeCode::Void, // 非法判别 (debug_assert 于 numeric_kind)
    }
  }
  /// 泛型判别 → wire 码 (哨兵中仅 void 有 wire 编码)
  fn value_type_code_generic(self) -> ValueTypeCode {
    match self.generic_kind() {
`;
for (const a of abstractTypes) out2 += `      Some(GenericKind::${a.name}) => ValueTypeCode::${a.code},\n`;
out2 += `      Some(GenericKind::Void) => ValueTypeCode::Void,
      _ => ValueTypeCode::Void, // Top/Bottom/ExternString 无 wire 编码 (原版 UNREACHABLE)
    }
  }
  /// 编码是否需要堆类型字节: 索引引用 / 非空或共享抽象引用
  #[inline]
  fn encoding_needs_heap_type(self) -> bool {
    if self.has_index() {
      return true;
    }
    if !self.is_abstract_ref() {
      return false;
    }
    !self.is_nullable() || self.is_shared()
  }
  #[inline]
  fn encoding_needs_shared(self) -> bool {
    self.is_abstract_ref() && self.is_shared()
  }
  #[inline]
  fn encoding_needs_exact(self) -> bool {
    self.is_exact()
  }

  // --- 短名 ---
  /// 单字符短名 (调试用)
  fn short_name(self) -> char {
    if self.is_ref() {
      return if self.is_nullable() { 'n' } else { 'r' };
    }
    if self.is_sentinel() {
      if self.is_void() {
        return 'v';
      }
      if self.is_top() {
        return '\\\\';
      }
      if self.is_bottom() {
        return '*';
      }
    }
    K_NUMERIC_SHORT_NAME[self.numeric_kind().map(NumericKind::zero_based_index).unwrap_or(0) as usize]
  }

  // --- 校验 ---
  /// 位域是否为合法编码 (原版 constexpr is_valid)
  fn is_valid(self) -> bool {
    let payload = fields::Payload::decode(self.bit_field());
    if self.is_numeric() {
      // 数值类型 payload 之外固定位必须恰好是 kNumericExtraBits
      if self.bit_field() & !fields::Payload::k_mask() != fields::NUMERIC_EXTRA_BITS {
        return false;
      }
      return (${NUM_GENERIC}..${NUM_STANDARD}).contains(&payload);
    }
    if self.ref_type_kind() > RefTypeKind::LAST_VALUE {
      return false;
    }
    if !self.has_index() {
      // 泛型类型必须在预定义集合内; 仅索引类型可 exact
      if payload >= ${NUM_GENERIC} {
        return false;
      }
      if self.is_exact() {
        return false;
      }
    }
    // 泛型与索引引用都必须保留位为零
    fields::Reserved::decode(self.bit_field()) == 0
  }

  /// Populate: 解码器先按索引建对象, 后补 shared/refKind 位
  #[inline]
  fn populate(&mut self, shared: bool, kind: RefTypeKind) {
    let bits = fields::IsShared::update(self.bit_field(), shared);
    let bits = fields::RefTypeKindF::update(bits, kind);
    self.set_bit_field(bits);
  }

  /// 内部位域写入 (populate 用)
  fn set_bit_field(&mut self, bits: u32);

  /// raw_bit_field()
  #[inline]
  fn raw_bit_field(self) -> u32 {
    self.bit_field()
  }
}

/// 数值 kind 稠密表: log2 尺寸 (FOREACH_NUMERIC_VALUE_TYPE log2Size 列)
static K_VALUE_KIND_SIZE_LOG2: [u8; ${NUM_NUMERIC}] = [${numeric.map((n) => n.log2).join(", ")}];
/// 数值 kind 稠密表: 字节数 (1 << log2Size)
static K_VALUE_KIND_SIZE: [u8; ${NUM_NUMERIC}] = [${numeric.map((n) => 1 << n.log2).join(", ")}];
/// 数值 kind 稠密表: 单字符短名
static K_NUMERIC_SHORT_NAME: [char; ${NUM_NUMERIC}] = [${numeric.map((n) => `'${n.shortName}'`).join(", ")}];
`;

// ===========================================================================
// 第三部分: newtype 类型族 + HeapType + 码表 + 预定义常量 + Load/Store + 名字
// ===========================================================================
let out3 = `
// ---------------------------------------------------------------------------
// newtype: ValueType / CanonicalValueType (构造宏共享)
// ---------------------------------------------------------------------------

macro_rules! wasm_type_newtype {
  ($name:ident, $doc:expr) => {
    #[doc = $doc]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct $name(u32);

    impl ValueTypeBits for $name {
      #[inline]
      fn bit_field(self) -> u32 {
        self.0
      }
      #[inline]
      fn set_bit_field(&mut self, bits: u32) {
        self.0 = bits;
      }
    }

    impl $name {
      /// 位域值 (const fn; ValueTypeBits::raw_bit_field 同语义)
      #[inline]
      pub const fn bits(self) -> u32 {
        self.0
      }

      /// FromRawBitField (不校验; 调用方保证或先行 is_valid)
      #[inline]
      pub const fn from_raw_bit_field(bits: u32) -> Self {
        Self(bits)
      }

      /// Primitive(NumericKind)
      #[inline]
      pub const fn primitive(kind: NumericKind) -> Self {
        Self(kind.bits())
      }

      /// Generic(kind, nullable, shared)
      /// (const fn 不能调 BitField::encode → 手写位装配; 位布局由 fields 断言保证)
      #[inline]
      pub const fn generic(kind: GenericKind, nullable: Nullability, shared: bool) -> Self {
        Self(kind.bits() | ((nullable as u32) << 2) | ((shared as u32) << 4))
      }

      /// 索引引用底层装配 (原版 ValueTypeBase(TypeIndex, nullable, exact, shared, refKind))
      #[inline]
      pub const fn indexed(index: u32, nullable: Nullability, exact: Exactness, shared: bool, ref_kind: RefTypeKind) -> Self {
        debug_assert!(index < (1u32 << 21));
        Self(
          TypeKind::IndexedRef as u32 // bits 0-1
            | ((nullable as u32) << 2)
            | ((exact as u32) << 3)
            | ((shared as u32) << 4)
            | ((ref_kind as u32) << 5)
            | (index << 8),
        )
      }

      /// AsNullable
      #[inline]
      pub const fn as_nullable(self, nullable: Nullability) -> Self {
        Self((self.0 & !(1 << 2)) | ((nullable as u32) << 2))
      }
      /// AsExact
      #[inline]
      pub const fn as_exact(self, exact: Exactness) -> Self {
        Self((self.0 & !(1 << 3)) | ((exact as u32) << 3))
      }
      /// shared 位更新 (AsNonShared 的补)
      #[inline]
      pub const fn as_shared(self, shared: bool) -> Self {
        Self((self.0 & !(1 << 4)) | ((shared as u32) << 4))
      }
    }
  };
}

wasm_type_newtype!(ValueType, "模块索引值类型 (module-specific type indices)");
wasm_type_newtype!(CanonicalValueType, "规范化索引值类型 (canonicalized type indices)");

// ---------------------------------------------------------------------------
// HeapType: is_numeric 恒 false; nullable 位忽略 (等值比较掩掉)
// ---------------------------------------------------------------------------

/// 堆类型 (ValueType 的子集视图: 非数值; nullable 位无意义)
#[derive(Debug, Clone, Copy, Eq)]
pub struct HeapType(u32);

impl ValueTypeBits for HeapType {
  #[inline]
  fn bit_field(self) -> u32 {
    self.0
  }
  #[inline]
  fn set_bit_field(&mut self, bits: u32) {
    self.0 = bits;
  }
}

/// 等值: 掩掉 is_ref 与 nullable 位 (原版 operator== 掩码);
/// Hash 同掩码 — 原版 hash_value 用 raw 与 == 不一致, 登记差异修正
impl PartialEq for HeapType {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    let mask = !(fields::IsRef::k_mask() | fields::IsNullable::k_mask());
    (self.0 & mask) == (other.0 & mask)
  }
}

impl std::hash::Hash for HeapType {
  #[inline]
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    let mask = !(fields::IsRef::k_mask() | fields::IsNullable::k_mask());
    (self.0 & mask).hash(state);
  }
}

impl HeapType {
  /// 位域值 (const fn; ValueTypeBits::raw_bit_field 同语义)
  #[inline]
  pub const fn bits(self) -> u32 {
    self.0
  }

  /// Generic(kind, shared) (const fn 手写位装配)
  #[inline]
  pub const fn generic(kind: GenericKind, shared: bool) -> Self {
    Self(kind.bits() | (1 << 2) | ((shared as u32) << 4))
  }

  /// nullable 位重置 (HeapType 内部位操作; 语义上 nullable 位无效)
  #[inline]
  pub const fn as_nullable(self, nullable: Nullability) -> Self {
    Self((self.0 & !(1 << 2)) | ((nullable as u32) << 2))
  }

  /// Index(index, shared, kind, exact)
  #[inline]
  pub const fn index(index: ModuleTypeIndex, shared: bool, kind: RefTypeKind, exact: Exactness) -> Self {
    Self(ValueType::indexed(index.index, K_NULLABLE, exact, shared, kind).bits())
  }

  /// FromBits (不校验; 须非数值)
  #[inline]
  pub const fn from_bits(bits: u32) -> Self {
    Self(bits)
  }

  /// from_code: wire 堆类型码 → 堆类型。
  /// 界外码 → Bottom (丢弃 shared; 原版 kFirst/kLast 前置检查);
  /// 界内未分配码 → Bottom (保留 shared; 原版 lookup 表 default 项)
  pub const fn from_code(code: u8, shared: bool) -> Self {
    use crate::wasm_constants::{FIRST_HEAP_TYPE_CODE, LAST_HEAP_TYPE_CODE};
    if code < FIRST_HEAP_TYPE_CODE || code > LAST_HEAP_TYPE_CODE {
      return Self::generic(GenericKind::Bottom, false);
    }
    let kind = match code {
`;
for (let i = 0; i < lookup.length; i++) {
  const kind = lookup[i];
  if (kind !== "Bottom") out3 += `      ${hx(FIRST_CODE + i)} => GenericKind::${kind},\n`;
}
out3 += `      _ => GenericKind::Bottom,
    };
    Self::generic(kind, shared)
  }

  /// AsExact (kAnySubtype 仅索引类型允许)
  #[inline]
  pub const fn as_exact(self, exact: Exactness) -> Self {
    Self((self.0 & !(1 << 3)) | ((exact as u32) << 3))
  }

  /// wire 码或索引; 抽象码符号扩展 (LEB128 首字节 7→32 位)
  #[inline]
  pub fn code(self) -> i32 {
    if self.has_index() {
      return ((self.0 >> 8) & 0x1F_FFFF) as i32;
    }
    (0xFFFF_FF80u32 | self.value_type_code_generic() as u32) as i32
  }

  /// 模块类型索引
  #[inline]
  pub const fn ref_index(self) -> ModuleTypeIndex {
    debug_assert!((self.0 & 0b11) == 0b11);
    ModuleTypeIndex::new((self.0 >> 8) & 0x1F_FFFF)
  }

  /// 堆类型名 (数字或 exact 前缀)
  pub fn name(self) -> String {
    if self.has_index() {
      if self.is_exact() {
        return format!("exact {}", fields::Payload::decode(self.0));
      }
      return fields::Payload::decode(self.0).to_string();
    }
    generic_heaptype_name_prefixed(self.0)
  }

  /// generic_heaptype_name()
  pub fn generic_heaptype_name(self) -> String {
    generic_heaptype_name_prefixed(self.0)
  }
}

// ---------------------------------------------------------------------------
// IndependentValueType / IndependentHeapType: 全局常量载体 (可转三者)
// ---------------------------------------------------------------------------

/// 非索引全局常量类型 (与 ValueType/CanonicalValueType/HeapType 互转)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndependentValueType(u32);

impl ValueTypeBits for IndependentValueType {
  #[inline]
  fn bit_field(self) -> u32 {
    self.0
  }
  #[inline]
  fn set_bit_field(&mut self, bits: u32) {
    self.0 = bits;
  }
}

impl IndependentValueType {
  /// 位域值 (const fn)
  #[inline]
  pub const fn bits(self) -> u32 {
    self.0
  }

  /// FromRawBitField
  #[inline]
  pub const fn from_raw_bit_field(bits: u32) -> Self {
    Self(bits)
  }

  #[inline]
  pub const fn primitive(kind: NumericKind) -> Self {
    Self(kind.bits())
  }

  #[inline]
  pub const fn generic(kind: GenericKind, nullable: Nullability, shared: bool) -> Self {
    Self(kind.bits() | ((nullable as u32) << 2) | ((shared as u32) << 4))
  }

  /// name()
  pub fn name(self) -> String {
    name_common(self.0)
  }
}

impl From<IndependentValueType> for ValueType {
  #[inline]
  fn from(v: IndependentValueType) -> Self {
    Self(v.bits())
  }
}
impl From<IndependentValueType> for CanonicalValueType {
  #[inline]
  fn from(v: IndependentValueType) -> Self {
    Self(v.bits())
  }
}

/// 独立堆类型常量 (默认可空/非共享)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndependentHeapType(IndependentValueType);

impl std::ops::Deref for IndependentHeapType {
  type Target = IndependentValueType;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl ValueTypeBits for IndependentHeapType {
  #[inline]
  fn bit_field(self) -> u32 {
    self.0.bits()
  }
  #[inline]
  fn set_bit_field(&mut self, bits: u32) {
    self.0 = IndependentValueType::from_raw_bit_field(bits);
  }
}

impl IndependentHeapType {
  /// 位域值 (const fn)
  #[inline]
  pub const fn bits(self) -> u32 {
    self.0.bits()
  }

  #[inline]
  pub const fn new(kind: GenericKind, nullable: Nullability, shared: bool) -> Self {
    Self(IndependentValueType::generic(kind, nullable, shared))
  }

  /// AsNonNull
  #[inline]
  pub const fn as_non_null(self) -> Self {
    let shared = (self.bits() & (1 << 4)) != 0;
    match GenericKind::from_bits(self.bits() & fields::GENERIC_KIND_MASK) {
      Some(k) => Self::new(k, K_NON_NULLABLE, shared),
      None => Self::new(GenericKind::Bottom, K_NON_NULLABLE, shared),
    }
  }

  /// heap_type()
  #[inline]
  pub const fn heap_type(self) -> HeapType {
    HeapType::from_bits(self.bits())
  }

  /// name()
  pub fn name(self) -> String {
    name_common(self.bits())
  }
}

impl From<IndependentHeapType> for HeapType {
  #[inline]
  fn from(v: IndependentHeapType) -> HeapType {
    v.heap_type()
  }
}
impl From<IndependentHeapType> for ValueType {
  #[inline]
  fn from(v: IndependentHeapType) -> Self {
    Self(v.bits())
  }
}
impl From<IndependentHeapType> for CanonicalValueType {
  #[inline]
  fn from(v: IndependentHeapType) -> Self {
    Self(v.bits())
  }
}

// ---------------------------------------------------------------------------
// ValueType / CanonicalValueType 特有构造 (索引引用族)
// ---------------------------------------------------------------------------

impl ValueType {
  /// Ref(index, shared, kind): 非空索引引用
  #[inline]
  pub const fn r#ref(index: ModuleTypeIndex, shared: bool, kind: RefTypeKind) -> Self {
    Self::indexed(index.index, K_NON_NULLABLE, K_ANY_SUBTYPE, shared, kind)
  }

  /// RefNull(index, shared, kind)
  #[inline]
  pub const fn ref_null(index: ModuleTypeIndex, shared: bool, kind: RefTypeKind) -> Self {
    Self::indexed(index.index, K_NULLABLE, K_ANY_SUBTYPE, shared, kind)
  }

  /// RefMaybeNull(index, nullable, shared, kind)
  #[inline]
  pub const fn ref_maybe_null(index: ModuleTypeIndex, nullable: Nullability, shared: bool, kind: RefTypeKind) -> Self {
    Self::indexed(index.index, nullable, K_ANY_SUBTYPE, shared, kind)
  }

  /// Ref(HeapType): 堆类型 → 非空引用
  #[inline]
  pub const fn from_heap_type(t: HeapType) -> Self {
    Self(t.as_nullable(K_NON_NULLABLE).bits())
  }

  /// RefNull(HeapType)
  #[inline]
  pub const fn from_heap_type_nullable(t: HeapType) -> Self {
    Self(t.as_nullable(K_NULLABLE).bits())
  }

  /// RefMaybeNull(HeapType, nullable)
  #[inline]
  pub const fn from_heap_type_maybe_null(t: HeapType, nullable: Nullability) -> Self {
    Self(t.as_nullable(nullable).bits())
  }

  /// AsNonNull
  #[inline]
  pub const fn as_non_null(self) -> Self {
    self.as_nullable(K_NON_NULLABLE)
  }

  /// AsExactIfIndexed
  #[inline]
  pub const fn as_exact_if_indexed(self, exact: Exactness) -> Self {
    if (self.bits() & 0b11) == 0b11 {
      self.as_exact(exact)
    } else {
      self
    }
  }

  /// AsNonShared
  #[inline]
  pub const fn as_non_shared(self) -> Self {
    if self.bits() & 1 != 0 {
      self.as_shared(false)
    } else {
      self
    }
  }

  /// Unpacked: i8/i16 → i32, f16 → f32
  #[inline]
  pub const fn unpacked(self) -> Self {
    match self.bits() {
      ${hx(numericKinds.find((n) => n.kind === "I8").v)} => Self::primitive(NumericKind::I32),
      ${hx(numericKinds.find((n) => n.kind === "I16").v)} => Self::primitive(NumericKind::I32),
      ${hx(numericKinds.find((n) => n.kind === "F16").v)} => Self::primitive(NumericKind::F32),
      _ => self,
    }
  }

  /// Canonicalize(index): 模块索引 → 规范索引 (const fn 手写 payload 位替换)
  #[inline]
  pub const fn canonicalize(self, index: CanonicalTypeIndex) -> CanonicalValueType {
    debug_assert!((self.bits() & 0b11) == 0b11);
    CanonicalValueType::from_raw_bit_field((self.bits() & !(((1u32 << 21) - 1) << 8)) | (index.index << 8))
  }

  /// heap_type()
  #[inline]
  pub const fn heap_type(self) -> HeapType {
    debug_assert!(self.bits() & 0b11 != 0b00);
    HeapType::from_bits(self.bits())
  }

  /// ref_index()
  #[inline]
  pub const fn ref_index(self) -> ModuleTypeIndex {
    debug_assert!((self.bits() & 0b11) == 0b11);
    ModuleTypeIndex::new((self.bits() >> 8) & 0x1F_FFFF)
  }

  /// Primitive(ValueKind) (增量过渡)
  pub const fn primitive_kind(kind: ValueKind) -> Self {
    match kind {
      ValueKind::I32 => Self::primitive(NumericKind::I32),
      ValueKind::I64 => Self::primitive(NumericKind::I64),
      ValueKind::F32 => Self::primitive(NumericKind::F32),
      ValueKind::F64 => Self::primitive(NumericKind::F64),
      ValueKind::S128 => Self::primitive(NumericKind::S128),
      ValueKind::I8 => Self::primitive(NumericKind::I8),
      ValueKind::I16 => Self::primitive(NumericKind::I16),
      ValueKind::F16 => Self::primitive(NumericKind::F16),
      ValueKind::Void => Self::generic(GenericKind::Void, K_NON_NULLABLE, false),
      // 原版 UNREACHABLE; 禁 panic → Bottom 兜底 (调用方错误输入)
      _ => Self::generic(GenericKind::Bottom, K_NON_NULLABLE, false),
    }
  }

  /// For(MachineType)
  pub const fn for_machine_type(t: MachineType) -> Self {
    match t.rep {
      MachineRepresentation::Word8 | MachineRepresentation::Word16 | MachineRepresentation::Word32 => Self::primitive(NumericKind::I32),
      MachineRepresentation::Word64 => Self::primitive(NumericKind::I64),
      MachineRepresentation::Float32 => Self::primitive(NumericKind::F32),
      MachineRepresentation::Float64 => Self::primitive(NumericKind::F64),
      MachineRepresentation::AnyTagged => Self::generic(GenericKind::Any, K_NULLABLE, false),
      MachineRepresentation::Simd128 => Self::primitive(NumericKind::S128),
      MachineRepresentation::Float16 => Self::primitive(NumericKind::F32),
      MachineRepresentation::None => Self::generic(GenericKind::Bottom, K_NON_NULLABLE, false),
    }
  }

  /// name()
  pub fn name(self) -> String {
    name_common(self.bits())
  }
}

impl CanonicalValueType {
  /// Ref(index, shared, kind)
  #[inline]
  pub const fn r#ref(index: CanonicalTypeIndex, shared: bool, kind: RefTypeKind) -> Self {
    Self::indexed(index.index, K_NON_NULLABLE, K_ANY_SUBTYPE, shared, kind)
  }

  /// RefNull(index, shared, kind)
  #[inline]
  pub const fn ref_null(index: CanonicalTypeIndex, shared: bool, kind: RefTypeKind) -> Self {
    Self::indexed(index.index, K_NULLABLE, K_ANY_SUBTYPE, shared, kind)
  }

  /// RefMaybeNull(type, nullable)
  #[inline]
  pub const fn ref_maybe_null(t: Self, nullable: Nullability) -> Self {
    t.as_nullable(nullable)
  }

  /// ref_index()
  #[inline]
  pub const fn ref_index(self) -> CanonicalTypeIndex {
    debug_assert!((self.bits() & 0b11) == 0b11);
    CanonicalTypeIndex::new((self.bits() >> 8) & 0x1F_FFFF)
  }

  /// 除索引外全部位 (哈希用)
  #[inline]
  pub const fn all_bits_without_index(self) -> u32 {
    self.bits() & !(((1u32 << 21) - 1) << 8)
  }

  /// 除索引外等值
  #[inline]
  pub const fn is_equal_except_index(self, other: Self) -> bool {
    self.all_bits_without_index() == other.all_bits_without_index()
  }

  /// IsFunctionType()
  #[inline]
  pub const fn is_function_type(self) -> bool {
    (self.bits() >> 5) & 0b111 == RefTypeKind::Function as u32
  }

  /// name()
  pub fn name(self) -> String {
    name_common(self.bits())
  }
}

// ---------------------------------------------------------------------------
// 名字表公共实现 (value-type.cc name / generic_heaptype_name)
// ---------------------------------------------------------------------------

/// generic_heaptype_name 带 shared 前缀
pub(crate) fn generic_heaptype_name_prefixed(bits: u32) -> String {
  let t = ValueType::from_raw_bit_field(bits);
  let base = match GenericKind::from_bits(bits) {
`;
for (const g of genericKinds) out3 += `    Some(GenericKind::${g.name}) => "${g.printable}",\n`;
out3 += `    None => "<invalid>",
  };
  if t.is_shared() {
    format!("shared {base}")
  } else {
    base.into()
  }
}

/// name() 公共实现
pub(crate) fn name_common(bits: u32) -> String {
  let t = ValueType::from_raw_bit_field(bits);
  if t.is_numeric() {
    return match t.numeric_kind() {
`;
for (const n of numeric) out3 += `      Some(NumericKind::${n.kind}) => "${n.typeName}".into(),\n`;
out3 += `      None => "<invalid>".into(),
    };
  }
  if t.has_index() {
    let mut s = String::from("(ref ");
    if t.is_nullable() {
      s.push_str("null ");
    }
    if t.is_exact() {
      s.push_str("exact ");
    }
    s.push_str(&fields::Payload::decode(bits).to_string());
    s.push(')');
    return s;
  }
  // 泛型; 可空 null 堆类型走简写特例 (nullref 族; 原版特判五种, NoWaitqueue 走通用名)
  if t.is_nullable() && let Some(k) = t.generic_kind() && is_null_kind(k) {
    let n = match k {
      GenericKind::NoCont => Some("nullcontref"),
      GenericKind::NoExn => Some("nullexnref"),
      GenericKind::NoExtern => Some("nullexternref"),
      GenericKind::NoFunc => Some("nullfuncref"),
      GenericKind::None => Some("nullref"),
      _ => None,
    };
    if let Some(n) = n {
      return if t.is_shared() { format!("(shared {n})") } else { n.into() };
    }
  }
  let mut buf = String::new();
  let shorthand = (t.is_nullable() || t.is_sentinel() || t.is_string_view()) && !t.is_exact();
  let append_ref = t.is_nullable() && !t.is_sentinel() && !t.is_exact();
  if !shorthand {
    buf.push_str("(ref ");
  }
  if !shorthand && t.is_nullable() {
    buf.push_str("null ");
  }
  buf.push_str(&generic_heaptype_name_prefixed(bits));
  if append_ref {
    buf.push_str("ref");
  }
  if !shorthand {
    buf.push(')');
  }
  buf
}

// ---------------------------------------------------------------------------
// ValueKind 辅助 (增量过渡; 原 free functions)
// ---------------------------------------------------------------------------

/// is_reference(kind)
#[inline]
pub const fn is_reference(kind: ValueKind) -> bool {
  matches!(kind, ValueKind::Ref | ValueKind::RefNull)
}

/// machine_type(kind) (FOREACH_VALUE_TYPE machineType 列; 原版 DCHECK_NE(kBottom, kind))
pub const fn machine_type(kind: ValueKind) -> MachineType {
  match kind {
    // Void/Top/Bottom → None (原版表列 None)
    ValueKind::Void | ValueKind::Top | ValueKind::Bottom => MachineType::none(),
    ValueKind::I32 => MachineType::int32(),
    ValueKind::I64 => MachineType::int64(),
    ValueKind::F32 => MachineType::float32(),
    ValueKind::F64 => MachineType::float64(),
    ValueKind::S128 => MachineType::simd128(),
    ValueKind::I8 => MachineType::int8(),
    ValueKind::I16 => MachineType::int16(),
    ValueKind::F16 => MachineType::float16(),
    ValueKind::Ref | ValueKind::RefNull => MachineType::any_tagged(),
  }
}

/// value_kind_size_log2(kind) (哨兵 -1 → None)
#[inline]
pub const fn value_kind_size_log2(kind: ValueKind) -> Option<u8> {
  match kind {
`;
for (const k of valueKindSeq) out3 += `    ValueKind::${k.kind} => ${k.log2 >= 0 ? `Some(${k.log2})` : "None"},\n`;
out3 += `  }
}

/// value_kind_size(kind)
#[inline]
pub const fn value_kind_size(kind: ValueKind) -> Option<u8> {
  match value_kind_size_log2(kind) {
    Some(l) => Some(1 << l),
    None => None,
  }
}

/// value_kind_full_size(kind) (引用 → kSystemPointerSize)
#[inline]
pub const fn value_kind_full_size(kind: ValueKind) -> Option<usize> {
  if is_reference(kind) {
    Some(K_SYSTEM_POINTER_SIZE)
  } else {
    match value_kind_size(kind) {
      Some(s) => Some(s as usize),
      None => None,
    }
  }
}

/// name(kind) (FOREACH_VALUE_TYPE typeName 列)
pub const fn kind_name(kind: ValueKind) -> &'static str {
  match kind {
`;
{
  const names = { Void: "<void>", Ref: "ref", RefNull: "ref null", Top: "<top>", Bottom: "<bot>" };
  for (const k of valueKindSeq) out3 += `    ValueKind::${k.kind} => "${names[k.kind] ?? numeric.find((n) => n.kind === k.kind).typeName}",\n`;
}
out3 += `  }
}

/// is_packed(kind)
#[inline]
pub const fn is_packed(kind: ValueKind) -> bool {
  matches!(kind, ValueKind::I8 | ValueKind::I16 | ValueKind::F16)
}

/// unpacked(kind): i8/i16 → i32, f16 → f32
#[inline]
pub const fn unpacked(kind: ValueKind) -> ValueKind {
  match kind {
    ValueKind::I8 | ValueKind::I16 => ValueKind::I32,
    ValueKind::F16 => ValueKind::F32,
    _ => kind,
  }
}

// ---------------------------------------------------------------------------
// 预定义全局类型常量 (kWasm*; value-type.h:1144-1208)
// ---------------------------------------------------------------------------
`;
{
  // numeric 8 个 → IndependentValueType; 其余 HeapType 族按可空/共享
  const heapConsts = [
    ["K_WASM_VOID", "Void", false, false], ["K_WASM_TOP", "Top", true, false], ["K_WASM_BOTTOM", "Bottom", false, false],
    ["K_WASM_FUNC_REF", "Func", true, false], ["K_WASM_ANY_REF", "Any", true, false], ["K_WASM_SHARED_ANY_REF", "Any", true, true],
    ["K_WASM_EXTERN_REF", "Extern", true, false], ["K_WASM_REF_EXTERN", "Extern", false, false], ["K_WASM_SHARED_EXTERN_REF", "Extern", true, true],
    ["K_WASM_EXN_REF", "Exn", true, false], ["K_WASM_SHARED_EXN_REF", "Exn", true, true],
    ["K_WASM_EQ_REF", "Eq", true, false], ["K_WASM_SHARED_EQ_REF", "Eq", true, true],
    ["K_WASM_I31_REF", "I31", true, false], ["K_WASM_REF_I31", "I31", false, false],
    ["K_WASM_STRUCT_REF", "Struct", true, false], ["K_WASM_ARRAY_REF", "Array", true, false],
    ["K_WASM_WAITQUEUE_REF", "Waitqueue", true, true], ["K_WASM_NULL_WAITQUEUE_REF", "NoWaitqueue", true, true],
    ["K_WASM_STRING_REF", "String", true, false], ["K_WASM_REF_STRING", "String", false, false],
    ["K_WASM_REF_NULL_EXTERN_STRING", "ExternString", true, false], ["K_WASM_REF_EXTERN_STRING", "ExternString", false, false],
    ["K_WASM_REF_NULL_SHARED_EXTERN_STRING", "ExternString", true, true], ["K_WASM_REF_SHARED_EXTERN_STRING", "ExternString", false, true],
    ["K_WASM_STRING_VIEW_WTF8", "StringViewWtf8", false, false], ["K_WASM_STRING_VIEW_WTF16", "StringViewWtf16", false, false],
    ["K_WASM_STRING_VIEW_ITER", "StringViewIter", false, false],
    ["K_WASM_NULL_REF", "None", true, false], ["K_WASM_SHARED_NULL_REF", "None", true, true], ["K_WASM_REF_NONE", "None", false, false],
    ["K_WASM_NULL_EXTERN_REF", "NoExtern", true, false], ["K_WASM_NULL_EXN_REF", "NoExn", true, false],
    ["K_WASM_NULL_FUNC_REF", "NoFunc", true, false], ["K_WASM_CONT_REF", "Cont", true, false], ["K_WASM_NULL_CONT_REF", "NoCont", true, false],
  ];
  for (const n of numeric) out3 += `pub const K_WASM_${n.kind.toUpperCase()}: IndependentValueType = IndependentValueType::primitive(NumericKind::${n.kind});\n`;
  out3 += "\n";
  for (const [name, kind, nullable, shared] of heapConsts) {
    out3 += `pub const ${name}: IndependentHeapType = IndependentHeapType::new(GenericKind::${kind}, ${nullable ? "K_NULLABLE" : "K_NON_NULLABLE"}, ${shared});\n`;
  }
  if (heapConsts.length + numeric.length !== predefined.length) {
    throw new Error(`预定义常量对数不符: ${heapConsts.length}+${numeric.length} != ${predefined.length}`);
  }
}
out3 += `
// ---------------------------------------------------------------------------
// MachineType 最小子集 (源: codegen/machine-type.h; 后续迁 codegen crate)
// ---------------------------------------------------------------------------

/// 机器表示 (value-type 所需子集)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MachineRepresentation {
  None,
  Word8,
  Word16,
  Word32,
  Word64,
  Float16,
  Float32,
  Float64,
  Simd128,
  AnyTagged,
}

/// ElementSizeLog2Of
#[inline]
pub const fn element_size_log2(rep: MachineRepresentation) -> u8 {
  match rep {
    MachineRepresentation::None => 0,
    MachineRepresentation::Word8 => 0,
    MachineRepresentation::Word16 | MachineRepresentation::Float16 => 1,
    MachineRepresentation::Word32 | MachineRepresentation::Float32 => 2,
    MachineRepresentation::Word64 | MachineRepresentation::Float64 => 3,
    MachineRepresentation::Simd128 => 4,
    MachineRepresentation::AnyTagged => K_TAGGED_SIZE_LOG2 as u8,
  }
}

/// ElementSizeInBytes
#[inline]
pub const fn element_size(rep: MachineRepresentation) -> u32 {
  1u32 << element_size_log2(rep)
}

/// MachineType {representation, signed} (最小子集)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MachineType {
  pub rep: MachineRepresentation,
  pub signed: bool,
}

impl MachineType {
  #[inline]
  pub const fn none() -> Self {
    Self { rep: MachineRepresentation::None, signed: false }
  }
  #[inline]
  pub const fn any_tagged() -> Self {
    Self { rep: MachineRepresentation::AnyTagged, signed: false }
  }
`;
for (const mt of ["Int8", "Uint8", "Int16", "Uint16", "Int32", "Uint32", "Int64", "Uint64", "Float16", "Float32", "Float64", "Simd128"]) {
  const rep = ({ Int8: "Word8", Uint8: "Word8", Int16: "Word16", Uint16: "Word16", Int32: "Word32", Uint32: "Word32", Int64: "Word64", Uint64: "Word64" })[mt] ?? mt;
  const signed = mt.startsWith("Int");
  const fname = mt[0].toLowerCase() + mt.slice(1);
  out3 += `  #[inline]
  pub const fn ${fname}() -> Self {
    Self { rep: MachineRepresentation::${rep}, signed: ${signed} }
  }
`;
}
out3 += `}

impl ValueType {
  /// machine_type(): 哨兵 → None, 引用 → AnyTagged, 数值查表
  pub fn machine_type(self) -> MachineType {
    if self.is_sentinel() {
      return MachineType::none();
    }
    if self.is_ref() {
      return MachineType::any_tagged();
    }
    K_MACHINE_TYPE[self.numeric_kind().map(NumericKind::zero_based_index).unwrap_or(0) as usize]
  }

  /// machine_representation()
  pub fn machine_representation(self) -> MachineRepresentation {
    self.machine_type().rep
  }
}

/// 数值 kind 机器类型表 (FOREACH_NUMERIC_VALUE_TYPE machineType 列)
const K_MACHINE_TYPE: [MachineType; ${NUM_NUMERIC}] = [
`;
for (const n of numeric) {
  const fn0 = n.mtype[0].toLowerCase() + n.mtype.slice(1);
  out3 += `  MachineType::${fn0}(), // ${n.kind}\n`;
}
out3 += `];

// ---------------------------------------------------------------------------
// LoadType (FOREACH_LOAD_TYPE)
// ---------------------------------------------------------------------------

/// 内存读取类型枚举
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoadTypeValue {
`;
for (const [suffix, ty] of loadTypes) out3 += `  ${ty}Load${suffix},\n`;
out3 += `}

/// LoadType 包装 (value/size/memtype 查表)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadType(pub LoadTypeValue);

impl LoadType {
  /// size_log_2()
  #[inline]
  pub const fn size_log_2(self) -> u8 {
    K_LOAD_SIZE_LOG2[self.0 as usize]
  }
  /// size()
  #[inline]
  pub const fn size(self) -> u8 {
    K_LOAD_SIZE[self.0 as usize]
  }
  /// value_type()
  #[inline]
  pub const fn value_type(self) -> ValueType {
    K_LOAD_VALUE_TYPE[self.0 as usize]
  }
  /// mem_type()
  #[inline]
  pub const fn mem_type(self) -> MachineType {
    K_LOAD_MEM_TYPE[self.0 as usize]
  }

  /// ForValueKind(kind, is_signed)
  pub const fn for_value_kind(kind: ValueKind, is_signed: bool) -> Self {
    match kind {
      ValueKind::I32 => Self(LoadTypeValue::I32Load),
      ValueKind::I64 => Self(LoadTypeValue::I64Load),
      ValueKind::F32 => Self(LoadTypeValue::F32Load),
      ValueKind::F64 => Self(LoadTypeValue::F64Load),
      ValueKind::S128 => Self(LoadTypeValue::S128Load),
      ValueKind::I8 => Self(if is_signed { LoadTypeValue::I32Load8S } else { LoadTypeValue::I32Load8U }),
      ValueKind::I16 => Self(if is_signed { LoadTypeValue::I32Load16S } else { LoadTypeValue::I32Load16U }),
      ValueKind::F16 => Self(LoadTypeValue::F32LoadF16),
      // 原版 UNREACHABLE; 禁 panic → I32Load 兜底
      _ => Self(LoadTypeValue::I32Load),
    }
  }
}

const K_LOAD_SIZE: [u8; ${loadTypes.length}] = [${loadTypes.map(([, , mt]) => 1 << MT_SIZE_LOG2[mt]).join(", ")}];
const K_LOAD_SIZE_LOG2: [u8; ${loadTypes.length}] = [${loadTypes.map(([, , mt]) => MT_SIZE_LOG2[mt]).join(", ")}];
const K_LOAD_VALUE_TYPE: [ValueType; ${loadTypes.length}] = [
`;
for (const [, ty] of loadTypes) out3 += `  ValueType::primitive(NumericKind::${ty}),\n`;
out3 += `];
const K_LOAD_MEM_TYPE: [MachineType; ${loadTypes.length}] = [
`;
for (const [, , mt] of loadTypes) {
  const fn0 = mt[0].toLowerCase() + mt.slice(1);
  out3 += `  MachineType::${fn0}(),\n`;
}
out3 += `];

// ---------------------------------------------------------------------------
// StoreType (FOREACH_STORE_TYPE)
// ---------------------------------------------------------------------------

/// 内存存储类型枚举
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreTypeValue {
`;
for (const [suffix, ty] of storeTypes) out3 += `  ${ty}Store${suffix},\n`;
out3 += `}

/// StoreType 包装
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoreType(pub StoreTypeValue);

impl StoreType {
  /// size_log_2()
  #[inline]
  pub const fn size_log_2(self) -> u8 {
    K_STORE_SIZE_LOG2[self.0 as usize]
  }
  /// size()
  #[inline]
  pub const fn size(self) -> u8 {
    1 << self.size_log_2()
  }
  /// value_type()
  #[inline]
  pub const fn value_type(self) -> ValueType {
    K_STORE_VALUE_TYPE[self.0 as usize]
  }
  /// mem_rep()
  #[inline]
  pub const fn mem_rep(self) -> MachineRepresentation {
    K_STORE_MEM_REP[self.0 as usize]
  }

  /// ForValueKind(kind)
  pub const fn for_value_kind(kind: ValueKind) -> Self {
    match kind {
      ValueKind::I32 => Self(StoreTypeValue::I32Store),
      ValueKind::I64 => Self(StoreTypeValue::I64Store),
      ValueKind::F32 => Self(StoreTypeValue::F32Store),
      ValueKind::F64 => Self(StoreTypeValue::F64Store),
      ValueKind::S128 => Self(StoreTypeValue::S128Store),
      ValueKind::I8 => Self(StoreTypeValue::I32Store8),
      ValueKind::I16 => Self(StoreTypeValue::I32Store16),
      ValueKind::F16 => Self(StoreTypeValue::F32StoreF16),
      // 原版 UNREACHABLE; 禁 panic → I32Store 兜底
      _ => Self(StoreTypeValue::I32Store),
    }
  }
}

const K_STORE_SIZE_LOG2: [u8; ${storeTypes.length}] = [${storeTypes.map(([, , mr]) => MT_SIZE_LOG2[mr]).join(", ")}];
const K_STORE_VALUE_TYPE: [ValueType; ${storeTypes.length}] = [
`;
for (const [, ty] of storeTypes) out3 += `  ValueType::primitive(NumericKind::${ty}),\n`;
out3 += `];
const K_STORE_MEM_REP: [MachineRepresentation; ${storeTypes.length}] = [
`;
for (const [, , mr] of storeTypes) out3 += `  MachineRepresentation::${mr},\n`;
out3 += `];
`;

const p = writeSrc("v8-wasm-core", "value_type.rs", out + out2 + out3);
console.log(`→ ${p} (三段合并)`);
