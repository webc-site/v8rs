#!/usr/bin/env bun
// v8/src/utils 各文件的散点常量/枚举/位标志 → v8-utils/src/<模块>.rs 基线
// 确定性转换: 正则提取上游数值常量与枚举, 语义体留 TODO 由手工修订;
// 修订就地改 src/, 复跑覆盖 (仅限重生成基线)
import { readV8, writeSrc, banner } from "../../lib.js";

// ---- 提取器: 抓首个 `static constexpr <type> <name> = <expr>;` / `const <type> <name> = <expr>;` ----
function grab(src, re, what) {
  const m = src.match(re);
  if (!m) throw new Error(`常量未找到: ${what}`);
  return m[1];
}

// ============ bit-vector.h → bit_vector.rs ============
{
  const h = readV8("src/utils/bit-vector.h");
  // kDataBits = kBitsPerSystemPointer (64 位平台 64); kDataBitShift = ...Log2 (6)
  const body = `${banner("src/utils/consts.js", "src/utils/bit-vector.h;src/utils/bit-vector.cc")}
//! 定长位向量, 移植自 v8/src/utils/bit-vector。
//!
//! 上游以 Zone 分配 + 单字 inline 存储; Rust 版收敛为 \`Vec<usize>\` 承载
//! (data 长度恒 >= 1, 与上游 inline 槽位约定一致), Zone 参数从 API 中移除。

/// 单个机器字位数 (kDataBits = kBitsPerSystemPointer)
pub const DATA_BITS: u32 = usize::BITS;
/// 位数以 2 为底的对数 (kDataBitShift)
pub const DATA_BIT_SHIFT: u32 = DATA_BITS.trailing_zeros();
`;
  writeSrc("v8-utils", "bit_vector.rs", body);
}

// ============ sparse-bit-vector.h → sparse_bit_vector.rs ============
{
  const h = readV8("src/utils/sparse-bit-vector.h");
  const words = grab(h, /kNumWordsPerSegment = (\d+)/, "kNumWordsPerSegment");
  const body = `${banner("src/utils/consts.js", "src/utils/sparse-bit-vector.h")}
//! 稀疏位向量 (小规模优化), 移植自 v8/src/utils/sparse-bit-vector。
//!
//! 上游 Segment 由 Zone 分配; Rust 版链表节点走 Box, Zone 参数从 API 中移除。

/// 每 Segment 字数 (kNumWordsPerSegment)
pub const NUM_WORDS_PER_SEGMENT: usize = ${words};
/// 单字位数 (kBitsPerWord = kBitsPerByte * kSystemPointerSize)
pub const BITS_PER_WORD: u32 = usize::BITS;
/// 每 Segment 位数 (kNumBitsPerSegment)
pub const NUM_BITS_PER_SEGMENT: u32 = BITS_PER_WORD * NUM_WORDS_PER_SEGMENT as u32;
`;
  writeSrc("v8-utils", "sparse_bit_vector.rs", body);
}

// ============ detachable-vector.{h,cc} → detachable_vector.rs ============
{
  const cc = readV8("src/utils/detachable-vector.cc");
  const min = grab(cc, /kMinimumCapacity = (\d+)/, "kMinimumCapacity");
  const body = `${banner("src/utils/consts.js", "src/utils/detachable-vector.h;src/utils/detachable-vector.cc")}
//! 可分离向量, 移植自 v8/src/utils/detachable-vector。
//!
//! 上游手工管理裸指针 backing store, detach() 仅切断引用不释放内存
//! (所有权转移给位拷贝出的另一实例); Rust 版 backing store 用
//! \`MaybeUninit\` 槽位 + 裸指针边界实现同一语义。

/// 初始容量 (kMinimumCapacity)
pub const MINIMUM_CAPACITY: usize = ${min};
`;
  writeSrc("v8-utils", "detachable_vector.rs", body);
}

// ============ identity-map.cc → identity_map.rs ============
{
  const cc = readV8("src/utils/identity-map.cc");
  const init = grab(cc, /kInitialIdentityMapSize = (\d+)/, "kInitialIdentityMapSize");
  const factor = grab(cc, /kResizeFactor = (\d+)/, "kResizeFactor");
  const body = `${banner("src/utils/consts.js", "src/utils/identity-map.h;src/utils/identity-map.cc")}
//! 身份映射 (地址 → 值), 移植自 v8/src/utils/identity-map。
//!
//! 上游键为堆对象地址, not_mapped 哨兵取 ReadOnlyRoots::not_mapped_symbol 地址;
//! Rust 版独立运行, 哨兵取 \`NOT_MAPPED\` (0 地址, 真实指针恒非 0)。
//! GC 失效 (invalidated) / 惰性 rehash / 80% 负载扩容 / 删除回填语义 1:1 保留。

/// 初始桶数 (kInitialIdentityMapSize)
const INITIAL_SIZE: usize = ${init};
/// 扩缩因子 (kResizeFactor)
const RESIZE_FACTOR: usize = ${factor};
`;
  writeSrc("v8-utils", "identity_map.rs", body);
}

