#!/usr/bin/env bun
// tool/src/compiler/turbofan-typer.js
// v8/src/compiler/turbofan-typer.h + turbofan-typer.cc
// → v8-compiler-passes/src/turbofan_typer.rs.skel
// Typer: 解析 Flag 位标志 / Visitor 的 Type##x 声明宏展开
// (opcode 列表来自 opcodes.h) 与 cc 对数。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/turbofan-typer.h");
const cc = readV8("src/compiler/turbofan-typer.cc");
const opcodesH = readV8("src/compiler/opcodes.h");

const need = (cond, msg) => {
  if (!cond) throw new Error(`turbofan-typer: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE Typer {", "Typer 定义"],
  ["enum Flag : uint8_t {", "Flag 位标志"],
  ["void Run();", "Run 无参重载"],
  ["void Typer::Run(const NodeVector& roots,", "cc Run(roots"],
  ["class Typer::Visitor : public Reducer {", "Visitor"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Flag 位值 (真解析) ——
const flagBody = h.slice(h.indexOf("enum Flag : uint8_t {"));
const flags = [...flagBody.slice(0, flagBody.indexOf("};")).matchAll(/k(\w+) = (1u << \d+)/g)].map((m) => [m[1], 1 << Number(m[2].replace("1u << ", ""))]);
need(flags.length === 2, `Flag ${flags}`);
need(flags.some(([n]) => n === "ThisIsReceiver") && flags.some(([n]) => n === "NewTargetIsReceiver"), "Flag 名不符");

// —— Visitor 的 Type##x 声明宏 (真解析: DECLARE_METHOD + 宏列表调用) ——
const visStart = cc.indexOf("class Typer::Visitor");
const vis = cc.slice(visStart, visStart + 100000); // Decorator 在 Visitor 之前定义, 不作上界
const declMacros = [...vis.matchAll(/#define DECLARE_METHOD\(x(?:, \.\.\.)?\) ([^\n]+)/g)].map((m) => m[1].trim());
const macroCalls = [...vis.matchAll(/^  (\w[\w_]*)\(DECLARE_METHOD\)/gm)].map((m) => m[1]);
// 显式 DECLARE_METHOD(...) 调用
const explicitDecls = [...vis.matchAll(/^  DECLARE_METHOD\((\w+)\)/gm)].map((m) => m[1]);
// 宏列表递归展开 (opcodes.h; JS_SIMPLE_UNOP_LIST 等含嵌套调用; 防环)
const expandSeen = new Set();
function expandList(name) {
  if (expandSeen.has(name)) return [];
  expandSeen.add(name);
  const m = opcodesH.match(new RegExp(`#define ${name}\\(V\\)\\s*\\\\\\n([\\s\\S]*?)\\n\\n`));
  if (!m) return null;
  const body = m[1].replace(/\\\n/g, " ").replace(/\/\/[^\n]*/g, "");
  const own = [...body.matchAll(/\bV\((\w+)(?:,[^)]*)?\)/g)].map((x) => x[1]);
  const nested = [...body.matchAll(/\b([A-Z0-9_]+_LIST)\(V\)/g)].flatMap((x) => expandList(x[1]) ?? []);
  return [...own, ...nested];
}
const nodeArityLists = ["COMMON_OP_LIST", "SIMPLIFIED_COMPARE_BINOP_LIST", "SIMPLIFIED_OTHER_OP_LIST", "JS_OBJECT_OP_LIST", "JS_CONTEXT_OP_LIST", "JS_OTHER_OP_LIST"].map((n) => expandList(n));
const unopTypingsArr = expandList("JS_SIMPLE_UNOP_LIST");
need(nodeArityLists.every((l) => l.length > 0), "宏列表展开失败");
const nodeTypings = nodeArityLists.reduce((acc, l) => acc + l.length, 0);
const unopTypings = unopTypingsArr.length;
need(explicitDecls.length >= 2 && explicitDecls.includes("Start") && explicitDecls.includes("IfException"), `显式声明 ${explicitDecls}`);
need(unopTypings >= 5, `一元展开 ${unopTypings}`);

// —— IMPOSSIBLE case (机器码不可类型) ——
const impossible = [...vis.matchAll(/DECLARE_IMPOSSIBLE_CASE\((\w+)\)/g)].map((m) => m[1]);
need(impossible.length >= 30, `IMPOSSIBLE ${impossible.length}`);

