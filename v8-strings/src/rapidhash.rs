// 由 tool/src/strings/string-hasher.h.js 从 v8/third_party/rapidhash-v8/rapidhash.h 生成; 修订就地改, 复跑覆盖
// 字符串 hash 语义绑定 (seed+迭代序), 1:1 自写, 禁库替换

//! rapidhash — wyhash 派生的高速哈希; u128 乘法内核。
//! 偏移算术: 产出侧字节偏移直接作切片索引 — PlainReader 输入 u8 (索引=字节偏移),
//! ConvertTo8BitReader 输入 u16 (压缩 2:1, 索引=产出偏移); 与原版 `off * x / y` 等价。

/// 读取器: 从输入单元流产出哈希位, 支持 16→8 位压缩 (Latin-1 内容的双字节串
/// 与单字节串哈希一致) 或原样读取。
pub trait HashReader<C> {
  /// 产出 64 bit。
  fn read64(p: &[C]) -> u64;
  /// 产出 32 bit (返回 u64, 哈希器自由移位)。
  fn read32(p: &[C]) -> u64;
  /// 读 1-3 单元并散布到 64 bit。
  fn read_small(p: &[C], k: usize) -> u64;
}

/// 原样字节读取 (PlainHashReader)。
pub struct PlainReader;
impl HashReader<u8> for PlainReader {
  #[inline]
  fn read64(p: &[u8]) -> u64 {
    let mut b = [0u8; 8];
    b.copy_from_slice(&p[..8]);
    u64::from_le_bytes(b)
  }

  #[inline]
  fn read32(p: &[u8]) -> u64 {
    let mut b = [0u8; 4];
    b.copy_from_slice(&p[..4]);
    u64::from(u32::from_le_bytes(b))
  }

  #[inline]
  fn read_small(p: &[u8], k: usize) -> u64 {
    (u64::from(p[0]) << 56) | (u64::from(p[k >> 1]) << 32) | u64::from(p[k - 1])
  }
}

/// Latin-1 压缩读取 (ConvertTo8BitHashReader): 每 u16 取低字节。
pub struct ConvertTo8BitReader;
impl HashReader<u16> for ConvertTo8BitReader {
  #[inline]
  fn read64(p: &[u16]) -> u64 {
    // 调用方 IsOnly8Bit 预检: 全部 ≤ 0xFF, 低字节即完整值
    let mut b = [0u8; 8];
    for (i, o) in b.iter_mut().enumerate() {
      *o = p[i] as u8;
    }
    u64::from_le_bytes(b)
  }

  #[inline]
  fn read32(p: &[u16]) -> u64 {
    let mut b = [0u8; 4];
    for (i, o) in b.iter_mut().enumerate() {
      *o = p[i] as u8;
    }
    u64::from(u32::from_le_bytes(b))
  }

  #[inline]
  fn read_small(p: &[u16], k: usize) -> u64 {
    (u64::from(p[0] as u8) << 56) | (u64::from(p[k >> 1] as u8) << 32) | u64::from(p[k - 1] as u8)
  }
}

/// 64×64 → 128 乘法 (rapid_mul128)。
#[inline]
fn mul128(a: u64, b: u64) -> (u64, u64) {
  let r = u128::from(a) * u128::from(b);
  (r as u64, (r >> 64) as u64)
}

/// 乘 xor 混合 (rapid_mix): 128 积高低半异或。
#[inline]
pub fn mix(a: u64, b: u64) -> u64 {
  let (lo, hi) = mul128(a, b);
  lo ^ hi
}

/// rapidhash 主体; len 为产出侧字节数 (压缩时 = u16 单元数)。
/// 数据流与指针隔离: a/b/seed/secret 与 see1/see2 仅受输入数据影响。
pub fn rapidhash<C, R: HashReader<C>>(
  p: &[C],
  len: usize,
  mut seed: u64,
  secret: &[u64; 3],
) -> u64 {
  seed ^= mix(seed ^ secret[0], secret[1]) ^ len as u64;
  let (mut a, mut b);
  if len <= 16 {
    if len >= 4 {
      // 首尾 32 bit (可重叠)
      let last = len - 4;
      a = (R::read32(p) << 32) | R::read32(&p[last..]);
      // delta = (len >= 8) ? 4 : 0
      let delta = (len & 24) >> (len >> 3);
      b = (R::read32(&p[delta..]) << 32) | R::read32(&p[last - delta..]);
    } else if len > 0 {
      a = R::read_small(p, len);
      b = 0;
    } else {
      a = 0;
      b = 0;
    }
  } else {
    let mut i = len;
    let mut off = 0usize;
    if i > 48 {
      let mut see1 = seed;
      let mut see2 = seed;
      loop {
        seed = mix(
          R::read64(&p[off..]) ^ secret[0],
          R::read64(&p[off + 8..]) ^ seed,
        );
        see1 = mix(
          R::read64(&p[off + 16..]) ^ secret[1],
          R::read64(&p[off + 24..]) ^ see1,
        );
        see2 = mix(
          R::read64(&p[off + 32..]) ^ secret[2],
          R::read64(&p[off + 40..]) ^ see2,
        );
        off += 48;
        i -= 48;
        if i < 48 {
          break;
        }
      }
      seed ^= see1 ^ see2;
    }
    if i > 16 {
      seed = mix(
        R::read64(&p[off..]) ^ secret[2],
        R::read64(&p[off + 8..]) ^ seed ^ secret[1],
      );
      if i > 32 {
        seed = mix(
          R::read64(&p[off + 16..]) ^ secret[2],
          R::read64(&p[off + 24..]) ^ seed,
        );
      }
    }
    // 末尾 2×64 bit
    a = R::read64(&p[off + i - 16..]);
    b = R::read64(&p[off + i - 8..]);
  }
  a ^= secret[1];
  b ^= seed;
  let (lo, hi) = mul128(a, b);
  mix(lo ^ secret[0] ^ len as u64, hi ^ secret[1])
}
