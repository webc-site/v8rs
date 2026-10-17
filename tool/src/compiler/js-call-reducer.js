#!/usr/bin/env bun
// tool/src/compiler/js-call-reducer.js
// 从 v8/src/compiler/js-call-reducer.h + js-call-reducer.cc
// 提取 Flag 位标志、4 个内嵌枚举、Reduce* 方法面与 Builtin 分派计数
// 生成 v8-compiler-frontend/src/js_call_reducer.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-call-reducer.h");
const cc = readV8("src/compiler/js-call-reducer.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["class V8_EXPORT_PRIVATE JSCallReducer final : public AdvancedReducer {", "JSCallReducer 类"],
  ["kBailoutOnUninitialized = 1u << 0,", "kBailoutOnUninitialized 1<<0"],
  ["kInlineJSToWasmCalls = 1u << 1,", "kInlineJSToWasmCalls 1<<1"],
  ["static constexpr int kMaxInlineMatchSequence = 3;", "startsWith 内联上限 3"],
  ["std::set<Node*> waitlist_;", "waitlist 成员"],
  ["std::unordered_set<Node*> generated_calls_with_array_like_or_spread_;", "防无限递归集合"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`js-call-reducer.h 锚点缺失: ${name}`);
}

// —— 2. 真解析: Flag 位标志 ——
const flagSrc = h.match(/enum Flag \{([\s\S]*?)\};/);
if (!flagSrc) throw new Error("Flag 枚举未找到");
const flags = [...flagSrc[1].matchAll(/(k\w+)\s*=\s*(1u << \d+|0u)/g)].map((m) => [m[1], m[2]]);

// —— 3. 真解析: 内嵌 enum class ——
const innerEnums = [...h.matchAll(/enum class (\w+) \{([^}]+)\};/g)]
  .map((m) => ({ name: m[1], vals: m[2].split(",").map((s) => s.trim()) }));

// —— 4. 真解析: Reduce*/私有方法面 ——
const priv = h.slice(h.indexOf(" private:"), h.indexOf("  JSGraph* const jsgraph_;"));
const reduceFns = [...priv.matchAll(/Reduction (Reduce\w+)\(/g)].map((m) => m[1]);
const uniqueReduce = [...new Set(reduceFns)];
const helperFns = [...priv.matchAll(/(?:bool|void|std::optional<Reduction>|std::pair<Node\*, Node\*>|Node\*) (?!Reduce)(\w+)\(/g)].map((m) => m[1])
  .filter((n) => !["Finalize", "Reduce"].includes(n));

// —— 5. .cc Builtin 分派计数 (builtin 内建名 case) ——
const builtinCases = new Set([...cc.matchAll(/case Builtin::k(\w+):/g)].map((m) => m[1]));
if (builtinCases.size < 150) throw new Error(`Builtin case 提取不足: ${builtinCases.size}`);
const reduceImpls = new Set([...cc.matchAll(/^Reduction JSCallReducer::(Reduce\w+)\(/gm)].map((m) => m[1]));

const body = `${banner("src/compiler/js-call-reducer.js", "src/compiler/js-call-reducer.h + js-call-reducer.cc", "JSCall/JSConstruct 强度削减: 内建方法逐个特化")}

/// JSCallReducer — 对 {JSConstruct}/{JSCall} 做强度削减, 为内联等铺路
/// v8: src/compiler/js-call-reducer.h JSCallReducer
pub struct JSCallReducer { /* AdvancedReducer + jsgraph/broker/temp_zone/flags/waitlist */ }

bitflags! {
    /// 操作模式位标志
    /// v8: src/compiler/js-call-reducer.h JSCallReducer::Flag
    pub struct Flags: u32 {
        const NO_FLAGS = ${flags.find((f) => f[1] === "0u")?.[0].replace("kNoFlags", "0x0") ?? "0x0"};
${flags.filter((f) => f[1] !== "0u").map(([k, v]) => `        const ${k.replace(/^k/, "").toUpperCase()} = 1 << ${v.match(/\d+/)[0]};`).join("\n")}
    }
}

impl JSCallReducer {
    /// String.prototype.startsWith 整段匹配序列内联的最大串长
    /// v8: src/compiler/js-call-reducer.h JSCallReducer::kMaxInlineMatchSequence
    pub const MAX_INLINE_MATCH_SEQUENCE: i32 = 3;

    /// 图组装器协作面
    /// v8: src/compiler/js-call-reducer.h JSCallReducer::RevisitForGraphAssembler
    /// v8: src/compiler/js-call-reducer.h JSCallReducer::ZoneForGraphAssembler
    /// v8: src/compiler/js-call-reducer.h JSCallReducer::JSGraphForGraphAssembler

    /// waitlist 收尾再试一轮
    /// v8: src/compiler/js-call-reducer.h JSCallReducer::Finalize
    fn finalize(&mut self);
}

// —— 内嵌枚举 (${innerEnums.length} 个) ——
${innerEnums.map((e) => `/// v8: src/compiler/js-call-reducer.h JSCallReducer::${e.name}\npub enum ${e.name} { ${e.vals.map((v) => v.replace(/^k/, "").replace(/ \/.*/, "")).join(", ")} }`).join("\n\n")}

// —— Reduce* 方法面 (私有, ${uniqueReduce.length} 个去重) ——
// .cc 实现核对: ${uniqueReduce.length} 声明 / ${reduceImpls.size} 实现
// 代表签名 (全部为 Reduction fn(Node* node, ...), 参数差异见原版 h):
${uniqueReduce.slice(0, 12).map((n) => `// fn ${n}(node: &Node, ...) -> Reduction /// v8: src/compiler/js-call-reducer.h JSCallReducer::${n}`).join("\n")}
// ... 其余 ${uniqueReduce.length - 12} 个同面

// —— 辅助面 (${helperFns.length} 个) ——
// ReplaceWithSubgraph / ReleaseEffectAndControlFromAssembler / DoPromiseChecks /
// CreateClosureFromBuiltinSharedFunctionInfo / CheckIfElementsKind /
// LoadReceiverElementsKind / IsBuiltinOrApiFunction / CheckArrayLength /
// CheckIfConstructor / ConvertHoleToUndefined
${helperFns.slice(0, 4).map((n) => `// ${n} /// v8: src/compiler/js-call-reducer.h JSCallReducer::${n}`).join("\n")}

// —— .cc Builtin 分派面: ${builtinCases.size} 个 case Builtin::k* ——
// (Math/Array/String/Promise/TypedArray/Collection/Reflect/Object 全家)
// 例: ${[...builtinCases].slice(0, 5).map((b) => `k${b}`).join(" / ")} ...

// —— 对数基准 ——
// Flag ${flags.length} / 内嵌枚举 ${innerEnums.length} (${innerEnums.map((e) => e.name).join(",")})
// Reduce 声明 ${uniqueReduce.length} vs 实现 ${reduceImpls.size} / Builtin case ${builtinCases.size}
`;

const p = writeSrc("v8-compiler-frontend", "js_call_reducer.rs.skel", body);
console.log(`✓ js-call-reducer: Flag ${flags.length} / 内嵌枚举 ${innerEnums.length} / Reduce ${uniqueReduce.length}(实现 ${reduceImpls.size}) / Builtin case ${builtinCases.size} / 辅助 ${helperFns.length}`);
