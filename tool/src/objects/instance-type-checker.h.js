#!/usr/bin/env bun
// tool/src/objects/instance-type-checker.h.js — 覆盖 v8/src/objects/instance-type-checker.h
//        + instance-type-inl.h (非 static-roots 段: 355-712 行)
//        → v8-objects-core/src/instance_type_checker.rs
// 映射登记:
//   - INSTANCE_TYPE_CHECKERS_SINGLE/RANGE 桶由 metagen 求解树机械展开
//     (SINGLE: num_own_values==1 && num_values==1 && !no_auto_checker → 精确比较;
//      RANGE: num_values>1 && !is_root → FIRST_/LAST_ 区间比较;
//      MULTIPLE 桶不产生 Is 检查器, 原版 INSTANCE_TYPE_CHECKERS 不含它)
//   - INSTANCE_TYPE_CHECKERS_CUSTOM + instance-type-inl.h 手写检查器 → 脚本模板段
//   - 追加: V(BigInt, BIGINT_TYPE) + V(FixedArrayExact, FIXED_ARRAY_TYPE) (checker.h:21-24)
// 跳过区段登记 (推迟):
//   - V8_STATIC_ROOTS_BOOL 快路径 (kNonJsReceiverMapLimit/CheckInstanceMap/
//     kUniqueMapRangeOfInstanceTypeRangeList/kStringMapEncodingMask): 依赖
//     static-roots 常量表, 归 L3 v8-roots / v8-objects-strings
//   - UniqueMapOfInstanceTypeCheck/UniqueMapOfInstanceType + Map::TryGetMapRootIdxFor:
//     依赖 RootIndex 枚举 (src/roots/roots.h → v8-roots crate), L3 后补
//   - IsXxx(Tagged<Map>) 重载的 static-roots 路径; 非 static-roots 路径等价于
//     instance_type 检查, 由 objects.rs 的 Cast 体系消费本模块函数
//   - IsSharedString 的 v8_flags.shared_string_table 分支 (需 v8-flags 集成, 登记)
import { banner, writeSrc } from "../../lib.js";
import { readFileSync, readdirSync } from "node:fs";
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
// 桶发射 (metagen instance_types.py _emit_buckets_walk 复刻)
// ---------------------------------------------------------------------------
function emitBuckets(solvedRoot) {
  const single = [], multiple = [], range = [];
  const walk = (n, isRoot) => {
    const tn = typeName(n.cls.name);
    const noChk = n.cls.anns.includes("NO_AUTO_CHECKER");
    if (n.num_own_values === 1 && !noChk) {
      const line = { cls: n.cls.name, ty: tn };
      if (n.num_values === 1) single.push(line);
      else multiple.push(line);
    }
    for (const ch of n.children) walk(ch, false);
    if (n.num_values > 1 && !noChk && !isRoot) {
      range.push({ cls: n.cls.name, first: `FIRST_${tn}`, last: `LAST_${tn}` });
    }
  };
  walk(solvedRoot, true);
  return { single, multiple, range };
}

// ---------------------------------------------------------------------------
// 运行: 采集 → participation → 求解 → 桶
// ---------------------------------------------------------------------------
const classes = harvestClasses();
const { participating } = participatingClasses(classes);
const { root: solved } = solveInstanceTypes(participating);
const { concrete, markers } = collect(solved);
const markerMap = new Map(markers.map((m) => [m.name, m.value]));
const { single, multiple, range } = emitBuckets(solved);

// INSTANCE_TYPE_CHECKERS_SINGLE 追加项 (instance-type-checker.h:21-24)
single.push({ cls: "BigInt", ty: "BIGINT_TYPE" });
single.push({ cls: "FixedArrayExact", ty: "FIXED_ARRAY_TYPE" });
// INSTANCE_TYPE_CHECKERS_RANGE 追加项 (instance-type-checker.h:26-29)
range.push({ cls: "CallableJSFunction", first: "FIRST_CALLABLE_JS_FUNCTION_TYPE", last: "LAST_CALLABLE_JS_FUNCTION_TYPE" });

