// 公共件 (genbench 9.7): bench.json 读取/解析转换、时间格式化、对比分组
import { readFile } from "node:fs/promises";

const ROOT_DIR = `${import.meta.dirname}/../..`;

const UNITS = [
  [1e6, "ms"],
  [1e3, "µs"],
  [1, "ns"],
];

const fmt_time = (ns) => {
  for (const [scale, unit] of UNITS) {
    if (ns >= scale) return `${(ns / scale).toFixed(2)} ${unit}`;
  }
  return `${ns.toFixed(2)} ns`;
};

const fmt_ratio = (r) => `${r.toFixed(2)}×`;

// 读取汇总结果 json (bench.js 产物)
const read_bench = async () => JSON.parse(await readFile(`${ROOT_DIR}/bench.json`, "utf8"));

// 按 (op, param) 分组并附对比数据: 每组以最快实现为基准, mean_ns / fastest 得倍率;
// gain 为参考实现 v8-ast 相对对比实现的平均收益倍数
const to_groups = (results) => {
  const map = new Map();
  for (const r of results) {
    const key = `${r.op}/${r.param}`;
    if (!map.has(key)) map.set(key, { op: r.op, param: r.param, items: [] });
    map.get(key).items.push(r);
  }
  const groups = [...map.values()];
  for (const g of groups) {
    g.items.sort((a, b) => a.impl.localeCompare(b.impl));
    g.fastest = g.items.reduce((m, i) => (i.mean_ns < m.mean_ns ? i : m));
    for (const i of g.items) i.vs_fastest = i.mean_ns / g.fastest.mean_ns;
  }
  const ratios = groups
    .filter((g) => g.items.length > 1)
    .map((g) => {
      const self = g.items.find((i) => i.impl === "v8-ast");
      const others = g.items.filter((i) => i.impl !== "v8-ast");
      return others.reduce((acc, o) => acc + o.mean_ns / self.mean_ns, 0) / others.length;
    });
  const gain = ratios.length ? ratios.reduce((a, b) => a + b, 0) / ratios.length : 1;
  return { groups, gain };
};

export { fmt_ratio, fmt_time, read_bench, to_groups };
export default { fmt_ratio, fmt_time, read_bench, to_groups };
