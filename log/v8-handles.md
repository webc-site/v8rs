# v8-handles 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 对数核对结论 (自 readme/zh.md 同名节迁入)

- 常量: kTaggedNullAddress=0x1, kCheckHandleThreshold=48*1024 (handles.h);
  kHandleBlockSize=KB-2 (handle-scope-implementer.h); kHandleZapValue=0x1baddead0baddeaf,
  kPersistentHandleZapValue=0x1baffed66baffedf (globals.h:1154/1156),
  kWeakHeapObjectTag=3 (globals.h/v8-internal.h) — 全部逐条命中
- 枚举: HeapObjectReferenceType 2 变体 WEAK/STRONG (globals.h 顺序 1:1)
- API 面: HandleScope 13 方法, Handle 9 方法, LocalHandles 7 面, LocalHandleScope 10 面,
  MaybeHandle 8 面, MaybeObjectHandle 6 面, PersistentHandles 13 面 + 数据成员 6/6,
  PersistentHandlesList 4 面, PersistentHandlesScope 4 面, SharedObjectConveyorHandles 4/4 面
  + 数据成员 2/2 + Persist id 语义 (push 前 size) — 全部命中

### 门禁实测 (自 readme/en.md「Verification」节补入, en.md 独有)

54 条语义测试覆盖全部规约; 门禁: clippy `-D warnings` 干净, nextest 54/54。

(zap 双常量另有对 v8_common 的编译期断言 — 属结构事实, 保留在 readme/en.md,
未迁入本台账。)