// ============ boxed-float.h + globals.h → boxed_float.rs ============
{
  const h = readV8("src/utils/boxed-float.h");
  const expShift = grab(h, /kExponentShift = (\d+)/, "kExponentShift");
  const quietBit = grab(h, /kQuietNanBit = (1 << \d+)/, "kQuietNanBit");
  const signBit = grab(h, /kSignBit = (1 << \d+)/, "kSignBit");
  const g = readV8("src/common/globals.h");
  // globals.h 先给 MIPS64 特例再给 #else 通用值, 取最后一组 (非 MIPS 默认)
  const all = (re) => [...g.matchAll(re)].map((m) => m[1]);
  const holeUp = all(/kHoleNanUpper32 = (0x[0-9A-Fa-f]+)/g);
  const holeLo = all(/kHoleNanLower32 = (0x[0-9A-Fa-f]+)/g);
  const undefUp = all(/kUndefinedNanUpper32 = (0x[0-9A-Fa-f]+)/g);
  const undefLo = all(/kUndefinedNanLower32 = (0x[0-9A-Fa-f]+)/g);
  if (![holeUp, holeLo, undefUp, undefLo].every((a) => a.length >= 2)) {
    throw new Error("globals.h: hole/undefined nan 常量不足");
  }
  const holeUpper = holeUp.at(-1), holeLower = holeLo.at(-1);
  const undefUpper = undefUp.at(-1), undefLower = undefLo.at(-1);
  const body = `${banner("src/utils/consts.js", "src/utils/boxed-float.h;src/common/globals.h")}
//! 位模式安全浮点包装 (Float32/Float64), 移植自 v8/src/utils/boxed-float。
//!
//! 去优化传参时保持 NaN 位模式不被静默改写。

/// Float32 指数域右移位数
const EXPONENT_SHIFT: u32 = ${expShift};
/// Float32 quiet-NaN 标志位
const QUIET_NAN_BIT: u32 = ${quietBit.replace("1 <<", "1 <<")};
/// Float32 符号位
const SIGN_BIT: u32 = ${signBit.replace("1 <<", "1 <<")};
/// hole NaN 位模式 (kHoleNanInt64, globals.h)
pub const HOLE_NAN_INT64: u64 = ((${holeUpper}u64) << 32) | ${holeLower}u64;
/// undefined NaN 位模式 (kUndefinedNanInt64, V8_ENABLE_UNDEFINED_DOUBLE)
pub const UNDEFINED_NAN_INT64: u64 = ((${undefUpper}u64) << 32) | ${undefLower}u64;
`;
  writeSrc("v8-utils", "boxed_float.rs", body);
}

// ============ allocation.h/.cc → allocation.rs ============
{
  const h = readV8("src/utils/allocation.h");
  const cc = readV8("src/utils/allocation.cc");
  const tries = grab(cc, /kAllocationTries = (\d+)/, "kAllocationTries");
  const body = `${banner("src/utils/consts.js", "src/utils/allocation.h;src/utils/allocation.cc")}
//! 内存分配辅助, 移植自 v8/src/utils/allocation 的可独立子集。
//!
//! 依赖 v8::PageAllocator 平台层的部分 (VirtualMemory/VirtualMemoryCage/
//! AllocatePages 等) 归 v8-base platform, 见 README 映射表。

/// 分配重试次数 (每次失败触发 OnCriticalMemoryPressure 释放内存)
pub const ALLOCATION_TRIES: usize = ${tries};

/// 地址空间保留是否用于可执行页 (JitPermission)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JitPermission {
  NoJit,
  MapAsJittable,
}
`;
  writeSrc("v8-utils", "allocation.rs", body);
}

// ============ ostreams.h → ostreams.rs ============
{
  const body = `${banner("src/utils/consts.js", "src/utils/ostreams.h;src/utils/ostreams.cc")}
//! 输出格式化包装器, 移植自 v8/src/utils/ostreams。
//!
//! 上游围绕 std::ostream 的 operator<<; Rust 版将 AsHex/AsUC16 等包装结构
//! 实现为 \`Display\`。FILE*/调试器流 (OFStream/DbgStreamBuf/AndroidLogStream)
//! 在 Rust 中由 std::io 承担, 见 README 映射表。

/// AsHexBytes 的字节序 (ByteOrder)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ByteOrder {
  /// 小端 (默认)
  #[default]
  LittleEndian,
  /// 大端
  BigEndian,
}
`;
  writeSrc("v8-utils", "ostreams.rs", body);
}

