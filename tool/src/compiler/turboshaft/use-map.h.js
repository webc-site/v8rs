#!/usr/bin/env bun
// tool/src/compiler/turboshaft/use-map.h.js
// 机械提取 v8/src/compiler/turboshaft/use-map.{h,cc}
// → v8-turboshaft/src/use_map.rs.skel
//
// 提取面: UseMap offset 编码; Simd128UseMap 过滤谓词; cc 方法面。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/use-map.h");
const cc = readV8("src/compiler/turboshaft/use-map.cc");

// 1. 哨兵
for (const [re, name] of [
  [/typedef bool \(\*FunctionType\)/, "filter 函数类型"],
  [/class V8_EXPORT_PRIVATE UseMap \{/, "UseMap"],
  [/struct PerOperationUses \{/, "PerOperationUses"],
  [/int32_t offset = 0;/, "offset 字段"],
  [/uint32_t count = 0;/, "count 字段"],
  [/class Simd128UseMap : public UseMap/, "Simd128UseMap"],
  [/has_revectorizable_simd128_output/, "simd 过滤"],
  [/RegisterRepresentation::Simd128\(\)/, "Simd128 表示"],
]) {
  if (!(re.test(h) || re.test(cc))) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc 方法面 (构造 + uses + AddUse)
const ccMethods = [...cc.matchAll(/^(?:base::Vector<const OpIndex>|void) UseMap::(\w+)\(/gm)].map((m) => m[1]);
if (ccMethods.length !== 2) throw new Error(`cc 方法 ${ccMethods.length} != 2`);
if (!/^UseMap::UseMap\(/m.test(cc)) throw new Error("cc 缺构造");

// 3. 编码规则注释
const enc = /offset < 0: -offset-1 indexes into \{saturated_uses_\}/.test(h) &&
  /offset = 0: definition not visited yet/.test(h) &&
  /offset > 0: offset indexes into \{uses_\}/.test(h);
if (!enc) throw new Error("offset 编码注释缺失");

let rust = banner(
  "src/compiler/turboshaft/use-map.h.js",
  "src/compiler/turboshaft/use-map.{h,cc}",
  "使用映射: OpIndex 到使用列表",
);

rust += `
/// 原版 UseMap::PerOperationUses — 三态 offset 编码:
/// - offset < 0: -offset-1 索引 saturated_uses_ (使用数溢出内联阈值)
/// - offset = 0: 定义尚未访问
/// - offset > 0: offset 索引 uses_ (平铺数组 + count 计数)
/// v8: use-map.h UseMap::PerOperationUses
#[derive(Default, Clone, Copy)]
pub struct PerOperationUses {
  pub offset: i32,
  pub count: u32,
}

/// 原版 UseMap — 全操作使用计算, filter 可跳过无关操作
/// (默认 filter 恒 false = 不过滤)。
/// v8: use-map.{h,cc} UseMap
pub struct UseMap {
  pub table: Vec<PerOperationUses>,
  pub uses: Vec<OpIndex>,
  pub saturated_uses: Vec<Vec<OpIndex>>,
}

impl UseMap {
  /// v8: cc UseMap::UseMap 构造 + 全图扫描 (AddUse 维护编码)
  pub fn new(graph: &Graph, filter: UseFilter) -> Self { todo!() }
  /// v8: cc uses — 读侧 (offset 三态解码)
  pub fn uses(&self, index: OpIndex) -> &[OpIndex] { todo!() }
}

/// v8: use-map.cc 方法面: 构造(全图扫描), ${ccMethods.join(", ")}
pub type UseFilter = fn(&Graph, &Operation) -> bool;

/// 原版 Simd128UseMap — 只算可重向量化 Simd128 操作的使用:
/// filter 判定 (非 simd 输出才记):
/// - 单输出 Simd128 -> 不滤 (记使用)
/// - ReturnOp/FrameStateOp: 逐输入检查 simd 输出
/// - 其余: inputs_rep 任一 Simd128 -> 滤
/// v8: use-map.h Simd128UseMap
pub struct Simd128UseMap(pub UseMap);
`;

const p = writeSrc("v8-turboshaft", "use_map.rs.skel", rust);
console.log(`written ${p}`);
console.log(`cc方法=${ccMethods.length} 编码规则=3态`);
