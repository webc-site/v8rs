// 英文语言文件 (genbench 9.3: 不用字典, export const 大写变量名)

export const REPORT_TITLE = "v8-zone Zone allocator benchmark";
export const GENERATED_LABEL = "generated";
export const OP_LABEL = "op";
export const COL_PARAM = "params";
export const COL_ALLOCS = "allocs/s";
export const COL_BYTES = "MB/s";
export const COL_RATIO = "vs baseline";
export const BASELINE_LABEL = "baseline";

export const OP_ALLOC = "alloc: batch allocate + bulk reclaim";
export const OP_ALIGNED = "aligned: over-allocate then align pointer";
export const OP_SCOPED = "scoped: nested scope allocate & rollback";
export const OP_RESET = "reset: multi-segment workload then DeleteAll/Reset";

export const MD_INTRO = `# ${REPORT_TITLE}

This report compares the v8rs \`Zone\` bump allocator against reference
implementations on the crate's core operations. Numbers come from criterion
(\`cargo bench -p v8-zone --features bench\`); a larger ratio means higher
throughput than the baseline \`v8rs-zone\`.
`;

export const MD_IMPL_NOTE = `Implementations: \`v8rs-zone\` (this crate), \`bumpalo\`
(bump-kernel reference only; Zone semantics stay self-implemented), \`std-vec\`
(std Vec batch allocation through the global jemalloc, generic allocator baseline).
`;

export const SVG_TITLE = "v8-zone benchmark: throughput relative to v8rs-zone";
