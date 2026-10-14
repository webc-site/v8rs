#!/usr/bin/env bun
// tool/src/objects/instance-type.h.js — 覆盖 v8/src/objects/instance-type.h
//        + instance-types-gen.h (metagen 路径, V8_USE_METAGEN=1)
//        + tools/metagen/instance_types.py (求解器复刻) / cpp_hier.py (头文件采集复刻)
//        → v8-objects-core/src/instance_type.rs
// X-macro 全量展开: TORQUE_ASSIGNED_INSTANCE_TYPES / TORQUE_ASSIGNED_INSTANCE_TYPE_LIST
//   由脚本内复刻 metagen 求解器从 V8_IT_* 注解头文件求解生成 (checkout 无生成产物);
//   ABI 锚点断言 (v8-internal.h Internals 常量) 全部通过方可写出
//
// v2 (OC-B): 采集器对齐 cpp_hier.py 真实语义, 修复三类漏采:
//   1. namespace internal 内所有 class/class-template 定义均采集 (不只 V8_OBJECT 标注),
//      V8_OBJECT 声明优先于同名遮蔽 (如 X::BodyDescriptor);
//   2. 模板基类解析 (resolve_logical_base 文本复刻): TaggedArrayBase<..> 的 Super 形参
//      按实参/默认实参绑定上溯, NON_EXPORTED_BASE(x) 宏解包;
//   3. participation 规则: candidates = 可达 HeapObject 且非 (NO_AUTO_CHECKER 无 OWN_TYPE);
//      participating = 叶子 或 带任意 V8_IT 注解; base 对 participating 集解析。
//   漏采后果 (v1): FIXED_ARRAY / WEAK_ARRAY_LIST / BYTE_ARRAY / 哈希字典族
//   (HASH_TABLE / NAME_DICTIONARY / ...) / TRANSITION_ARRAY 等整族缺失,
//   DEPENDENT_CODE_TYPE 别名错落到 0。
import { banner, writeSrc } from "../../lib.js";
import { writeFileSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const CRATE = "v8-objects-core";

// ---------------------------------------------------------------------------
// 通用: 去注释 (保护字符串/字符字面量)
// ---------------------------------------------------------------------------
function stripComments(src) {
  let out = "", i = 0, inStr = false, inChr = false;
  while (i < src.length) {
    const c = src[i], n = src[i + 1];
    if (inStr || inChr) {
      out += c;
      if (c === "\\") { out += n ?? ""; i += 2; continue; }
      if (inStr && c === '"') inStr = false;
      if (inChr && c === "'") inChr = false;
      i++; continue;
    }
    if (c === '"') { inStr = true; out += c; i++; continue; }
    if (c === "'") { inChr = true; out += c; i++; continue; }
    if (c === "/" && n === "/") { while (i < src.length && src[i] !== "\n") i++; continue; }
    if (c === "/" && n === "*") { i += 2; while (i < src.length && !(src[i] === "*" && src[i + 1] === "/")) i++; i += 2; continue; }
    out += c; i++;
  }
  return out;
}

// ---------------------------------------------------------------------------
// 头文件采集 (cpp_hier.py 复刻): namespace internal 内全部类定义 + V8_IT_* 注解
// ---------------------------------------------------------------------------
function walk(dir, out = []) {
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, e.name);
    if (e.isDirectory()) walk(p, out);
    else if (e.name.endsWith(".h")) out.push(p);
  }
  return out;
}

const V8_ROOT = join(import.meta.dir, "../../../v8");

// 目标配置: V8_ENABLE_WEBASSEMBLY 开 / V8_INTL_SUPPORT 开 / V8_TEMPORAL_SUPPORT 关
// (与 v8rs 目标配置及 V8 默认构建一致; all-objects.h 中 temporal 头被 #ifdef 排除)
const EXCLUDE = (p) => p.includes("js-temporal-objects");

// `namespace (v8::)internal {` 区域 (internal 限定过滤)
function internalRegions(src) {
  const re = /namespace\s+(v8\s*::\s*internal|internal)\s*\{/g;
  const regions = [];
  let m;
  while ((m = re.exec(src))) {
    let depth = 1, j = re.lastIndex;
    while (j < src.length && depth > 0) {
      if (src[j] === "{") depth++;
      else if (src[j] === "}") depth--;
      j++;
    }
    regions.push([m.index, j]);
  }
  return regions;
}

const QUAL = "(?:V8_EXPORT_PRIVATE\\s+|EXPORT_TEMPLATE_DECLARE\\([^)]*\\)\\s+|alignas\\([^)]*\\)\\s+)*";
// V8_OBJECT 标注声明 (pack 压缩类的权威声明)
const DECL = new RegExp(
  "(?:V8_OBJECT|V8_ABSTRACT_OBJECT)\\s+" + QUAL +
  "(?:template\\s*<((?:[^<>]|<[^<>]*>)*)>\\s+)?" + QUAL +
  "class\\s+" + QUAL + "(\\w+)(?!\\s*::)\\s*(?:final\\s+)?(?::\\s*([^{;]+))?\\{",
  "g",
);
// 无 V8_OBJECT 前缀的普通类/模板定义 (metagen 采集所有定义, 如 OrderedHashTable)
const DECL2 = new RegExp(
  "(?:^|\\n)\\s*(?:template\\s*<((?:[^<>]|<[^<>]*>)*)>\\s+)?" + QUAL +
  "class\\s+" + QUAL + "(\\w+)(?!\\s*::)\\s*(?:final\\s+)?(?::\\s*([^{;]+))?\\{",
  "g",
);
const MARK =
  /V8_IT_(ABSTRACT|REUSE_PARENT|NO_AUTO_CHECKER|NO_AUTO_DISPATCH|OWN_TYPE|FIXED_VALUE\(([^)]*)\)|FLAG_BITS\(([^)]*)\)|ORDER\((FIRST|LAST)\))/g;

