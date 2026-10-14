#!/usr/bin/env bun
// tool/src/interpreter/bytecode-operands.h.js
// 机械提取 v8/src/interpreter/bytecode-operands.{h,cc} 的操作数类型/刻度/位标志体系
// → v8-interpreter/src/bytecode_operands.rs
//
// 对数基准 (clang -E 独立验证, 2026-10-11):
//   OPERAND_TYPE_LIST = 23 (NON_REGISTER 15 + REGISTER 8)
//   OPERAND_SCALE_LIST = 3, OPERAND_TYPE_INFO_LIST = 5
// 产物: OperandScale / OperandSize / OperandTypeInfo / OperandType /
//       ImplicitRegisterUse 五枚举 + BytecodeOperands 静态谓词 + 名称表
import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/interpreter/bytecode-operands.h");
const ccSrc = readV8("src/interpreter/bytecode-operands.cc");

// —— 机械解析辅助: 抽取 #define NAME(V) 多行宏体 (拼接续行, 去注释) ——
function macroBody(src, name) {
  const re = new RegExp(`#define\\s+${name}\\(`);
  const m = re.exec(src);
  if (!m) throw new Error(`宏缺失: ${name}`);
  const allLines = src.slice(m.index).split("\n");
  const body = [];
  // 首行 = "#define NAME(args) 体..." — 取 () 后余体
  const head = allLines[0].slice(allLines[0].indexOf(")") + 1);
  let start = 1;
  if (head.trim()) {
    const cont = head.trimEnd().endsWith("\\");
    body.push(cont ? head.trimEnd().slice(0, -1) : head);
    if (!cont) start = allLines.length; // 单行宏, 无续行
  }
  for (let i = start; i < allLines.length; i++) {
    const line = allLines[i];
    const cont = line.endsWith("\\");
    body.push(cont ? line.slice(0, -1) : line);
    if (!cont) break;
  }
  return body.join("\n").replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*$/gm, "");
}

// 解析 V(Name, InfoType) 式条目
function parseEntries(body) {
  const out = [];
  const re = /V\s*\(([^()]*)\)/g;
  let m;
  while ((m = re.exec(body)) !== null) {
    const parts = m[1].split(",").map((s) => s.trim()).filter(Boolean);
    out.push(parts);
  }
  return out;
}

function expectEntries(src, macro, want) {
  const got = parseEntries(macroBody(src, macro));
  if (got.length !== want) throw new Error(`${macro} 对数不符: 期望 ${want} 实得 ${got.length}`);
  return got;
}

// —— 各子表机械展开 (期望条数 = clang -E 基准分解) ——
const invalidList = expectEntries(hSrc, "INVALID_OPERAND_TYPE_LIST", 1);
const unsignedFixed = expectEntries(hSrc, "UNSIGNED_FIXED_SCALAR_OPERAND_TYPE_LIST", 7);
const unsignedScalable = expectEntries(hSrc, "UNSIGNED_SCALABLE_SCALAR_OPERAND_TYPE_LIST", 6);
const signedScalable = expectEntries(hSrc, "SIGNED_SCALABLE_SCALAR_OPERAND_TYPE_LIST", 1);
const regInput = expectEntries(hSrc, "REGISTER_INPUT_OPERAND_TYPE_LIST", 3);
const regOutput = expectEntries(hSrc, "REGISTER_OUTPUT_OPERAND_TYPE_LIST", 4);

const nonRegisterList = [...invalidList, ...unsignedFixed, ...unsignedScalable, ...signedScalable];
const regInOut = [["RegInOut", "OperandTypeInfo::kScalableSignedByte"]];
const registerList = [...regInput, ...regOutput, ...regInOut];
const operandTypeList = [...nonRegisterList, ...registerList];
if (operandTypeList.length !== 23) throw new Error(`OPERAND_TYPE_LIST 对数不符: 期望 23 实得 ${operandTypeList.length}`);

const scaleList = expectEntries(hSrc, "OPERAND_SCALE_LIST", 3);
const infoList = expectEntries(hSrc, "OPERAND_TYPE_INFO_LIST", 5);

