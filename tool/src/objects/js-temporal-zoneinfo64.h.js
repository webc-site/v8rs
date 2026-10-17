#!/usr/bin/env bun
// tool/src/objects/js-temporal-zoneinfo64.h.js — ZoneInfo64Provider 单例骨架
// 组→文件映射: js-temporal-zoneinfo64.h + js-temporal-zoneinfo64.cc
// 产物: v8-objects-js-core/src/js_temporal_zoneinfo64.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-temporal-zoneinfo64.h");
const cc = readV8("src/objects/js-temporal-zoneinfo64.cc");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. 类签名: 非堆对象, 进程级单例 ────────────────────────────────────
must(h.includes("class ZoneInfo64Provider {"), "ZoneInfo64Provider 类");
must(h.includes("static ZoneInfo64Provider& Singleton();"), "Singleton 声明");
must(h.includes("temporal_rs::Provider& Provider() { return *provider; }"), "Provider() 内联");
must(h.includes("#include \"temporal_rs/Provider.d.hpp\""), "依赖 temporal_rs::Provider");
must(h.includes("#ifdef V8_INTL_SUPPORT"), "INTL 条件编译");
must(h.includes("UDataMemory* memory;"), "INTL 分支 UDataMemory* memory");
// 禁拷贝/移动 4 连
eq((h.match(/ZoneInfo64Provider\((?:const ZoneInfo64Provider&|ZoneInfo64Provider&&)\) = delete;/g) || []).length, 2, "拷贝/移动构造 delete 计数");
eq((h.match(/ZoneInfo64Provider operator=/g) || []).length, 2, "拷贝/移动赋值 delete 计数");
must(h.includes("std::unique_ptr<temporal_rs::Provider> provider;"), "provider 唯一指针");

// ── 2. cc 关键锚点 (静态数据 / ICU 加载 / 单例) ────────────────────────
must(cc.includes("extern \"C\" uint32_t zoneinfo64_static_data[];"), "静态数据 extern 声明");
must(cc.includes("extern \"C\" size_t zoneinfo64_static_data_len;"), "静态数据长度 extern");
must(cc.includes("std::span<const uint32_t> data_span(zoneinfo64_static_data,"), "静态数据 span 接线");
must(cc.includes("ZoneInfo64Provider& ZoneInfo64Provider::Singleton() {"), "Singleton 定义");
must(cc.includes("static ZoneInfo64Provider* SINGLETON = new ZoneInfo64Provider();"), "惰性单例 (泄漏式, 无析构顺序问题)");
must(cc.includes("ZoneInfo64Provider::ZoneInfo64Provider() {"), "构造定义");
must(cc.includes("ZoneInfo64Provider::~ZoneInfo64Provider() {"), "析构定义");

// ── 3. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-temporal-zoneinfo64.h.js", "src/objects/js-temporal-zoneinfo64.h + js-temporal-zoneinfo64.cc", "ZoneInfo64Provider 单例差集骨架")}
//! ZoneInfo64Provider: 时区数据供给 (temporal_rs::Provider 包装), 进程级单例;
//! ICU4C udata 或内置 zoneinfo64_static_data 两路取数。

/// v8: src/objects/js-temporal-zoneinfo64.h ZoneInfo64Provider::Singleton
// Rust 侧: std::sync::OnceLock<ZoneInfo64Provider> (对应 C++ 函数内 static 泄漏式单例)
// pub fn singleton() -> &'static ZoneInfo64Provider

/// v8: src/objects/js-temporal-zoneinfo64.cc zoneinfo64_static_data
// 数据源二选一: V8_INTL_SUPPORT → UDataMemory (udata_open, 析构 udata_close);
//   否则 → extern "C" zoneinfo64_static_data[len] (u32 数组, span 接线)
// Provider(): 解引用 provider 唯一指针; 删除拷贝/移动 (4 个 = delete)
`;

const p = writeSrc("v8-objects-js-core", "js_temporal_zoneinfo64.rs.skel", body);
console.log(
  `js-temporal-zoneinfo64.h.js: ${p} — delete 计数 4/4、cc 锚 7/7、断言 ${checks} 项全部通过`,
);