// 模板形参表 "class D, typename E, class Super = FixedArrayBase" → [{name, def}]
function parseTplParams(s) {
  if (!s) return [];
  const parts = [];
  let depth = 0, cur = "";
  for (const ch of s) {
    if (ch === "<" || ch === "(") depth++;
    else if (ch === ">" || ch === ")") depth--;
    if (ch === "," && depth === 0) { parts.push(cur); cur = ""; continue; }
    cur += ch;
  }
  if (cur.trim()) parts.push(cur);
  return parts.map((p) => {
    const eq = p.indexOf("=");
    const head = (eq >= 0 ? p.slice(0, eq) : p).trim();
    return { name: head.split(/\s+/).pop(), def: eq >= 0 ? p.slice(eq + 1).trim() : null };
  });
}

// 基类说明 "public TaggedArrayBase<FixedArray, Object, FixedArrayBase>" → {tpl, args} / {name}
function parseBaseSpecs(basesRaw) {
  if (!basesRaw) return [];
  const segs = [];
  let depth = 0, cur = "";
  for (const ch of basesRaw) {
    if (ch === "<") depth++;
    else if (ch === ">") depth--;
    if (ch === "," && depth === 0) { segs.push(cur); cur = ""; continue; }
    cur += ch;
  }
  if (cur.trim()) segs.push(cur);
  const specs = [];
  for (let seg of segs) {
    seg = seg.trim();
    if (!seg) continue;
    seg = seg.replace(/^(public|protected|private)\s+/, "").replace(/^virtual\s+/, "").trim();
    const lt = seg.indexOf("<");
    if (lt >= 0) {
      const argsStr = seg.slice(lt + 1).replace(/\s*>\s*$/, "");
      const args = [];
      let d = 0, a = "";
      for (const ch of argsStr) {
        if (ch === "<") d++;
        else if (ch === ">") d--;
        if (ch === "," && d === 0) { args.push(a.trim()); a = ""; continue; }
        a += ch;
      }
      if (a.trim()) args.push(a.trim());
      specs.push({ tpl: seg.slice(0, lt).trim(), args });
    } else {
      // NON_EXPORTED_BASE(FixedArray) 等宏基类解包
      const nb = seg.match(/^NON_EXPORTED_BASE\((.+)\)\s*$/);
      if (nb) specs.push(...parseBaseSpecs(nb[1]));
      else specs.push({ name: seg });
    }
  }
  return specs;
}

function harvestClasses() {
  const files = walk(join(V8_ROOT, "src")).filter((p) => !EXCLUDE(p));
  const classes = [];
  for (const f of files) {
    const src = stripComments(readFileSync(f, "utf8"));
    const regions = internalRegions(src);
    if (!regions.length) continue;
    const inInternal = (idx) => regions.some(([a, b]) => idx >= a && idx <= b);
    for (const re of [DECL, DECL2]) {
      re.lastIndex = 0;
      let m;
      while ((m = re.exec(src))) {
        if (!inInternal(m.index)) continue;
        const cls = {
          name: m[2],
          params: parseTplParams(m[1]),
          bases: parseBaseSpecs(m[3]),
          anns: [],
          marked: re === DECL,
          file: f.slice(V8_ROOT.length + 1),
        };
        // 类体花括号配对
        const open = src.indexOf("{", m.index + m[0].length - 1);
        let depth = 0, j = open, str = false, chr = false;
        while (j < src.length) {
          const c = src[j];
          if (str || chr) {
            if (c === "\\") { j += 2; continue; }
            if (str && c === '"') str = false;
            if (chr && c === "'") chr = false;
            j++; continue;
          }
          if (c === '"') str = true;
          else if (c === "'") chr = true;
          else if (c === "{") depth++;
          else if (c === "}") { depth--; if (depth === 0) break; }
          j++;
        }
        const body = src.slice(open, j);
        let mm;
        MARK.lastIndex = 0;
        while ((mm = MARK.exec(body))) {
          if (mm[1] === "FIXED_VALUE") cls.anns.push(`FIXED_VALUE(${mm[2]})`);
          else if (mm[1] === "FLAG_BITS") cls.anns.push(`FLAG_BITS(${mm[3]})`);
          else if (mm[1] === "ORDER") cls.anns.push(`ORDER(${mm[4]})`);
          else cls.anns.push(mm[1]);
        }
        classes.push(cls);
      }
    }
  }
  return classes;
}

// V8_OBJECT 标注声明优先于同名遮蔽
function makeByName(classes) {
  const byName = new Map();
  for (const c of classes) {
    const prev = byName.get(c.name);
    if (!prev || (!prev.marked && c.marked)) byName.set(c.name, c);
  }
  return byName;
}

// resolve_logical_base 文本复刻: 沿基类链 (含模板实例化 + 形参替换) 上溯,
// 返回第一个 ∈ targetSet 或 "HeapObject" 的名字
function makeResolver(byName) {
  function resolveSpec(spec, map, targetSet, seen) {
    if (spec.tpl) {
      const tplCls = byName.get(spec.tpl);
      if (!tplCls) return null;
      const key = spec.tpl + "<" + spec.args.join(",") + ">";
      if (seen.has(key)) return null;
      seen.add(key);
      const m2 = {};
      tplCls.params.forEach((p, i) => { m2[p.name] = spec.args[i] ?? p.def ?? p.name; });
      for (const b of tplCls.bases) {
        const r = resolveSpec(b, m2, targetSet, seen);
        if (r) return r;
      }
      return null;
    }
    let n = spec.name;
    if (map && map[n]) n = map[n];
    const lt = n.indexOf("<");
    if (lt >= 0) return resolveSpec(parseBaseSpecs(n)[0] ?? null, null, targetSet, seen);
    if (n === "HeapObject" || targetSet.has(n)) return n;
    const c = byName.get(n);
    if (!c || seen.has(n)) return null;
    seen.add(n);
    for (const b of c.bases) {
      const r = resolveSpec(b, null, targetSet, seen);
      if (r) return r;
    }
    return null;
  }
  return (name, targetSet) => {
    const c = byName.get(name);
    if (!c) return null;
    for (const b of c.bases) {
      const r = resolveSpec(b, null, targetSet, new Set());
      if (r) return r;
    }
    return null;
  };
}

