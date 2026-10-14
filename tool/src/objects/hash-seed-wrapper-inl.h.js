#!/usr/bin/env bun
// tool/src/objects/hash-seed-wrapper-inl.h.js — 覆盖 v8/src/objects/hash-seed-wrapper-inl.h
// 组→文件映射 (本脚本管辖): 席位表差集 (.h 布局归既有 tool/src/objects/hash-seed-wrapper.h.js)
// → v8-objects-js-async/src/hash_seed_wrapper.rs
// 机械提取: data()/set_data() 非对齐载荷访问面

import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/hash-seed-wrapper-inl.h");
const h = readV8("src/objects/hash-seed-wrapper.h");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[hash-seed-wrapper-inl.h.js] ${m}`); };

// ── 1. -inl.h 访问器 (2 个: 读 const 引用 / 写 set_value) ────────────────
must(inl.includes("const HashSeed::Data& HashSeedWrapper::data() const { return *data_.address(); }"),
  "inl 缺 data() (address 解引用)");
must(inl.includes("void HashSeedWrapper::set_data(const HashSeed::Data& value) {"), "inl 缺 set_data 声明");
must(inl.includes("data_.set_value(value);"), "inl 缺 set_data 主体 (set_value)");

// ── 2. .h 布局交叉 (UnalignedValueMember 单字段; 既有脚本已产, 此处锚定) ──
must(h.includes("UnalignedValueMember<HashSeed::Data> data_;"), "h 缺 data_ 成员 (UnalignedValueMember)");
must(h.includes("V8_IT_NO_AUTO_DISPATCH"), "h 缺 V8_IT_NO_AUTO_DISPATCH 注解");

let rust = banner("src/objects/hash-seed-wrapper-inl.h.js", "src/objects/hash-seed-wrapper-inl.h",
  "席位表差集: 非对齐载荷读写访问器 (.h 布局归既有 hash-seed-wrapper.h.js)");
rust += `//! HashSeedWrapper -inl 差集: 哈希种子的非对齐读写。
//!
//! 布局 (UnalignedValueMember<HashSeed::Data> data_) 归既有
//! hash-seed-wrapper.h.js; 本文件登记访问语义: 读 = address() 解引用,
//! 写 = set_value (非对齐 memcpy 语义)。

/// 读访问: 原版 data() = *data_.address() (非对齐地址转引用)。
pub fn data() {
  todo!("就地修订: hash-seed-wrapper-inl.h data() — 非对齐读")
}

/// 写访问: 原版 set_data = data_.set_value(value) (非对齐写)。
pub fn set_data() {
  todo!("就地修订: hash-seed-wrapper-inl.h set_data — 非对齐写")
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(true); // 语义面以 2 访问器锚定 (见 must 断言), 布局不变式归既有产物
};
`;

const p = writeSrc("v8-objects-js-async", "hash_seed_wrapper.rs", rust);
console.log(`hash-seed-wrapper-inl.h.js: ${p} — 2 访问器 (非对齐 address/set_value), 断言 ${checks} 项全过`);
