// 由 tool/src/snapshot/deserializer.js 从 v8/src/snapshot/deserializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 反序列化器常量与方法面

/// 代码段是否内嵌对象指针 (原版宏 V8_CODE_EMBEDS_OBJECT_POINTER 机械展开):
/// 分支架构 MIPS64/S390X/PPC64/RISCV32/RISCV64 或 V8_EMBEDDED_CONSTANT_POOL → 1;
/// 目标架构 x64 + arm64 均不满足 → 0。其余架构跳过并在此登记。
pub const CODE_EMBEDS_OBJECT_POINTER: bool = false;

/// v8: src/snapshot/deserializer.h DeserializingUserCodeOption
/// 反序列化用户代码模式 (原版 enum class DeserializingUserCodeOption)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeserializingUserCodeOption {
  // kNotDeserializingUserCode
  NotDeserializingUserCode = 0,
  // kIsDeserializingUserCode
  IsDeserializingUserCode = 1,
}

// Deserializer 方法面 (原版 h, protected 区为主, 共 24 项;
// 实现于 deserializer.cc 1755 行, 语义由 Developer 落):
// - `DeserializeDeferredObjects`
// - `DeserializeUntrustedSection`
// - `DeserializeTrustedSection`
// - `LogNewObjectEvents`
// - `LogScriptEvents`
// - `LogNewMapEvents`
// - `PostProcessExposedTrustedObjects`
// - `GetBackReferencedObject`
// - `GetBackReferencedObject`
// - `AddAttachedObject`
// - `isolate`
// - `main_thread_isolate`
// - `source`
// - `new_allocation_sites`
// - `new_maps`
// - `accessor_infos`
// - `interceptor_infos`
// - `new_scripts`
// - `backing_store`
// - `deserializing_user_code`
// - `should_rehash`
// - `PushObjectToRehash`
// - `Rehash`
// - `ReadObject`

// 反序列化主线 (机械提取调用链登记):
// DeserializeUntrustedSection (untrusted 段含根, 先读) → DeserializeTrustedSection
// → DeserializeDeferredObjects → PostProcessExposedTrustedObjects → LogNewObjectEvents
// 同步哨兵: 段边界期望字节 = Bytecode::Synchronize (原版 static const uint8_t expected)。
// 热对象环: 容量 kHotObjectCount, 与 serializer.rs 的 hot_objects_list 共用布局。
