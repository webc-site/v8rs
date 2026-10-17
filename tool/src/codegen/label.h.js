#!/usr/bin/env bun
// tool/src/codegen/label.h.js
// 机械提取 v8/src/codegen/label.h — Label 距离枚举与移动语义分支。
// 产物: v8-codegen-core/src/label_generated.rs
//   (label.rs 为手写产物, 本脚本落 _generated 旁路, 禁覆写)

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/label.h";
const h = readV8(H_REL);

const distM = h.match(/enum Distance\s*\{([^}]+)\};/);
if (!distM) throw new Error("未找到 Distance 枚举");
const dists = [...distM[1].matchAll(/k(\w+)/g)].map((m) => "k" + m[1]);
if (dists.join() !== "kNear,kFar") throw new Error(`Distance 条目异常: ${dists.join()}`);
if (!h.includes("#if !V8_TARGET_ARCH_ARM64")) throw new Error("缺 arm64 移动语义分支锚");
if (!h.includes("Label(Label&& other) V8_NOEXCEPT")) throw new Error("缺移动构造锚");

let rust = banner("src/codegen/label.h.js", H_REL, "Label 距离枚举");
rust += `
/// 跳转距离 (原 enum Distance): near = 8 位有符号位移, far = 32 位
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Distance {
  /// kNear: 8 位有符号位移
  Near = 0,
  /// kFar: 32 位有符号位移
  Far = 1,
}

/// 原 class Label — 一次性绑定标号 (bind 仅一次)
/// 平台分支登记:
/// - arm64: Assembler 持 Label 指针回填远目标, 禁移动/拷贝
/// - 其他目标 (含 x64): DEBUG 态提供移动构造 (旧 Label 清零防 DCHECK)
pub struct Label;

// —— 提取不变式断言 ——
const _: () = {
  assert!((Distance::Near as u8) < (Distance::Far as u8));
};
`;

const p = writeSrc("v8-codegen-core", "label_generated.rs", rust);
console.log(`written ${p}`);
