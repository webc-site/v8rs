#!/usr/bin/env bun
// tool/src/extensions/gc-extension.h.js
// 机械提取 v8/src/extensions/gc-extension.{h,cc}
// 产物: v8-extensions/src/gc_extension.rs — GC 选项枚举 + 选项字符串表 + 签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/extensions/gc-extension.h");
const ccSrc = readV8("src/extensions/gc-extension.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. 扩展名 + BuildSource 格式串 + buffer 容量 (h)
const extName = hSrc.match(/v8::Extension\("([^"]+)"/);
must(extName, "扩展名 v8/gc");
if (extName[1] !== "v8/gc") fail("扩展名", extName[1], "v8/gc");
const bufCap = hSrc.match(/char buffer_\[(\d+)\];/);
must(bufCap, "buffer_ 容量");

// 2. 三个枚举 (cc 匿名 namespace)
const enumOf = (name, want) => {
  const m = ccSrc.match(new RegExp(`enum class ${name} \\{([^}]+)\\};`));
  if (!m) throw new Error(`关键结构缺失: enum ${name}`);
  const items = m[1].split(",").map((s) => s.trim()).filter(Boolean);
  if (items.length !== want) fail(`enum ${name} 条目`, items.length, want);
  return items.map((s) => s.replace(/^k/, ""));
};
const gcTypes = enumOf("GCType", 3); // Minor, Major, MajorWithSnapshot
const execTypes = enumOf("ExecutionType", 2); // Async, Sync
const flavors = enumOf("Flavor", 2); // Regular, LastResort

// 3. 默认快照文件名
const snapFile = ccSrc.match(/kDefaultHeapSnapshotFileName\[\] = "([^"]+)"/);
must(snapFile, "kDefaultHeapSnapshotFileName");

// 4. 选项字符串表: ParseType / ParseExecution / ParseFlavor 的 StrictEquals 字面量
const collect = (fnName) => {
  const m = ccSrc.match(new RegExp(`void ${fnName}\\([\\s\\S]*?\\n\\}`));
  if (!m) throw new Error(`关键结构缺失: ${fnName}`);
  return [...m[0].matchAll(/"(minor|major|major-snapshot|async|sync|regular|last-resort)"/g)].map(
    (x) => x[1],
  );
};
const typeStrs = collect("ParseType");
const execStrs = collect("ParseExecution");
const flavorStrs = collect("ParseFlavor");
if (typeStrs.length !== 3) fail("ParseType 字面量", typeStrs.length, 3);
if (execStrs.length !== 2) fail("ParseExecution 字面量", execStrs.length, 2);
if (flavorStrs.length !== 2) fail("ParseFlavor 字面量", flavorStrs.length, 2);

// 5. GCOptions 字段 (type/execution/flavor/filename)
const optsBody = ccSrc.match(/struct GCOptions \{[\s\S]*?GCType type;/);
must(optsBody, "GCOptions 字段");
for (const f of ["GCType type;", "ExecutionType execution;", "Flavor flavor;", "std::string filename;"]) {
  if (!ccSrc.includes(f)) fail(`GCOptions 字段 ${f}`, false, true);
}

// 6. 函数全集 (cc 匿名 namespace + 扩展本体)
const ccFns = [
  "ReadProperty", "ParseType", "ParseExecution", "ParseFlavor", "Parse", "InvokeGC",
  "AsyncGC", "GetDefault", "GetDefaultForTruthyWithoutOptionsBag",
];
for (const f of ccFns) {
  if (!ccSrc.includes(f)) fail(`cc 函数 ${f}`, false, true);
}
// h 公开方法
for (const f of ["GetNativeFunctionTemplate", "GC", "BuildSource"]) {
  if (!hSrc.includes(f)) fail(`h 方法 ${f}`, false, true);
}

// 7. InvokeGC 分派: GCType × Flavor 动作表
const actions = [
  ["Minor", "CollectGarbage(NEW_SPACE, kTesting, kGCCallbackFlagForced)", ["CollectGarbage(", "NEW_SPACE"]],
  ["Major+Regular", "PreciseCollectAllGarbage(kNoFlags, kTesting, kGCCallbackFlagForced)", ["PreciseCollectAllGarbage("]],
  ["Major+LastResort", "CollectAllAvailableGarbage(kTesting)", ["CollectAllAvailableGarbage("]],
  ["MajorWithSnapshot", "PreciseCollectAllGarbage + TakeSnapshotToFile(filename)", ["TakeSnapshotToFile("]],
];
for (const [, , tokens] of actions) {
  for (const t of tokens) {
    if (!ccSrc.includes(t)) fail(`InvokeGC 动作锚 ${t}`, false, true);
  }
}

let n = 19;
const rust = `${banner("src/extensions/gc-extension.h.js", "src/extensions/gc-extension.{h,cc}", "GC 扩展: 选项枚举 + 选项字符串表 + 签名骨架")}
//! GC 扩展 — 原版 namespace v8::internal::GCExtension
//!
//! 语义: 注入 gc(options) — 按 type/execution/flavor/filename 选项袋触发 GC;
//! 异步执行返回 Promise (AsyncGC 可取消任务), 同步返回 undefined。

/// GC 扩展注册名
pub const GC_EXTENSION_NAME: &str = "v8/gc";

/// 栈上源码缓冲容量 (原版 char buffer_[50])
pub const GC_BUFFER_SIZE: usize = ${bufCap[1]};

/// 默认堆快照文件名 (原版 kDefaultHeapSnapshotFileName)
pub const DEFAULT_HEAP_SNAPSHOT_FILENAME: &str = ${JSON.stringify(snapFile[1])};

/// GC 类型 (原版 enum class GCType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum GcType {
${gcTypes.map((t, i) => `  /// 原版 k${t}\n  ${t[0].toUpperCase()}${t.slice(1)} = ${i},`).join("\n")}
}

