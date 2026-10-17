#!/usr/bin/env bun
// T0-tool覆盖审计: 扫描 v8/src 全量 *.cc/*.h/*.tq 与 tool/src 现有脚本,
// 输出 golden/tool-coverage.json 缺口清单 (每目录: 文件数/已有脚本数/缺口文件表)。
// 只审计不写实现; 复跑幂等 (排序 + 固定换行, 无时间戳)。
// 对数基准: 正则圈定脚本内 src/ 字面引用 + 人工核对 (见文末对数日志)。
import { readdirSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { root } from "./lib.js";

const V8_SRC = join(root, "v8", "src");
const TOOL_SRC = join(root, "tool", "src");
const OUT = join(root, "golden", "tool-coverage.json");

// 架构范围外登记 (todo 五: 先 x64 + arm64, ia32/mips64/ppc/s390/riscv/loong64/arm 后置不阻塞里程碑)
// 命中者不计入 gap, 单独计数; 新增条目必须写明理由
const OUT_OF_SCOPE_DIRS = [
  "src/codegen/arm",
  "src/codegen/ia32",
  "src/codegen/loong64",
  "src/codegen/mips64",
  "src/codegen/ppc",
  "src/codegen/riscv",
  "src/codegen/s390",
  "src/diagnostics/arm",
  "src/diagnostics/ia32",
  "src/diagnostics/loong64",
  "src/diagnostics/mips64",
  "src/diagnostics/ppc",
  "src/diagnostics/riscv",
  "src/diagnostics/s390",
  "src/regexp/arm",
  "src/regexp/ia32",
  "src/regexp/loong64",
  "src/regexp/mips64",
  "src/regexp/ppc",
  "src/regexp/riscv",
  "src/regexp/s390",
  "src/execution/arm",
  "src/execution/ia32",
  "src/execution/loong64",
  "src/execution/mips64",
  "src/execution/ppc",
  "src/execution/riscv",
  "src/execution/s390",
  "src/deoptimizer/arm",
  "src/deoptimizer/ia32",
  "src/deoptimizer/loong64",
  "src/deoptimizer/mips64",
  "src/deoptimizer/ppc",
  "src/deoptimizer/riscv",
  "src/deoptimizer/s390",
  "src/builtins/arm",
  "src/builtins/ia32",
  "src/builtins/loong64",
  "src/builtins/mips64",
  "src/builtins/ppc",
  "src/builtins/riscv",
  "src/builtins/s390",
  "src/wasm/baseline/arm",
  "src/wasm/baseline/ia32",
  "src/wasm/baseline/loong64",
  "src/wasm/baseline/mips64",
  "src/wasm/baseline/ppc",
  "src/wasm/baseline/riscv",
  "src/wasm/baseline/s390",
  "src/wasm/interpreter/riscv",
];
const isOutOfScope = (f) => OUT_OF_SCOPE_DIRS.some((d) => f.startsWith(`${d}/`));

// 递归收集文件, 返回相对根的路径数组 (排序)
function walk(dir, filter) {
  const out = [];
  const rec = (d) => {
    for (const e of readdirSync(d, { withFileTypes: true })) {
      const p = join(d, e.name);
      if (e.isDirectory()) {
        if (e.name === "target" || e.name === "node_modules") continue;
        rec(p);
      } else if (filter(p)) out.push(p.slice(dir.length + 1));
    }
  };
  rec(dir);
  return out.sort();
}

// 1. v8 全量: 仅 *.cc/*.h/*.tq
const v8Files = walk(V8_SRC, (p) => p.endsWith(".cc") || p.endsWith(".h") || p.endsWith(".tq"));
// 顶层目录 = 首段 (v8/src 直下无目标文件, 仅 DEPS/OWNERS 等, 已被扩展名过滤)
const v8ByDir = new Map();
for (const f of v8Files) {
  const top = f.includes("/") ? f.slice(0, f.indexOf("/")) : "(root)";
  if (!v8ByDir.has(top)) v8ByDir.set(top, []);
  v8ByDir.get(top).push(f);
}

// 2. tool 脚本: 仅 *.js (排除 target/node_modules)
const toolFiles = walk(TOOL_SRC, (p) => p.endsWith(".js"));
// 脚本归属目录: tool/src/<dir>/... → <dir>; tool/src/<name>.js 单文件组 → <name>
const toolByDir = new Map();
for (const f of toolFiles) {
  const top = f.includes("/") ? f.slice(0, f.indexOf("/")) : f.replace(/\.js$/, "");
  if (!toolByDir.has(top)) toolByDir.set(top, []);
  toolByDir.get(top).push(f);
}

// 3. 引用提取: 脚本内 src/...cc|h|tq 字面量 (readV8 实参/banner/注释/sigList 实参全含)
// 圈定正则: src/ 开头, 字符集 [A-Za-z0-9_./-] , 后缀 .cc/.h/.tq
// (?!\.(?:js|rs)\b): 排除产物路径 (xxx.cc.rs.skel 的 .cc 是 skel 文件名一部分, 非 v8 源引用)
const REF_RE = /(?<!third_party\/[A-Za-z0-9_.\/-]*?)src\/[A-Za-z0-9_.\/-]*\.(?:cc|h|tq)(?!\.(?:js|rs)\b)/g;
// 族名简写规范化: "xxx.h/.cc" / "xxx.h/.cc/-inl.h" 是作者表达同名族, 拆为独立文件引用
function expandRef(r) {
  const m = r.match(/^(.+\.h)((?:\/[-.a-z]+)+)$/);
  if (!m) return [r];
  const out = [m[1]];
  for (const suf of m[2].split("/")) {
    if (suf) out.push(m[1].replace(/\.h$/, "") + suf);
  }
  return out;
}
const refs = new Set(); // "src/objects/map.h" 形
const refByScript = new Map(); // script -> [refs]
for (const f of toolFiles) {
  const txt = readFileSync(join(TOOL_SRC, f), "utf8");
  const hits = [...new Set((txt.match(REF_RE) ?? []).flatMap(expandRef))].sort();
  refByScript.set(f, hits);
  for (const h of hits) refs.add(h);
}

// v8 文件集合 ("src/" 前缀对齐引用形)
const v8Set = new Set(v8Files.map((f) => `src/${f}`));
// 覆盖 = 引用 ∩ v8集合; 陈旧引用 = 引用 − v8集合 (上游已删, 脚本跑会炸)
const covered = new Set([...refs].filter((r) => v8Set.has(r)));
const stale = [...refs].filter((r) => !v8Set.has(r)).sort();
// 反向映射: v8 文件 -> 引用它的脚本表
const scriptsOf = new Map();
for (const [script, hits] of refByScript) {
  for (const h of hits) {
    if (!v8Set.has(h)) continue;
    if (!scriptsOf.has(h)) scriptsOf.set(h, []);
    scriptsOf.get(h).push(script);
  }
}
for (const v of scriptsOf.values()) v.sort();

// 3b. 产物反推: crate src/*.rs 的 banner 头补覆盖关系。
// 背景: 脚本可用相对名数组 / 模板变量拼路径 (聚合脚本形态), 字面正则抓不到,
// 会被误记 gap (实证: builtins 268 / debug gdb-server 16 假缺口)。
// banner 由 tool/lib.js banner() 统一生成, 格式固定:
//   "// 由 tool/<gen> 从 v8/<v8rel> 生成; 修订就地改, 复跑覆盖"
const BANNER_RE = /^\/\/\s*由\s+(tool\/[^\s]+)\s+从\s+(v8\/[^\s]+)\s+生成/m;
for (const e of readdirSync(root, { withFileTypes: true })) {
  if (!e.isDirectory() || !/^(v8-[a-z0-9-]+|rs8)$/.test(e.name)) continue;
  const crateSrc = join(root, e.name, "src");
  let rsFiles = [];
  try {
    rsFiles = walk(crateSrc, (p) => p.endsWith(".rs") || p.endsWith(".rs.skel"));
  } catch {
    continue; // crate 尚无 src/
  }
  for (const f of rsFiles) {
    const head = readFileSync(join(crateSrc, f), "utf8").slice(0, 600);
    const m = head.match(BANNER_RE);
    if (!m) continue;
    const v8rel = m[2].startsWith("v8/") ? m[2].slice(2) : m[2]; // v8/src/... → src/...
    if (!v8Set.has(v8rel)) continue; // banner 写错或不指向源码面, 不计入
    covered.add(v8rel);
    if (!scriptsOf.has(v8rel)) scriptsOf.set(v8rel, []);
    if (!scriptsOf.get(v8rel).includes(m[1])) scriptsOf.get(v8rel).push(m[1]);
  }
}
for (const v of scriptsOf.values()) v.sort();

// 4. 按目录组装 (目录全集 = v8 目录 ∪ tool 目录, 排序)
const allDirs = [...new Set([...v8ByDir.keys(), ...toolByDir.keys()])].sort();
const dirs = {};
let gapTotal = 0;
let gapInScopeTotal = 0;
for (const d of allDirs) {
  const files = v8ByDir.get(d) ?? [];
  const scripts = toolByDir.get(d) ?? [];
  const gapAll = files.filter((f) => !covered.has(`src/${f}`)).sort();
  const gap = gapAll.filter((f) => !isOutOfScope(`src/${f}`));
  const oos = gapAll.filter((f) => isOutOfScope(`src/${f}`));
  const hit = files.filter((f) => covered.has(`src/${f}`)).sort();
  gapTotal += gapAll.length;
  gapInScopeTotal += gap.length;
  dirs[d] = {
    // v8 侧文件数 (*.cc/*.h/*.tq, 递归含子目录)
    file_count: files.length,
    // tool 侧已有脚本数 (递归含子目录; 单文件组如 common.js 计 1)
    script_count: scripts.length,
    scripts,
    covered_count: hit.length,
    // 架构范围外未覆盖文件数 (不计入 gap_total, 不阻塞里程碑)
    out_of_scope_count: oos.length,
    gap_count: gap.length,
    // 缺口文件表 (v8/src/<dir>/ 下未被任何脚本引用且在范围内的文件, 相对 v8/src 路径)
    gap_files: gap,
  };
}

// 5. 写出 (固定键序 + 2 空格 + 尾换行, 复跑字节一致)
const doc = {
  generated_by: "tool/tool-coverage.js (T0-tool覆盖审计, 只审计不写实现)",
  v8_total: v8Files.length,
  tool_scripts_total: toolFiles.length,
  covered_total: covered.size,
  gap_total: gapTotal,
  // 范围内缺口 (扣掉 OUT_OF_SCOPE_DIRS 登记的非目标架构)
  gap_in_scope_total: gapInScopeTotal,
  // 陈旧引用 (脚本引了但 v8/src 已无此文件, 人工核对后修脚本或删引用)
  stale_refs: stale,
  dirs,
};
mkdirSync(join(root, "golden"), { recursive: true });
writeFileSync(OUT, `${JSON.stringify(doc, null, 2)}\n`);

// 6. 对数日志 (console, 人工核对用)
console.log(`v8 文件 ${v8Files.length} / tool 脚本 ${toolFiles.length} / 覆盖 ${covered.size} / 缺口 ${gapTotal} (范围内 ${gapInScopeTotal}) / 陈旧引用 ${stale.length}`);
for (const s of stale) console.log(`  陈旧引用: ${s}`);
for (const d of allDirs) {
  const e = dirs[d];
  console.log(`  ${d}: 文件 ${e.file_count} / 脚本 ${e.script_count} / 已覆 ${e.covered_count} / 缺口 ${e.gap_count} / 范围外 ${e.out_of_scope_count}`);
}
console.log(`已写出 golden/tool-coverage.json`);
