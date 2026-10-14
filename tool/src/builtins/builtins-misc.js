#!/usr/bin/env bun
// tool/src/builtins/builtins-misc.js — v8-builtins-misc 代码生成器
// 来源: number/math/object/function/iterator/json/date/intl/wasm 等杂项 .cc/.h/.tq
// → v8-builtins-misc/src/{number, object_func, data_time_json, system, wasm, error, lib}.rs

import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { root, readV8, writeSrc } from "../../lib.js";
import { ccBuiltins, tqDecls, fnName, fmt, anchors } from "./builtins-lib.js";

const crate = "v8-builtins-misc";

const groups = [
  {
    mod: "number",
    alias: "NumberBuiltin",
    tqAlias: "NUMBER_TQ_DECLS",
    files: {
      cc: [
        "builtins-number.cc",
        "builtins-number-gen.cc",
        "builtins-number-tsa.cc",
        "builtins-bigint.cc",
        "builtins-bigint-gen.cc",
        "builtins-math.cc",
        "builtins-math-xsum.cc",
      ],
      tq: ["number.tq", "builtins-bigint.tq", "math.tq", "boolean.tq", "symbol.tq"],
    },
    anchors: [
      ["BUILTIN(NumberPrototypeToExponential)", "builtins-number.cc"],
      ["BUILTIN(MathSumPrecise)", "builtins-math.cc"],
    ],
    predicates: [
      {
        name: "is_bigint",
        doc: "是否为 BigInt 相关内建",
        filter: (name) => name.startsWith("BigInt") || name.endsWith("BigInt"),
      },
      {
        name: "is_number_prototype",
        doc: "是否为 Number 原型链方法",
        filter: (name) => name.startsWith("NumberPrototype"),
      },
      {
        name: "is_math",
        doc: "是否为 Math 数学内建",
        filter: (name) => name.startsWith("Math"),
      },
      {
        name: "is_baseline_or_feedback",
        doc: "是否为 Baseline 或包含反馈内建",
        filter: (name) => name.includes("Baseline") || name.includes("Feedback"),
      },
      {
        name: "is_trampoline",
        doc: "是否为字符串内化蹦床内建",
        filter: (name) => name.includes("Trampoline"),
      },
    ],
  },
  {
    mod: "object_func",
    alias: "ObjectFuncBuiltin",
    tqAlias: "OBJECT_FUNC_TQ_DECLS",
    files: {
      cc: [
        "builtins-object.cc",
        "builtins-object-gen.cc",
        "builtins-function.cc",
        "builtins-constructor-gen.cc",
        "builtins-conversion-gen.cc",
        "builtins-symbol.cc",
        "builtins-error.cc",
        "builtins-callsite.cc",
      ],
      tq: [
        "object.tq",
        "object-fromentries.tq",
        "function.tq",
        "constructor.tq",
        "conversion.tq",
        "console.tq",
        "symbol.tq",
      ],
    },
    anchors: [
      ["BUILTIN(ObjectDefineProperty)", "builtins-object.cc"],
      ["BUILTIN(FunctionConstructor)", "builtins-function.cc"],
    ],
    predicates: [
      {
        name: "is_callsite",
        doc: "是否为 CallSite 栈帧内建",
        filter: (name) => name.startsWith("CallSite"),
      },
      {
        name: "is_constructor",
        doc: "是否为构造器相关内建",
        filter: (name) =>
          (name.includes("Constructor") || name.startsWith("Construct")) &&
          !name.startsWith("CallSite"),
      },
      {
        name: "is_function_prototype",
        doc: "是否为 Function 原型方法",
        filter: (name) => name.startsWith("FunctionPrototype"),
      },
      {
        name: "is_error",
        doc: "是否为 Error 异常内建",
        filter: (name) => name.startsWith("Error"),
      },
      {
        name: "is_baseline_or_feedback",
        doc: "是否为 Baseline 或包含反馈内建",
        filter: (name) => name.includes("Baseline") || name.includes("Feedback"),
      },
    ],
  },
  {
    mod: "data_time_json",
    alias: "DateTimeJsonBuiltin",
    tqAlias: "DATA_TIME_JSON_TQ_DECLS",
    files: {
      cc: [
        "builtins-json.cc",
        "builtins-date.cc",
        "builtins-date-gen.cc",
        "builtins-intl.cc",
        "builtins-intl-gen.cc",
        "builtins-temporal.cc",
        "builtins-console.cc",
      ],
      tq: ["console.tq"],
    },
    anchors: [["Json", "builtins-json.cc"]],
    predicates: [
      {
        name: "is_date",
        doc: "是否为 Date 日期内建",
        filter: (name) => name.startsWith("Date") && !name.startsWith("DateTimeFormat"),
      },
      {
        name: "is_intl",
        doc: "是否为 Intl 国际化内建",
        filter: (name) =>
          name.startsWith("Collator") ||
          name.startsWith("DateTimeFormat") ||
          name.startsWith("DisplayNames") ||
          name.startsWith("DurationFormat") ||
          name.startsWith("Intl") ||
          name.startsWith("ListFormat") ||
          name.startsWith("Locale") ||
          name.startsWith("NumberFormat") ||
          name.startsWith("PluralRules") ||
          name.startsWith("RelativeTimeFormat") ||
          name.startsWith("Segment") ||
          name.startsWith("V8BreakIterator") ||
          name.endsWith("Intl") ||
          name.includes("LocaleLower") ||
          name.includes("LocaleUpper"),
      },
      {
        name: "is_temporal",
        doc: "是否为 Temporal 新一代时间内建",
        filter: (name) => name.includes("Temporal"),
      },
      {
        name: "is_json",
        doc: "是否为 JSON 内建",
        filter: (name) => name.startsWith("Json") || name.includes("ToJson"),
      },
      {
        name: "is_console",
        doc: "是否为 Console 控制台内建",
        filter: (name) => name.startsWith("Console"),
      },
    ],
  },
  {
    mod: "system",
    alias: "SystemBuiltin",
    tqAlias: "SYSTEM_TQ_DECLS",
    files: {
      cc: [
        "builtins-global.cc",
        "builtins-global-gen.cc",
        "builtins-handler-gen.cc",
        "builtins-ic-gen.cc",
        "builtins-internal.cc",
        "builtins-internal-gen.cc",
        "builtins-call-gen.cc",
        "builtins-interpreter-gen.cc",
        "builtins-weak-refs.cc",
        "builtins-struct.cc",
        "builtins-trace.cc",
        "builtins-shadow-realm.cc",
        "builtins-shadow-realm-gen.cc",
        "builtins-atomics-synchronization.cc",
        "builtins-api.cc",
        "builtins-abstract-module-source.cc",
        "builtins-async-module.cc",
        "builtins-effects-analyzer.cc",
        "js-trampoline-assembler.cc",
      ],
      tq: ["ic.tq", "ic-callable.tq"],
    },
    anchors: [
      [
        "TF_BUILTIN(Call_ReceiverIsNullOrUndefined_Baseline",
        "builtins-call-gen.cc",
      ],
    ],
    predicates: [
      {
        name: "is_ic",
        doc: "是否为 IC 内联缓存内建",
        filter: (name) => name.includes("IC") && !name.includes("URI"),
      },
      {
        name: "is_trampoline",
        doc: "是否为蹦床辅助内建",
        filter: (name) => name.includes("Trampoline"),
      },
      {
        name: "is_call",
        doc: "是否为函数调用调度内建",
        filter: (name) => name.startsWith("Call") || name.startsWith("DirectCEntry"),
      },
      {
        name: "is_interpreter",
        doc: "是否为解释器入栈/出栈辅助内建",
        filter: (name) => name.startsWith("Interpreter"),
      },
      {
        name: "is_shadow_realm",
        doc: "是否为 ShadowRealm 隔离内建",
        filter: (name) => name.startsWith("ShadowRealm"),
      },
      {
        name: "is_atomics",
        doc: "是否为 Atomics 原子同步内建",
        filter: (name) => name.startsWith("Atomics"),
      },
    ],
  },
  {
    mod: "wasm",
    alias: "WasmBuiltin",
    tqAlias: "WASM_TQ_DECLS",
    files: {
      cc: ["builtins-wasm-gen.cc"],
      tq: [
        "wasm.tq",
        "wasm-to-js.tq",
        "js-to-wasm.tq",
        "wasm-strings.tq",
        "wasm-interpreter-wrappers.tq",
      ],
    },
    anchors: [["Wasm", "builtins-wasm-gen.cc"]],
    predicates: [
      {
        name: "is_conversion",
        doc: "是否为 Wasm 类型转换内建",
        filter: (name) => name.includes("ToNumber") || name.includes("ToString"),
      },
      {
        name: "is_wrapper",
        doc: "是否为 Wasm 包装器内建",
        filter: (name) => name.includes("Wrapper"),
      },
      {
        name: "is_deopt",
        doc: "是否为 Wasm 去优化延续内建",
        filter: (name) => name.includes("Deopt"),
      },
    ],
  },
];

