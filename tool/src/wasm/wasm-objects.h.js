#!/usr/bin/env bun
// tool/src/wasm/wasm-objects.h.js
// 机械提取 v8/src/wasm/wasm-objects.{h,tq} 的对象布局常量、枚举与 tq 类清单
// 布局偏移以公式登记 (依赖 ExposedTrustedObject/kTaggedSize 等布局参数, 不折叠,
// 禁编造数值); DRUMBRAKE 条件字段按开关两态登记

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-objects.h");
const tqSrc = readV8("src/wasm/wasm-objects.tq");

function must(re, what) {
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

// 原版锚点 1-based 行号 (生成注释登记对译依据用, 锚缺失即抛不猜)
function lineNo(src, sub, what) {
  const i = src.split("\n").findIndex((l) => l.includes(sub));
  if (i < 0) throw new Error(`锚缺失: ${what || sub}`);
  return i + 1;
}
const noeHLine = lineNo(hSrc, "enum NewOrExistingEntry : bool");
const exposedTqLine = lineNo(tqSrc, "extern class WasmTrustedInstanceData extends ExposedTrustedObject;");
const lenOffHLine = lineNo(hSrc, "static constexpr size_t kLengthOffset = sizeof(ExposedTrustedObject);");

// 1. OnResume
const onResumeMatch = must(/enum class OnResume : int \{ kContinue, kThrow \};/, "OnResume");

// 2. InternalizeString
must(/enum InternalizeString : bool \{ kInternalize = true, kNoInternalize = false \};/, "InternalizeString");

// 3. WasmMemoryObject::kNoMaximum
const noMaximumMatch = must(/static constexpr int kNoMaximum = (-?\d+);/, "kNoMaximum");
if (parseInt(noMaximumMatch[1], 10) !== -1) throw new Error("kNoMaximum 不符");

// 4. FeedbackConstants
const fbHeader = must(/static constexpr int kHeaderSlots = (\d+);/, "FeedbackConstants::kHeaderSlots")[1];
const fbSlots = must(/static constexpr int kSlotsPerInstruction = (\d+);/, "FeedbackConstants::kSlotsPerInstruction")[1];
if (fbHeader !== "1" || fbSlots !== "2") throw new Error("FeedbackConstants 不符");

// 5. NewOrExistingEntry
must(/enum NewOrExistingEntry : bool \{ kNewEntry, kExistingEntry \};/, "NewOrExistingEntry");

// 6. WasmDispatchTable 布局公式 (偏移依赖布局参数, 以公式登记)
const dt = must(/static constexpr size_t kLengthOffset = sizeof\(ExposedTrustedObject\);/, "kLengthOffset");
void dt;
for (const anchor of [
  "static constexpr size_t kCapacityOffset = kLengthOffset + kUInt32Size;",
  "static constexpr size_t kEntriesOffset =",
  "static constexpr size_t kHeaderSize = kEntriesOffset;",
  "static constexpr size_t kEntrySize = kImplicitArgBias + kTaggedSize;",
  "static constexpr int kMaxLength = (kMaxInt - kEntriesOffset) / kEntrySize;",
]) {
  if (!hSrc.includes(anchor)) throw new Error(`WasmDispatchTable 锚缺失: ${anchor}`);
}

// 7. 实例字段名单 (X-macro; IF_WASM_DRUMBRAKE 条目按开关注释两态登记, 且保持宏内
// 原序 —— 名单顺序敏感, DRUMBRAKE 注释项须钉在原位 (复审 WOR1 F4/WOR2 F7 的产物侧
// 修订, R5W2 收拢入模板): 逐行扫描记录各条目的原版行号)
function fieldList(name, noteSuffix) {
  const all = hSrc.split("\n");
  const s = all.findIndex((l) => l.startsWith(`#define ${name}`));
  if (s < 0) throw new Error(`未找到 ${name}`);
  const entries = [];
  for (let i = s + 1; i < all.length; i++) {
    const line = all[i];
    const cond = line.match(/^\s*IF_WASM_DRUMBRAKE\(V,\s*(\w+),?\s*\\?\s*$/);
    if (cond) {
      let j = i + 1;
      for (; j < all.length && !/"\w+"\s*\)/.test(all[j]); j++);
      const nm = (all[j] || "").match(/"(\w+)"\)/);
      if (!nm) throw new Error(`${name} DRUMBRAKE 条目缺名`);
      entries.push({ offset: cond[1], name: nm[1], drumbake: true, ifLine: i + 1, nameLine: j + 1 });
      i = j;
      continue;
    }
    const plain = line.match(/^\s*V\((\w+),\s*"(\w+)"\)/);
    if (plain) {
      entries.push({ offset: plain[1], name: plain[2], drumbake: false, line: i + 1 });
      if (!/\\\s*$/.test(line)) break;
    }
  }
  const plain = entries.filter((e) => !e.drumbake);
  const drumbake = entries.filter((e) => e.drumbake);
  for (const e of drumbake) {
    const next = entries[entries.indexOf(e) + 1];
    if (!next) throw new Error(`${name} DRUMBRAKE 位后无普通条目`);
    e.note = `  // v8: wasm-objects.h:${e.ifLine}-${e.nameLine} IF_WASM_DRUMBRAKE 位在 ${next.name}(:${next.line}) 之前, 顺序敏感数组须保持此序${noteSuffix}`;
  }
  return { entries, plain, drumbake };
}
const tagged = fieldList("WASM_TAGGED_INSTANCE_DATA_FIELDS", "");
const prot = fieldList("WASM_PROTECTED_INSTANCE_DATA_FIELDS", " (复审 WOR2 F7, 同 TAGGED 侧规则)");
if (tagged.plain.length !== 13 || tagged.drumbake.length !== 1) {
  throw new Error(`tagged 字段数不符: ${tagged.plain.length}+${tagged.drumbake.length}`);
}
if (prot.plain.length !== 8 || prot.drumbake.length !== 1) {
  throw new Error(`protected 字段数不符: ${prot.plain.length}+${prot.drumbake.length}`);
}

