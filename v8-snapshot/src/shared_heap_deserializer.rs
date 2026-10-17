// 由 tool/src/snapshot/shared-heap-deserializer.js 从 v8/src/snapshot/shared-heap-deserializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 共享堆反序列化器

// SharedHeapDeserializer (原版): 初始化共享 isolate 中未随 startup 快照
// 装载的对象。方法面 (原版 h, 共 2 项):
// - `DeserializeIntoIsolate`
// - `DeserializeStringTable`

// 契约: 按运行时旗标决定是否把共享堆对象装入 Isolate;
// cc 实现: DeserializeIntoIsolate, DeserializeStringTable (字符串表单独装载)。
