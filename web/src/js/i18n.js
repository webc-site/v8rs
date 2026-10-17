// 多语言系统，参考 webc 设计
let current_lang = (typeof localStorage !== "undefined" && localStorage.getItem("LANG")) || "zh";
const listeners = new Set();

export const DICT = {
  zh: {
    title: "V8RS 性能对标与回归大盘",
    tagline: "超越原版 V8 的纯 Rust 现代化 JavaScript / WebAssembly 高性能引擎",
    desc: "100% 内存安全、无 C++ 旧代包袱、零 panic，搭载自研 rusty_alloc-api 分配器，针对 SIMD/AES 深度优化，在全量 js-engine-benchmark 测试中实现极致性能与能效比。",
    overallScore: "综合基准得分",
    scorePerMb: "能效比 (Score/MB)",
    memoryUsage: "二进制体积",
    testsPassed: "全工程单元测试",
    v8Compare: "对标官方 C++ V8",
    benchmarkDetails: "全量 js-engine-benchmark (v8-v7) 8 项基准",
    benchmarkDesc: "覆盖操作系统调度、数值计算、光线追踪、形式化文法解析、正则状态机与动态内存分配敏感型负载。",
    nameCol: "基准项目",
    descCol: "测试内容与场景",
    v8rsScore: "V8RS 得分",
    v8Score: "原版 C++ V8",
    speedup: "性能对比",
    latency: "单次耗时",
    platformMatrix: "全平台构建与持续集成矩阵",
    platformDesc: "全流程自动化测试与 Release 发布，覆盖 3 大操作系统 + Linux ARM64 交叉编译：",
    platformCol: "目标平台",
    targetCol: "Target Triple",
    compilerCol: "编译工具链",
    ciStatus: "CI 状态",
    verified: "测试验证",
    viewOnGithub: "查看 GitHub 仓库",
    actionsReport: "GitHub Actions 报告",
    footerText: "基于 Rust 2024 构建 · 遵循 MIT 开源协议 · 针对全平台硬件级优化",
    switchLang: "English",
  },
  en: {
    title: "V8RS Benchmark & Performance Dashboard",
    tagline: "Pure Rust Modern JavaScript & WebAssembly Engine Outperforming Native V8",
    desc: "100% memory safe, zero C++ legacy bloat, zero panic, powered by rusty_alloc-api with SIMD/AES hardware acceleration. Crushing benchmarks in throughput and Score/MB.",
    overallScore: "Suite Score",
    scorePerMb: "Score / MB",
    memoryUsage: "Binary Size",
    testsPassed: "Unit Tests Passed",
    v8Compare: "vs Official C++ V8",
    benchmarkDetails: "Full js-engine-benchmark (v8-v7) 8 Benchmarks",
    benchmarkDesc: "Comprehensive workloads covering OS scheduling, ray tracing, numerical PDEs, parser ASTs, regexp and splay tree allocation.",
    nameCol: "Benchmark",
    descCol: "Workload Description",
    v8rsScore: "V8RS Score",
    v8Score: "Native C++ V8",
    speedup: "Comparison",
    latency: "Latency",
    platformMatrix: "Cross-Platform Build & CI Matrix",
    platformDesc: "Automated continuous regression testing and optimized release publishing across tier-1 platforms including Linux ARM64:",
    platformCol: "Platform",
    targetCol: "Target Triple",
    compilerCol: "Toolchain",
    ciStatus: "CI Status",
    verified: "Verified",
    viewOnGithub: "GitHub Repository",
    actionsReport: "GitHub Actions",
    footerText: "Engineered in Rust 2024 · MIT Licensed · Micro-architectural Target Optimizations",
    switchLang: "中文",
  },
};

export const getLang = () => current_lang;

export const setLang = (lang) => {
  current_lang = lang;
  if (typeof localStorage !== "undefined") {
    localStorage.setItem("LANG", lang);
  }
  for (const fn of listeners) {
    fn(current_lang, DICT[current_lang]);
  }
};

export const toggleLang = () => {
  setLang(current_lang === "zh" ? "en" : "zh");
};

export const onLang = (fn) => {
  listeners.add(fn);
  fn(current_lang, DICT[current_lang]);
  return () => listeners.delete(fn);
};
