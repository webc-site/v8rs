#!/usr/bin/env bun
// v8/src/utils/sha-256.{h,cc} → v8-utils/src/sha256.rs
// 确定性转换: 从 sha-256.cc 提取 K[64] 常量表与 SHA256_init 初始 state,
// 算法体 (Transform/update/final) 为位运算直译模板; 语义修订就地改 src/, 复跑覆盖
import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/utils/sha-256.cc");

// ---- K[64] 常量表: 抓取 `static const uint32_t K[64] = {...};` 区间 ----
const kMatch = src.match(/static const uint32_t K\[64\] = \{([\s\S]*?)\};/);
if (!kMatch) throw new Error("sha-256.cc: K[64] 表未找到");
const K = kMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (K.length !== 64) throw new Error(`sha-256.cc: K 表长度 ${K.length} != 64`);
for (const k of K) if (!/^0x[0-9a-f]+$/.test(k)) throw new Error(`K 表非法项: ${k}`);

// ---- 初始 state: 抓取 SHA256_init 内 8 行 state 赋值 ----
const initMatch = src.match(/void SHA256_init[\s\S]*?void SHA256_update/);
if (!initMatch) throw new Error("sha-256.cc: SHA256_init 未找到");
const state = [
  ...initMatch[0].matchAll(/ctx->state\[(\d)\] = (0x[0-9a-f]+);/g),
].map((m) => m[2]);
if (state.length !== 8) throw new Error(`sha-256.cc: init state 长度 ${state.length} != 8`);

const body = `${banner("src/utils/sha-256.js", "src/utils/sha-256.h;src/utils/sha-256.cc")}
//! SHA-256 摘要, 移植自 v8/src/utils/sha-256 (源自 Omaha installer, 极简实现)。
//!
//! 上游为 C 风格 CTX + init/update/final 三段式; Rust 版聚合为 \`Sha256\` 结构,
//! 摘要定长 \`[u8; 32]\` (\`SHA256_final\` 返回 buf 前 32 字节的约定收敛为返回值)。

use super::utils::ror32;

/// 摘要字节数 (kSizeOfSha256Digest)
pub const SIZE_OF_SHA256_DIGEST: usize = 32;
/// 十六进制格式化后长度, 含结尾 NUL (kSizeOfFormattedSha256Digest)
pub const SIZE_OF_FORMATTED_SHA256_DIGEST: usize = SIZE_OF_SHA256_DIGEST * 2 + 1;

/// 轮常量表 K[64] (生成自 sha-256.cc)
const K: [u32; 64] = [
${K.map((k) => `  ${k},`).join("\n")}
];

/// 初始链接状态 (SHA256_init 中的 8 个魔数)
const INIT_STATE: [u32; 8] = [
${state.map((s) => `  ${s},`).join("\n")}
];

/// SHA-256 上下文 (LITE_SHA256_CTX)
#[derive(Clone)]
pub struct Sha256 {
  /// 已处理字节数
  count: u64,
  /// 输入缓冲, 满 64 字节触发一次压缩
  buf: [u8; 64],
  /// 链接状态
  state: [u32; 8],
}

impl Default for Sha256 {
  fn default() -> Self {
    Self::new()
  }
}

impl Sha256 {
  /// SHA256_init
  pub fn new() -> Self {
    Sha256 { count: 0, buf: [0; 64], state: INIT_STATE }
  }

  /// SHA256_update: 追加数据
  pub fn update(&mut self, data: &[u8]) {
    let mut i = (self.count & 63) as usize;
    self.count += data.len() as u64;
    for &b in data {
      self.buf[i] = b;
      i += 1;
      if i == 64 {
        self.transform();
        i = 0;
      }
    }
  }

  /// SHA256_final: 填充 + 长度大端写入, 输出 32 字节摘要
  pub fn finalize(mut self) -> [u8; SIZE_OF_SHA256_DIGEST] {
    let cnt = self.count << 3; // 位长
    // 0x80 结尾符 + 0 填充至 (count & 63) == 56
    self.update(&[0x80]);
    while self.count & 63 != 56 {
      self.update(&[0]);
    }
    // 位长按大端 8 字节追加 (上游逐字节取 cnt 高 8 位后左移, 等价)
    let mut c = cnt;
    let mut tail = [0u8; 8];
    for i in (0..8).rev() {
      tail[i] = c as u8;
      c >>= 8;
    }
    self.update(&tail);

    let mut digest = [0u8; SIZE_OF_SHA256_DIGEST];
    for (chunk, &s) in digest.chunks_exact_mut(4).zip(self.state.iter()) {
      chunk[0] = (s >> 24) as u8;
      chunk[1] = (s >> 16) as u8;
      chunk[2] = (s >> 8) as u8;
      chunk[3] = s as u8;
    }
    digest
  }

  /// SHA256_Transform: 单块 64 字节压缩 (消息扩展 W[64] + 64 轮)
  fn transform(&mut self) {
    let mut w = [0u32; 64];
    for (t, chunk) in self.buf.chunks_exact(4).enumerate() {
      w[t] = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
    }
    for t in 16..64 {
      let s0 = ror32(w[t - 15], 7) ^ ror32(w[t - 15], 18) ^ (w[t - 15] >> 3);
      let s1 = ror32(w[t - 2], 17) ^ ror32(w[t - 2], 19) ^ (w[t - 2] >> 10);
      w[t] = w[t - 16]
        .wrapping_add(s0)
        .wrapping_add(w[t - 7])
        .wrapping_add(s1);
    }

    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;
    for t in 0..64 {
      let s0 = ror32(a, 2) ^ ror32(a, 13) ^ ror32(a, 22);
      let maj = (a & b) ^ (a & c) ^ (b & c);
      let t2 = s0.wrapping_add(maj);
      let s1 = ror32(e, 6) ^ ror32(e, 11) ^ ror32(e, 25);
      let ch = (e & f) ^ (!e & g);
      let t1 = h
        .wrapping_add(s1)
        .wrapping_add(ch)
        .wrapping_add(K[t])
        .wrapping_add(w[t]);

      h = g;
      g = f;
      f = e;
      e = d.wrapping_add(t1);
      d = c;
      c = b;
      b = a;
      a = t1.wrapping_add(t2);
    }
    self.state[0] = self.state[0].wrapping_add(a);
    self.state[1] = self.state[1].wrapping_add(b);
    self.state[2] = self.state[2].wrapping_add(c);
    self.state[3] = self.state[3].wrapping_add(d);
    self.state[4] = self.state[4].wrapping_add(e);
    self.state[5] = self.state[5].wrapping_add(f);
    self.state[6] = self.state[6].wrapping_add(g);
    self.state[7] = self.state[7].wrapping_add(h);
  }
}

/// SHA256_hash: 一次性计算摘要
pub fn sha256_hash(data: &[u8]) -> [u8; SIZE_OF_SHA256_DIGEST] {
  let mut ctx = Sha256::new();
  ctx.update(data);
  ctx.finalize()
}
`;

writeSrc("v8-utils", "sha256.rs", body);
console.log("sha256.rs: K=64 state=8 ✓");
