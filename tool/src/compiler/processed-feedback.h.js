#!/usr/bin/env bun
// tool/src/compiler/processed-feedback.h.js
// v8/src/compiler/processed-feedback.h
// → v8-compiler-passes/src/processed_feedback.rs.skel
// 反馈容器: 解析 ProcessedFeedback::Kind 17 值枚举 / As* 访问器 / 子类清单。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/processed-feedback.h");

const need = (cond, msg) => {
  if (!cond) throw new Error(`processed-feedback: ${msg}`);
};

for (const [s, name] of [
  ["class ProcessedFeedback : public ZoneObject {", "基类"],
  ["enum Kind {", "Kind 枚举"],
  ["bool IsInsufficient() const { return kind() == kInsufficient; }", "IsInsufficient"],
  ["class InsufficientFeedback final : public ProcessedFeedback {", "InsufficientFeedback"],
  ["class GlobalAccessFeedback : public ProcessedFeedback {", "GlobalAccessFeedback"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— Kind 枚举成员 (顺序 = 判别值) ——
const kindBody = h.slice(h.indexOf("enum Kind {"), h.indexOf("};", h.indexOf("enum Kind {")));
const kinds = [...kindBody.matchAll(/k(\w+),/g)].map((m) => m[1]);
need(kinds.length === 17, `Kind 成员 ${kinds.length}`);
need(kinds[0] === "Insufficient", "kInsufficient 不在首位");

// —— As* 访问器 (真解析) ——
const asMethods = [...h.matchAll(/^\s{2}(\w+) const& As(\w+)\(\) const;/gm)].map((m) => m[2]);
need(asMethods.length === kinds.length - 1, `As* 数 ${asMethods.length} vs Kind-1 ${kinds.length - 1}`);
for (const k of kinds.slice(1)) {
  need(asMethods.includes(k), `As${k} 缺席`);
}

// —— 本文件定义的子类 (其余为前向声明, 实现在外部头) ——
const subclasses = [...h.matchAll(/^class (\w+)(?: final)? : public ProcessedFeedback \{/gm)].map((m) => m[1]);
need(subclasses.length === 9, `本文件子类 ${subclasses.length}`);
for (const s of ["InsufficientFeedback", "GlobalAccessFeedback", "ElementAccessFeedback", "NamedAccessFeedback", "HomomorphicPropertyAccessFeedback", "MegaDOMPropertyAccessFeedback", "CallFeedback", "ProxyFeedback", "SingleValueFeedback"]) {
  need(subclasses.includes(s), `子类 ${s} 缺席`);
}
// 文件头部前向声明 (含后来在本文件定义者)
const forwarded = [...h.matchAll(/^class (\w+Feedback);/gm)].map((m) => m[1]);
const externalDefs = forwarded.filter((f) => !subclasses.includes(f));
need(externalDefs.length >= 6, `外部定义 ${externalDefs.length}`);

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const body = `${banner("src/compiler/processed-feedback.h.js", "src/compiler/processed-feedback.h", "反馈槽处理后的多态容器")}
//! processed-feedback — ProcessedFeedback 族: 各反馈槽种类的处理结果
//!
//! Kind 判别 ${kinds.length} 值; IsInsufficient 短路; As* 为静态下取
//! (Rust 侧以 enum 重构, As* → 模式匹配)。

/// 原版 ProcessedFeedback::Kind (顺序 = 判别值)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumCount, strum::EnumIter)]
#[repr(u8)]
pub enum ProcessedFeedbackKind {
${kinds.map((k) => `  /// k${k}\n  ${k},`).join("\n")}
}

/// 原版 ProcessedFeedback — 基类面 (kind + slot_kind)
pub struct ProcessedFeedback {
  /// kind_
  pub kind: ProcessedFeedbackKind,
  /// slot_kind_ — 原 FeedbackSlotKind
  pub slot_kind: FeedbackSlotKind,
}

impl ProcessedFeedback {
  /// 原版 IsInsufficient
  /// v8: src/compiler/processed-feedback.h ProcessedFeedback::IsInsufficient
  pub fn is_insufficient(&self) -> bool {
    self.kind == ProcessedFeedbackKind::Insufficient
  }
}

/// Rust 重构面 — 原 As* 静态下取 ${asMethods.length} 个, 对应模式匹配臂
/// v8: src/compiler/processed-feedback.h ProcessedFeedback::As*
///
/// 本文件定义 (${subclasses.length}): ${subclasses.join(", ")}
/// 外部定义 (前向声明, ${externalDefs.length}): ${externalDefs.join(", ")}
${subclasses.map((s) => `/// v8: src/compiler/processed-feedback.h class ${s}\npub struct ${s};`).join("\n\n")}

/// 原版 InsufficientFeedback — 空反馈占位
/// v8: src/compiler/processed-feedback.h InsufficientFeedback
pub fn as_insufficient(fb: &ProcessedFeedback) -> &InsufficientFeedback {
  todo!("v8: src/compiler/processed-feedback.h ProcessedFeedback::AsInsufficient")
}

/// 原版 GlobalAccessFeedback — 全局访问 (property cell / script context 槽)
/// v8: src/compiler/processed-feedback.h GlobalAccessFeedback
impl GlobalAccessFeedback {
  pub fn is_megamorphic(&self) -> bool {
    todo!("v8: src/compiler/processed-feedback.h GlobalAccessFeedback::IsMegamorphic")
  }

  pub fn is_property_cell(&self) -> bool {
    todo!("v8: src/compiler/processed-feedback.h GlobalAccessFeedback::IsPropertyCell")
  }

  pub fn is_script_context_slot(&self) -> bool {
    todo!("v8: src/compiler/processed-feedback.h GlobalAccessFeedback::IsScriptContextSlot")
  }

  pub fn get_constant_hint(&self, broker: &JSHeapBroker) -> OptionalObjectRef {
    todo!("v8: src/compiler/processed-feedback.h GlobalAccessFeedback::GetConstantHint")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "processed_feedback.rs.skel", body);
console.log(`written ${p}\nkinds=${kinds.length} as_methods=${asMethods.length} local_subclasses=${subclasses.length} external=${externalDefs.length}`);