// metagen participation 流水线 (cpp_hier.py 复刻)
function participatingClasses(classes) {
  const byName = makeByName(classes);
  const resolve = makeResolver(byName);

  // 1. heap 集: 基类链可达 HeapObject
  const heap = new Map();
  for (const c of byName.values()) {
    if (c.name === "HeapObject") { heap.set(c.name, c); continue; }
    if (resolve(c.name, new Set(["HeapObject"])) === "HeapObject") heap.set(c.name, c);
  }
  // 2. candidates: 剔除 NO_AUTO_CHECKER 且无 OWN_TYPE
  const candidates = new Map();
  for (const [n, c] of heap) {
    if (c.anns.includes("NO_AUTO_CHECKER") && !c.anns.includes("OWN_TYPE")) continue;
    candidates.set(n, c);
  }
  // 3. non_leaves: 被某 candidate 选为最近基类
  const candSet = new Set(candidates.keys());
  const nonLeaves = new Set();
  for (const n of candSet) {
    const p = resolve(n, candSet);
    if (p !== null && p !== "HeapObject") nonLeaves.add(p);
  }
  // 4. participating: 叶子 或 带任意 V8_IT 注解
  const participating = new Map();
  for (const [n, c] of candidates) {
    if (!nonLeaves.has(n) || c.anns.length > 0) participating.set(n, c);
  }
  // 5. base 对 participating 集解析 (跳过未参与中间类)
  const partSet = new Set(participating.keys());
  const out = [];
  for (const [n, c] of participating) out.push({ ...c, base: resolve(n, partSet) });
  return { participating: out, heapCount: heap.size, candCount: candidates.size };
}

// ---------------------------------------------------------------------------
// metagen instance_types.py 求解器复刻 (即 Torque instance-type-generator.cc)
// ---------------------------------------------------------------------------
const INT_MAX = 2 ** 31 - 1, INT_MIN = -(2 ** 31);

function parseAnns(c) {
  let value = -1, num_flags_bits = -1, abstract = false, reuse = false, high = false, low = false;
  for (const a of c.anns) {
    if (a === "ABSTRACT") abstract = true;
    else if (a === "REUSE_PARENT") reuse = true;
    else if (a.startsWith("FIXED_VALUE")) value = Number(a.slice(a.indexOf("(") + 1, -1));
    else if (a.startsWith("FLAG_BITS")) num_flags_bits = Number(a.slice(a.indexOf("(") + 1, -1));
    else if (a === "ORDER(FIRST)") low = true;
    else if (a === "ORDER(LAST)") high = true;
  }
  return { value, num_flags_bits, abstract, reuse, high, low };
}

function solveInstanceTypes(list) {
  const byName = new Map();
  for (const c of list) if (!byName.has(c.name)) byName.set(c.name, c);
  const mk = (c) => ({ cls: c, anns: parseAnns(c), children: [], start: INT_MAX, end: INT_MIN, value: -1, num_values: 0, num_own_values: 0 });
  const nodes = new Map();
  for (const c of list) if (!nodes.has(c.name)) nodes.set(c.name, mk(c));
  let root = null;
  const extraRoots = [];
  for (const n of nodes.values()) {
    const parent = n.cls.base ? nodes.get(n.cls.base) : null;
    if (!parent) { if (root) extraRoots.push(n.cls.name); root = n; }
    else parent.children.push(n);
  }
  if (extraRoots.length) throw new Error(`多根: ${root?.cls.name} + ${extraRoots.join(",")}`);

  // propagate_constraints (后序)
  const propagate = (n) => {
    for (const ch of n.children) {
      propagate(ch);
      if (ch.start < n.start) n.start = ch.start;
      if (ch.end > n.end) n.end = ch.end;
      n.num_values += ch.num_values;
    }
    if (!n.anns.abstract && !n.anns.reuse) n.num_own_values = 1;
    n.num_values += n.num_own_values;
    if (n.anns.num_flags_bits !== -1) {
      n.children.length = 0;
      n.num_values = 1 << n.anns.num_flags_bits;
      n.num_own_values = n.num_values;
      n.start = 0;
      n.end = n.num_values - 1;
    }
    if (n.anns.value !== -1) {
      n.value = n.anns.value;
      if (n.value < n.start) n.start = n.value;
      if (n.value > n.end) n.end = n.value;
    }
  };
  propagate(root);

  const selectOwn = (n, start) => {
    if (n.value === -1) n.value = start;
    else if (n.value < start) throw new Error(`无法赋实例类型 ${n.value} 给 ${n.cls.name} (start=${start}) ${n.cls.file}`);
    return n.value + n.num_own_values;
  };
  // CompareUnconstrainedTypes: num_values 降序, 名字升序
  const uncKey = (n) => [-n.num_values, n.cls.name];

  const solve = (n, start, dest) => {
    if (n.start < start) throw new Error(`start ${n.start} < ${start} @ ${n.cls.name} ${n.cls.file}`);
    let lowest = null, highest = null;
    const constrained = [], unconstrained = [];
    for (const ch of n.children) {
      if (ch.anns.high) {
        if (highest) throw new Error(`双 highest: ${n.cls.name}`);
        highest = ch;
      } else if (ch.anns.low) {
        if (lowest) throw new Error(`双 lowest: ${n.cls.name}`);
        lowest = ch;
      } else if (ch.start > ch.end) unconstrained.push(ch);
      else constrained.push(ch);
    }
    n.children = [];
    constrained.sort((a, b) => a.start - b.start);
    unconstrained.sort((a, b) => { const ka = uncKey(a), kb = uncKey(b); return ka[0] - kb[0] || (ka[1] < kb[1] ? -1 : ka[1] > kb[1] ? 1 : 0); });
    let ownPending = n.num_own_values > 0;
    if (lowest) start = solve(lowest, start, n.children);
    for (const con of constrained) {
      if (ownPending) {
        const fits = (n.value !== -1 && n.value < con.start) ||
          (n.value === -1 && start + n.num_own_values <= con.start);
        if (fits) { start = selectOwn(n, start); ownPending = false; }
      }
      const rest = [];
      for (const uc of unconstrained) {
        if (uc.num_values + start <= con.start) start = solve(uc, start, n.children);
        else rest.push(uc);
      }
      unconstrained.length = 0;
      unconstrained.push(...rest);
      start = solve(con, start, n.children);
    }
    if (ownPending) { start = selectOwn(n, start); ownPending = false; }
    for (const uc of unconstrained) start = solve(uc, start, n.children);
    if (highest) start = solve(highest, start, n.children);
    n.end = start - 1;
    n.start = n.children.length ? n.children[0].start : start;
    if (n.value !== -1 && n.value < n.start) n.start = n.value;
    n.num_values = n.end - n.start + 1;
    if (n.num_values > 0) dest.push(n);
    return start;
  };
  const dest = [];
  solve(root, 0, dest);
  return { root: dest[0], participating: list.length };
}

