// 由 tool/src/sandbox/isolate.h.js 从 v8/src/sandbox/isolate.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// IsolateForSandbox 门面骨架

/// 原 class IsolateForSandbox — 沙箱开启时持 Isolate*, 关闭时空实现。
/// 方法面 (h 9 / inl 45 条):
// - NOLINT
// - GetExternalPointerTableFor
// - GetExternalPointerTableSpaceFor
// - GetTrustedPointerTableFor
// - GetTrustedPointerTableSpaceFor
// - GetJSDispatchTable
// - GetExternalPointerTableTagFor
// - SharesPointerTablesWith
// - GetIsolate

pub struct IsolateForSandbox {
  /// 沙箱开启时为 isolate 指针槽位, 关闭时无字段
  _priv: (),
}

impl IsolateForSandbox {
  /// 沙箱开启分支: IsolateForSandbox(IsolateT*) 存指针
  /// 沙箱关闭分支: constexpr 空构造 + GetCurrentIsolateForSandbox() 返回 {}
  // TODO(dev): 补 trusted/cage base 指针读取实现。
}

// 原签名: V8_INLINE IsolateForSandbox GetCurrentIsolateForSandbox();

// —— 提取不变式断言 ——
const _: () = {
  let _i = IsolateForSandbox { _priv: () };
};