// 生成各模块的源码
const exportsByMod = new Map();

for (const g of groups) {
  // 校验锚点
  for (const [sub, file] of g.anchors ?? []) {
    anchors(readV8(`src/builtins/${file}`), [sub], file);
  }

  // 提取 ccBuiltins
  const seen = new Map();
  for (const f of g.files.cc) {
    for (const b of ccBuiltins(readV8(`src/builtins/${f}`))) {
      const v = b.name.replace(/_/g, "");
      seen.set(b.name, {
        name: b.name,
        variant: v,
        fn: fnName(b.name),
        kind: b.kind,
      });
    }
  }

  // 排序
  const builtins = [...seen.values()].sort((a, b) => (a.name < b.name ? -1 : 1));

  // 提取 tqDecls
  const tqList = [];
  for (const f of g.files.tq ?? []) {
    for (const d of tqDecls(readV8(`src/builtins/${f}`))) {
      tqList.push(d);
    }
  }

  // 构建 Rust 代码
  const lines = [];
  lines.push(`// 由 tool/src/builtins/builtins-misc.js 生成; 来源 v8/src/builtins`);
  lines.push(`// 内建名表 + 核心调度元数据 (${builtins.length} 内建)`);
  lines.push("");
  lines.push("use strum::{Display, EnumIter, EnumString};");
  lines.push("use v8_builtins_core::{descriptor_name, find_builtin};");
  lines.push("");
  lines.push(`/// 内建名表 (共 ${builtins.length} 项)`);
  lines.push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]");
  lines.push("pub enum Builtin {");
  for (const b of builtins) {
    if (b.variant !== b.name) {
      lines.push(`  #[strum(serialize = "${b.name}")]`);
    }
    lines.push(`  ${b.variant},`);
  }
  lines.push("}");
  lines.push("");
  lines.push("impl Builtin {");
  lines.push("  /// 获取内建的 V8 原版规范名称");
  lines.push("  #[inline]");
  lines.push("  pub const fn name(&self) -> &'static str {");
  lines.push("    match self {");
  for (const b of builtins) {
    lines.push(`      Self::${b.variant} => "${b.name}",`);
  }
  lines.push("    }");
  lines.push("  }");
  lines.push("");
  lines.push("  /// 关联至全局注册表 `BUILTIN_TABLE` 获取全局内建 ID");
  lines.push("  #[inline]");
  lines.push("  pub fn global_id(&self) -> Option<usize> {");
  lines.push("    find_builtin(self.name())");
  lines.push("  }");
  lines.push("");
  lines.push("  /// 获取对应的接口描述符符号");
  lines.push("  #[inline]");
  lines.push("  pub fn descriptor_name(&self) -> String {");
  lines.push("    descriptor_name(self.name())");
  lines.push("  }");
  lines.push("");

  // 生成谓词
  for (const p of g.predicates) {
    lines.push(`  /// ${p.doc}`);
    lines.push("  #[inline]");
    lines.push(`  pub const fn ${p.name}(&self) -> bool {`);
    const matched = builtins.filter((b) => p.filter(b.name));
    if (matched.length === 0) {
      lines.push("    false");
    } else {
      lines.push("    matches!(");
      lines.push("      self,");
      const pattern = matched.map((b) => `Self::${b.variant}`).join(" | ");
      lines.push(`      ${pattern}`);
      lines.push("    )");
    }
    lines.push("  }");
    lines.push("");
  }
  lines.push("}");
  lines.push("");

  // 零成本工厂函数
  lines.push(`// 统一提供 ${builtins.length} 项零成本内建工厂函数`);
  const fnList = [];
  for (const b of builtins) {
    fnList.push(b.fn);
    lines.push(`pub const fn ${b.fn}() -> Builtin {`);
    lines.push(`  Builtin::${b.variant}`);
    lines.push("}");
    lines.push("");
  }

  // TQ 声明
  lines.push(`/// 来自 ${g.mod} 相关 .tq 的声明元数据 (共 ${tqList.length} 条)`);
  lines.push("pub const TQ_DECLS: &[(&str, &str)] = &[");
  for (const d of tqList.slice(0, 1000)) {
    lines.push(`  ("${d.kind}", "${d.name}"),`);
  }
  lines.push("];");
  lines.push("");

  writeSrc(crate, `${g.mod}.rs`, fmt(lines.join("\n")));

  exportsByMod.set(g.mod, {
    alias: g.alias,
    tqAlias: g.tqAlias,
    fns: fnList,
    count: builtins.length,
    tqCount: tqList.length,
  });

  console.log(`[builtins-misc.js] ${g.mod}: ${builtins.length} builtins, ${tqList.length} tq`);
}

