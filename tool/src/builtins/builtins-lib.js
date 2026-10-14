// tool/src/builtins/builtins-lib.js — builtins 分组脚本共享引擎
// 被 tool/src/builtins/*.js import; 与 lib.js 同等对待 (子代理禁改)
// 只 import node 内置 + tool/lib.js
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { banner, readV8, writeSrc } from "../../lib.js";

export { banner, readV8, writeSrc };

const repoRoot = join(import.meta.dir, "..", "..", "..");

// 生成体经 rustfmt 归一, 复跑幂等; 失败即 throw
export function fmt(body) {
  const r = spawnSync(
    "rustfmt",
    ["--edition", "2024", "--config-path", join(repoRoot, "rustfmt.toml")],
    { input: body, encoding: "utf8" },
  );
  if (r.status !== 0) throw new Error(`rustfmt 归一失败: ${r.stderr}`);
  return r.stdout;
}

// 多锚点校验: 缺失即 throw (原版漂移停下修脚本)
export function anchors(content, subs, file) {
  for (const s of subs) {
    if (!content.includes(s)) throw new Error(`锚点失配: ${file} 缺少 "${s.slice(0, 64)}"`);
  }
}

// .cc 侧: BUILTIN(Name) / TF_BUILTIN(Name, Asm) / Generate_Name( → 内建名清单
export function ccBuiltins(src) {
  const out = [];
  for (const m of src.matchAll(/^BUILTIN\((\w+)\)/gm)) out.push({ name: m[1], kind: "BUILTIN" });
  for (const m of src.matchAll(/^TF_BUILTIN\((\w+),\s*(\w+)\)/gm)) out.push({ name: m[1], kind: "TF", asm: m[2] });
  for (const m of src.matchAll(/void Builtins::Generate_(\w+)\(/g)) out.push({ name: m[1], kind: "ASM" });
  return out;
}

// .h 侧: enum / kXxx 静态常量 → 规则条目
export function hRules(src) {
  const enums = [];
  for (const m of src.matchAll(/enum(?:\s+(?:class|struct))?\s+(\w+)\s*(?::\s*([\w:]+))?\s*\{/g))
    enums.push({ name: m[1], ty: m[2] ?? null });
  const consts = [];
  for (const m of src.matchAll(/static\s+constexpr?\s*([\w:<>, ]+?)\s+(k\w+)\s*=\s*([^;]+);/g))
    consts.push({ ty: m[1].trim(), name: m[2], expr: m[3].trim().slice(0, 120) });
  return { enums, consts };
}

// .tq 侧: builtin/macro/struct/type 声明行
export function tqDecls(src) {
  const out = [];
  for (const m of src.matchAll(/^(javascript builtin|builtin|macro|extern macro|struct|type)\s+(\w+)/gm))
    out.push({ kind: m[1], name: m[2] });
  return out;
}

// 内建名 → Rust 骨架函数名 (关键字加 r# 转义)
const RAW = new Set(["typeof", "type", "match", "move", "ref", "use", "where", "loop", "mod", "crate", "self", "super", "extern", "fn", "struct", "enum", "trait", "impl", "for", "in", "if", "else", "while", "return", "let", "const", "static", "async", "await", "dyn", "union", "try", "yield", "do", "abstract", "become", "box", "final", "macro", "override", "priv", "unsized", "virtual"]);
export function fnName(name) {
  const s = name.replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
  return RAW.has(s) ? `r#${s}` : s;
}

// 产物头
export function head(generator, files, note) {
  return `// 由 tool/src/builtins/${generator} 生成; 来源 v8/src/builtins 下 ${files.length} 文件\n// ${note}\n// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义\n`;
}

// 注意: 独立 /// 行后必须紧跟条目, 否则 rustfmt 报悬空注释; 无条目时用 //
// modBody: 内建名表 (strum 全派生) + 签名骨架 (todo! 占位, 修订填语义)
export function modBody(generator, files, builtins, libMods) {
  const o = [];
  o.push(head(generator, files, `内建名表 + 签名骨架 (${builtins.length} 内建)`));
  o.push('use strum::{Display, EnumIter, EnumString};\n');
  o.push(`/// 内建名表 (来源: ${files.slice(0, 6).join(", ")}${files.length > 6 ? " 等" : ""})`);
  o.push('#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]');
  o.push('#[strum(serialize_all = "PascalCase")]');
  o.push('pub enum Builtin {');
  for (const b of builtins) {
    const v = b.name.replace(/_/g, "");
    if (v !== b.name) o.push(`  #[strum(serialize = "${b.name}")]`);
    o.push(`  ${v},`);
  }
  o.push('}\n');
  if (builtins.length) o.push('/// 签名骨架 (修订阶段填语义; 当前 todo! 占位)');
  for (const b of builtins.slice(0, 800)) {
    o.push(`/// v8: Builtins::Generate_${b.name} (${b.kind}${b.asm ? `/${b.asm}` : ""})`);
    o.push(`#[allow(dead_code)]`);
    o.push(`pub fn ${fnName(b.name)}() {`);
    o.push(`  todo!()`);
    o.push(`}\n`);
  }
  return fmt(o.join("\n"));
}

// libBody: 模块索引
export function libBody(generator, mods) {
  const o = [head(generator, mods.map((m) => m + ".rs"), "模块索引")];
  for (const m of mods) o.push(`pub mod ${m};`);
  o.push("");
  for (const m of mods) o.push(`pub use ${m}::Builtin as ${m[0].toUpperCase()}${m.slice(1)}Builtin;`);
  o.push('mod error;\npub use error::{Error, Result};');
  return fmt(o.join("\n") + "\n");
}

// groupRun: 分组脚本通用主流程; groups = [{mod, files:{cc:[],h:[],tq:[]}, anchors:[[substr,file]]}]
export function groupRun(generator, crate, groups) {
  const mods = [];
  let total = 0;
  for (const g of groups) {
    const files = [...g.files.cc, ...g.files.h, ...g.files.tq];
    for (const [sub, file] of g.anchors ?? []) anchors(readV8(`src/builtins/${file}`), [sub], file);
    const seen = new Map();
    for (const f of g.files.cc) for (const b of ccBuiltins(readV8(`src/builtins/${f}`))) seen.set(b.name, b);
    // .tq 声明未进 BUILTIN_LIST 的也登记 (宏/类型, 修订阶段分流)
    const tqExtra = [];
    for (const f of g.files.tq) for (const d of tqDecls(readV8(`src/builtins/${f}`))) tqExtra.push(d);
    const builtins = [...seen.values()].sort((a, b) => (a.name < b.name ? -1 : 1));
    let extra = "";
    if (tqExtra.length) {
      extra = `\n// .tq 声明 ${tqExtra.length} 条 (macro/struct/type/extern, 修订阶段分流)\n#[allow(dead_code)]
pub const TQ_DECLS: &[(&str, &str)] = &[\n`;
      extra += tqExtra.slice(0, 1000).map((d) => `  ("${d.kind}", "${d.name}"),`).join("\n") + "\n];\n";
    }
    // .h 枚举/常量清单
    const hr = { enums: [], consts: [] };
    for (const f of g.files.h) {
      const r = hRules(readV8(`src/builtins/${f}`));
      hr.enums.push(...r.enums.map((e) => ({ ...e, file: f })));
      hr.consts.push(...r.consts.map((c) => ({ ...c, file: f })));
    }
    if (hr.enums.length || hr.consts.length) {
      extra += `\n// .h 规则: 枚举 ${hr.enums.map((e) => e.name).join(", ") || "无"}; 常量 ${hr.consts.length} 条\n`;
      for (const c of hr.consts.slice(0, 100)) extra += `// ${c.file}: ${c.ty} ${c.name} = ${c.expr}\n`;
    }
    writeSrc(crate, `${g.mod}.rs`, modBody(generator, files, builtins, g.mod) + fmt(extra || "// (无 .tq/.h 附加规则)\n"));
    mods.push(g.mod);
    total += builtins.length;
    console.log(`[${generator}] ${g.mod}: 内建 ${builtins.length}, tq ${tqExtra.length}, h枚举 ${hr.enums.length}/常量 ${hr.consts.length}`);
  }
  writeSrc(crate, "lib.rs", libBody(generator, mods));
  console.log(`[${generator}] 完成: ${mods.length} 模块, 内建 ${total}`);
}
