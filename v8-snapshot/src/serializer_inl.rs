// 由 tool/src/snapshot/serializer-inl.js 从 v8/src/snapshot/serializer-inl.h 生成; 修订就地改, 复跑覆盖
// 序列化器内联函数面

// 内联函数面 (原版 serializer-inl.h, 共 1 项):
// - `IsNotMappedSymbol`: 判定对象是否非 not_mapped_symbol 哨兵。
//   分支登记: V8_EXTERNAL_CODE_SPACE_BOOL 为真时按完整指针比较
//   (压缩态 InstructionStream 可能与哨兵压缩值同形), 否则按压缩指针比较。