// ---------------------------------------------------------------------------
// 发射辅助
// ---------------------------------------------------------------------------
// CapifyStringWithUnderscores 复刻 (src/torque/utils.cc)
function capify(s) {
  const res = [];
  const jsPos = s.indexOf("JS");
  let prevLower = false;
  for (let i = 0; i < s.length; i++) {
    const c = s[i];
    if ((prevLower && c >= "A" && c <= "Z") || (jsPos !== -1 && i === jsPos + 2)) res.push("_");
    res.push(c.toUpperCase());
    prevLower = (c >= "a" && c <= "z") || (c >= "0" && c <= "9");
  }
  return res.join("");
}
const typeName = (clsName) => capify(clsName) + "_TYPE";
// Rust 枚举变体名: INTERNALIZED_TWO_BYTE_STRING_TYPE → InternalizedTwoByteString
function variantName(ty) {
  return ty.replace(/_TYPE$/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => (p ? c.toUpperCase() : c.toUpperCase()))
    .replace(/^Js([A-Z])/, "JS$1");
}

// 收集求解树: concrete 值表 + 全部 FIRST_/LAST_ 标记
function collect(solvedRoot) {
  const concrete = new Map(); // TYPE 常量名 → 值 (own value)
  const markers = []; // {name, value} FIRST_/LAST_
  const order = []; // 求解树前序 concrete 名 (保证 enum 顺序 = 层级顺序)
  const walkTree = (n) => {
    const tn = typeName(n.cls.name);
    markers.push({ name: `FIRST_${tn}`, value: n.start });
    if (n.num_own_values === 1) {
      concrete.set(tn, n.value);
      order.push(tn);
    }
    for (const ch of n.children) walkTree(ch);
    if (n.num_own_values <= 1) markers.push({ name: `LAST_${tn}`, value: n.end });
  };
  walkTree(solvedRoot);
  return { concrete, markers, order };
}

// ---------------------------------------------------------------------------
// 运行: 采集 → 求解 → 锚点断言
// ---------------------------------------------------------------------------
const classes = harvestClasses();
const { participating, heapCount, candCount } = participatingClasses(classes);
const { root: solved } = solveInstanceTypes(participating);
const { concrete, markers, order } = collect(solved);

// ABI 锚点 (include/v8-internal.h Internals, 锁定值)
const ANCHORS = {
  SYMBOL_TYPE: 0x80, // kFirstNonstringType
  ODDBALL_TYPE: 0x83,
  FOREIGN_TYPE: 0xcc,
  JS_SPECIAL_API_OBJECT_TYPE: 0x410,
  JS_OBJECT_TYPE: 0x421,
};
for (const [k, v] of Object.entries(ANCHORS)) {
  if (concrete.get(k) !== v) throw new Error(`锚点失配 ${k}: 期望 0x${v.toString(16)} 实得 ${concrete.get(k)}`);
}
const firstApi = markers.find((m) => m.name === "FIRST_JS_API_OBJECT_TYPE").value;
const lastApi = markers.find((m) => m.name === "LAST_JS_API_OBJECT_TYPE").value;
if (firstApi !== 0x422 || lastApi !== 0x80a) throw new Error(`JS API 区间失配: ${firstApi.toString(16)}-${lastApi.toString(16)}`);
const lastType = markers.find((m) => m.name === "LAST_HEAP_OBJECT_TYPE").value;
if (lastType >= 1 << 15) throw new Error(`LAST_TYPE 越界: ${lastType}`);
// 字符串类型全部 < FIRST_NONSTRING_TYPE; 非字符串全部 >= (STRING_TYPE_LIST 断言复刻)
for (const [ty, v] of concrete) {
  if (ty.endsWith("STRING_TYPE") && !ty.startsWith("FIRST") && v >= 0x80) throw new Error(`字符串类型越界 ${ty}=0x${v.toString(16)}`);
}
// v2 修复漏采回归锚: 数组/哈希字典族必须存在 (v1 整族缺失)
for (const ty of ["FIXED_ARRAY_TYPE", "WEAK_ARRAY_LIST_TYPE", "BYTE_ARRAY_TYPE", "HASH_TABLE_TYPE",
  "NAME_DICTIONARY_TYPE", "NUMBER_DICTIONARY_TYPE", "SWISS_NAME_DICTIONARY_TYPE", "TRANSITION_ARRAY_TYPE",
  "ORDERED_HASH_SET_TYPE", "TRUSTED_FIXED_ARRAY_TYPE"]) {
  if (!concrete.has(ty)) throw new Error(`漏采回归: ${ty} 缺失`);
}
// String FLAG_BITS(7) 子树独占 0..127: FIRST_STRING_TYPE=0 且 SYMBOL_TYPE=0x80 (锚点已断言)
const firstString = markers.find((m) => m.name === "FIRST_STRING_TYPE").value;
if (firstString !== 0) throw new Error(`FIRST_STRING_TYPE 失配: ${firstString}`);

