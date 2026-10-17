#!/usr/bin/env bun
// tool/src/snapshot/deserializer.js
// 机械提取 v8/src/snapshot/deserializer.{h,cc} 的 V8_CODE_EMBEDS_OBJECT_POINTER
// 宏展开、DeserializingUserCodeOption 枚举与 Deserializer 方法面。cc 1754 行只提取常量。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/deserializer.h");
const ccSrc = readV8("src/snapshot/deserializer.cc");

// 1. V8_CODE_EMBEDS_OBJECT_POINTER 宏: 平台分支机械展开。
//    目标架构 x64 + arm64 均不在此列 (MIPS64/S390X/PPC64/RISCV/RISCV64),
//    也不是 V8_EMBEDDED_CONSTANT_POOL 架构 → 0
const embedM = hSrc.match(
  /#if defined\(V8_TARGET_ARCH_MIPS64\)[\s\S]*?#define V8_CODE_EMBEDS_OBJECT_POINTER (\d)[\s\S]*?#else\s*\n#define V8_CODE_EMBEDS_OBJECT_POINTER (\d)/,
);
if (!embedM) throw new Error("未找到 V8_CODE_EMBEDS_OBJECT_POINTER 宏分支");
const embedsPtr = parseInt(embedM[2], 10);
if (embedsPtr !== 0) throw new Error(`x64/arm64 展开异常: ${embedsPtr}`);

// 2. DeserializingUserCodeOption 枚举 (h 尾部)
const duoM = hSrc.match(/enum class DeserializingUserCodeOption \{([^}]+)\};/);
if (!duoM) throw new Error("未找到 DeserializingUserCodeOption 枚举");
const duo = duoM[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (duo.length !== 2) throw new Error(`DeserializingUserCodeOption 变体数不符: 期望 2, 实得 ${duo.length}`);

// 3. Deserializer 方法面 (h)
const desM = hSrc.match(/class Deserializer : public SerializerDeserializer \{([\s\S]*?)\n private:/);
if (!desM) throw new Error("未找到 Deserializer 类体");
const desMethods = [...desM[1].matchAll(/^\s{2}(?:inline\s+)?(?:virtual\s+)?(?:[\w:<>,&*]+\s)+?&?(\w+)\(/gm)]
  .map((m) => m[1])
  .filter((n) => !["Deserializer"].includes(n));
if (desMethods.length < 20) throw new Error(`Deserializer 方法过少: ${desMethods.length}`);

// 4. cc 规模 + 预期同步哨兵登记
const ccLines = ccSrc.split("\n").length;
if (ccLines < 1700) throw new Error(`deserializer.cc 行数异常: ${ccLines}`);
const syncM = ccSrc.includes("static const uint8_t expected = kSynchronize;");
if (!syncM) throw new Error("未找到 kSynchronize 预期哨兵");

console.log(`宏展开 embeds_ptr=${embedsPtr} + 枚举 ${duo.length} + 方法 ${desMethods.length}`);

let rust = banner("src/snapshot/deserializer.js", "src/snapshot/deserializer.{h,cc}", "反序列化器常量与方法面");
rust += `
/// 代码段是否内嵌对象指针 (原版宏 V8_CODE_EMBEDS_OBJECT_POINTER 机械展开):
/// 分支架构 MIPS64/S390X/PPC64/RISCV32/RISCV64 或 V8_EMBEDDED_CONSTANT_POOL → 1;
/// 目标架构 x64 + arm64 均不满足 → 0。其余架构跳过并在此登记。
pub const CODE_EMBEDS_OBJECT_POINTER: bool = ${embedsPtr !== 0};

/// 反序列化用户代码模式 (原版 enum class DeserializingUserCodeOption)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeserializingUserCodeOption {
  /// ${duo[0]}
  NotDeserializingUserCode = 0,
  /// ${duo[1]}
  IsDeserializingUserCode = 1,
}

/// Deserializer 方法面 (原版 h, protected 区为主, 共 ${desMethods.length} 项;
/// 实现于 deserializer.cc ${ccLines} 行, 语义由 Developer 落):
`;
for (const m of desMethods) rust += `/// - \`${m}\`\n`;
rust += `
/// 反序列化主线 (机械提取调用链登记):
/// DeserializeUntrustedSection (untrusted 段含根, 先读) → DeserializeTrustedSection
/// → DeserializeDeferredObjects → PostProcessExposedTrustedObjects → LogNewObjectEvents
/// 同步哨兵: 段边界期望字节 = Bytecode::Synchronize (原版 static const uint8_t expected)。
/// 热对象环: 容量 kHotObjectCount, 与 serializer.rs 的 hot_objects_list 共用布局。
`;

const p = writeSrc("v8-snapshot", "deserializer.rs", rust);
console.log(`written ${p}`);
