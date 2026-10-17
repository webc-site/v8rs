// 由 tool/src/codegen/compilation-cache.h.js 从 v8/src/codegen/compilation-cache.{h,cc} 生成; 修订就地改, 复跑覆盖
// 编译缓存骨架

/// RegExp 子缓存代数 (原 kGenerations)
pub const GENERATIONS: i32 = 2;

/// 原类面 (分缓存类别):
/// - class CompilationCacheEval   — eval/函数字面量缓存
/// - class CompilationCacheRegExp — RegExp 源串+标志缓存
/// - class CompilationCacheScript — 脚本源缓存
/// - 原签名: DirectHandle<CompilationCacheTable> GetTable(int generation)
/// TODO(dev): 补代龄淘汰与表存取实现。

pub struct CompilationCacheEval;
pub struct CompilationCacheRegExp;
pub struct CompilationCacheScript;

// —— 提取不变式断言 ——
const _: () = {
  assert!(GENERATIONS == 2);
};
