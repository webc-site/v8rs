// 中文语言文件 (genbench 9.3: 不用字典, export const 大写变量名)

export const REPORT_TITLE = "v8-zone Zone 分配器评测";
export const GENERATED_LABEL = "生成时间";
export const OP_LABEL = "操作";
export const COL_PARAM = "参数";
export const COL_ALLOCS = "allocs/秒";
export const COL_BYTES = "MB/秒";
export const COL_RATIO = "相对基线";
export const BASELINE_LABEL = "基线";

export const OP_ALLOC = "alloc: 批量分配 + 整体回收";
export const OP_ALIGNED = "aligned: 过量分配后取整对齐指针";
export const OP_SCOPED = "scoped: 作用域嵌套分配与回滚";
export const OP_RESET = "reset: 多段负载后 DeleteAll/Reset 复位";

export const MD_INTRO = `# ${REPORT_TITLE}

本报告对比 v8rs \`Zone\` bump 分配器与各对照库在本 crate 核心操作上的表现。
数据来自 criterion (\`cargo bench -p v8-zone --features bench\`)，
倍率越大表示吞吐高于基线 v8rs-zone。
`;

export const MD_IMPL_NOTE =
  "参与评测的实现：`v8rs-zone`（本 crate）、`bumpalo`" +
  "（仅作 bump 内核对照，Zone 语义 API 保持自研）、`std-vec`" +
  "（std Vec 批量分配，经全局 jemalloc 的通用分配器基线）。";

export const SVG_TITLE = "v8-zone 评测：相对 v8rs-zone 的吞吐倍率";