// ============ hashing.h + rapidhash secret → utils.rs 散列区块 ============
{
  const secret = readV8("third_party/rapidhash-v8/secret.h");
  // hashing.h: kRapidhashSecret1 = SECRET[0], kRapidhashSecret2 = SECRET[1]
  const secMatch = secret.match(/RAPIDHASH_DEFAULT_SECRET\[3\] = \{\s*(0x[0-9a-f]+)ull,\s*(0x[0-9a-f]+)ull,\s*(0x[0-9a-f]+)ull/);
  if (!secMatch) throw new Error("secret.h: RAPIDHASH_DEFAULT_SECRET 未找到");
  const s1 = secMatch[1], s2 = secMatch[2];
  const hh = readV8("src/base/hashing.h");
  const m = grab(hh, /uint64_t m = uint64_t\{(0x[0-9A-Fa-f]+)\}/, "hash_combine m");
  const r = grab(hh, /const uint32_t r = (\d+)/, "hash_combine r");
  const g = readV8("src/common/globals.h");
  // kSmiValueSize = 31 (64 位), kSmiHashMask = (1 << 30) - 1; 从公式直算
  const body = `${banner("src/utils/consts.js", "src/base/hashing.h;third_party/rapidhash-v8/secret.h;src/common/globals.h")}
//! 通用工具函数集, 移植自 v8/src/utils/utils.{h,cc,inl}。
//!
//! 含 base/hashing 的整数散列原语 (hash64/hash32/hash_combine/Hasher) 就地副本,
//! 算法与 v8 base/hashing.h + rapidhash 保持位级一致 (Map/Set 哈希需跨实现一致)。

/// rapidhash "mum" 混合器 secret1 (kRapidhashSecret1)
const RAPIDHASH_SECRET1: u64 = ${s1};
/// rapidhash "mum" 混合器 secret2 (kRapidhashSecret2)
const RAPIDHASH_SECRET2: u64 = ${s2};
/// hash_combine 64 位乘子 (MurmurHash 尾部混合)
const HASH_COMBINE_M: u64 = ${m};
/// hash_combine 右移位数
const HASH_COMBINE_R: u32 = ${r};
/// Smi 整数散列掩码 (kSmiHashMask, kSmiValueSize-1 位 → 非负 Smi 载荷)
pub const SMI_HASH_MASK: u32 = (1u32 << 30) - 1;

/// 32 位循环右移 (bits::RotateRight32)
#[inline]
pub const fn ror32(v: u32, s: u32) -> u32 {
  v.rotate_right(s)
}

/// rapidhash "mum" 混合器: 128 位乘积高低半异或 (rapid_mix, rapidhash.h:204)
#[inline]
const fn rapid_mix(a: u64, b: u64) -> u64 {
  let p = (a as u128) * (b as u128);
  (p as u64) ^ ((p >> 64) as u64)
}

/// 规范整数散列 hash64: rapid_mix((key^s1), (key^s2)) (hashing.h:179, 64 位支)
#[inline]
pub const fn hash64(key: u64) -> u64 {
  rapid_mix(key ^ RAPIDHASH_SECRET1, key ^ RAPIDHASH_SECRET2)
}

/// 规范整数散列 hash32: hash64 截断低 32 位 (hashing.h:196, 64 位支)
#[inline]
pub const fn hash32(key: u32) -> u32 {
  hash64(key as u64) as u32
}

/// 双值合并散列 (MurmurHash 尾部混合, hashing.h:80, 64 位支)
#[inline]
pub const fn hash_combine(seed: usize, hash: usize) -> usize {
  let m = HASH_COMBINE_M as usize;
  let mut h = hash.wrapping_mul(m);
  h ^= h >> HASH_COMBINE_R;
  h = h.wrapping_mul(m);
  (seed ^ h).wrapping_mul(m)
}

/// 多字段增量散列器 (base::Hasher, hashing.h:112)
///
/// 语义与 C++ 逐一对应: 有符号整型 bit_cast 无符号后散列,
/// 单字节类型 (bool/u8/u16) 直接取值不散列 (hashing.h:218-221 trivial 分支)。
#[derive(Debug, Clone, Copy)]
pub struct Hasher {
  hash: usize,
}

impl Default for Hasher {
  fn default() -> Self {
    Self::new()
  }
}

impl Hasher {
  /// 默认零种子构造
  pub const fn new() -> Self {
    Self { hash: 0 }
  }

  /// 指定种子构造
  pub const fn with_seed(seed: usize) -> Self {
    Self { hash: seed }
  }

  /// 当前散列值
  pub const fn hash(&self) -> usize {
    self.hash
  }

  /// 直接合并一个既有散列值 (AddHash)
  pub fn add_hash(&mut self, other: usize) -> &mut Self {
    self.hash = hash_combine(self.hash, other);
    self
  }

  /// 混入 i32: bit_cast 到 u32 后 hash32 (hashing.h:236 signed 分支)
  pub fn add_i32(&mut self, v: i32) -> &mut Self {
    self.add_hash(hash32(v as u32) as usize)
  }

  /// 混入 u64 (hash_value(unsigned long long))
  pub fn add_u64(&mut self, v: u64) -> &mut Self {
    self.add_hash(hash64(v) as usize)
  }

  /// 逐字节混入字节串 (AddRange + OneByteVector; u8 走 trivial 分支直接取值)
  pub fn add_bytes(&mut self, bytes: &[u8]) -> &mut Self {
    for b in bytes {
      self.add_hash(*b as usize);
    }
    self
  }
}
`;
  writeSrc("v8-utils", "utils.rs", body);
}

console.log("consts.js: bit_vector/sparse_bit_vector/detachable_vector/identity_map/boxed_float/allocation/ostreams/utils 基线 ✓");
