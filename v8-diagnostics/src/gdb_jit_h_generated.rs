// 由 tool/src/diagnostics/gdb-jit.h.js 从 v8/src/diagnostics/gdb-jit.h 生成; 修订就地改, 复跑覆盖
// GDB JIT 接口声明面

// 注: gdb-jit.cc 数据结构由 tool/src/diagnostics/gdb-jit.js 落 gdb_jit_data.rs。
// ENABLE_GDB_JIT_INTERFACE 门控面, 语义实现登记跳过。

/// 原 maglev::MaglevVariableInfo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaglevVariableInfo {
  pub node_id: i32, // 原 node_id
  pub spill_slot_index: i32, // 原 spill_slot_index
  pub start_pc: i32, // 原 start_pc
}

impl MaglevVariableInfo {
  /// 原 GetMaglevGraphFilename: `maglev-<func_name>-<opt_id>.mgl`
  #[must_use]
  pub fn graph_filename(func_name: &str, opt_id: i32) -> String {
    format!("maglev-{func_name}-{opt_id}.mgl")
  }
}

/// 原 namespace GDBJITInterface 导出函数签名骨架
// 原签名: RegisterMaglevVariableLocations( uintptr_t code_start, const std::vector<maglev::MaglevVariableInfo>& vars)
// 原签名: AddRegionForTesting(const base::AddressRegion region)
// 原签名: ClearCodeMapForTesting()
// 原签名: NumOverlapEntriesForTesting(const base::AddressRegion region)

// —— 提取不变式断言 ——
const _: () = {
  assert!(MaglevVariableInfo { node_id: 0, spill_slot_index: 0, start_pc: 0 }.start_pc == 0);
};