// ---------------------------------------------------------------------------
// 字符串 tag 位域 (instance-type.h 手写段, 逐常量提取)
// ---------------------------------------------------------------------------
const S = {
  K_IS_NOT_STRING_MASK: 0xffffff80,
  K_STRING_TAG: 0x0,
  K_STRING_REPRESENTATION_MASK: 0x7,
  K_SEQ_STRING_TAG: 0x0,
  K_CONS_STRING_TAG: 0x1,
  K_EXTERNAL_STRING_TAG: 0x2,
  K_SLICED_STRING_TAG: 0x3,
  K_THIN_STRING_TAG: 0x5,
  K_IS_INDIRECT_STRING_MASK: 0x1,
  K_IS_INDIRECT_STRING_TAG: 0x1,
  K_THIN_STRING_TAG_BIT: 0x4,
  K_STRING_ENCODING_MASK: 0x8,
  K_TWO_BYTE_STRING_TAG: 0x0,
  K_ONE_BYTE_STRING_TAG: 0x8,
  K_UNCACHED_EXTERNAL_STRING_MASK: 0x10,
  K_UNCACHED_EXTERNAL_STRING_TAG: 0x10,
  K_SHARED_STRING_MASK: 0x20,
  K_SHARED_STRING_TAG: 0x20,
  K_IS_NOT_INTERNALIZED_MASK: 0x40,
  K_NOT_INTERNALIZED_TAG: 0x40,
  K_INTERNALIZED_TAG: 0x0,
};
// 组合 tag (与原版 enum InstanceType 字符串段表达式一致)
const TWO = S.K_TWO_BYTE_STRING_TAG, ONE = S.K_ONE_BYTE_STRING_TAG;
const INT2 = TWO | S.K_SEQ_STRING_TAG | S.K_INTERNALIZED_TAG;
const INT1 = ONE | S.K_SEQ_STRING_TAG | S.K_INTERNALIZED_TAG;
const E_INT2 = TWO | S.K_EXTERNAL_STRING_TAG | S.K_INTERNALIZED_TAG;
const E_INT1 = ONE | S.K_EXTERNAL_STRING_TAG | S.K_INTERNALIZED_TAG;
const U_INT2 = E_INT2 | S.K_UNCACHED_EXTERNAL_STRING_TAG;
const U_INT1 = E_INT1 | S.K_UNCACHED_EXTERNAL_STRING_TAG;
const STRING_TYPES = [
  ["INTERNALIZED_TWO_BYTE_STRING_TYPE", INT2],
  ["INTERNALIZED_ONE_BYTE_STRING_TYPE", INT1],
  ["EXTERNAL_INTERNALIZED_TWO_BYTE_STRING_TYPE", E_INT2],
  ["EXTERNAL_INTERNALIZED_ONE_BYTE_STRING_TYPE", E_INT1],
  ["UNCACHED_EXTERNAL_INTERNALIZED_TWO_BYTE_STRING_TYPE", U_INT2],
  ["UNCACHED_EXTERNAL_INTERNALIZED_ONE_BYTE_STRING_TYPE", U_INT1],
  ["SEQ_TWO_BYTE_STRING_TYPE", INT2 | S.K_NOT_INTERNALIZED_TAG],
  ["SEQ_ONE_BYTE_STRING_TYPE", INT1 | S.K_NOT_INTERNALIZED_TAG],
  ["CONS_TWO_BYTE_STRING_TYPE", TWO | S.K_CONS_STRING_TAG | S.K_NOT_INTERNALIZED_TAG],
  ["CONS_ONE_BYTE_STRING_TYPE", ONE | S.K_CONS_STRING_TAG | S.K_NOT_INTERNALIZED_TAG],
  ["SLICED_TWO_BYTE_STRING_TYPE", TWO | S.K_SLICED_STRING_TAG | S.K_NOT_INTERNALIZED_TAG],
  ["SLICED_ONE_BYTE_STRING_TYPE", ONE | S.K_SLICED_STRING_TAG | S.K_NOT_INTERNALIZED_TAG],
  ["EXTERNAL_TWO_BYTE_STRING_TYPE", E_INT2 | S.K_NOT_INTERNALIZED_TAG],
  ["EXTERNAL_ONE_BYTE_STRING_TYPE", E_INT1 | S.K_NOT_INTERNALIZED_TAG],
  ["UNCACHED_EXTERNAL_TWO_BYTE_STRING_TYPE", U_INT2 | S.K_NOT_INTERNALIZED_TAG],
  ["UNCACHED_EXTERNAL_ONE_BYTE_STRING_TYPE", U_INT1 | S.K_NOT_INTERNALIZED_TAG],
  ["THIN_TWO_BYTE_STRING_TYPE", TWO | S.K_THIN_STRING_TAG | S.K_NOT_INTERNALIZED_TAG],
  ["THIN_ONE_BYTE_STRING_TYPE", ONE | S.K_THIN_STRING_TAG | S.K_NOT_INTERNALIZED_TAG],
  ["SHARED_SEQ_TWO_BYTE_STRING_TYPE", (INT2 | S.K_NOT_INTERNALIZED_TAG) | S.K_SHARED_STRING_TAG],
  ["SHARED_SEQ_ONE_BYTE_STRING_TYPE", (INT1 | S.K_NOT_INTERNALIZED_TAG) | S.K_SHARED_STRING_TAG],
  ["SHARED_EXTERNAL_TWO_BYTE_STRING_TYPE", (E_INT2 | S.K_NOT_INTERNALIZED_TAG) | S.K_SHARED_STRING_TAG],
  ["SHARED_EXTERNAL_ONE_BYTE_STRING_TYPE", (E_INT1 | S.K_NOT_INTERNALIZED_TAG) | S.K_SHARED_STRING_TAG],
  ["SHARED_UNCACHED_EXTERNAL_TWO_BYTE_STRING_TYPE", (U_INT2 | S.K_NOT_INTERNALIZED_TAG) | S.K_SHARED_STRING_TAG],
  ["SHARED_UNCACHED_EXTERNAL_ONE_BYTE_STRING_TYPE", (U_INT1 | S.K_NOT_INTERNALIZED_TAG) | S.K_SHARED_STRING_TAG],
];
// 字符串类型 24 个, 互异, 全部 < 0x80
if (STRING_TYPES.length !== 24) throw new Error(`字符串类型数 ${STRING_TYPES.length} != 24`);
if (new Set(STRING_TYPES.map(([, v]) => v)).size !== 24) throw new Error("字符串类型值重复");

