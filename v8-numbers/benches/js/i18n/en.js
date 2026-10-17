// 英文语言字符串
export const TITLE = "v8-numbers conversion benchmark";
export const DESC =
  "Fixed deterministic dataset of 1000 doubles (all magnitude orders, " +
  "integer/fraction/exponent switch boundaries, extremes) and matching string sets. " +
  "Compared: std format/parse, ryu, itoa (todo 10.2). Time = per single conversion.";
export const OP_HEADER = "op";
export const IMPL_HEADER = "impl";
export const TIME_HEADER = "ns/conv";
export const REL_HEADER = "vs base";
export const ITER_HEADER = "ns/iter(1000)";
export const ERR_HEADER = "std err";
export const GENERATED = "generated";
export const CRITERION_HINT = "Reproduce: ./bench.js (cargo bench -p v8-numbers --features bench)";
export const SVG_TITLE = "v8-numbers benchmark (lower is better)";

export default {
  TITLE,
  DESC,
  OP_HEADER,
  IMPL_HEADER,
  TIME_HEADER,
  REL_HEADER,
  ITER_HEADER,
  ERR_HEADER,
  GENERATED,
  CRITERION_HINT,
  SVG_TITLE,
};
