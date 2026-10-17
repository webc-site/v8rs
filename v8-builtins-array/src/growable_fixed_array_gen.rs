// 由 tool/src/builtins/growable-fixed-array-gen.js 从 v8/src/builtins/growable-fixed-array-gen.cc + src/builtins/growable-fixed-array-gen.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/growable-fixed-array-gen.cc, src/builtins/growable-fixed-array-gen.h)
//! 任务映射: v8-builtins-array; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 0 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[];

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[("GrowableFixedArray", "CodeStubAssembler")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 0);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