// 结构锚: 关键顺序前提 (范围谓词依赖枚举次序)
for (const needle of [
  "kNone = 0,\n  kByte = 1,\n  kShort = 2,\n  kQuad = 4,",
  "kReadWriteAccumulator = kReadAccumulator | kWriteAccumulator,",
  "kReadAndClobberAccumulator = kReadAccumulator | kClobberAccumulator,",
  "kReadAccumulatorWriteShortStar = kReadAccumulator | kWriteShortStar",
]) {
  if (!hSrc.includes(needle)) throw new Error(`结构锚缺失: ${JSON.stringify(needle.slice(0, 40))}`);
}
for (const needle of [
  'case ImplicitRegisterUse::kNone:\n      return "None";',
  'case OperandSize::kQuad:\n      return "Quad";',
]) {
  if (!ccSrc.includes(needle)) throw new Error(`bytecode-operands.cc 结构锚缺失: ${JSON.stringify(needle.slice(0, 30))}`);
}

// OperandType → OperandTypeInfo 映射
const infoOf = new Map(operandTypeList.map(([name, info]) => [name, info.replace("OperandTypeInfo::k", "")]));
const infoIndex = new Map(infoList.map(([name], i) => [name, i]));

// 类型名转 Rust 变体 (PascalCase)
const pascal = (s) => s.split("_").map((w) => w.charAt(0).toUpperCase() + w.slice(1)).join("");

let rust = banner("src/interpreter/bytecode-operands.h.js", "src/interpreter/bytecode-operands.{h,cc}", "Ignition 操作数类型/刻度/累加器位标志体系 (clang -E 对数: OPERAND_TYPE_LIST=23, SCALE=3, INFO=5)");