// 变体名 (与 instance-type.h.js 的 variantName 一致)
function variantName(ty) {
  return ty.replace(/_TYPE$/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => (p ? c.toUpperCase() : c.toUpperCase()))
    .replace(/^Js([A-Z])/, "JS$1");
}
// ClassName → snake_case 函数名后缀 (IsJSArray → is_js_array)
function snake(cls) {
  return cls.replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2").toLowerCase();
}
// TYPE 常量 → instance_type.rs 中的路径
// concrete → InstanceType::变体; FIRST_/LAST_ 标记 → u16 常量; 别名 → InstanceType 常量
const INSTANCE_ALIASES = new Set([
  "FIRST_UNIQUE_NAME_TYPE", "LAST_UNIQUE_NAME_TYPE", "FIRST_NONSTRING_TYPE", "LAST_STRING_TYPE",
  "JS_FUNCTION_TYPE", "FIRST_CALLABLE_JS_FUNCTION_TYPE", "LAST_CALLABLE_JS_FUNCTION_TYPE",
  "LAST_SPECIAL_RECEIVER_TYPE", "LAST_CUSTOM_ELEMENTS_RECEIVER", "FIRST_TYPE", "LAST_TYPE",
  "BIGINT_TYPE", "DEPENDENT_CODE_TYPE", "PROPERTY_DICTIONARY_TYPE",
]);
const U16_ALIASES = new Set(["LAST_STRING_TYPE"]);
// 别名 → 数值 (instance-type.h 伪类型表达式在此落值)
function aliasValue(ty) {
  switch (ty) {
    case "FIRST_UNIQUE_NAME_TYPE": return 0;
    case "LAST_UNIQUE_NAME_TYPE": return concrete.get("SYMBOL_TYPE");
    case "FIRST_NONSTRING_TYPE": return concrete.get("SYMBOL_TYPE");
    case "LAST_STRING_TYPE": return concrete.get("SYMBOL_TYPE") - 1;
    case "JS_FUNCTION_TYPE": return concrete.get("JS_FUNCTION_WITH_PROTOTYPE_TYPE");
    case "FIRST_CALLABLE_JS_FUNCTION_TYPE": return markerMap.get("FIRST_JS_FUNCTION_TYPE");
    case "LAST_CALLABLE_JS_FUNCTION_TYPE": return concrete.get("JS_CLASS_CONSTRUCTOR_TYPE") - 1;
    case "LAST_SPECIAL_RECEIVER_TYPE": return markerMap.get("LAST_JS_SPECIAL_OBJECT_TYPE");
    case "LAST_CUSTOM_ELEMENTS_RECEIVER": return markerMap.get("LAST_JS_CUSTOM_ELEMENTS_OBJECT_TYPE");
    case "FIRST_TYPE": return markerMap.get("FIRST_HEAP_OBJECT_TYPE");
    case "LAST_TYPE": return markerMap.get("LAST_HEAP_OBJECT_TYPE");
    case "BIGINT_TYPE": return concrete.get("BIG_INT_BASE_TYPE");
    case "DEPENDENT_CODE_TYPE": return concrete.get("WEAK_ARRAY_LIST_TYPE");
    case "PROPERTY_DICTIONARY_TYPE": return concrete.get("NAME_DICTIONARY_TYPE");
    default: throw new Error(`未知别名 ${ty}`);
  }
}
function tyPath(ty) {
  if (concrete.has(ty)) return { expr: `InstanceType::${variantName(ty)}`, kind: "instance", value: concrete.get(ty) };
  if (INSTANCE_ALIASES.has(ty)) {
    return { expr: ty, kind: U16_ALIASES.has(ty) ? "u16" : "alias", value: aliasValue(ty) };
  }
  if (markerMap.has(ty)) return { expr: ty, kind: "u16", value: markerMap.get(ty) };
  throw new Error(`类型 ${ty} 无处解析 (既非 concrete/标记/别名)`);
}

