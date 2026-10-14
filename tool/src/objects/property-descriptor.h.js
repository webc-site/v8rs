#!/usr/bin/env bun
// tool/src/objects/property-descriptor.h.js — 覆盖 v8/src/objects/property-descriptor.h (单文件族)
// → v8-objects-maps/src/property_descriptor.rs
// 机械提取: PropertyDescriptor 字段骨架 (6 bool 位段 + 4 handle 槽) +
//   ES6 6.2.4 判定谓词 (纯布尔公式 1:1) + ToAttributes/转换入口签名 (todo!)
// 对数 oracle: clang -E 预处理 v8/src/objects/property-descriptor.h
//   (字段/方法声明清单计数取自编译器产物)

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { readV8, writeSrc, banner, root } from "../../lib.js";

const h = readV8("src/objects/property-descriptor.h");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── clang -E 对数基准 (原版侧 = 编译器预处理产物; 缺失头自动空 stub) ──────
const ppShim = join(tmpdir(), "v8rs-r3o4-pp-shim");
function clangE(rel) {
  const args = [
    "-E", "-P", "-x", "c++", "-std=c++20", "-I", "v8", "-I", "v8/include",
    "-I", ppShim, "-DV8_INTL_SUPPORT=1", "-DV8_ENABLE_WEBASSEMBLY=1", "-",
  ];
  for (let round = 0; round < 48; round++) {
    try {
      return execFileSync("clang", args, {
        input: `#include "${rel}"\n`,
        encoding: "utf8",
        cwd: root,
        maxBuffer: 512 * 1024 * 1024,
      });
    } catch (e) {
      const err = String(e.stderr ?? "");
      const m = err.match(/fatal error: '([^']+)' file not found/);
      if (!m) throw new Error(`clang -E ${rel} 失败: ${err.slice(0, 300)}`);
      const stub = join(ppShim, m[1]);
      mkdirSync(dirname(stub), { recursive: true });
      writeFileSync(stub, "#pragma once\n");
    }
  }
  throw new Error(`clang -E ${rel}: stub 轮数超限`);
}

// ── 产物 fmt 归一 (与 cargo +nightly fmt 同配置; 幂等) ────────────────────
function fmtNormalize(p) {
  const rustfmt = execFileSync("rustup", ["which", "--toolchain", "nightly", "rustfmt"],
    { encoding: "utf8", cwd: root }).trim();
  execFileSync(rustfmt, ["--edition", "2024", "--config-path", join(root, "rustfmt.toml"), p],
    { cwd: root });
}


const hpp = clangE("src/objects/property-descriptor.h");

// ── 1. 位段字段 (6 bool : 1) + handle 槽 (4) ─────────────────────────────
// 内部类锚: 前置声明 (class PropertyDescriptorObject;) 后紧跟 — 避开 v8 公开 API 同名类
const clsStart = hpp.indexOf("class PropertyDescriptorObject;\nclass PropertyDescriptor");
must(clsStart >= 0, "pp 未找到内部 PropertyDescriptor 类 (前置声明锚)");
const seg = hpp.slice(clsStart);
const clsBody = seg.slice(0, seg.indexOf("\n};"));
const bitFields = [...clsBody.replace(/\n\s+/g, " ").matchAll(/bool (\w+) : 1;/g)].map((m) => m[1]);
must(JSON.stringify(bitFields) === JSON.stringify([
  "enumerable_", "has_enumerable_", "configurable_", "has_configurable_",
  "writable_", "has_writable_",
]), `pp 位段字段失配: ${JSON.stringify(bitFields)}`);
const handles = [...clsBody.replace(/\n\s+/g, " ").matchAll(/IndirectHandle<([\w<>, |]+)> (\w_?\w*);/g)]
  .map((m) => [m[2], m[1]]);
must(JSON.stringify(handles) === JSON.stringify([
  ["value_", "JSAny"],
  ["get_", "UnionOf<JSAny, FunctionTemplateInfo>"],
  ["set_", "UnionOf<JSAny, FunctionTemplateInfo>"],
  ["name_", "JSAny"],
]), `pp handle 槽失配: ${JSON.stringify(handles)}`);

// ── 2. ES6 6.2.4 判定谓词 (pp 公式 1:1) ──────────────────────────────────
must(clsBody.includes("return desc->has_get() || desc->has_set();"), "pp IsAccessorDescriptor 公式失配");
must(clsBody.includes("return desc->has_value() || desc->has_writable();"), "pp IsDataDescriptor 公式失配");
must(clsBody.includes("!IsAccessorDescriptor(desc) && !IsDataDescriptor(desc);"), "pp IsGenericDescriptor 公式失配");
must(clsBody.includes("!has_enumerable() && !has_configurable() && !has_writable() &&"), "pp is_empty 公式失配");
must(clsBody.includes("has_configurable() && has_enumerable() && !has_value() &&"), "pp IsRegularAccessorProperty 公式失配");
must(clsBody.includes("has_configurable() && has_enumerable() && has_value() &&"), "pp IsRegularDataProperty 公式失配");
must(clsBody.includes("has_enumerable() && !enumerable() ? DONT_ENUM : NONE"), "pp ToAttributes 公式失配");

// ── 3. 转换入口签名 (pp 声明锚) ──────────────────────────────────────────
for (const a of [
  "DirectHandle<JSObject> ToObject(Isolate* isolate);",
  "DirectHandle<PropertyDescriptorObject> ToPropertyDescriptorObject(",
  "static bool ToPropertyDescriptor(Isolate* isolate, Handle<JSAny> obj,",
  "static void CompletePropertyDescriptor(Isolate* isolate,",
]) must(hpp.includes(a), `pp 缺声明锚: ${a}`);

