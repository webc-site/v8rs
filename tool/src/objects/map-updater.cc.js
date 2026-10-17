#!/usr/bin/env bun
// tool/src/objects/map-updater.cc.js — 覆盖 v8/src/objects/map-updater.cc (单文件族; .h 侧归 map-updater.h.js)
// → v8-objects-maps/src/map_updater_cc_generated.rs
// 机械提取: 全部 29 个 MapUpdater:: 方法定义 + 3 个匿名命名空间 helper 的签名骨架
//   (函数体留 todo! + /// v8: 行号锚)
// 对数 oracle: clang -E 预处理 v8/src/objects/map-updater.cc
//   (定义清单: 名字序 + 计数 1:1 取自编译器产物; 配置 INTL+WASM 开)

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { readV8, writeSrc, banner, root } from "../../lib.js";

const cc = readV8("src/objects/map-updater.cc");

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


// ── 定义清单扫描 (列首 "RET Class::method(" / "RET helper("; 剥 __attribute__) ──
const HELPER_NAMES = new Set(["EqualImmutableValues", "GeneralizeFieldType", "PrintGeneralization"]);
function scanDefs(rawText) {
  const out = [];
  const lines = rawText.replace(/__attribute__\(\(.*?\)\)/g, "")
    .split("\n").map((l) => l.replace(/^\s+/, ""));
  for (let i = 0; i < lines.length; i++) {
    // 组: 1=RET 前缀, 2=类方法名, 3=裸函数名 (helper)
    const m = lines[i].match(/^([A-Za-z_][\w:<>,&* ]*? )?(?:MapUpdater::(\w+)|\b(\w+))\s*\(/);
    if (!m) continue;
    const klass = m[2] ? "MapUpdater" : "";
    const name = m[2] ?? m[3];
    if (!klass && !HELPER_NAMES.has(name)) continue;
    // 括号配对扫完整签名 (跨行), 再找 "{"
    const start = lines[i].indexOf("(");
    let depth = 0, j = i, k = start, seenClose = -1;
    for (; j < lines.length; j++) {
      const line = j === i ? lines[i] : lines[j];
      for (; k < line.length; k++) {
        const ch = line[k];
        if (ch === "(") depth++;
        else if (ch === ")") {
          depth--;
          if (depth === 0) { seenClose = j; break; }
        }
      }
      if (seenClose >= 0) break;
      k = 0;
    }
    if (seenClose < 0) throw new Error(`签名未闭合 @${name}`);
    // 判别定义 vs 声明/调用: 闭括号后第一个 ";" 或 "{" 谁先出现
    // (";" 在前 = 调用/声明; "{" 在前 = 定义, 允许前置 const/初始化列表)
    let ej = seenClose;
    let verdict = "";
    outer: for (let y = seenClose; y < Math.min(seenClose + 16, lines.length); y++) {
      const from = y === seenClose ? k + 1 : 0;
      for (let x = from; x < lines[y].length; x++) {
        const ch = lines[y][x];
        if (ch === "{" || ch === ";") { verdict = ch; ej = y; break outer; }
      }
    }
    if (verdict !== "{") continue;
    // 参数文本
    const sigText = lines.slice(i, seenClose + 1).join(" ").replace(/\s+/g, " ");
    const pOpen = sigText.indexOf("(");
    let d = 0, pe = -1;
    for (let x = pOpen; x < sigText.length; x++) {
      if (sigText[x] === "(") d++;
      else if (sigText[x] === ")") { d--; if (d === 0) { pe = x; break; } }
    }
    const paramsText = sigText.slice(pOpen + 1, pe);
    // 参数拆分 (顶层逗号)
    const params = [];
    if (paramsText.trim()) {
      let pd = 0, cur = "";
      for (const ch of paramsText) {
        if (ch === "<" || ch === "(") pd++;
        else if (ch === ">" || ch === ")") pd--;
        if (ch === "," && pd === 0) { params.push(cur); cur = ""; }
        else cur += ch;
      }
      params.push(cur);
    }
    const parsed = params.map((p) => {
      p = p.replace(/=\s*[^,]*$/, "").trim();
      const words = p.split(/\s+/);
      const pname = (words[words.length - 1] ?? "").replace(/[^A-Za-z0-9_]/g, "");
      const ty = words.slice(0, -1).join(" ") || "usize";
      return { type: ty, name: pname || "arg" };
    });
    const ret = (m[1] ?? "").trim();
    // 展示签名: 到闭括号 + const 后缀, 剔函数体
    const afterClose = sigText.slice(pe + 1).replace(/\{[\s\S]*$/, "").trim();
    const sig = sigText.slice(0, pe + 1) + (afterClose ? ` ${afterClose}` : "");
    out.push({ line: i + 1, klass, name, ret, params: parsed, sig });
    i = ej;
  }
  return out;
}

const rawDefs = scanDefs(cc);
must(rawDefs.filter((d) => d.klass).length === 29, `原文 MapUpdater 方法 ${rawDefs.filter((d) => d.klass).length} != 29`);
must(rawDefs.filter((d) => !d.klass).length === 3, `原文匿名 helper ${rawDefs.filter((d) => !d.klass).length} != 3`);

// pp 侧: 取 .cc 自身命名空间 (最后一次 "namespace v8::internal {") 之后的内容再扫
const pp = clangE("src/objects/map-updater.cc");
const tailZone = pp.slice(pp.lastIndexOf("namespace v8::internal {"));
const ppDefs = scanDefs(tailZone);
must(ppDefs.length === rawDefs.length, `pp 定义数 ${ppDefs.length} != 原文 ${rawDefs.length}`);
rawDefs.forEach((d, i) => {
  must(ppDefs[i].name === d.name && ppDefs[i].klass === d.klass,
    `定义序失配 @${i}: 原文 ${d.klass ?? ""}${d.klass ? "::" : ""}${d.name} vs pp ${ppDefs[i].klass ?? ""}${ppDefs[i].klass ? "::" : ""}${ppDefs[i].name}`);
});
// 关键方法锚 (名字必在清单)
for (const n of ["Update", "UpdateImpl", "TryUpdateNoLock", "FindRootMap", "FindTargetMap",
  "BuildDescriptorArray", "ConstructNewMap", "ReconfigureToDataField", "GeneralizeField"])
  must(rawDefs.some((d) => d.klass === "MapUpdater" && d.name === n), `缺方法 ${n}`);

// ── Rust 骨架生成 ─────────────────────────────────────────────────────────
const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/(^|_)([A-Z]+)/g, (_, p, c) => p + c.toLowerCase()).replace(/__/g, "_").toLowerCase();
function rustTy(cpp) {
  cpp = cpp.trim();
  if (cpp === "bool") return "bool";
  if (/^(int|uint32_t|int32_t|uint64_t|size_t)$/.test(cpp)) return "u32";
  if (/^(double|float)$/.test(cpp)) return "f64";
  if (cpp === "const char*") return "&str";
  return "usize"; // 句柄/指针/枚举/Isolate 等不透明占位
}
function rustRet(cpp) {
  cpp = cpp.trim();
  if (cpp === "void") return "";
  if (cpp === "bool") return "bool";
  if (cpp === "double") return "f64";
  if (/^std::optional</.test(cpp)) return "Option<usize>";
  if (/::State$/.test(cpp) || cpp.endsWith("State")) return "u8";
  return "usize";
}
// 本波可脱离 Isolate 的纯逻辑语义面就地填体 (Developer 回填); 其余保持 todo! + 旁注。
// key = rustName, value = { ret?: Rust 返回类型覆盖, body: 函数体行 (含缩进) }。
const FILLS = {
  // v8: map-updater.cc:27 — inline bool EqualImmutableValues(Tagged, Tagged):
  //   obj1 == obj2 对所有 kind 成立; AccessorPair 比较为后续 TODO (原版亦如此)。
  equal_immutable_values: {
    ret: "bool",
    body: ["  _obj1 == _obj2"],
  },
};
// 未填体面统一旁注 (readme/zh.md MU-ISO): 需 Isolate + DirectHandle<Map>/FieldType + 过渡树迁移状态机。
const B_TODO = "  todo!(\"HC/波次登记: MU-ISO — 依赖 v8-runtime/v8-heap-core/v8-ic\")";

function fnBlock(d, rustName) {
  const params = d.params.map((p) => `_${p.name}: ${rustTy(p.type)}`);
  const fill = FILLS[rustName];
  const ret = fill?.ret ?? rustRet(d.ret);
  const retStr = ret ? ` -> ${ret}` : "";
  const doc = `/// v8: map-updater.cc:${d.line} — ${d.sig.replace(/\s+/g, " ").trim()}`;
  const body = fill ? fill.body : [B_TODO];
  // clippy too_many_arguments (>7 形参): 骨架折叠为单一元组参数 (逐元素注释保可读)
  if (params.length > 7) {
    // type_complexity: 元组提为类型别名
    const alias = rustName.split("_").map((w) => w[0].toUpperCase() + w.slice(1)).join("") + "Args";
    const tyDoc = `/// 原版 ${d.name} ${params.length} 形参折叠元组 (骨架)。`;
    const aliasDef = [tyDoc, `pub type ${alias} = (`,
      // 元组别名内不落 &str (生命周期); 原类型保留在注释
      ...d.params.map((p) => `  ${p.type === "const char*" ? "usize" : rustTy(p.type)}, // ${p.name}${p.type === "const char*" ? " (const char*)" : ""}`),
      ");"];
    return [...aliasDef, "", doc, `pub fn ${rustName}(_args: ${alias})${retStr} {`, ...body, "}"];
  }
  const one = `pub fn ${rustName}(${params.join(", ")})${retStr} {`;
  if (one.length <= 96) {
    return [doc, one, ...body, "}"];
  }
  return [doc, `pub fn ${rustName}(`,
    ...params.map((p) => `    ${p},`),
    `)${retStr} {`, ...body, "}"];
}

// 重载去重 (同名方法按出现序加 _2/_3)
const used = new Map();
let rust = banner("src/objects/map-updater.cc.js", "src/objects/map-updater.cc",
  "MapUpdater 方法签名骨架 (clang -E 对数 29+3; 纯逻辑面就地填, 其余推迟登记 readme/zh.md 台账 MU-ISO)");
rust += `//! MapUpdater 的 .cc 实现清单骨架: 29 个方法 + 3 个匿名 helper。
//! 参数类型为粗粒度占位 (句柄→usize/索引→u32/bool/double/&str), 为 handlified clang -E 对数存档。
//! 可脱离 Isolate 的语义面已在 \`crate::map_updater::MapUpdater\` 用 Tagged 类型 1:1 实现
//! (find_root_map/find_field_owner/generalize_field/reconfigure_to_data_field/
//!  try_reconfigure_to_data_field_inplace/reconfigure_elements_kind/complete_inobject_slack_tracking)。
//! EqualImmutableValues 为纯 Tagged 相等比较, 本波就地填体。
//! 其余 todo! 推迟 (readme/zh.md MU-ISO): 需 Isolate + DirectHandle<Map>/FieldType + 过渡树迁移
//! 状态机 (Update/UpdateImpl/FindTargetMap/ConstructNewMap/BuildDescriptorArray/Normalize/ApplyPrototypeTransition),
//! 解锁 crate v8-runtime / v8-heap-core / v8-ic。State 枚举见 crate::map_updater_generated。

`;

const blocks = [];
for (const d of rawDefs) {
  const base = d.klass ? `map_updater_${snake(d.name)}` : snake(d.name);
  const n = (used.get(base) ?? 0) + 1;
  used.set(base, n);
  const rustName = n === 1 ? base : `${base}_${n}`;
  blocks.push(...fnBlock(d, rustName), "");
}
rust += blocks.join("\n");

const p = writeSrc("v8-objects-maps", "map_updater_cc_generated.rs", rust);
fmtNormalize(p);
console.log(`map_updater_cc_generated.rs: ${rawDefs.filter((d) => d.klass).length} 方法 + ${rawDefs.filter((d) => !d.klass).length} helper (pp 定义清单 1:1), 断言 ${checks} 项通过 → ${p}`);