// 单类型检查器发射 (INSTANCE_TYPE_CHECKER1 宏机械展开)
const singleLines = single.map(({ cls, ty }) => {
  const p = tyPath(ty);
  const cmp = `ty.raw() == ${p.value}`;
  return `/// 原版 InstanceTypeChecker::Is${cls} (== ${ty})。
#[inline]
#[must_use]
pub const fn is_${snake(cls)}(ty: InstanceType) -> bool {
  ${cmp}
}`;
});
// 区间检查器发射 (INSTANCE_TYPE_CHECKER_RANGE1 宏机械展开)
// 0 下界 / 65535 上界的比较冗余, 省略 (clippy 类型极限比较)
const rangeLines = range.map(({ cls, first, last }) => {
  const f = tyPath(first), l = tyPath(last);
  let cond;
  if (f.value === 0 && l.value === 65535) cond = "true";
  else if (f.value === 0) cond = `ty.raw() <= ${l.value}`;
  else if (l.value === 65535) cond = `ty.raw() >= ${f.value}`;
  else cond = `ty.raw() >= ${f.value} && ty.raw() <= ${l.value}`;
  return `/// 原版 InstanceTypeChecker::Is${cls} (${first}..=${last})。
#[inline]
#[must_use]
pub const fn is_${snake(cls)}(ty: InstanceType) -> bool {
  ${cond}
}`;
});

if (!singleLines.length || !rangeLines.length) throw new Error("桶为空");