// 生成 error.rs
const errorRs = `use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Number/BigInt operation error: {0}")]
  NumberError(String),

  #[error("Date/Time or Intl format error: {0}")]
  DateTimeError(String),

  #[error("System builtin dispatch error: {0}")]
  SystemError(String),

  #[error("WebAssembly runtime error: {0}")]
  WasmError(String),
}

pub type Result<T> = result::Result<T, Error>;
`;
writeSrc(crate, "error.rs", fmt(errorRs));

// 生成 lib.rs
const libLines = [];
libLines.push("// 由 tool/src/builtins/builtins-misc.js 生成; 来源 v8/src/builtins");
libLines.push("// 杂项内建统一入口");
libLines.push("");
libLines.push("#![forbid(unsafe_code)]");
libLines.push("");
for (const g of groups) {
  libLines.push(`mod ${g.mod};`);
}
libLines.push("mod error;");
libLines.push("");
libLines.push("pub use error::{Error, Result};");

for (const g of groups) {
  const exp = exportsByMod.get(g.mod);
  libLines.push(`pub use ${g.mod}::{`);
  libLines.push(`  Builtin as ${exp.alias}, TQ_DECLS as ${exp.tqAlias},`);
  for (const f of exp.fns) {
    libLines.push(`  ${f},`);
  }
  libLines.push("};");
}

writeSrc(crate, "lib.rs", fmt(libLines.join("\n")));
console.log(`[builtins-misc.js] 全部生成完毕！`);
