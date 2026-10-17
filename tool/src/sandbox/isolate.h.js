#!/usr/bin/env bun
// tool/src/sandbox/isolate.h.js
// 机械提取 v8/src/sandbox/isolate.{h,-inl.h} — IsolateForSandbox 沙箱侧门面。
// 产物: v8-sandbox/src/isolate.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/sandbox/isolate.h";
const INL_REL = "src/sandbox/isolate-inl.h";
const h = readV8(H_REL);
const inl = readV8(INL_REL);

// 类体方法面
const classM = h.match(/class V8_EXPORT_PRIVATE IsolateForSandbox final\s*\{([\s\S]*?)\n\};/);
if (!classM) throw new Error("未找到 IsolateForSandbox 类");
const hMethods = [...classM[1].matchAll(/^\s*(?:template <[\s\S]*?>\s*\n)?\s*(?:inline\s+)?[A-Za-z].*?\b(\w+)\(/gm)].map((m) => m[1]);
if (hMethods.length < 8) throw new Error(`isolate.h 方法面不足: ${hMethods.length}`);
const inlMethods = [...inl.matchAll(/^\s*(?:inline\s+)?[A-Za-z].*?\b(\w+)\(/gm)].map((m) => m[1]);
if (inlMethods.length < 8) throw new Error(`isolate-inl.h 方法面不足: ${inlMethods.length}`);
if (!h.includes("GetCurrentIsolateForSandbox()")) throw new Error("缺 GetCurrentIsolateForSandbox 锚");
// 沙箱开闭双分支构造
if (!h.includes("#ifdef V8_ENABLE_SANDBOX") || !h.includes("#else  // V8_ENABLE_SANDBOX")) throw new Error("缺沙箱开闭双分支锚");

let rust = banner("src/sandbox/isolate.h.js", "src/sandbox/isolate.{h,-inl.h}", "IsolateForSandbox 门面骨架");
rust += `
/// 原 class IsolateForSandbox — 沙箱开启时持 Isolate*, 关闭时空实现。
/// 方法面 (h ${hMethods.length} / inl ${inlMethods.length} 条):
${[...new Set(hMethods)].map((s) => `// - ${s}`).join("\n")}

pub struct IsolateForSandbox {
  /// 沙箱开启时为 isolate 指针槽位, 关闭时无字段
  _priv: (),
}

impl IsolateForSandbox {
  /// 沙箱开启分支: IsolateForSandbox(IsolateT*) 存指针
  /// 沙箱关闭分支: constexpr 空构造 + GetCurrentIsolateForSandbox() 返回 {}
  // TODO(dev): 补 trusted/cage base 指针读取实现。
}

// 原签名: V8_INLINE IsolateForSandbox GetCurrentIsolateForSandbox();

// —— 提取不变式断言 ——
const _: () = {
  let _i = IsolateForSandbox { _priv: () };
};
`;

const p = writeSrc("v8-sandbox", "isolate.rs", rust);
console.log(`written ${p} (h ${hMethods.length}, inl ${inlMethods.length})`);