// ---------------------------------------------------------------------------
// 自定义检查器 (instance-type-inl.h:355-712 非 static-roots 段, 1:1 语义模板)
// ---------------------------------------------------------------------------
const customBody = `// ── 自定义检查器 (INSTANCE_TYPE_CHECKERS_CUSTOM + instance-type-inl.h 手写段) ──
// IsSharedString 的 v8_flags.shared_string_table 分支推迟 (需 v8-flags 集成)。
#[inline]
#[must_use]
pub const fn is_internalized_string(ty: InstanceType) -> bool {
  ty.is_internalized()
}

/// 原版 IsSeqString: 表示位 == kSeqStringTag。
#[inline]
#[must_use]
pub const fn is_seq_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_MASK)) == K_SEQ_STRING_TAG
}

/// 原版 IsSeqOneByteString。
#[inline]
#[must_use]
pub const fn is_seq_one_byte_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_AND_ENCODING_MASK))
    == (K_STRING_TAG | K_SEQ_ONE_BYTE_STRING_TAG)
}

/// 原版 IsSeqTwoByteString。
#[inline]
#[must_use]
pub const fn is_seq_two_byte_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_AND_ENCODING_MASK))
    == (K_STRING_TAG | K_SEQ_TWO_BYTE_STRING_TAG)
}

/// 原版 IsExternalString。
#[inline]
#[must_use]
pub const fn is_external_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_MASK)) == K_EXTERNAL_STRING_TAG
}

/// 原版 IsExternalOneByteString。
#[inline]
#[must_use]
pub const fn is_external_one_byte_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_AND_ENCODING_MASK))
    == (K_STRING_TAG | K_EXTERNAL_ONE_BYTE_STRING_TAG)
}

/// 原版 IsExternalTwoByteString。
#[inline]
#[must_use]
pub const fn is_external_two_byte_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_AND_ENCODING_MASK))
    == (K_STRING_TAG | K_EXTERNAL_TWO_BYTE_STRING_TAG)
}

/// 原版 IsUncachedExternalString。
#[inline]
#[must_use]
pub const fn is_uncached_external_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_UNCACHED_EXTERNAL_STRING_MASK | K_STRING_REPRESENTATION_MASK))
    == (K_EXTERNAL_STRING_TAG | K_UNCACHED_EXTERNAL_STRING_TAG)
}

/// 原版 IsConsString。
#[inline]
#[must_use]
pub const fn is_cons_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_MASK)) == K_CONS_STRING_TAG
}

/// 原版 IsSlicedString。
#[inline]
#[must_use]
pub const fn is_sliced_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_MASK)) == K_SLICED_STRING_TAG
}

/// 原版 IsThinString。
#[inline]
#[must_use]
pub const fn is_thin_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_STRING_REPRESENTATION_MASK)) == K_THIN_STRING_TAG
}

/// 原版 IsOneByteString (编码位)。
#[inline]
#[must_use]
pub const fn is_one_byte_string(ty: InstanceType) -> bool {
  (ty.raw() as u32 & K_STRING_ENCODING_MASK) == K_ONE_BYTE_STRING_TAG
}

/// 原版 IsTwoByteString (编码位)。
#[inline]
#[must_use]
pub const fn is_two_byte_string(ty: InstanceType) -> bool {
  (ty.raw() as u32 & K_STRING_ENCODING_MASK) == K_TWO_BYTE_STRING_TAG
}

/// 原版 IsSharedString (shared_string_table 分支推迟)。
#[inline]
#[must_use]
pub const fn is_shared_string(ty: InstanceType) -> bool {
  (ty.raw() as u32 & K_SHARED_STRING_MASK) == K_SHARED_STRING_TAG
}

/// 原版 IsIndirectString (cons/sliced/thin)。
#[inline]
#[must_use]
pub const fn is_indirect_string(ty: InstanceType) -> bool {
  let v = ty.raw() as u32;
  (v & (K_IS_NOT_STRING_MASK | K_IS_INDIRECT_STRING_MASK)) == K_IS_INDIRECT_STRING_TAG
}

/// 原版 IsDirectString。
#[inline]
#[must_use]
pub const fn is_direct_string(ty: InstanceType) -> bool {
  !is_indirect_string(ty)
}

/// 原版 IsReferenceComparable: 引用可比 (String/BigInt/HeapNumber 除外)。
#[inline]
#[must_use]
pub const fn is_reference_comparable(ty: InstanceType) -> bool {
  !is_string(ty) && !is_big_int(ty) && !matches!(ty, InstanceType::HeapNumber)
}

/// 原版 IsGcSafeCode。
#[inline]
#[must_use]
pub const fn is_gc_safe_code(ty: InstanceType) -> bool {
  is_code(ty)
}

/// 原版 IsAbstractCode: BytecodeArray 或 Code。
#[inline]
#[must_use]
pub const fn is_abstract_code(ty: InstanceType) -> bool {
  is_bytecode_array(ty) || is_code(ty)
}

/// 原版 IsFreeSpaceOrFiller。
#[inline]
#[must_use]
pub const fn is_free_space_or_filler(ty: InstanceType) -> bool {
  matches!(ty, InstanceType::FreeSpace | InstanceType::Filler)
}

/// 原版 IsMaybeReadOnlyJSObject: 仅用于跨 API 传数据的包装对象。
#[inline]
#[must_use]
pub const fn is_maybe_read_only_js_object(ty: InstanceType) -> bool {
  is_js_external_object(ty) || is_js_message_object(ty)
}

/// 原版 IsPropertyDictionary。
#[inline]
#[must_use]
pub const fn is_property_dictionary(ty: InstanceType) -> bool {
  ty.raw() == PROPERTY_DICTIONARY_TYPE.raw()
}

/// 原版 IsNativeContextSpecific: 必须绑定某 native context 的堆对象类型。
#[inline]
#[must_use]
pub const fn is_native_context_specific(ty: InstanceType) -> bool {
  // 所有 context map 都绑定 native context。
  if is_context(ty) {
    return true;
  }
  // 非 JSReceiver 不绑定任何 native context。
  if !is_js_receiver(ty) {
    return false;
  }
  if is_maybe_read_only_js_object(ty) {
    // 跨 API 包装对象不绑定。
    return false;
  }
  if is_always_shared_space_js_object(ty) {
    // 共享空间 JSObject 不绑定。
    return false;
  }
  // Wasm struct/array 不绑定。
  !is_wasm_object(ty)
}

/// 原版 IsJSApiWrapperObject。
#[inline]
#[must_use]
pub const fn is_js_api_wrapper_object(ty: InstanceType) -> bool {
  is_jsapi_object_with_embedder_slots(ty) || is_js_special_object(ty)
}

/// 原版 IsCppHeapPointerWrapperObject。
#[inline]
#[must_use]
pub const fn is_cpp_heap_pointer_wrapper_object(ty: InstanceType) -> bool {
  is_js_api_wrapper_object(ty)
    || is_cpp_heap_external_object(ty)
    || is_cpp_gc_managed_base(ty)
    || is_embedder_data_array(ty)
}

/// 原版 IsSpecialReceiverInstanceType (objects-inl.h:916)。
#[inline]
#[must_use]
pub const fn is_special_receiver_instance_type(ty: InstanceType) -> bool {
  ty.raw() <= LAST_SPECIAL_RECEIVER_TYPE.raw()
}

/// 原版 IsCustomElementsReceiverInstanceType (objects-inl.h:929)。
#[inline]
#[must_use]
pub const fn is_custom_elements_receiver_instance_type(ty: InstanceType) -> bool {
  ty.raw() <= LAST_CUSTOM_ELEMENTS_RECEIVER.raw()
}`;

