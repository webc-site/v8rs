#!/usr/bin/env bun
// tool/src/strings/unittest-data.js — v8/test/unittests/strings/unicode-unittest.cc 的
// IncrementalUTF8DecodingVsNonIncrementalUtf8Decoding 大表 → v8-strings/tests/unicode_cases.rs
// 黄金数据数据文件 (7.3: 黄金数据直接转数据文件); 复跑幂等
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { readV8 } from "../../lib.js";

const src = readV8("test/unittests/strings/unicode-unittest.cc");

// 提取 IncrementalUTF8Decoding 测试内的 TestCase data[] = { {...}, {...} };
const fnStart = src.indexOf("TEST(UnicodeTest, IncrementalUTF8DecodingVsNonIncrementalUtf8Decoding)");
const arrStart = src.indexOf("TestCase data[] = {", fnStart);
const arrEnd = src.indexOf("\n  };", arrStart);
const body = src.slice(arrStart, arrEnd);

// 每个 TestCase 项: {{bytes...}, {expected...}}
const items = [...body.matchAll(/\{\{([^}]*)\},\s*\{([^}]*)\}\}/g)];
if (items.length < 80) throw new Error(`用例提取过少: ${items.length}`);

const nums = (s) =>
  s.split(",").map((t) => t.trim()).filter((t) => /^0x[0-9A-Fa-f]+$/.test(t)).map((t) => parseInt(t, 16));

const cases = items.map((m) => {
  const bytes = nums(m[1]);
  const expected = nums(m[2]);
  if (bytes.length === 0 || bytes.length !== expected.length && expected.length !== 1) {
    // 长度不等是正常现象 (如截断), 只要求都非空
    if (bytes.length === 0 || expected.length === 0) throw new Error("空用例");
  }
  const bs = bytes.map((b) => "0x" + b.toString(16).padStart(2, "0")).join(", ");
  const es = expected.map((b) => "0x" + b.toString(16)).join(", ");
  return `  (&[${bs}], &[${es}]),`;
});

const out = `// 由 tool/src/strings/unittest-data.js 从 v8/test/unittests/strings/unicode-unittest.cc 生成; 修订就地改, 复跑覆盖
// 黄金数据: IncrementalUTF8DecodingVsNonIncrementalUtf8Decoding 大表 (${items.length} 组, UTF-8-test.txt 派生)

/// (输入字节, 期望码点序列)。
#[rustfmt::skip]
pub static CASES: &[(&[u8], &[u32])] = &[
${cases.join("\n")}
];
`;

const p = join(import.meta.dir, "..", "..", "..", "v8-strings", "tests", "unicode_cases.rs");
mkdirSync(dirname(p), { recursive: true });
writeFileSync(p, out);
console.log(`written: v8-strings/tests/unicode_cases.rs (${items.length} 组)`);