// 8. tq 类清单
const tqClasses = [...tqSrc.matchAll(/^extern class (\w+)/gm)].map((m) => m[1]);
if (tqClasses.length < 25) throw new Error(`tq 类数异常: ${tqClasses.length}`);

console.log(`wasm-objects: tagged=${tagged.plain.length}+${tagged.drumbake.length}D protected=${prot.plain.length}+${prot.drumbake.length}D tq=${tqClasses.length}`);

// —— 生成 Rust ——
let rust = banner(
  "src/wasm/wasm-objects.h.js",
  "src/wasm/wasm-objects.{h,tq}",
  "Wasm 对象布局常量、枚举与 torque 类清单",
);

rust += `
/// 挂起恢复行为 (对应原版 enum class OnResume : int)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum OnResume {
  /// kContinue
  Continue = 0,
  /// kThrow
  Throw = 1,
}

/// 字符串内部化标记 (对应原版 InternalizeString)
pub type InternalizeString = bool;
/// kInternalize
pub const INTERNALIZE: InternalizeString = true;
/// kNoInternalize
pub const NO_INTERNALIZE: InternalizeString = false;

/// 内存对象无最大页数哨兵 (原版 WasmMemoryObject::kNoMaximum)
pub const MEMORY_NO_MAXIMUM: i32 = -1;

/// 反馈常量布局 (对应原版 FeedbackConstants)
pub mod feedback_constants {
  pub const HEADER_SLOTS: i32 = ${fbHeader};
  pub const SLOTS_PER_INSTRUCTION: i32 = ${fbSlots};
}

/// 调度表条目写入情境 (对应原版 WasmDispatchTable::NewOrExistingEntry)
/// (整改 R5W2 F14a) 对译依据: 原版 v8/src/wasm/wasm-objects.h:${noeHLine}
/// \`enum NewOrExistingEntry : bool { kNewEntry, kExistingEntry };\` —— C++ bool 底
/// 型枚举器自 0 起隐式递增, 实际取值 kNewEntry=0、kExistingEntry=1; Rust 禁
/// repr(bool) (E0539), 而 bool 布局保证恰为 1 字节取 0/1, 故 repr(u8) + 0/1
/// 判别式与原版逐值同形。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NewOrExistingEntry {
  /// kNewEntry
  NewEntry = 0,
  /// kExistingEntry
  ExistingEntry = 1,
}

/// WasmDispatchTable 布局 (公式登记, 依赖布局参数, 不折叠 — Developer 侧按
/// 构建配置代入: EXPOSED_TRUSTED_OBJECT_SIZE / K_UINT32_SIZE / K_TAGGED_SIZE /
/// TAGGED_SIZE_8_BYTES)
pub mod dispatch_table_layout {
  pub const K_UINT32_SIZE: usize = 4;
  pub const K_TAGGED_SIZE: usize = 4; // 指针压缩开; 关闭时为 8
  pub const TAGGED_SIZE_8_BYTES: bool = false; // 压缩开 → false
  /// 布局参数: sizeof(ExposedTrustedObject)。
  /// A-1 深水哨兵 (整改 R5W2 F14c): 原 todo!() 在 const 上下文被禁 (E0080),
  /// 改登记为可在 const 上下文折叠的显式占位 0。
  /// 硬约束: 本模块接线进 lib.rs 前, 必须先把占位 0 替换为
  /// sizeof(ExposedTrustedObject) 真值, 未替值不得接线 —— 真值不可从原版
  /// wasm-objects.{h,tq} 径证: tq:${exposedTqLine} \`extern class
  /// WasmTrustedInstanceData extends ExposedTrustedObject;\` 无类体,
  /// h:${lenOffHLine} 仅登记公式形; 取值依赖堆对象布局件 (v8-heap-base/
  /// v8-objects-core) 与指针压缩/SANDBOX 构建开关。见 v8-wasm-objects
  /// readme A-1 台账 (双语已登「必先替值后接线」)。
  pub const EXPOSED_TRUSTED_OBJECT_SIZE: usize = 0; // A-1 占位: 接线前必替换真值

  /// kLengthOffset = sizeof(ExposedTrustedObject)
  pub const LENGTH_OFFSET: usize = EXPOSED_TRUSTED_OBJECT_SIZE;
  pub const CAPACITY_OFFSET: usize = LENGTH_OFFSET + K_UINT32_SIZE;
  pub const PROTECTED_OFFHEAP_DATA_OFFSET: usize = CAPACITY_OFFSET + K_UINT32_SIZE;
  pub const PROTECTED_USES_OFFSET: usize = PROTECTED_OFFHEAP_DATA_OFFSET + K_TAGGED_SIZE;
  pub const TABLE_TYPE_OFFSET: usize = PROTECTED_USES_OFFSET + K_TAGGED_SIZE;
  pub const PADDING_SIZE: usize = if TAGGED_SIZE_8_BYTES { K_UINT32_SIZE } else { 0 };
  pub const ENTRIES_OFFSET: usize = TABLE_TYPE_OFFSET + K_UINT32_SIZE + PADDING_SIZE;
  pub const HEADER_SIZE: usize = ENTRIES_OFFSET;
  /// 条目: [target(u32) | sig(u32) | implicit_arg(tagged)]; DRUMBRAKE 开时 target 位放 function_index
  pub const TARGET_BIAS: usize = 0;
  pub const SIG_BIAS: usize = TARGET_BIAS + K_UINT32_SIZE;
  pub const IMPLICIT_ARG_BIAS: usize = SIG_BIAS + K_UINT32_SIZE;
  pub const ENTRY_SIZE: usize = IMPLICIT_ARG_BIAS + K_TAGGED_SIZE;
}

/// WasmTrustedInstanceData tagged 字段名单 (原版 WASM_TAGGED_INSTANCE_DATA_FIELDS)
/// 偏移由 torque 生成, 此处只登记 1:1 名单
pub const TAGGED_INSTANCE_FIELDS: &[(&str, &str)] = &[
`;
for (const f of tagged.entries) {
  rust += f.drumbake
    ? `  // 仅 V8_ENABLE_DRUMBRAKE: ("${f.offset}", "${f.name}"),\n${f.note}\n`
    : `  ("${f.offset}", "${f.name}"),\n`;
}
rust += `];

/// WasmTrustedInstanceData protected 字段名单 (原版 WASM_PROTECTED_INSTANCE_DATA_FIELDS)
pub const PROTECTED_INSTANCE_FIELDS: &[(&str, &str)] = &[
`;
for (const f of prot.entries) {
  rust += f.drumbake
    ? `  // 仅 V8_ENABLE_DRUMBRAKE: ("${f.offset}", "${f.name}"),\n${f.note}\n`
    : `  ("${f.offset}", "${f.name}"),\n`;
}
rust += `];

/// 字段计数 (原版 kTaggedFieldsCount / kProtectedFieldsCount, PLUS_ONE 展开)
/// DRUMBRAKE 关闭态
pub const TAGGED_FIELDS_COUNT: usize = ${tagged.plain.length};
pub const TAGGED_FIELDS_COUNT_DRUMBRAKE: usize = ${tagged.plain.length + tagged.drumbake.length};
pub const PROTECTED_FIELDS_COUNT: usize = ${prot.plain.length};
pub const PROTECTED_FIELDS_COUNT_DRUMBRAKE: usize = ${prot.plain.length + prot.drumbake.length};

/// torque 类清单 (原版 wasm-objects.tq, extern class 1:1)
pub const TQ_CLASSES: &[&str] = &[
`;
for (const c of tqClasses) {
  rust += `  "${c}",\n`;
}
rust += `];

// —— 提取不变式断言 ——
// (整改 R5W2 F14b) 原形 TQ_CLASSES.contains(...) 处于 const 上下文违 E0015
// (slice::contains 非 const fn)。选型: 保持编译期折叠 —— 块内 const fn 以
// while 循环逐元素逐字节比较替代 contains (不降级为运行时 #[cfg(test)] 断言,
// 以免断言面削弱为仅测试期生效); 口径与 readme 工具链登记 (本版 nightly const
// 上下文受限, 不变式统一 assert! 折叠) 同源, 该类补登于彼。
const _: () = {
  const fn tq_contains(classes: &[&str], name: &str) -> bool {
    let mut i = 0;
    while i < classes.len() {
      let a = classes[i].as_bytes();
      let b = name.as_bytes();
      if a.len() == b.len() {
        let mut j = 0;
        let mut same = true;
        while j < a.len() {
          if a[j] != b[j] {
            same = false;
          }
          j += 1;
        }
        if same {
          return true;
        }
      }
      i += 1;
    }
    false
  }
  assert!(MEMORY_NO_MAXIMUM == -1);
  assert!(feedback_constants::HEADER_SLOTS == 1);
  assert!(feedback_constants::SLOTS_PER_INSTRUCTION == 2);
  // 修订 (R5W): 原骨架误写 TAGGED_FIELDS_COUNT == 14; 原版 PLUS_ONE 展开
  // 非 DRUMBRAKE 态 ${tagged.plain.length} 项 (wasm-objects.h:721-736 WASM_TAGGED_INSTANCE_DATA_FIELDS)
  assert!(TAGGED_FIELDS_COUNT == ${tagged.plain.length});
  assert!(TAGGED_FIELDS_COUNT_DRUMBRAKE == ${tagged.plain.length + tagged.drumbake.length});
  assert!(PROTECTED_FIELDS_COUNT == ${prot.plain.length});
  assert!(tq_contains(TQ_CLASSES, "WasmTrustedInstanceData"));
  assert!(tq_contains(TQ_CLASSES, "WasmDispatchTable"));
};
`;

const p = writeSrc("v8-wasm-objects", "wasm_objects_consts.rs", rust);
console.log(`written ${p}`);
