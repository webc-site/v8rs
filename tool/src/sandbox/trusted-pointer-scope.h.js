#!/usr/bin/env bun
// tool/src/sandbox/trusted-pointer-scope.h.js
// 机械提取 v8/src/sandbox/trusted-pointer-scope.{h,cc} — 可信指针发布作用域。
// State/Storage 枚举 + 沙箱开闭双分支类面骨架。
// 产物: v8-sandbox/src/trusted_pointer_scope.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/sandbox/trusted-pointer-scope.h";
const CC_REL = "src/sandbox/trusted-pointer-scope.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

const stateM = h.match(/enum class State : uint8_t\s*\{([^}]+)\};/);
if (!stateM) throw new Error("未找到 State 枚举");
const state = stateM[1].split(",").map((s) => s.trim()).filter(Boolean);
if (state.length !== 3) throw new Error(`State 条数不符: ${state.length}`);
const storageM = h.match(/enum class Storage : uint8_t\s*\{([^}]+)\};/);
if (!storageM) throw new Error("未找到 Storage 枚举");
const storage = storageM[1].split(",").map((s) => s.trim()).filter(Boolean);
if (storage.length !== 3) throw new Error(`Storage 条数不符: ${storage.length}`);
if (!h.includes("void TrackPointer(TrustedPointerTableEntry* entry);")) throw new Error("缺 TrackPointer 声明锚");
// 双分支: 沙箱开启有完整 Scope, 关闭时为空实现
const branches = (h.match(/class TrustedPointerPublishingScope/g) || []).length;
if (branches < 2) throw new Error(`双分支 Scope 类数量异常: ${branches}`);

let rust = banner("src/sandbox/trusted-pointer-scope.h.js", "src/sandbox/trusted-pointer-scope.{h,cc}", "TrustedPointerPublishingScope 骨架");
rust += `
/// 发布状态机 (原 enum class State : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
${state.map((n, i) => `  /// ${n}\n  ${n.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// 指针追踪存储形态 (原 enum class Storage : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Storage {
${storage.map((n, i) => `  /// ${n}\n  ${n.replace(/^k/, "")} = ${i},`).join("\n")}
}

/// 原 class TrustedPointerPublishingScope (沙箱开启分支)
/// - MarkSuccess/MarkFailure 置状态
/// - TrackPointer(TrustedPointerTableEntry*) 登记待发布指针
/// 沙箱关闭分支: MarkSuccess/MarkFailure 为空实现。
pub struct TrustedPointerPublishingScope {
  pub state: State,
  pub storage: Storage,
}

impl TrustedPointerPublishingScope {
  // 原签名: void MarkSuccess() { state_ = State::kSuccess; }
  // 原签名: void MarkFailure() { state_ = State::kFailure; }
  // 原签名: void TrackPointer(TrustedPointerTableEntry* entry);
  // TODO(dev): 补 TrackPointer 存储 (单例/向量) 与析构回滚。
}

/// 原 class DisableTrustedPointerPublishingScope (开关面, 双分支各一份)
pub struct DisableTrustedPointerPublishingScope;

// —— 提取不变式断言 ——
const _: () = {
  assert!(State::InProgress as u8 == 0);
  assert!(Storage::Empty as u8 == 0);
};
`;

const p = writeSrc("v8-sandbox", "trusted_pointer_scope.rs", rust);
console.log(`written ${p} (State 3, Storage 3)`);
