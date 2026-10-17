// 目的: 中文报告字符串 (export const 大写变量名, 不用字典)
export const TITLE = "v8-bigint 评测报告";
export const SUB = "crate 自包含 criterion 评测; 固定种子确定性伪随机样本。";
export const COL_OP = "操作";
export const COL_RADIX = "进制";
export const COL_BITS = "位宽";
export const COL_RATIO = "对比库 / v8rs";
export const SEC_TABLE = "汇总";
export const SEC_SVG = "图表";
export const SEC_NOTES = "说明";
export const NOTE_MALACHITE = "对比库仅 num-bigint: malachite-nz 在本 workspace 编译过重, 未引入。";
export const NOTE_BARRETT =
  "Barrett 除法切换点为 >=13000 digit (~832k bit), 超出 64k bit 样本上限, 本轮不评测; div/rem 已覆盖 Burnikel-Ziegler 边界 (57 digit)。";
export const NOTE_ALGO =
  "乘法位宽跨越 schoolbook / Karatsuba (34) / Toom-Cook (210) / FFT (720) digit 阈值; 字符串转换覆盖 radix 10 (通用快速路径) 与 16 (2 幂 shortcut)。";
export const NOTE_SAMPLES =
  "样本: 已规格化 (msd 非零) 随机 digit; div/rem 除数取被除数一半位宽 (64 bit 档跳过)。";
export const NOTE_TIME =
  "数值为短测量窗口 (预热 1s / 测量 2s) 下 criterion 均值点估计; 可用环境变量 BENCH_ARGS 覆盖做长测。";
