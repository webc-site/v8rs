#!/usr/bin/env bun
// tool/src/codegen/machine-type.cc.js
// 机械提取 v8/src/codegen/machine-type.cc 的字符串映射表 + is_subtype 真值对,
// 附 atomic-memory-order.h 枚举与 callable.h 骨架 → v8-codegen-core/src/machine_type.rs
// 注: MachineRepresentation/MachineSemantic 枚举本体由 machine-type.h 覆盖脚本
// (tool/src/compiler/use-info.js → v8-compiler-graph) 负责, 本脚本不重复定义。

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/codegen/machine-type.cc");
const atomicSrc = readV8("src/codegen/atomic-memory-order.h");
const callableSrc = readV8("src/codegen/callable.h");

// —— 提取 MachineReprToString switch 表: (variant, string) ——
function extractReturnSwitch(src, enumName) {
  const fnMatch = src.match(
    new RegExp(`const char\\* ${enumName.replace("$", "\\$")}[^{]*\\{([\\s\\S]*?)\\n\\}`),
  );
  if (!fnMatch) throw new Error(`未找到 ${enumName} 相关函数`);
  const rows = [];
  const re = /case MachineRepresentation::(\w+):\s*return "([^"]*)";/g;
  let m;
  while ((m = re.exec(fnMatch[1])) !== null) rows.push([m[1], m[2]]);
  return rows;
}

const reprRows = extractReturnSwitch(ccSrc, "MachineReprToString");

// —— MachineSemantic operator<< 表: case kX: return os << "STR" ——
const semFn = ccSrc.match(
  /std::ostream& operator<<\(std::ostream& os, MachineSemantic type\) \{([\s\S]*?)\n\}/,
);
if (!semFn) throw new Error("未找到 MachineSemantic operator<<");
const semRows = [];
{
  const re = /case MachineSemantic::(\w+):\s*(?:return os <<|os <<) "([^"]*)"/g;
  let m;
  while ((m = re.exec(semFn[1])) !== null) semRows.push([m[1], m[2]]);
}

// 对数守卫
const EXPECT_REPR = 21; // kNone..kSandboxedPointer (原版 switch 全分支)
const EXPECT_SEM = 11; // kNone..kAny
if (reprRows.length !== EXPECT_REPR)
  throw new Error(`MachineRepresentation 表 ${reprRows.length} != ${EXPECT_REPR}`);
if (semRows.length !== EXPECT_SEM)
  throw new Error(`MachineSemantic 表 ${semRows.length} != ${EXPECT_SEM}`);
if (reprRows[0][0] !== "kNone" || reprRows.at(-1)[0] !== "kSandboxedPointer")
  throw new Error("repr 表首尾不符");
if (semRows[0][0] !== "kNone" || semRows.at(-1)[0] !== "kAny")
  throw new Error("semantic 表首尾不符");

// —— is_subtype 真值对 (除 rep1==rep2 外): 机械对应 machine-type.cc IsSubtype ——
const SUBTYPE_PAIRS = [
  ["kTaggedSigned", "kTagged"],
  ["kTaggedPointer", "kTagged"],
  ["kCompressedPointer", "kCompressed"],
];

// —— atomic-memory-order.h: AtomicMemoryOrder 枚举 ——
const amo = atomicSrc.match(
  /enum class AtomicMemoryOrder : uint8_t \s*\{([^}]*)\}/,
);
if (!amo) throw new Error("未找到 AtomicMemoryOrder 枚举");
const amoEntries = amo[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean)
  .map((e) => {
    const [n, v] = e.split("=").map((x) => x.trim());
    return { name: n, value: v === undefined ? null : parseInt(v, 10) };
  });
if (amoEntries.length !== 2) throw new Error("AtomicMemoryOrder 条数 != 2");

// —— callable.h: 结构骨架 ——
if (!callableSrc.includes("class Callable final")) throw new Error("缺 Callable");
if (!callableSrc.includes("Handle<Code> code()")) throw new Error("缺 code()");

function rustStr(s) {
  return `"${s.replace(/\\/g, "\\\\").replace(/"/g, '\\"')}"`;
}