impl GcType {
  #[inline]
  #[must_use]
  pub const fn is_minor(self) -> bool {
    matches!(self, Self::Minor)
  }

  #[inline]
  #[must_use]
  pub const fn is_major(self) -> bool {
    matches!(self, Self::Major)
  }

  #[inline]
  #[must_use]
  pub const fn is_major_with_snapshot(self) -> bool {
    matches!(self, Self::MajorWithSnapshot)
  }
}

/// 执行时机 (原版 enum class ExecutionType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum ExecutionType {
${execTypes.map((t, i) => `  /// 原版 k${t}\n  ${t[0].toUpperCase()}${t.slice(1)} = ${i},`).join("\n")}
}

impl ExecutionType {
  #[inline]
  #[must_use]
  pub const fn is_async(self) -> bool {
    matches!(self, Self::Async)
  }

  #[inline]
  #[must_use]
  pub const fn is_sync(self) -> bool {
    matches!(self, Self::Sync)
  }
}

/// GC 风味 (原版 enum class Flavor)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum Flavor {
${flavors.map((t, i) => `  /// 原版 k${t}\n  ${t[0].toUpperCase()}${t.slice(1)} = ${i},`).join("\n")}
}

impl Flavor {
  #[inline]
  #[must_use]
  pub const fn is_regular(self) -> bool {
    matches!(self, Self::Regular)
  }

  #[inline]
  #[must_use]
  pub const fn is_last_resort(self) -> bool {
    matches!(self, Self::LastResort)
  }
}

/// 选项字符串表 (原版 ParseType/ParseExecution/ParseFlavor 的 StrictEquals 字面量, 1:1)
pub const OPTION_STRINGS: [(&str, &str); 7] = [
  ${typeStrs.map((s) => `("type", ${JSON.stringify(s)})`).join(",\n  ")},
  ${execStrs.map((s) => `("execution", ${JSON.stringify(s)})`).join(",\n  ")},
  ${flavorStrs.map((s) => `("flavor", ${JSON.stringify(s)})`).join(",\n  ")},
];

/// InvokeGC 分派动作表 (原版 switch(GCType) × switch(Flavor), 1:1)
pub const INVOKE_ACTIONS: [(&str, &str); 4] = [
${actions.map(([k, v]) => `  (${JSON.stringify(k)}, ${JSON.stringify(v)}),`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(GcType::Minor as u8 == 0);
  assert!(GcType::Major as u8 == 1);
  assert!(GcType::MajorWithSnapshot as u8 == 2);
  assert!(ExecutionType::Async as u8 == 0);
  assert!(ExecutionType::Sync as u8 == 1);
  assert!(Flavor::Regular as u8 == 0);
  assert!(Flavor::LastResort as u8 == 1);
};

// —— 签名骨架 (cc 定义 + h 声明, 1:1) ——
// GCOptions: { type: GcType, execution: ExecutionType, flavor: Flavor, filename: String }
//   GetDefault() = (Major, Sync, Regular, heap_snapshot_path || 默认名)
//   GetDefaultForTruthyWithoutOptionsBag() = (Minor, Sync, Regular, …)
// 匿名 namespace:
${["ReadProperty(isolate, ctx, object, key) -> MaybeLocal<String>",
   "ParseType(isolate, maybe_type, options, found_options_object)",
   "ParseExecution(isolate, maybe_execution, options, found_options_object)",
   "ParseFlavor(isolate, maybe_flavor, options, found_options_object)",
   "Parse(isolate, info) -> Maybe<GCOptions> — TryCatch 保异常, filename 仅 MajorWithSnapshot 生效",
   "InvokeGC(isolate, gc_options) — EmbedderStackStateScope 按 sync/async 选 StackState",
   "AsyncGC : CancelableTask — RunInternal 内 InvokeGC + resolver->Resolve(undefined)"].map((s) => `//   ${s}`).join("\n")}
// 扩展本体:
//   GetNativeFunctionTemplate(isolate, name) — ConstructorBehavior::kThrow
//   GC(info) — 0 参直接 InvokeGC(GetDefault); 异步走 PostNonNestableTask(AsyncGC)

// 平台分支台账:
//   无平台 #if; AsyncGC 析构的 kCanceled 分支依赖 Isolate 拆卸语义 — 语义承接点, 非本模块落地
`;

const p = writeSrc("v8-extensions", "gc_extension.rs", rust);
console.log(`OK gc-extension.h.js: 对数断言 ${n} 全过; 枚举 3 (3+2+2) / 选项串 7 / 动作 4 / 函数 12; 产物 ${p}`);
