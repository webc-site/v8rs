// 由 tool/src/snapshot/read-only-deserializer.js 从 v8/src/snapshot/read-only-deserializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 只读空间反序列化器

// ReadOnlyDeserializer (原版, 继承 Deserializer<Isolate>): 反序列化 RO blob,
// 建立只读根表。方法面 (原版 h, 共 2 项):
// - `DeserializeIntoIsolate`
// - `PostProcessNewObjects`

// cc 实现 (479 行, 机械清点): DeserializeIntoIsolate, PostProcessNewObjects
// 分支登记: V8_STATIC_ROOTS x0 — 静态根模式下 RO 页直接按
// 预编译常量表落位, 跳过重定位; 否则走 EncodedTagged 重定位路径。