let rust = banner("src/objects/property-descriptor.h.js", "src/objects/property-descriptor.h",
  "PropertyDescriptor 字段骨架 + ES6 6.2.4 谓词 (clang -E 对数)");
rust += `//! PropertyDescriptor: 属性描述符 (ES6 6.2.4)。
//! 骨架基线; handle 槽以 Option<usize> 占位 (语义面归 Developer 修订),
//! .cc 转换流程骨架归 v8-objects-rest 的 property_descriptor_cc.rs.skel
//! (tool/src/objects/property-descriptor.cc.js, 他席产物)。

/// 属性描述符 (6 布尔位段 + value/get/set/name 四载荷槽)。
#[derive(Debug, Clone, Default)]
pub struct PropertyDescriptor {
  /// enumerable_ : 1
  pub enumerable: bool,
  /// has_enumerable_ : 1
  pub has_enumerable: bool,
  /// configurable_ : 1
  pub configurable: bool,
  /// has_configurable_ : 1
  pub has_configurable: bool,
  /// writable_ : 1
  pub writable: bool,
  /// has_writable_ : 1
  pub has_writable: bool,
  /// value_ (IndirectHandle<JSAny> 占位)
  pub value: Option<usize>,
  /// get_ (IndirectHandle<UnionOf<JSAny, FunctionTemplateInfo>> 占位)
  pub get: Option<usize>,
  /// set_ (同 get_)
  pub set: Option<usize>,
  /// name_ (IndirectHandle<JSAny> 占位)
  pub name: Option<usize>,
}

impl PropertyDescriptor {
  /// 空描述符 (原版默认构造: 六布尔位全 false, 四载荷槽全空)。
  #[inline]
  #[must_use]
  pub const fn empty() -> Self {
    Self {
      enumerable: false,
      has_enumerable: false,
      configurable: false,
      has_configurable: false,
      writable: false,
      has_writable: false,
      value: None,
      get: None,
      set: None,
      name: None,
    }
  }

  /// v8: property-descriptor.h:29 — ES6 6.2.4.1 IsAccessorDescriptor
  #[inline]
  #[must_use]
  pub const fn is_accessor_descriptor(desc: &Self) -> bool {
    desc.get.is_some() || desc.set.is_some()
  }

  /// v8: property-descriptor.h:34 — ES6 6.2.4.2 IsDataDescriptor
  #[inline]
  #[must_use]
  pub const fn is_data_descriptor(desc: &Self) -> bool {
    desc.value.is_some() || desc.has_writable
  }

  /// v8: property-descriptor.h:39 — ES6 6.2.4.3 IsGenericDescriptor
  #[inline]
  #[must_use]
  pub const fn is_generic_descriptor(desc: &Self) -> bool {
    !Self::is_accessor_descriptor(desc) && !Self::is_data_descriptor(desc)
  }

  /// v8: property-descriptor.h:57 — is_empty (六槽皆空)
  #[inline]
  #[must_use]
  pub const fn is_empty(&self) -> bool {
    !self.has_enumerable &&
      !self.has_configurable &&
      !self.has_writable &&
      self.value.is_none() &&
      self.get.is_none() &&
      self.set.is_none()
  }

  /// v8: property-descriptor.h:62 — IsRegularAccessorProperty
  #[inline]
  #[must_use]
  pub const fn is_regular_accessor_property(&self) -> bool {
    self.has_configurable &&
      self.has_enumerable &&
      self.value.is_none() &&
      !self.has_writable &&
      self.get.is_some() &&
      self.set.is_some()
  }

  /// v8: property-descriptor.h:69 — IsRegularDataProperty
  #[inline]
  #[must_use]
  pub const fn is_regular_data_property(&self) -> bool {
    self.has_configurable &&
      self.has_enumerable &&
      self.value.is_some() &&
      self.has_writable &&
      self.get.is_none() &&
      self.set.is_none()
  }

  /// v8: property-descriptor.h:112 — ToAttributes (DONT_ENUM|DONT_DELETE|READ_ONLY 位组合)
  pub fn to_attributes(&self) -> u32 {
    todo!()
  }

  /// v8: property-descriptor.h:44 — ES6 6.2.4.4 ToObject
  pub fn to_object(_isolate: &str) -> usize {
    todo!()
  }

  /// v8: property-descriptor.h:46 — ToPropertyDescriptorObject
  pub fn to_property_descriptor_object(_isolate: &str) -> usize {
    todo!()
  }

  /// v8: property-descriptor.h:50 — ES6 6.2.4.5 ToPropertyDescriptor (静态)
  pub fn to_property_descriptor(_isolate: &str, _obj: usize) -> bool {
    todo!()
  }

  /// v8: property-descriptor.h:54 — ES6 6.2.4.6 CompletePropertyDescriptor (静态)
  pub fn complete_property_descriptor(_isolate: &str) {
    todo!()
  }
}

// —— 提取不变式断言 ——
const _: () = {
  let d = PropertyDescriptor::empty();
  assert!(d.is_empty());
  assert!(PropertyDescriptor::is_generic_descriptor(&d));
  let mut a = PropertyDescriptor::empty();
  a.has_configurable = true;
  a.has_enumerable = true;
  a.get = Some(1);
  a.set = Some(2);
  assert!(a.is_regular_accessor_property());
  let mut v = PropertyDescriptor::empty();
  v.has_configurable = true;
  v.has_enumerable = true;
  v.has_writable = true;
  v.value = Some(3);
  assert!(v.is_regular_data_property());
};
`;

const p = writeSrc("v8-objects-maps", "property_descriptor.rs", rust);
fmtNormalize(p);
console.log(`property_descriptor.rs: 字段 ${bitFields.length}+${handles.length} + 谓词 6 + 签名 5 (pp 对数), 断言 ${checks} 项通过 → ${p}`);