rust += `
/// receiver 转换模式 (原版 src/common/globals.h ConvertReceiverMode; 供字节码 receiver 模式查询就近落位)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ReceiverMode {
  /// kNotNullOrUndefined: 调用方保证 receiver 非空非 undefined (CallProperty 族)
  NotNullOrUndefined = 0,
  /// kNullOrUndefined: receiver 可为 null/undefined (CallUndefinedReceiver 族)
  NullOrUndefined = 1,
  /// kAny: 任意 (CallAnyReceiver / Construct 族)
  Any = 2,
}

/// 操作数刻度 (原版 enum class OperandScale : uint8_t; 可缩放操作数的缩放系数)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OperandScale {
`;
for (const [name, scale] of scaleList) {
  rust += `  /// 原版 k${name}\n  ${pascal(name)} = ${scale.replace(/\D/g, "")},\n`;
}
rust += `}

impl OperandScale {
  /// 原版 kLast = kQuadruple
  pub const LAST: OperandScale = Self::Quadruple;
  /// 表索引 (原版 OperandScaleAsIndex: scale >> 1, Single->0 Double->1 Quadruple->2)
  #[inline]
  #[must_use]
  pub const fn as_index(self) -> usize {
    ((self as u8) >> 1) as usize
  }
  /// 原版名 (bytecode-operands.cc OperandScaleToString)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
`;
for (const [name] of scaleList) rust += `      Self::${pascal(name)} => "${name}",\n`;
rust += `    }
  }
}

/// 操作数尺寸类 (原版 enum class OperandSize : uint8_t; 数值即字节数)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OperandSize {
  None = 0,
  Byte = 1,
  Short = 2,
  Quad = 4,
}

impl OperandSize {
  /// 原版 kLast = kQuad
  pub const LAST: OperandSize = Self::Quad;
  /// 原版名 (bytecode-operands.cc OperandSizeToString)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::None => "None",
      Self::Byte => "Byte",
      Self::Short => "Short",
      Self::Quad => "Quad",
    }
  }
}

/// 操作数元信息 (原版 enum class OperandTypeInfo : uint8_t; 列 = 可缩放/无符号/基准尺寸)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OperandTypeInfo {
`;
for (const [name, scalable, unsigned, size] of infoList) {
  rust += `  /// 可缩放=${scalable} 无符号=${unsigned} 基准=${size.replace("OperandSize::k", "")}\n  ${pascal(name)},\n`;
}
rust += `}

impl OperandTypeInfo {
  /// 是否可缩放 (bytecode-traits.h OperandTypeInfoTraits::kIsScalable)
  #[inline]
  #[must_use]
  pub const fn is_scalable(self) -> bool {
    matches!(self, Self::ScalableSignedByte | Self::ScalableUnsignedByte)
  }
  /// 是否无符号
  #[inline]
  #[must_use]
  pub const fn is_unsigned(self) -> bool {
    matches!(self, Self::ScalableUnsignedByte | Self::FixedUnsignedByte | Self::FixedUnsignedShort)
  }
  /// 未缩放基准尺寸 (OperandTypeInfoTraits::kUnscaledSize)
  #[inline]
  #[must_use]
  pub const fn unscaled_size(self) -> OperandSize {
    match self {
      Self::None => OperandSize::None,
      Self::ScalableSignedByte | Self::ScalableUnsignedByte | Self::FixedUnsignedByte => OperandSize::Byte,
      Self::FixedUnsignedShort => OperandSize::Short,
    }
  }
}

/// 操作数类型全表 (原版 enum class OperandType : uint8_t; 顺序敏感 — 范围谓词依赖)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OperandType {
`;
operandTypeList.forEach(([name], i) => {
  rust += `  /// 原版 k${name} (${infoOf.get(name)})\n  ${pascal(name)} = ${i},\n`;
});
rust += `}

impl OperandType {
  /// 原版 kLast (COUNT_OPERAND_TYPES 求和 = 最后一个变体的值)
  pub const LAST: OperandType = Self::${pascal(operandTypeList[operandTypeList.length - 1][0])};
  /// 操作数类型总数 (原版 BytecodeOperands::kOperandTypeCount = kLast + 1)
  pub const OPERAND_TYPE_COUNT: usize = ${operandTypeList.length};
  /// 对应元信息 (bytecode-traits.h OperandTraits::kOperandTypeInfo)
  #[inline]
  #[must_use]
  pub const fn type_info(self) -> OperandTypeInfo {
    OPERAND_TYPE_INFOS[self as usize]
  }
  /// 原版名 (bytecode-operands.cc OperandTypeToString)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
`;
for (const [name] of operandTypeList) rust += `      Self::${pascal(name)} => "${name}",\n`;
rust += `    }
  }

  #[inline]
  pub fn scale_for_operand(self, operand: u32) -> OperandScale {
    if BytecodeOperands::is_scalable_signed_byte(self) {
      BytecodeOperands::scale_for_signed_operand(operand as i32)
    } else if BytecodeOperands::is_scalable_unsigned_byte(self) {
      BytecodeOperands::scale_for_unsigned_operand(operand)
    } else {
      OperandScale::Single
    }
  }

  #[inline]
  pub fn size(self, scale: OperandScale) -> OperandSize {
    crate::bytecode_traits::OPERAND_KIND_SIZES[scale.as_index()][self as usize]
  }
}

/// 每个操作数类型的元信息表 (OPERAND_TYPE_LIST 顺序)
pub static OPERAND_TYPE_INFOS: [OperandTypeInfo; OperandType::OPERAND_TYPE_COUNT] = [
`;
for (const [name] of operandTypeList) rust += `  OperandTypeInfo::${infoOf.get(name)},\n`;
rust += `];

/// 非寄存器操作数 (原版 NON_REGISTER_OPERAND_TYPE_LIST, 共 ${nonRegisterList.length} 种)
pub const NON_REGISTER_OPERAND_TYPES: [OperandType; ${nonRegisterList.length}] = [
`;
for (const [name] of nonRegisterList) rust += `  OperandType::${pascal(name)},\n`;
rust += `];

/// 寄存器操作数 (原版 REGISTER_OPERAND_TYPE_LIST, 共 ${registerList.length} 种)
pub const REGISTER_OPERAND_TYPES: [OperandType; ${registerList.length}] = [
`;
for (const [name] of registerList) rust += `  OperandType::${pascal(name)},\n`;
rust += `];

/// 寄存器输入操作数 (原版 REGISTER_INPUT_OPERAND_TYPE_LIST)
pub const REGISTER_INPUT_OPERAND_TYPES: [OperandType; ${regInput.length}] = [
`;
for (const [name] of regInput) rust += `  OperandType::${pascal(name)},\n`;
rust += `];

/// 寄存器输出操作数 (原版 REGISTER_OUTPUT_OPERAND_TYPE_LIST)
pub const REGISTER_OUTPUT_OPERAND_TYPES: [OperandType; ${regOutput.length}] = [
`;
for (const [name] of regOutput) rust += `  OperandType::${pascal(name)},\n`;
rust += `];

/// 累加器隐式读写语义位 (原版 enum class ImplicitRegisterUse : uint8_t)
/// 零依赖位标志: u8 新类型 + 命名常量 (对齐原版位布局)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ImplicitRegisterUse(pub u8);

impl ImplicitRegisterUse {
  /// 无隐式累加器访问 (原版 kNone = 0)
  pub const NONE: Self = Self(0);
  /// 读累加器 (1 << 0)
  pub const READ_ACCUMULATOR: Self = Self(1 << 0);
  /// 写累加器 (1 << 1)
  pub const WRITE_ACCUMULATOR: Self = Self(1 << 1);
  /// 覆写 (旧值不可存活) (1 << 2)
  pub const CLOBBER_ACCUMULATOR: Self = Self(1 << 2);
  /// ShortStar 隐式写寄存器 (1 << 3)
  pub const WRITE_SHORT_STAR: Self = Self(1 << 3);
  /// 读 + 写 (原版 kReadWriteAccumulator)
  pub const READ_WRITE_ACCUMULATOR: Self = Self(Self::READ_ACCUMULATOR.0 | Self::WRITE_ACCUMULATOR.0);
  /// 读 + 覆写 (原版 kReadAndClobberAccumulator)
  pub const READ_AND_CLOBBER_ACCUMULATOR: Self = Self(Self::READ_ACCUMULATOR.0 | Self::CLOBBER_ACCUMULATOR.0);
  /// 读 + ShortStar 写 (原版 kReadAccumulatorWriteShortStar)
  pub const READ_ACCUMULATOR_WRITE_SHORT_STAR: Self = Self(Self::READ_ACCUMULATOR.0 | Self::WRITE_SHORT_STAR.0);

  /// 原始位值
  #[inline]
  #[must_use]
  pub const fn bits(self) -> u8 {
    self.0
  }

  /// 是否包含全部指定位
  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    self.0 & other.0 == other.0
  }

  /// 是否有任一交叠位
  #[inline]
  #[must_use]
  pub const fn intersects(self, other: Self) -> bool {
    self.0 & other.0 != 0
  }

  /// 原版名 (bytecode-operands.cc ImplicitRegisterUseToString)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self.0 {
      0 => "None",
      1 => "ReadAccumulator",
      2 => "WriteAccumulator",
      4 => "ClobberAccumulator",
      8 => "WriteShortStar",
      5 => "ReadAndClobberAccumulator",
      3 => "ReadWriteAccumulator",
      9 => "ReadAccumulatorWriteShortStar",
      _ => "<组合>",
    }
  }
}

impl std::ops::BitOr for ImplicitRegisterUse {
  type Output = Self;
  #[inline]
  fn bitor(self, rhs: Self) -> Self {
    Self(self.0 | rhs.0)
  }
}

/// 操作数体系静态谓词集合 (原版 class BytecodeOperands : AllStatic)
pub struct BytecodeOperands;

impl BytecodeOperands {
  /// 操作数类型总数 = kLast + 1
  pub const OPERAND_TYPE_COUNT: usize = OperandType::OPERAND_TYPE_COUNT;
  /// 操作数刻度总数 (OPERAND_SCALE_COUNT 展开 = 3)
  pub const OPERAND_SCALE_COUNT: usize = ${scaleList.length};

  /// 刻度 -> 表索引 (原版 OperandScaleAsIndex)
  #[inline]
  #[must_use]
  pub const fn operand_scale_as_index(scale: OperandScale) -> usize {
    scale.as_index()
  }

  /// 是否读累加器
  #[inline]
  #[must_use]
  pub const fn reads_accumulator(use_: ImplicitRegisterUse) -> bool {
    use_.contains(ImplicitRegisterUse::READ_ACCUMULATOR)
  }

  /// 是否写累加器
  #[inline]
  #[must_use]
  pub const fn writes_accumulator(use_: ImplicitRegisterUse) -> bool {
    use_.contains(ImplicitRegisterUse::WRITE_ACCUMULATOR)
  }

  /// 是否覆写累加器
  #[inline]
  #[must_use]
  pub const fn clobbers_accumulator(use_: ImplicitRegisterUse) -> bool {
    use_.contains(ImplicitRegisterUse::CLOBBER_ACCUMULATOR)
  }

  /// 是否写或覆写累加器
  #[inline]
  #[must_use]
  pub const fn writes_or_clobbers_accumulator(use_: ImplicitRegisterUse) -> bool {
    use_.intersects(ImplicitRegisterUse(ImplicitRegisterUse::WRITE_ACCUMULATOR.0 | ImplicitRegisterUse::CLOBBER_ACCUMULATOR.0))
  }

  /// 是否隐式写寄存器 (ShortStar)
  #[inline]
  #[must_use]
  pub const fn writes_implicit_register(use_: ImplicitRegisterUse) -> bool {
    use_.contains(ImplicitRegisterUse::WRITE_SHORT_STAR)
  }

  /// 是否可缩放有符号字节操作数 (原版范围 kImm..=kRegInOut)
  #[inline]
  #[must_use]
  pub const fn is_scalable_signed_byte(t: OperandType) -> bool {
    (t as u8) >= OperandType::Imm as u8 && (t as u8) <= OperandType::RegInOut as u8
  }

  /// 是否可缩放无符号字节操作数 (原版范围 kConstantPoolIndex..=kRegCount)
  #[inline]
  #[must_use]
  pub const fn is_scalable_unsigned_byte(t: OperandType) -> bool {
    (t as u8) >= OperandType::ConstantPoolIndex as u8 && (t as u8) <= OperandType::RegCount as u8
  }

  #[inline]
  pub fn scale_for_signed_operand(v: i32) -> OperandScale {
    if v >= i8::MIN as i32 && v <= i8::MAX as i32 {
      OperandScale::Single
    } else if v >= i16::MIN as i32 && v <= i16::MAX as i32 {
      OperandScale::Double
    } else {
      OperandScale::Quadruple
    }
  }

  #[inline]
  pub fn scale_for_unsigned_operand(v: u32) -> OperandScale {
    if v <= u8::MAX as u32 {
      OperandScale::Single
    } else if v <= u16::MAX as u32 {
      OperandScale::Double
    } else {
      OperandScale::Quadruple
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(OperandType::OPERAND_TYPE_COUNT == 23);
  assert!(BytecodeOperands::OPERAND_SCALE_COUNT == 3);
  assert!(OperandType::Imm as u8 == 14);
  assert!(OperandType::RegInOut as u8 == 22);
  assert!(OperandType::ConstantPoolIndex as u8 == 8);
  assert!(OperandType::RegCount as u8 == 13);
  assert!(OperandScale::Quadruple as u8 == 4);
  assert!(OperandSize::Quad as u8 == 4);
  assert!(ImplicitRegisterUse::READ_WRITE_ACCUMULATOR.bits() == 3);
  assert!(ImplicitRegisterUse::READ_AND_CLOBBER_ACCUMULATOR.bits() == 5);
  assert!(ImplicitRegisterUse::READ_ACCUMULATOR_WRITE_SHORT_STAR.bits() == 9);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_operands.rs", rust);
console.log(`written ${p} (${operandTypeList.length} operand types, ${scaleList.length} scales, ${infoList.length} type-infos)`);
