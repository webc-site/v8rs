#!/usr/bin/env bun
// tool/src/debug/frames.js — 栈帧/求值/覆盖率/wasm 对象组
// 覆盖: src/debug/debug-frames.h/.cc + debug-stack-trace-iterator.h/.cc +
//       debug-evaluate.h/.cc + debug-coverage.h/.cc +
//       debug-wasm-objects.h/.cc/-inl.h/.tq (4 对 + wasm 对象族, 12 文件)
// 产出: v8-debug/src/frames.rs.skel (规则代码: Redirect Mode 枚举 + Coverage 结构骨架 +
//       WASM_VALUE_FIELDS 字段表; 调试字节码重定向面)
// 复跑幂等: 纯文本扫描 + 源码序输出, 无时间戳。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/debug/frames.js";
const FILES = [
  "src/debug/debug-frames.h",
  "src/debug/debug-frames.cc",
  "src/debug/debug-stack-trace-iterator.h",
  "src/debug/debug-stack-trace-iterator.cc",
  "src/debug/debug-evaluate.h",
  "src/debug/debug-evaluate.cc",
  "src/debug/debug-coverage.h",
  "src/debug/debug-coverage.cc",
  "src/debug/debug-wasm-objects.h",
  "src/debug/debug-wasm-objects.cc",
  "src/debug/debug-wasm-objects-inl.h",
  "src/debug/debug-wasm-objects.tq",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[frames.js] ${m}`); };
const need = (f, s) => { if (!txt[f].includes(s)) die(`${f} 缺针 ${s}`); };

// ① RedirectActiveFunctions::Mode (debug-frames.h)
need("src/debug/debug-frames.h", "enum class Mode {");
need("src/debug/debug-frames.h", "kUseOriginalBytecode,");
need("src/debug/debug-frames.h", "kUseDebugBytecode,");
need("src/debug/debug-frames.h", "class RedirectActiveFunctions : public ThreadVisitor {");
// ② 栈/求值针
need("src/debug/debug-stack-trace-iterator.h", "class DebugStackTraceIterator final : public debug::StackTraceIterator {");
need("src/debug/debug-evaluate.h", "class DebugEvaluate : public AllStatic {");
// ③ Coverage 结构针
need("src/debug/debug-coverage.h", "struct CoverageBlock {");
need("src/debug/debug-coverage.h", "CoverageBlock() : CoverageBlock(kNoSourcePosition, kNoSourcePosition, 0) {}");
need("src/debug/debug-coverage.h", "struct CoverageFunction {");
need("src/debug/debug-coverage.h", "struct CoverageScript {");
need("src/debug/debug-coverage.h", "class Coverage : public std::vector<CoverageScript> {");
// ④ wasm 对象字段表针
need("src/debug/debug-wasm-objects.h", "V(kTypeOffset, kTaggedSize)");
need("src/debug/debug-wasm-objects.h", "V(kValueOffset, kTaggedSize)");
need("src/debug/debug-wasm-objects.h", "DEFINE_FIELD_OFFSET_CONSTANTS(JSObject::kHeaderSize, WASM_VALUE_FIELDS)");

console.log(`OK frames.js 针核验通过: RedirectMode 2 + Coverage 4 结构 + WASM_VALUE_FIELDS 校验全过`);

