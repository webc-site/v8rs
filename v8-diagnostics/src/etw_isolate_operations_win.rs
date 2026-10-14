// 由 tool/src/diagnostics/etw-isolate-operations-win.h.js 从 v8/src/diagnostics/etw-isolate-operations-win.{h,cc} 生成; 修订就地改, 复跑覆盖
// ETW isolate 操作接口骨架

// Windows 平台模块 (ETW), 接入时建议 #[cfg(windows)]。
// 语义面: 虚方法实现 (中断/代码事件句柄/URL 过滤回调) 登记跳过。

use std::sync::Mutex;

/// 原 class EtwIsolateOperations — 可注入虚接口 (测试替换 Instance)
pub trait EtwIsolateOperations {
  // 原签名: public: virtual void SetEtwCodeEventHandler(Isolate* isolate, uint32_t options)
  // 原签名: void ResetEtwCodeEventHandler(Isolate* isolate)
  // 原签名: FilterETWSessionByURLResult RunFilterETWSessionByURLCallback( Isolate* isolate, const std::string& payload)
  // 原签名: void RequestInterrupt(Isolate* isolate, InterruptCallback callback, void* data)
  // 原签名: bool HeapReadOnlySpaceWritable(Isolate* isolate)
  // 原签名: std::optional<Tagged<GcSafeCode>> HeapGcSafeTryFindCodeForInnerPointer(Isolate* isolate, Address address)
}

/// 原 EtwIsolateOperations::Instance() 全局单例槽
pub static INSTANCE: Mutex<Option<()>> = Mutex::new(None);

// 单例操作原签名:
// static EtwIsolateOperations* Instance()
// static void SetInstanceForTesting( EtwIsolateOperations* etw_isolate_operations)

// —— 提取不变式断言 ——
const _: () = {
  // 虚方法 6 条 + Instance/SetInstanceForTesting 2 静态
};
