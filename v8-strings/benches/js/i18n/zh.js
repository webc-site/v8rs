// zh 语言字符串 (export const 大写变量名, 不用字典)。

export const TITLE = "v8-strings 核心操作评测";
export const CORPUS_NOTE =
  "语料: 固定确定性样本 (ASCII / Latin / CJK / 代理对混合), 长度 1k 与 32 码点; 数值为 criterion 整样本每次迭代均值纳秒。";
export const METHOD_NOTE =
  "方法: criterion 短程测量 (sample-size 10, 预热 1s, 测量 1s), jemalloc 全局分配器。";
export const RATIO_NOTE = "倍率: 相对 std 基线的纳秒比 (越小越快)。";
export const COL_IMPL = "实现";
export const COL_RESULT = "ns / 样本";
export const COL_VS_STD = "对比 std";
export const OP_TRANSCODE_ENCODE = "UTF-16 转 UTF-8";
export const OP_TRANSCODE_DECODE = "UTF-8 转 UTF-16";
export const OP_COPY_CHARS = "CopyChars (良构复制)";
export const OP_SCAN_IDENTIFIER = "标识符 start/continue 扫描";
export const OP_CASE_LOWER = "ASCII 小写映射";
export const OP_FIND_BYTE = "单字节搜索";
export const OP_FIND_SUBSTRING = "子串搜索";
export const OP_NORMALIZE = "NFC 规范化";
