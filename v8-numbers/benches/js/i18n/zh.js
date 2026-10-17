// 中文语言字符串
export const TITLE = "v8-numbers 数值转换评测";
export const DESC =
  "固定确定性数据集: 1000 条 double (覆盖各量级、整数/小数/指数切换边界、极值) " +
  "及对应字符串集。对照库: std format/parse、ryu、itoa (见 todo 10.2)。耗时 = 单次转换。";
export const OP_HEADER = "操作";
export const IMPL_HEADER = "实现";
export const TIME_HEADER = "纳秒/次";
export const REL_HEADER = "相对基准";
export const ITER_HEADER = "纳秒/轮(1000)";
export const ERR_HEADER = "标准误差";
export const GENERATED = "生成时间";
export const CRITERION_HINT = "复现: ./bench.js (cargo bench -p v8-numbers --features bench)";
export const SVG_TITLE = "v8-numbers 评测 (越低越快)";

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
