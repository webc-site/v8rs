// 由 tool/src/sandbox/external-pointer-table.cc.js 从 v8/src/sandbox/ 表实现族 + compactible 表基类 + 指针/作用域/校验器族 (39 缺口文件登记) 生成; 修订就地改, 复跑覆盖
// 沙箱表族骨架

/// 四表定义头清单 (对数基准)
// src/sandbox/external-pointer-table.cc — ExternalPointerTable (5 项):
// void ExternalPointerTable::SetUpFromReadOnlyArtifacts( Space* read_only_space, const ReadOnlyArtifacts* artifacts)
// uint32_t ExternalPointerTable::EvacuateAndSweepAndCompact(Space* space, Space* from_space, Counters* counters)
// uint32_t ExternalPointerTable::SweepAndCompact(Space* space, Counters* counters)
// uint32_t ExternalPointerTable::Sweep(Space* space, Counters* counters)
// void ExternalPointerTable::Verify(Isolate* isolate, Space* space)

// src/sandbox/trusted-pointer-table.cc — TrustedPointerTable (3 项):
// void TrustedPointerTable::SetUpFromReadOnlyArtifacts( Space* read_only_space, const ReadOnlyArtifacts* artifacts)
// uint32_t TrustedPointerTable::Sweep(Space* space, Counters* counters)
// void TrustedPointerTable::Verify(Isolate* isolate, Space* space)

// src/sandbox/js-dispatch-table.cc — JSDispatchTable (3 项):
// void JSDispatchTable::Verify(Isolate* isolate, Space* space)
// void JSDispatchTable::PrintEntry(JSDispatchHandle handle)
// void JSDispatchTable::PrintCurrentTieringRequest(JSDispatchHandle handle, Isolate* isolate, std::ostream& os)

// src/sandbox/cppheap-pointer-table.cc — CppHeapPointerTable (2 项):
// uint32_t CppHeapPointerTable::SweepAndCompact(Space* space, Counters* counters)
// void CppHeapPointerTable::Verify(Isolate* isolate, Space* space)

pub mod sandbox_tables_skel {
  /// 表族 (外部指针表/可信指针表/JS 分发表/CppHeap 指针表)
  pub const TABLES: [&str; 4] = ["ExternalPointerTable", "TrustedPointerTable", "JSDispatchTable", "CppHeapPointerTable"];

  /// 可压缩表基类 (原版 CompactibleExternalEntityTable: 分段迁移 + 紧缩)
  pub fn compact_skel() {
    unimplemented!("骨架")
  }

  /// JS 分发表入口: code 指针 + 入口点地址双字段 (原版 JSDispatchEntry)
  pub const JS_DISPATCH_ENTRY_FIELDS: [&str; 2] = ["code", "entrypoint"];

  /// 受信指针作用域 (原版 TrustedPointerScope: 发布/撤销窗口)
  pub fn trusted_pointer_scope_skel() {
    unimplemented!("骨架")
  }

  /// 沙箱视角 Isolate 门面 (原版 isolate.h/inl.h: IsolateForSandbox)
  pub fn isolate_skel() {
    unimplemented!("骨架")
  }

  /// 字节码校验器 (原版 BytecodeVerifier: 沙箱内字节码完整性)
  pub fn bytecode_verifier_skel() {
    unimplemented!("骨架")
  }

  /// 陷阱 fuzz 钩子 (原版 TrapFuzzer)
  pub fn trap_fuzzer_skel() {
    unimplemented!("骨架")
  }

  /// 测试面登记 (原版 testing.cc: 沙箱测试 API)
  pub const TESTING_ANCHOR: bool = true;

  /// 缺口文件登记数 (含本脚本覆盖族)
  pub const REGISTERED_FILES: usize = 33;
  
  pub const ANCHORED_BYTES: [(&str, usize); 25] = [
    ("compactible-external-entity-table.h", 0),
    ("compactible-external-entity-table-inl.h", 0),
    ("cppheap-pointer.h", 0),
    ("cppheap-pointer-inl.h", 0),
    ("cppheap-pointer-tag.h", 0),
    ("cppheap-pointer-table.h", 0),
    ("cppheap-pointer-table-inl.h", 0),
    ("indirect-pointer.h", 0),
    ("indirect-pointer-inl.h", 0),
    ("isolate.h", 0),
    ("isolate-inl.h", 0),
    ("trusted-pointer-scope.h", 0),
    ("sandboxable-thread.h", 0),
    ("external-strings-cage.h", 0),
    ("bytecode-verifier.h", 0),
    ("trap-fuzzer.h", 0),
    ("testing.h", 0),
    ("trusted-pointer-scope.cc", 0),
    ("sandboxable-thread.cc", 0),
    ("external-strings-cage.cc", 0),
    ("bytecode-verifier.cc", 0),
    ("trap-fuzzer.cc", 0),
    ("testing.cc", 0),
    ("cppheap-pointer-table-inl.h", 0),
    ("indirect-pointer-tag.h", 0),
  ];
}
