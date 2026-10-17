// 由 tool/src/snapshot/static-roots-gen.js 从 v8/src/snapshot/static-roots-gen.{h,cc} 生成; 修订就地改, 复跑覆盖
// 静态根表生成器

// StaticRootsTableGen (原版 AllStatic 门面): 从运行中 isolate 落静态根表
// (V8_STATIC_ROOTS 构建产物, mksnapshot 专用)。
// 方法面:
// - `write` (原版 write): 落 C++ 头 (根偏移表; include guard 由输出文件名
//   推导 defname, 绝对值常量可跨文件复制)
// - `write_text` (原版 write_text): 落可读文本版
// 输出经 std::ofstream 二进制写; 具体表内容由 Developer 对齐 roots 产物落。