let rust = banner(
  "src/codegen/machine-type.cc.js",
  "src/codegen/machine-type.cc;src/codegen/atomic-memory-order.h;src/codegen/callable.h",
  "机器表示字符串表 + 原子内存序 + Callable 骨架",
);

rust += `
// MachineRepresentation/MachineSemantic 枚举本体见 machine-type.h 覆盖脚本
// (tool/src/compiler/use-info.js → v8-compiler-graph/src/machine_type.rs);
// 本模块只收 machine-type.cc 的输出映射与谓词。

/// 原版 MachineReprToString: representation → 调试字符串 (按 switch 分支顺序)
pub const MACHINE_REPR_TO_STRING: &[(&str, &str)] = &[
${reprRows.map(([k, v]) => `  (${rustStr(k)}, ${rustStr(v)}),`).join("\n")}
];

/// 原版 operator<<(MachineSemantic): semantic → 调试字符串
pub const MACHINE_SEMANTIC_TO_STRING: &[(&str, &str)] = &[
${semRows.map(([k, v]) => `  (${rustStr(k)}, ${rustStr(v)}),`).join("\n")}
];

/// 原版 IsSubtype 真值对 (rep1 != rep2 且返回 true 的全部组合);
/// 其余情形: 相等即子类型, 其他一律 false。
pub const IS_SUBTYPE_EXTRA_PAIRS: &[(&str, &str)] = &[
${SUBTYPE_PAIRS.map(([a, b]) => `  (${rustStr(a)}, ${rustStr(b)}),`).join("\n")}
];

/// 原版 AtomicMemoryOrder (编码须与 Wasm wire bytes 一致)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AtomicMemoryOrder {
${amoEntries.map((e) => `  /// ${e.name}\n  ${e.name.replace(/^k/, "")} = ${e.value ?? amoEntries.indexOf(e)},`).join("\n")}
}

impl AtomicMemoryOrder {
  /// 原版 hash_value: 直接取编码值
  #[inline]
  #[must_use]
  pub const fn hash_value(self) -> u8 {
    self as u8
  }

  /// 原版 operator<< 文本
  #[inline]
  #[must_use]
  pub const fn name(self) -> &'static str {
    match self {
      Self::SeqCst => "kSeqCst",
      Self::AcqRel => "kAcqRel",
    }
  }
}

/// 原版 Callable: 代码句柄 + 调用接口描述符 的绑定。
/// Handle<Code> / CallInterfaceDescriptor 依赖 v8-heap/v8-codegen 接线, 此处为骨架。
#[derive(Debug, Clone)]
pub struct Callable {
  /// 原版 code_: Handle<Code>
  pub code: HandleCode,
  /// 原版 descriptor_: CallInterfaceDescriptor
  pub descriptor: CallInterfaceDescriptor,
}

/// 占位类型: Handle<Code> (待 Developer 接线到堆侧)
pub type HandleCode = u32;
/// 占位类型: CallInterfaceDescriptor (待 Developer 接线到 interface_descriptors)
pub type CallInterfaceDescriptor = u32;

impl Callable {
  /// 原版构造: Callable(code, descriptor)
  #[inline]
  #[must_use]
  pub const fn new(code: HandleCode, descriptor: CallInterfaceDescriptor) -> Self {
    Self { code, descriptor }
  }

  /// 原版 code()
  #[inline]
  #[must_use]
  pub const fn code(&self) -> HandleCode {
    self.code
  }

  /// 原版 descriptor()
  #[inline]
  #[must_use]
  pub const fn descriptor(&self) -> CallInterfaceDescriptor {
    self.descriptor
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(MACHINE_REPR_TO_STRING.len() == ${reprRows.length});
  assert!(MACHINE_SEMANTIC_TO_STRING.len() == ${semRows.length});
  assert!(AtomicMemoryOrder::SeqCst as u8 == 0);
  assert!(AtomicMemoryOrder::AcqRel as u8 == 1);
};
`;

const p = writeSrc("v8-codegen-core", "machine_type.rs", rust);
console.log(
  `machine-type.cc: repr 表 ${reprRows.length} / semantic 表 ${semRows.length} / subtype 对 ${SUBTYPE_PAIRS.length} / AtomicMemoryOrder ${amoEntries.length} / Callable 1 → ${p}`,
);
