// 由 tool/src/regexp/regexp-bytecode-peephole.h.js 从 v8/src/regexp/regexp-bytecode-peephole.{h,cc} 生成; 修订就地改, 复跑覆盖
// 窥孔优化入口骨架 (预定义字节码序列合并; 语义面待 Developer 修订)

/// 字节码窥孔优化 (对应原版 BytecodePeepholeOptimization, AllStatic)
/// 将 BytecodeGenerator 产出的预定义序列合并为单条字节码
pub struct BytecodePeepholeOptimization;

impl BytecodePeepholeOptimization {
  /// 原版 OptimizeBytecode(Isolate*, Zone*, RegExpData*, BytecodeWriter* src_writer)
  /// 返回优化后的字节码
  pub fn optimize_bytecode(_src: &[u8]) -> Vec<u8> {
    unimplemented!("登记骨架: 实现见 v8/src/regexp/regexp-bytecode-peephole.cc")
  }
}