const body = `${banner("src/compiler/turbofan-typer.js", "src/compiler/turbofan-typer.h + turbofan-typer.cc", "图类型标注 (Typer pass)")}
//! turbofan-typer — Typer + Decorator (新节点即时定型)
//!
//! Visitor 声明面: DECLARE_METHOD(Node) 组: COMMON(${nodeArityLists[0].length})
//! + SIMPLIFIED_COMPARE(${nodeArityLists[1].length}) + SIMPLIFIED_OTHER(${nodeArityLists[2].length})
//! + JS_OBJECT(${nodeArityLists[3].length}) + JS_CONTEXT(${nodeArityLists[4].length})
//! + JS_OTHER(${nodeArityLists[5].length}) = ${nodeTypings};
//! DECLARE_METHOD(Type) 组 (JS_SIMPLE_UNOP): ${unopTypings};
//! 显式: ${explicitDecls.join(", ")}; IMPOSSIBLE(机器): ${impossible.length}。

/// 原版 Typer::Flag 位标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TyperFlags(u8);

impl TyperFlags {
  /// kNoFlags
  pub const NONE: Self = Self(0);
${flags.map(([n, v]) => `  /// k${n} = ${v}\n  pub const ${n.replace(/_/g, "").toUpperCase()}: Self = Self(${v});`).join("\n")}
}

/// 原版 Typer — 类型标注 pass
pub struct Typer<'z, 'g> {
  /// flags_
  pub flags: TyperFlags,
  /// graph_
  pub graph: &'g mut TFGraph<'z>,
  /// decorator_
  pub decorator: *mut Decorator,
  /// cache_
  pub cache: *const TypeCache,
  /// broker_
  pub broker: *mut JSHeapBroker,
  /// operation_typer_
  pub operation_typer: OperationTyper<'z>,
  /// singleton_false_ / singleton_true_
  pub singleton_false: Type,
  pub singleton_true: Type,
}

impl Typer<'_, '_> {
  /// 原版 Run — 全图标注 (GraphVisitor)
  /// v8: src/compiler/turbofan-typer.cc Typer::Run
  pub fn run(&mut self, roots: Option<&ZoneVector<Node>>, induction_vars: Option<&mut LoopVariableOptimizer>) {
    todo!("v8: src/compiler/turbofan-typer.cc Typer::Run (两个重载)")
  }
}

/// 原版 Typer::Visitor — 逐节点定型
/// Type##x 面: ${nodeTypings + unopTypings + explicitDecls.length} 个声明
/// (Common ${nodeArityLists[0].length} / SimplifiedCompare ${nodeArityLists[1].length} /
/// SimplifiedOther ${nodeArityLists[2].length} / JSObject ${nodeArityLists[3].length} /
/// JSContext ${nodeArityLists[4].length} / JSOther ${nodeArityLists[5].length} /
/// JSUnop ${unopTypings} / 显式 ${explicitDecls.length});
/// IMPOSSIBLE 面 ${impossible.length} (机器码 UNREACHABLE 定型)。
pub struct TyperVisitor<'t, 'z> {
  /// typer_
  pub typer: &'t mut Typer<'z, 'static>,
  /// induction_vars_
  pub induction_vars: *mut LoopVariableOptimizer,
  /// weakened_nodes_
  pub weakened_nodes: ZoneSet<NodeId>,
}

impl TyperVisitor<'_, '_> {
  /// 原版 TypeType — 分派总入口 (Visitor 模板)
  /// v8: src/compiler/turbofan-typer.cc Typer::Visitor::TypeType
  pub fn type_type(&mut self, node: &Node) -> Type {
    todo!("v8: src/compiler/turbofan-typer.cc Typer::Visitor::TypeType (分派 ${nodeTypings + unopTypings + explicitDecls.length} 声明面 + ${impossible.length} IMPOSSIBLE)")
  }

  /// 原版 TypeConstant — 常量定型
  /// v8: src/compiler/turbofan-typer.cc Typer::Visitor::TypeConstant
  pub fn type_constant(&mut self, value: Handle<Object>) -> Type {
    todo!("v8: src/compiler/turbofan-typer.cc Typer::Visitor::TypeConstant")
  }

  /// 原版 Weaken — 防类型振荡弱化
  /// v8: src/compiler/turbofan-typer.cc Typer::Visitor::Weaken
  pub fn weaken(&mut self, node: &Node, current_type: Type, previous_type: Type) -> Type {
    todo!("v8: src/compiler/turbofan-typer.cc Typer::Visitor::Weaken")
  }

  /// 原版 InductionVariablePhiTypeIsPrefixedPoint — 归纳变量定点判定
  /// v8: src/compiler/turbofan-typer.cc Typer::Visitor::InductionVariablePhiTypeIsPrefixedPoint
  pub fn induction_var_phi_type_is_prefixed_point(&self, induction_var: &InductionVariable) -> bool {
    todo!("v8: src/compiler/turbofan-typer.cc Typer::Visitor::InductionVariablePhiTypeIsPrefixedPoint")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "turbofan_typer.rs.skel", body);
console.log(
  [
    `written ${p}`,
    `flags=${flags.length} (${flags.map(([n, v]) => `${n}:${v}`).join(",")})`,
    `declare_method宏=${declMacros.length} 宏调用=${macroCalls.join(",")}`,
    `nodetypings=${nodeTypings} unop=${unopTypings} 显式=${explicitDecls.length} impossible=${impossible.length}`,
  ].join("\n"),
);
