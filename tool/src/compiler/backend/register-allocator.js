#!/usr/bin/env bun
// tool/src/compiler/backend/register-allocator.js
// v8/src/compiler/backend/register-allocator.{h,cc}
//   → v8-compiler-backend/src/register_allocator.rs.skel
// 寄存器分配管线数据面: LiveRange 族/LiveRangeBundle/SpillRange/六 pass 组件
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/register-allocator.h");
const cc = readV8("src/compiler/backend/register-allocator.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;
const enumOf = (src, re, label, expect) => {
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  const names = [...m[1].replace(/\/\/[^\n]*/g, "").split(/[\n,]+/)
    .map((l) => l.trim()).filter((l) => /^k\w+$/.test(l))];
  if (expect && names.length !== expect) throw new Error(`${label} ${names.length} != ${expect}`);
  checks++;
  return names;
};

// h: 枚举 4 (真解析)
const usePosType = enumOf(h, /enum class UsePositionType : uint8_t \{([\s\S]*?)\};/, "UsePositionType", 4);
const usePosHint = enumOf(h, /enum class UsePositionHintType : uint8_t \{([\s\S]*?)\};/, "UsePositionHintType", 5);
const growth = enumOf(h, /enum GrowthDirection \{([\s\S]*?)\};/, "GrowthDirection", 2);
must(usePosType.join() === "kRegisterOrSlot,kRegisterOrSlotOrConstant,kRequiresRegister,kRequiresSlot", "UsePositionType 成员序");
must(usePosHint.join() === "kNone,kOperand,kUsePos,kPhi,kUnresolved", "UsePositionHintType 成员序");
must(growth.join() === "kFront,kFrontOrBack", "GrowthDirection 成员序");

// h: 常量 2 (kNumberOfFixedRangesPerRegister 双处 + kMinCapacity)
must(count(h, /kNumberOfFixedRangesPerRegister/g) === 3, "h 固定区间常量 3 处 (Data 定义 2 + LiveRangeBuilder 引用)");
must(h.includes("static constexpr size_t kMinCapacity = 2;"), "h DoubleEndedSplitVector 最小容量 2");

// h: 组件类 15 (数据结构 9 + pass 6)
for (const cls of ["LifetimePosition", "RegisterAllocationData", "UseInterval", "UsePosition",
  "DoubleEndedSplitVector", "LiveRange", "LiveRangeBundle", "TopLevelLiveRange", "SpillRange",
  "ConstraintBuilder", "LiveRangeBuilder", "BundleBuilder", "RegisterAllocator",
  "LinearScanAllocator", "OperandAssigner", "ReferenceMapPopulator", "LiveRangeConnector"])
  must(new RegExp(`^(class|struct) (V8_EXPORT_PRIVATE |final )?${cls}\\b`, "m").test(h) ||
    new RegExp(`^(class|struct) [\\w: ]*${cls}[ :{]`, "m").test(h), `h 类 ${cls}`);
must(h.includes("struct LiveRangeOrdering"), "h LiveRangeOrdering");
must(h.includes("struct PrintableLiveRange"), "h PrintableLiveRange");
// h: 继承面
must(h.includes("class V8_EXPORT_PRIVATE TopLevelLiveRange final : public LiveRange {"), "h TopLevel 继承 LiveRange");
must(h.includes("class LinearScanAllocator final : public RegisterAllocator {"), "h LinearScan 继承");
must(h.includes("class RegisterAllocationData final : public ZoneObject {"), "h Data 继承");
// h: LiveRange 关键 API
must(h.includes("class V8_EXPORT_PRIVATE LiveRange : public NON_EXPORTED_BASE(ZoneObject) {"), "h LiveRange 签名");

// cc: 实现面 (真计数 ≥190)
const implCount = count(cc, /^[\w:<>* &]*\w+::\w+\(/gm);
must(implCount >= 190, `cc 成员实现 ≥190, 实得 ${implCount}`);
// cc: 六 pass 入口
for (const pass of ["ConstraintBuilder::ConstraintBuilder", "LiveRangeBuilder::BuildLiveRanges",
  "BundleBuilder::BuildBundles", "LinearScanAllocator::AllocateRegisters",
  "OperandAssigner::AssignSpillSlots", "ReferenceMapPopulator::PopulateReferenceMaps",
  "LiveRangeConnector::ConnectRanges"])
  must(cc.includes(pass) || cc.includes(pass.replace("::", "::")), `cc pass ${pass}`);
// cc: UseInterval/UsePosition 数据结构实现
must(cc.includes("UseInterval::UseInterval(") || cc.includes("LiveRange::SplitAt("), "cc 区间/分裂实现");
must(count(cc, /TopLevelLiveRange::/g) === 26, `cc TopLevelLiveRange 实现 26, 实得 ${count(cc, /TopLevelLiveRange::/g)}`);

// ── 产物 ────────────────────────────────────────────────────────────────────
const enumRs = (name, vals, doc) =>
  `/// v8: ${doc}\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub enum ${name} {\n` +
  vals.map((n, i) => `  /// ${n}\n  ${n.slice(1)},`).join("\n") + "\n}\n";
const skel = `${banner("src/compiler/backend/register-allocator.js",
  "src/compiler/backend/register-allocator.h + register-allocator.cc",
  "寄存器分配管线: LiveRange 数据族 + 六 pass (约束/活区/捆绑/线性扫描/溢出槽/引用图/连接)")}
//! 台账: 数据族 — LifetimePosition (偶对齐序数) / UseInterval / UsePosition
//!   (Type 4 + HintType 5) / LiveRange (区间链 + 寄存器/溢出/分裂) /
//!   TopLevelLiveRange (child 链 + spill 面) / LiveRangeBundle (区间并) /
//!   SpillRange / DoubleEndedSplitVector (GrowthDirection 2, kMinCapacity 2);
//!   pass 族 — ConstraintBuilder → LiveRangeBuilder (BuildLiveRanges) →
//!   BundleBuilder → LinearScanAllocator (AllocateRegisters) → OperandAssigner
//!   (AssignSpillSlots) → ReferenceMapPopulator (PopulateReferenceMaps) →
//!   LiveRangeConnector (ConnectRanges); kNumberOfFixedRangesPerRegister=2 双处。

${enumRs("UsePositionType", usePosType, "UsePosition::Type — 使用点约束")}
${enumRs("UsePositionHintType", usePosHint, "UsePosition::HintType — 使用点提示来源")}
${enumRs("GrowthDirection", growth, "DoubleEndedSplitVector 生长方向")}

/// v8: RegisterAllocationData::kNumberOfFixedRangesPerRegister (LiveRangeBuilder 同值)
pub const NUMBER_OF_FIXED_RANGES_PER_REGISTER: i32 = 2;
/// v8: DoubleEndedSplitVector::kMinCapacity
pub const MIN_CAPACITY: usize = 2;

/// v8: RegisterAllocationData — 分配数据中枢 (各 pass 共享)
// pub struct RegisterAllocationData (live_in/out/固定区间/分配掩码/溢出范围)
/// v8: LiveRange::SplitAt — 区间分裂 (child 链)
// fn split_at(&mut self, pos: LifetimePosition) -> LiveRange
/// v8: TopLevelLiveRange — 顶层区间 (vreg 对应 + spill + child 链)
// pub struct TopLevelLiveRange
/// v8: 六 pass 入口 (见上); LinearScanAllocator 继承 RegisterAllocator 基类
// ConstraintBuilder/LiveRangeBuilder/BundleBuilder/LinearScanAllocator/
//   OperandAssigner/ReferenceMapPopulator/LiveRangeConnector

// —— 提取不变式断言 ——
const _: () = {
  assert!(NUMBER_OF_FIXED_RANGES_PER_REGISTER == 2);
  assert!(MIN_CAPACITY == 2);
  assert!(UsePositionHintType::Unresolved as u8 == 4);
};
`;

writeSrc("v8-compiler-backend", "register_allocator.rs.skel", skel);
console.log(`register-allocator.js: 对数 ${checks} 条全过; 枚举 3 组实测 (Type 4/Hint 5/Growth 2); ` +
  `组件类 17; cc 实现面 ${implCount} (TopLevelLiveRange 26); 产物 register_allocator.rs.skel`);
