// 由 tool/src/heap/cppgc-internal/visitor.h.js 从 v8/src/heap/cppgc-internal/visitor.h 生成; 修订就地改, 复跑覆盖
// VisitorFactory/VisitorKey 钥匙惯用法 + cppgc::Visitor 最小前向面
// 推迟台账: VisitorBase/RootVisitorBase/ConservativeTracingVisitor (visitor.h:25-74) 依赖
// HeapBase/PageBackend/CagedHeap/GlobalGcInfoTable 与 cppgc::Visitor 虚面 (include/cppgc/visitor.h),
// 随 visitor 族后续批次挂载 (见 readme/zh.md)。

/// v8: include/cppgc/visitor.h:75-77 Visitor::Key — 外部不可构造, 仅工厂能造。
#[derive(Clone, Copy, Debug)]
pub struct VisitorKey {
  _priv: (),
}

impl VisitorKey {
  /// 原版 visitor.h:77 `Key() = default` private + friend internal::VisitorFactory →
  /// Rust 私有的工厂构造入口。
  const fn from_factory() -> Self {
    Self { _priv: () }
  }
}

/// v8: src/heap/cppgc-internal/visitor.h:18-21 VisitorFactory
pub struct VisitorFactory;

impl VisitorFactory {
  /// v8: src/heap/cppgc-internal/visitor.h:20 VisitorFactory::CreateKey
  pub const fn create_key() -> VisitorKey {
    VisitorKey::from_factory()
  }
}

/// v8: include/cppgc/visitor.h:74 cppgc::Visitor — 前向声明面 (TraceCallback 签名所需)。
/// 完整虚面 (Trace(Member/WeakMember)/TraceImpl/VisitWeak/Root 联动) 随 visitor 族后续批次挂载,
/// 本面仅提供钥匙约束与指针身份, 供 GCInfo.trace 回调按原版调用约定接收。
pub struct Visitor {
  key: VisitorKey,
}

impl Visitor {
  /// v8: include/cppgc/visitor.h:79 `explicit Visitor(Key)` — 原版 public 但 Key 只有工厂能造,
  /// Rust 侧同约束 (VisitorBase 后续批次即以此构造)。
  pub const fn new_with_key(key: VisitorKey) -> Self {
    Self { key }
  }

  /// v8: include/cppgc/visitor.h:77 Key 的持有 (钥匙仅证明构造合法性, 无运行期语义)。
  pub const fn key(&self) -> VisitorKey {
    self.key
  }
}
