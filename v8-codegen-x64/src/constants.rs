// 由 tool/src/codegen/x64/constants-x64.h.js 从 v8/src/codegen/x64/constants-x64.h 生成; 修订就地改, 复跑覆盖
// wave1 纯数据面: 零依赖常量表。

// kRootRegister 实际值相对 IsolateData 起点带偏移, 以利用负位移;
// x64 最小操作数编码允许 int8 偏移, 故 bias 使 IsolateData 前 32 个 8 字节槽可按此编码。
pub const K_ROOT_REGISTER_BIAS: i32 = 128;

// 代码范围最大值, 保证范围内所有 Code 对象之间可用 pc 相对调用。
pub const K_MAX_PC_RELATIVE_CODE_RANGE_IN_MB: usize = 2048;

const _: () = assert!(K_ROOT_REGISTER_BIAS == 128); // 金标准: kRootRegisterBias = 128
const _: () = assert!(K_MAX_PC_RELATIVE_CODE_RANGE_IN_MB == 2048); // 金标准: kMaxPCRelativeCodeRangeInMB = 2048