// ---------------------------------------------------------------------------
// Rust 发射
// ---------------------------------------------------------------------------
// 汇总 single/range/custom 引用的 instance_type 常量 → 生成导入表
const allLines = [...singleLines, ...rangeLines, customBody]
  .join("\n")
  .split("\n")
  .filter((l) => !l.trimStart().startsWith("///"))
  .join("\n");
const referenced = new Set();
for (const m of allLines.matchAll(/\b([A-Z][A-Z0-9_]{2,})\b/g)) {
  const n = m[1];
  if (n === "InstanceType" || n.startsWith("K_")) continue;
  if (markerMap.has(n) || INSTANCE_ALIASES.has(n)) referenced.add(n);
}
const importList = ["InstanceType", ...[...referenced].sort()].join(", ");
const importBlock = referenced.size
  ? `use crate::instance_type::{${importList}};`
  : "use crate::instance_type::InstanceType;";

const body = `${banner("src/objects/instance-type-checker.h.js", "src/objects/instance-type-checker.h + instance-type-inl.h", `桶机械展开: SINGLE ${single.length} (含追加 2) / RANGE ${range.length} (含追加 1); 自定义检查器 1:1; static-roots 快路径与 RootIndex 依赖推迟`)}
//! 实例类型检查器 (InstanceTypeChecker): 对象类型判定体系。
//!
//! SINGLE 桶 = 精确比较, RANGE 桶 = FIRST_/LAST_ 区间比较,
//! 均由 metagen 求解树机械展开; 自定义检查器为字符串位域与组合判定。
//! 对 Tagged<Map> 的判定经 objects::Cast 体系消费本模块 (等价非 static-roots 路径)。

${importBlock}
use crate::instance_type::{
  K_CONS_STRING_TAG, K_EXTERNAL_ONE_BYTE_STRING_TAG, K_EXTERNAL_STRING_TAG,
  K_EXTERNAL_TWO_BYTE_STRING_TAG, K_IS_INDIRECT_STRING_MASK, K_IS_INDIRECT_STRING_TAG,
  K_IS_NOT_STRING_MASK, K_ONE_BYTE_STRING_TAG, K_SEQ_ONE_BYTE_STRING_TAG, K_SEQ_STRING_TAG,
  K_SEQ_TWO_BYTE_STRING_TAG, K_SHARED_STRING_MASK, K_SHARED_STRING_TAG, K_SLICED_STRING_TAG,
  K_STRING_ENCODING_MASK, K_STRING_REPRESENTATION_AND_ENCODING_MASK,
  K_STRING_REPRESENTATION_MASK, K_STRING_TAG, K_THIN_STRING_TAG, K_TWO_BYTE_STRING_TAG,
  K_UNCACHED_EXTERNAL_STRING_MASK, K_UNCACHED_EXTERNAL_STRING_TAG,
};

// ── 单类型检查器 (INSTANCE_TYPE_CHECKERS_SINGLE, INSTANCE_TYPE_CHECKER1 展开) ──
${singleLines.join("\n\n")}

// ── 区间检查器 (INSTANCE_TYPE_CHECKERS_RANGE, INSTANCE_TYPE_CHECKER_RANGE1 展开) ──
${rangeLines.join("\n\n")}

${customBody}

// ── 对数登记 ──
/// SINGLE 桶检查器数 (含 BigInt/FixedArrayExact 追加)。
pub const SINGLE_CHECKER_COUNT: usize = ${single.length};
/// RANGE 桶检查器数 (含 CallableJSFunction 追加)。
pub const RANGE_CHECKER_COUNT: usize = ${range.length};
/// MULTIPLE 桶条目数 (不产生 Is 检查器, 仅类型登记)。
pub const MULTIPLE_BUCKET_COUNT: usize = ${multiple.length};
`;

writeSrc(CRATE, "instance_type_checker.rs", body);
console.log(`instance_type_checker.rs: SINGLE ${single.length} / RANGE ${range.length} / MULTIPLE(登记) ${multiple.length}`);