// ---------------------------------------------------------------------------
// Rust 发射
// ---------------------------------------------------------------------------
const allVariants = [
  ...STRING_TYPES,
  ...order.filter((ty) => !ty.startsWith("FIRST_") && !ty.startsWith("LAST_")).map((ty) => [ty, concrete.get(ty)]),
];
// 变体名唯一性 + 判别值唯一性
const vn = allVariants.map(([ty]) => variantName(ty));
if (new Set(vn).size !== vn.length) {
  const dup = vn.filter((v, i) => vn.indexOf(v) !== i);
  throw new Error(`变体名重复: ${dup.join(",")}`);
}
if (new Set(allVariants.map(([, v]) => v)).size !== allVariants.length) throw new Error("判别值重复");

// 伪类型别名所需求解值 (instance-type.h:194-219 的表达式在此落值)
const firstJsFunc = markers.find((m) => m.name === "FIRST_JS_FUNCTION_TYPE").value;
const classCtor = concrete.get("JS_CLASS_CONSTRUCTOR_TYPE");
const lastSpecial = markers.find((m) => m.name === "LAST_JS_SPECIAL_OBJECT_TYPE").value;
const lastCustom = markers.find((m) => m.name === "LAST_JS_CUSTOM_ELEMENTS_OBJECT_TYPE").value;
const firstHeap = markers.find((m) => m.name === "FIRST_HEAP_OBJECT_TYPE").value;
const lastHeap = markers.find((m) => m.name === "LAST_HEAP_OBJECT_TYPE").value;

// Rust 代码生成
const variantLines = allVariants
  .map(([ty, v]) => `  ${variantName(ty)} = ${v},`)
  .join("\n");

const fromRawArms = allVariants
  .map(([ty, v]) => `      ${v} => Ok(Self::${variantName(ty)}),`)
  .join("\n");

// FIRST_/LAST_ marker 常量 (Rust 枚举禁重复判别值 → const)
const markerLines = markers
  .map((m) => `pub const ${m.name}: u16 = ${m.value};`)
  .join("\n");

// 伪类型别名: 值反查变体 (空洞值无变体 → u16 常量)
const byValue = new Map(allVariants.map(([ty, v]) => [v, variantName(ty)]));
const stringMap = new Map(STRING_TYPES);
const firstNonstring = concrete.get("SYMBOL_TYPE");
const alias = (name, value, comment) => {
  const v = byValue.get(value);
  if (v) return `/// 伪类型: ${comment}。\npub const ${name}: InstanceType = InstanceType::${v};`;
  return `/// 伪类型: ${comment}。该值是字符串段空洞, 无对应变体 → 以 u16 落值。\npub const ${name}: u16 = ${value};`;
};
const aliasLines = [
  alias("FIRST_UNIQUE_NAME_TYPE", stringMap.get("INTERNALIZED_TWO_BYTE_STRING_TYPE") ?? 0, "首个 unique name (内部化字符串)"),
  alias("LAST_UNIQUE_NAME_TYPE", concrete.get("SYMBOL_TYPE"), "末个 unique name (Symbol)"),
  alias("FIRST_NONSTRING_TYPE", firstNonstring, "非字符串类型首值 (kFirstNonstringType=0x80)"),
  alias("LAST_STRING_TYPE", firstNonstring - 1, "字符串类型上界 (字符串类型稀疏)"),
  alias("JS_FUNCTION_TYPE", concrete.get("JS_FUNCTION_WITH_PROTOTYPE_TYPE"), "JS_FUNCTION_WITH_PROTOTYPE_TYPE 便捷别名"),
  alias("FIRST_CALLABLE_JS_FUNCTION_TYPE", firstJsFunc, "可调用 JS 函数首值 (FIRST_JS_FUNCTION_TYPE)"),
  alias("LAST_CALLABLE_JS_FUNCTION_TYPE", classCtor - 1, "可调用 JS 函数末值 (JS_CLASS_CONSTRUCTOR_TYPE - 1)"),
  alias("LAST_SPECIAL_RECEIVER_TYPE", lastSpecial, "需特殊属性查找的 JSReceiver 上界 (LAST_JS_SPECIAL_OBJECT_TYPE)"),
  alias("LAST_CUSTOM_ELEMENTS_RECEIVER", lastCustom, "可能有 elements 的自定义 JSReceiver 上界 (LAST_JS_CUSTOM_ELEMENTS_OBJECT_TYPE)"),
  alias("FIRST_TYPE", firstHeap, "FIRST_HEAP_OBJECT_TYPE 别名"),
  alias("LAST_TYPE", lastHeap, "LAST_HEAP_OBJECT_TYPE 别名"),
  alias("BIGINT_TYPE", concrete.get("BIG_INT_BASE_TYPE"), "BIG_INT_BASE_TYPE 别名"),
  alias("DEPENDENT_CODE_TYPE", concrete.get("WEAK_ARRAY_LIST_TYPE"), "DEPENDENT_CODE 借用 WEAK_ARRAY_LIST_TYPE"),
].join("\n");

