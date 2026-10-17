#!/usr/bin/env bun
// tool/src/execution/isolate.js — v8/src/execution isolate 核心族
// 覆盖: src/execution/isolate.h + isolate.cc + isolate-inl.h +
//       isolate-data.h + isolate-data-fields.h + isolate-utils.h + isolate-utils-inl.h
// 产出: v8-execution-isolate/src/isolate_data.rs
// 破环登记 (todo.md 五): objects/heap/execution 同属 C++ 一个 target, Rust 不成环
// 复跑幂等: 纯文本扫描 + 固定排序 + 确定性输出, 无时间戳。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/execution/isolate.js";
const FILES = [
  "src/execution/isolate.h",
  "src/execution/isolate.cc",
  "src/execution/isolate-inl.h",
  "src/execution/isolate-data.h",
  "src/execution/isolate-data-fields.h",
  "src/execution/isolate-utils.h",
  "src/execution/isolate-utils-inl.h",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[isolate.js] ${m}`); };

const fieldsH = txt["src/execution/isolate-data-fields.h"];

function getMacroBody(name) {
  const re = new RegExp(`^#define\\s+${name}\\(V\\)\\s*\\\\?\\n((?:.*\\\\\\n)*.*)`, "gm");
  let match;
  let best = "";
  while ((match = re.exec(fieldsH)) !== null) {
    const body = match[1].replace(/\\\n/g, "\n");
    if (body.includes("V(") || body.length > best.length) {
      best = body;
    }
  }
  if (!best) die(`找不到宏定义 ${name}`);
  return best;
}

function parseFields(body) {
  const fields = [];
  for (let line of body.split("\n")) {
    line = line.trim();
    if (!line || line.startsWith("/*") || line.startsWith("//") || line.startsWith("#")) continue;

    // V(Name, ...)
    let m = line.match(/^V\((\w+),/);
    if (m) {
      fields.push(m[1]);
      continue;
    }
    // PADDING_FIELD(..., V, Name, ...)
    m = line.match(/^PADDING_FIELD\([^,]+,\s*V,\s*(\w+),/);
    if (m) {
      fields.push(m[1]);
      continue;
    }
    // IF_WASM(V, Name, ...) / IF_INTL(V, Name, ...)
    m = line.match(/^IF_\w+\(V,\s*(\w+),/);
    if (m) {
      fields.push(m[1]);
      continue;
    }
    // 嵌套宏展开
    if (line === "ISOLATE_DATA_FAST_C_CALL_PADDING(V)") {
      fields.push(...parseFields(getMacroBody("ISOLATE_DATA_FAST_C_CALL_PADDING")));
      continue;
    }
    if (line === "ISOLATE_DATA_FIELDS_POINTER_COMPRESSION(V)") {
      fields.push(...parseFields(getMacroBody("ISOLATE_DATA_FIELDS_POINTER_COMPRESSION")));
      continue;
    }
    if (line === "ISOLATE_DATA_FIELDS_SANDBOX(V)") {
      fields.push(...parseFields(getMacroBody("ISOLATE_DATA_FIELDS_SANDBOX")));
      continue;
    }
    if (line === "ISOLATE_DATA_FIELDS_TIERING(V)") {
      fields.push(...parseFields(getMacroBody("ISOLATE_DATA_FIELDS_TIERING")));
      continue;
    }
  }
  return fields;
}

const mainRaw = parseFields(getMacroBody("ISOLATE_DATA_FIELDS"));
const subRaw = parseFields(getMacroBody("ISOLATE_DATA_SUBFIELDS"));

if (mainRaw.length !== 53) die(`ISOLATE_DATA_FIELDS 主字段数=${mainRaw.length}, 期望 53`);
if (subRaw.length !== 31) die(`ISOLATE_DATA_SUBFIELDS 子字段数=${subRaw.length}, 期望 31`);

const allRaw = [...mainRaw, ...subRaw];
if (allRaw.length !== 84) die(`总字段数=${allRaw.length}, 期望 84`);

const allFieldIds = allRaw.map((rawName) => {
  return rawName.startsWith("k") && rawName.length > 1 && rawName[1] === rawName[1].toUpperCase()
    ? rawName.slice(1)
    : rawName;
});

const out = [];
out.push(banner(GEN, FILES.join(", "), "IsolateData 核心字段标识符与字段布局"));
out.push(`
/// Isolate 内部数据字段标识符 (对应原版 enum class IsolateFieldId : uint8_t)
/// 按原版 ISOLATE_DATA_FIELDS 与 ISOLATE_DATA_SUBFIELDS 顺序展开
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum IsolateFieldId {`);

allFieldIds.forEach((name, idx) => {
  out.push(`  /// k${name} = ${idx}`);
  out.push(`  ${name} = ${idx},`);
});

out.push(`}

impl IsolateFieldId {
  /// 字段总数
  pub const COUNT: usize = ${allFieldIds.length};

  /// 全部字段 ID 列表
  pub const ALL: [IsolateFieldId; ${allFieldIds.length}] = [`);
for (const name of allFieldIds) {
  out.push(`    IsolateFieldId::${name},`);
}
out.push(`  ];

  /// 字段名称字面量
  #[inline]
  pub const fn name(self) -> &'static str {
    match self {`);
for (const name of allFieldIds) {
  out.push(`      IsolateFieldId::${name} => "${name}",`);
}
out.push(`    }
  }

  /// 转换为 u8 原始值
  #[inline]
  pub const fn as_u8(self) -> u8 {
    self as u8
  }
}

// ── 编译期静态断言 ──
const _: () = {
  assert!(IsolateFieldId::COUNT == ${allFieldIds.length});
  assert!(IsolateFieldId::ALL.len() == ${allFieldIds.length});
};
`);

writeSrc("v8-execution-isolate", "isolate_data.rs", out.join("\n") + "\n");
console.log(`OK isolate.js: 字段 ${allFieldIds.length} 项 → v8-execution-isolate/src/isolate_data.rs`);
