#!/usr/bin/env bun
// tool/src/compiler/feedback-source.js
// 从 v8/src/compiler/feedback-source.h + feedback-source.cc
// 提取 FeedbackSource / EmbeddedFeedbackSource 值类型 (vector+slot / bytecode+offset) 与 Hash/Equal
// 生成 v8-compiler-frontend/src/feedback_source.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/feedback-source.h");
const cc = readV8("src/compiler/feedback-source.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["struct FeedbackSource {", "FeedbackSource 结构"],
  ["bool IsValid() const { return !vector.is_null() && !slot.IsInvalid(); }", "IsValid 判据"],
  ["FeedbackSource() { DCHECK(!IsValid()); }", "默认构造即无效"],
  ["V8_EXPORT_PRIVATE int index() const;", "index() 声明"],
  ["return base::hash_combine(source.vector.address(), source.slot);", "Hash 组合"],
  ["return lhs.vector.equals(rhs.vector) && lhs.slot == rhs.slot;", "Equal 判据"],
  ["struct EmbeddedFeedbackSource {", "EmbeddedFeedbackSource 结构"],
  ["bool IsValid() const { return !bytecode_array_.is_null(); }", "Embedded IsValid 判据"],
  ["return base::hash_combine(source.bytecode_array_.address(),", "Embedded Hash 组合"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`feedback-source.h 锚点缺失: ${name}`);
}
const ccAnchors = [
  ["FeedbackSource::FeedbackSource(IndirectHandle<FeedbackVector> vector_,", "handle 构造"],
  ["FeedbackSource::FeedbackSource(FeedbackVectorRef vector_, FeedbackSlot slot_)", "Ref 构造"],
  ["return FeedbackVector::GetIndex(slot);", "index = GetIndex(slot)"],
  ['return os << "FeedbackSource(" << p.slot << ")";', "Display: slot"],
  ['return os << "FeedbackSource(INVALID)";', "Display: INVALID"],
  ["EmbeddedFeedbackSource::EmbeddedFeedbackSource(", "Embedded 构造"],
  ['return os << "EmbeddedFeedbackSource(" << p.offset_ << ")";', "Embedded Display: offset"],
];
for (const [lit, name] of ccAnchors) {
  if (!cc.includes(lit)) throw new Error(`feedback-source.cc 锚点缺失: ${name}`);
}

// —— 2. 真解析: 两结构成员清单 ——
const membersOf = (src, head) => {
  const i = src.indexOf(head);
  if (i < 0) throw new Error(`定位失败: ${head}`);
  const seg = src.slice(i, src.indexOf("};", i));
  return [...seg.matchAll(/^\s+(?:V8_EXPORT_PRIVATE\s+)?(?:IndirectHandle<(\w+)>|(\w+))\s+(\w+);/gm)]
    .map((m) => ({ ty: m[1] ?? m[2], name: m[3] }));
};
const fsMembers = membersOf(h, "struct FeedbackSource");
const embMembers = membersOf(h, "struct EmbeddedFeedbackSource");
if (fsMembers.length !== 2 || embMembers.length !== 2) {
  throw new Error(`成员提取异常: ${fsMembers.length}/${embMembers.length}`);
}

// —— 3. .cc 定义面计数 ——
const defs = (cc.match(/^[A-Za-z].*::|^bool operator|^std::ostream|^inline size_t/gm) || []).length;

const body = `${banner("src/compiler/feedback-source.js", "src/compiler/feedback-source.h + feedback-source.cc", "反馈槽值类型: FeedbackVector+FeedbackSlot / BytecodeArray+offset")}

/// 反馈来源 — (vector, slot) 二元组
/// v8: src/compiler/feedback-source.h FeedbackSource
#[derive(Clone, Copy)]
pub struct FeedbackSource {
    pub vector: IndirectHandle<FeedbackVector>,
    pub slot: FeedbackSlot,
}

impl FeedbackSource {
    /// 默认构造为无效 (DCHECK(!IsValid()))
    /// v8: src/compiler/feedback-source.h FeedbackSource::FeedbackSource()
    pub fn invalid() -> Self;
    /// v8: src/compiler/feedback-source.h FeedbackSource::IsValid
    pub fn is_valid(&self) -> bool { !self.vector.is_null() && !self.slot.is_invalid() }
    /// v8: src/compiler/feedback-source.cc FeedbackSource::index
    /// CHECK(IsValid()); 返回 FeedbackVector::GetIndex(slot)
    pub fn index(&self) -> i32;
}

impl PartialEq for FeedbackSource {
    /// v8: src/compiler/feedback-source.h FeedbackSource::Equal
    fn eq(&self, other: &Self) -> bool { self.vector == other.vector && self.slot == other.slot }
}
impl Eq for FeedbackSource {}

impl std::hash::Hash for FeedbackSource {
    /// v8: src/compiler/feedback-source.h FeedbackSource::Hash
    /// base::hash_combine(vector.address(), slot)
    fn hash<H: std::hash::Hasher>(&self, state: &mut H);
}

/// 嵌入式提示来源 — (bytecode_array, offset) 二元组
/// v8: src/compiler/feedback-source.h EmbeddedFeedbackSource
#[derive(Clone, Copy)]
pub struct EmbeddedFeedbackSource {
    pub bytecode_array: IndirectHandle<BytecodeArray>,
    pub offset: i32,
}

impl EmbeddedFeedbackSource {
    /// v8: src/compiler/feedback-source.h EmbeddedFeedbackSource::IsValid
    pub fn is_valid(&self) -> bool { !self.bytecode_array.is_null() }
}

impl PartialEq for EmbeddedFeedbackSource {
    /// v8: src/compiler/feedback-source.h EmbeddedFeedbackSource::Equal
    fn eq(&self, other: &Self) -> bool {
        self.bytecode_array == other.bytecode_array && self.offset == other.offset
    }
}
impl Eq for EmbeddedFeedbackSource {}

impl std::hash::Hash for EmbeddedFeedbackSource {
    /// v8: src/compiler/feedback-source.h EmbeddedFeedbackSource::Hash
    /// base::hash_combine(bytecode_array.address(), offset)
    fn hash<H: std::hash::Hasher>(&self, state: &mut H);
}

// Display: "FeedbackSource(slot)" / "FeedbackSource(INVALID)"
//          "EmbeddedFeedbackSource(offset)" / "(INVALID)"
// —— 对数基准 ——
// FeedbackSource 成员 ${fsMembers.map((m) => m.name).join("+")} / Embedded ${embMembers.map((m) => m.name).join("+")}
// cc 定义面 ${defs} 处
`;

const p = writeSrc("v8-compiler-frontend", "feedback_source.rs.skel", body);
console.log(`✓ feedback-source: 成员 ${fsMembers.length}+${embMembers.length} / cc 定义 ${defs} / 锚点 ${anchors.length + ccAnchors.length}`);