const body = `${banner("src/objects/instance-type.h.js", "src/objects/instance-type.h + instance-types-gen.h (metagen) + tools/metagen/instance_types.py", `X-macro 全量展开 (v2 采集器: 模板基类/participation 对齐 cpp_hier); 参与类 ${participating}, concrete ${concrete.size}, 变体 ${allVariants.length}; WASM/INTL 开, TEMPORAL 关`)}
//! 实例类型 (InstanceType): 16 位堆对象类型编号。
//!
//! 编号由 metagen 求解器 (tools/metagen/instance_types.py, 即 Torque
//! instance-type-generator.cc 的移植) 从头文件 V8_IT_* 注解求解得出,
//! ABI 锚点 (v8-internal.h: kJSObjectType 等) 生成期已断言全部命中。
//! 字符串类型 (bit7=0) 硬编码段在前, 其余按继承层级区间排布。

use crate::error::{Error, Result};

// ── 字符串 tag 位域 (instance-type.h:25-114) ───────────────────────────────
/// bit7-15 任一置位 → 非字符串。
pub const K_IS_NOT_STRING_MASK: u32 = 0x${S.K_IS_NOT_STRING_MASK.toString(16)};
/// bit7=0 → 字符串。
pub const K_STRING_TAG: u32 = 0x${S.K_STRING_TAG.toString(16)};
/// bit0-2: 字符串表示。
pub const K_STRING_REPRESENTATION_MASK: u32 = 0x${S.K_STRING_REPRESENTATION_MASK.toString(16)};
/// 顺序字符串。
pub const K_SEQ_STRING_TAG: u32 = 0x${S.K_SEQ_STRING_TAG.toString(16)};
/// cons 字符串。
pub const K_CONS_STRING_TAG: u32 = 0x${S.K_CONS_STRING_TAG.toString(16)};
/// 外部字符串。
pub const K_EXTERNAL_STRING_TAG: u32 = 0x${S.K_EXTERNAL_STRING_TAG.toString(16)};
/// 切片字符串。
pub const K_SLICED_STRING_TAG: u32 = 0x${S.K_SLICED_STRING_TAG.toString(16)};
/// thin 字符串。
pub const K_THIN_STRING_TAG: u32 = 0x${S.K_THIN_STRING_TAG.toString(16)};
/// bit0: 间接字符串 (cons/sliced/thin)。
pub const K_IS_INDIRECT_STRING_MASK: u32 = 0x${S.K_IS_INDIRECT_STRING_MASK.toString(16)};
/// bit0=1 → 间接字符串。
pub const K_IS_INDIRECT_STRING_TAG: u32 = 0x${S.K_IS_INDIRECT_STRING_TAG.toString(16)};
/// bit2: thin 字符串专有位。
pub const K_THIN_STRING_TAG_BIT: u32 = 0x${S.K_THIN_STRING_TAG_BIT.toString(16)};
/// bit3: 编码 (0=双字节, 1=单字节)。
pub const K_STRING_ENCODING_MASK: u32 = 0x${S.K_STRING_ENCODING_MASK.toString(16)};
/// 双字节编码 tag。
pub const K_TWO_BYTE_STRING_TAG: u32 = 0x${S.K_TWO_BYTE_STRING_TAG.toString(16)};
/// 单字节编码 tag。
pub const K_ONE_BYTE_STRING_TAG: u32 = 0x${S.K_ONE_BYTE_STRING_TAG.toString(16)};
/// 表示 + 编码合并掩码。
pub const K_STRING_REPRESENTATION_AND_ENCODING_MASK: u32 = K_STRING_REPRESENTATION_MASK | K_STRING_ENCODING_MASK;
/// 顺序单字节。
pub const K_SEQ_ONE_BYTE_STRING_TAG: u32 = K_SEQ_STRING_TAG | K_ONE_BYTE_STRING_TAG;
/// 顺序双字节。
pub const K_SEQ_TWO_BYTE_STRING_TAG: u32 = K_SEQ_STRING_TAG | K_TWO_BYTE_STRING_TAG;
/// 外部单字节。
pub const K_EXTERNAL_ONE_BYTE_STRING_TAG: u32 = K_EXTERNAL_STRING_TAG | K_ONE_BYTE_STRING_TAG;
/// 外部双字节。
pub const K_EXTERNAL_TWO_BYTE_STRING_TAG: u32 = K_EXTERNAL_STRING_TAG | K_TWO_BYTE_STRING_TAG;
/// bit4: 外部字符串数据指针未缓存。
pub const K_UNCACHED_EXTERNAL_STRING_MASK: u32 = 0x${S.K_UNCACHED_EXTERNAL_STRING_MASK.toString(16)};
/// 未缓存 tag。
pub const K_UNCACHED_EXTERNAL_STRING_TAG: u32 = 0x${S.K_UNCACHED_EXTERNAL_STRING_TAG.toString(16)};
/// bit5: 共享字符串。
pub const K_SHARED_STRING_MASK: u32 = 0x${S.K_SHARED_STRING_MASK.toString(16)};
/// 共享 tag。
pub const K_SHARED_STRING_TAG: u32 = 0x${S.K_SHARED_STRING_TAG.toString(16)};
/// bit6: 未内部化。
pub const K_IS_NOT_INTERNALIZED_MASK: u32 = 0x${S.K_IS_NOT_INTERNALIZED_MASK.toString(16)};
/// 未内部化 tag。
pub const K_NOT_INTERNALIZED_TAG: u32 = 0x${S.K_NOT_INTERNALIZED_TAG.toString(16)};
/// 内部化 tag。
pub const K_INTERNALIZED_TAG: u32 = 0x${S.K_INTERNALIZED_TAG.toString(16)};
/// 表示+编码+共享合并掩码。
pub const K_STRING_REPRESENTATION_ENCODING_AND_SHARED_MASK: u32 =
  K_STRING_REPRESENTATION_AND_ENCODING_MASK | K_SHARED_STRING_MASK;
/// cons 快捷候选掩码 (含非字符串位与非内部化位)。
pub const K_SHORTCUT_TYPE_MASK: u32 =
  K_IS_NOT_STRING_MASK | K_IS_NOT_INTERNALIZED_MASK | K_STRING_REPRESENTATION_MASK;
/// cons 快捷候选 tag。
pub const K_SHORTCUT_TYPE_TAG: u32 = K_CONS_STRING_TAG | K_NOT_INTERNALIZED_TAG;

/// InstanceType 全集 (字符串硬编码段 + 求解段, 判别值 1:1)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum InstanceType {
${variantLines}
}

impl InstanceType {
  /// 原始 16 位值。
  #[inline]
  #[must_use]
  pub const fn raw(self) -> u16 {
    self as u16
  }

  /// 原始值 → 枚举; 未知值 (含 FIRST_/LAST_ 区间空洞) 落 \`Err\`。
  #[inline]
  pub const fn from_raw(raw: u16) -> Result<Self> {
    match raw {
${fromRawArms}
      _ => Err(Error::InvalidInstanceType(raw)),
    }
  }

  /// 是否字符串类型 (bit7=0)。
  #[inline]
  #[must_use]
  pub const fn is_string(self) -> bool {
    (self as u16 as u32 & K_IS_NOT_STRING_MASK) == K_STRING_TAG
  }

  /// 是否内部化字符串 (字符串且 bit6=0)。
  #[inline]
  #[must_use]
  pub const fn is_internalized(self) -> bool {
    let v = self as u16 as u32;
    (v & K_IS_NOT_STRING_MASK) == K_STRING_TAG && (v & K_IS_NOT_INTERNALIZED_MASK) == K_INTERNALIZED_TAG
  }
}

// ── FIRST_/LAST_ 区间标记 (TORQUE_ASSIGNED_INSTANCE_TYPES 的区间位, u16) ───
${markerLines}

// ── 伪类型别名 (instance-type.h:194-219; 原版为枚举别名, Rust 枚举禁重复判别值 → const) ──
${aliasLines}

/// PROPERTY_DICTIONARY_TYPE: 目标配置字典类型。
/// v8_enable_swiss_name_dictionary 默认 false (BUILD.gn:425) → NAME_DICTIONARY_TYPE。
pub const PROPERTY_DICTIONARY_TYPE: InstanceType = InstanceType::${variantName("NAME_DICTIONARY_TYPE")};

/// IsShortcutCandidate: cons 且未内部化 (可被 GC 快捷)。
#[inline]
#[must_use]
pub const fn is_shortcut_candidate(ty: u16) -> bool {
  (ty as u32 & K_SHORTCUT_TYPE_MASK) == K_SHORTCUT_TYPE_TAG
}

// ── 对数登记 (生成期断言的运行期复述, tests 复核) ──────────────────────────
/// 采集候选类数 (heap 可达集)。
pub const HEAP_REACHABLE_CLASSES: usize = ${heapCount};
/// participation 过滤后的候选类数。
pub const CANDIDATE_CLASSES: usize = ${candCount};
/// 求解参与类数 (cpp_hier participating 集)。
pub const PARTICIPATING_CLASSES: usize = ${participating.length};
/// concrete 实例类型数 (TORQUE_ASSIGNED_INSTANCE_TYPE_LIST 行数)。
pub const CONCRETE_TYPE_COUNT: usize = ${concrete.size};
/// 字符串硬编码类型数 (INSTANCE_TYPE_LIST_BASE 行数)。
pub const STRING_TYPE_COUNT: usize = ${STRING_TYPES.length};
/// 枚举变体总数 (字符串 ${STRING_TYPES.length} + concrete ${concrete.size})。
pub const VARIANT_COUNT: usize = ${allVariants.length};
/// LAST_TYPE 值 (须 < 2^15, Torque 有符号处理约束)。
pub const LAST_TYPE_VALUE: u16 = ${lastHeap};
`;

writeFileSync("/tmp/script_concrete.txt", [...concrete.entries()].sort((a,b)=>a[1]-b[1]).map(([k,v])=>`${v}	${k}`).join("\n"));
writeSrc(CRATE, "instance_type.rs", body);
console.log(`instance_type.rs: 采集 ${heapCount} 可达 / ${candCount} 候选 / ${participating.length} 参与, concrete ${concrete.size} + 字符串 24 = ${allVariants.length} 变体, LAST_TYPE=${lastType} (0x${lastType.toString(16)}), 锚点全中`);
