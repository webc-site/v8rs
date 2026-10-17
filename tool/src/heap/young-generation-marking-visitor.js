#!/usr/bin/env bun
// tool/src/heap/young-generation-marking-visitor.js
// 机械提取 v8/src/heap/young-generation-marking-visitor{.h,-inl.h} 族:
// YoungGenerationMarkingVisitor 三枚举 + 模板静态开关

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/young-generation-marking-visitor.h");
const inlSrc = readV8("src/heap/young-generation-marking-visitor-inl.h");

const anchors = [
  [hSrc, "enum class YoungGenerationMarkingVisitationMode { kParallel, kConcurrent };"],
  [hSrc, "class YoungGenerationMarkingVisitor final"],
  [hSrc, "enum class ObjectVisitationMode {"],
  [hSrc, "enum class SlotTreatmentMode {"],
  [hSrc, "static constexpr bool EnableConcurrentVisitation() {"],
  [hSrc, "YoungPendingAllocations::Snapshot* young_pending_allocations_snapshot ="],
  [inlSrc, "VisitPointersImpl"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// 枚举真提取
function enumBody(src, name) {
  const m = src.match(new RegExp(`enum class ${name} \\{([^}]*)\\};`, "s"));
  if (!m) throw new Error(`未找到枚举 ${name}`);
  return m[1].replace(/\/\/[^\n]*/g, "").split(",").map((s) => s.trim()).filter(Boolean);
}
const visitation = enumBody(hSrc, "YoungGenerationMarkingVisitationMode");
const objectMode = enumBody(hSrc, "ObjectVisitationMode");
const slotMode = enumBody(hSrc, "SlotTreatmentMode");
if (visitation.join() !== "kParallel,kConcurrent") throw new Error("VisitationMode 漂移");
if (objectMode.join() !== "kVisitDirectly,kPushToWorklist") throw new Error("ObjectVisitationMode 漂移");
if (slotMode.join() !== "kReadOnly,kReadWrite") throw new Error("SlotTreatmentMode 漂移");
console.log(`young-generation-marking-visitor: 枚举 3 (${visitation.length}/${objectMode.length}/${slotMode.length})`);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
function emitEnum(rustName, doc, entries) {
  let out = `/// ${doc}\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub enum ${rustName} {\n`;
  entries.forEach((k, i) => (out += `  /// ${k}\n  ${snake(k)} = ${i},\n`));
  return out + "}\n\n";
}

let rust = banner(
  "src/heap/young-generation-marking-visitor.js",
  "src/heap/young-generation-marking-visitor.h, src/heap/young-generation-marking-visitor-inl.h",
  "新生代标记 visitor (并发/并行两实例化)",
);

rust += emitEnum("YoungGenerationMarkingVisitationMode", "访问模式 (h 顶层枚举)", visitation);
rust += emitEnum("ObjectVisitationMode", "对象访问: 直访或推工作表", objectMode);
rust += emitEnum("SlotTreatmentMode", "槽位处理: 只读/读写 (并发下锁字符串表)", slotMode);

rust += `/// v8: src/heap/young-generation-marking-visitor.h YoungGenerationMarkingVisitor<marking_mode>
/// 构造: (heap, local_pretenuring_feedback, young_pending_allocations_snapshot = nullptr)
/// 静态开关: EnableConcurrentVisitation() = marking_mode == kConcurrent
/// VisitPointers(ObjectSlot|MaybeObjectSlot) → VisitPointersImpl (inl)
/// v8: src/heap/young-generation-marking-visitor-inl.h VisitPointersImpl /
///     VisitJSShamoObject 等实现面

// —— 提取不变式断言 ——
const _: () = {
  assert!(YoungGenerationMarkingVisitationMode::Concurrent == 1);
  assert!(ObjectVisitationMode::PushToWorklist == 1);
  assert!(SlotTreatmentMode::ReadWrite == 1);
};
`;

const p = writeSrc("v8-heap-core", "young_generation_marking_visitor.rs.skel", rust);
console.log(`written ${p}`);
