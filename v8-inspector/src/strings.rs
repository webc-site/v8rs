// 由 tool/src/inspector/strings.js 从 v8/src/inspector/string-16.h, src/inspector/string-16.cc, src/inspector/string-util.h, src/inspector/string-util.cc, src/inspector/crc32.h, src/inspector/crc32.cc, src/inspector/search-util.h, src/inspector/search-util.cc, src/inspector/v8-regex.h, src/inspector/v8-regex.cc, src/inspector/v8-string-conversions.h, src/inspector/v8-string-conversions.cc, src/inspector/v8-value-utils.h, src/inspector/v8-value-utils.cc 生成; 修订就地改, 复跑覆盖
// UTF-16 协议字符串层 + IEEE 802.3 CRC32 + 源码魔术注释定位

use crate::error::{Error, Result};

/// 未匹配索引常数 (v8/src/inspector/string-16.h:35)
pub const K_STRING16_NOT_FOUND: usize = usize::MAX;

/// 1:1 转写 v8/src/inspector/crc32.cc 中的标准多项式 0xedb88320 表
pub const CRC32_TABLE: [u32; 256] = [
  0x00000000, 0x77073096, 0xee0e612c, 0x990951ba, 0x076dc419, 0x706af48f, 0xe963a535, 0x9e6495a3,
  0x0edb8832, 0x79dcb8a4, 0xe0d5e91e, 0x97d2d988, 0x09b64c2b, 0x7eb17cbd, 0xe7b82d07, 0x90bf1d91,
  0x1db71064, 0x6ab020f2, 0xf3b97148, 0x84be41de, 0x1adad47d, 0x6ddde4eb, 0xf4d4b551, 0x83d385c7,
  0x136c9856, 0x646ba8c0, 0xfd62f97a, 0x8a65c9ec, 0x14015c4f, 0x63066cd9, 0xfa0f3d63, 0x8d080df5,
  0x3b6e20c8, 0x4c69105e, 0xd56041e4, 0xa2677172, 0x3c03e4d1, 0x4b04d447, 0xd20d85fd, 0xa50ab56b,
  0x35b5a8fa, 0x42b2986c, 0xdbbbc9d6, 0xacbcf940, 0x32d86ce3, 0x45df5c75, 0xdcd60dcf, 0xabd13d59,
  0x26d930ac, 0x51de003a, 0xc8d75180, 0xbfd06116, 0x21b4f4b5, 0x56b3c423, 0xcfba9599, 0xb8bda50f,
  0x2802b89e, 0x5f058808, 0xc60cd9b2, 0xb10be924, 0x2f6f7c87, 0x58684c11, 0xc1611dab, 0xb6662d3d,
  0x76dc4190, 0x01db7106, 0x98d220bc, 0xefd5102a, 0x71b18589, 0x06b6b51f, 0x9fbfe4a5, 0xe8b8d433,
  0x7807c9a2, 0x0f00f934, 0x9609a88e, 0xe10e9818, 0x7f6a0dbb, 0x086d3d2d, 0x91646c97, 0xe6635c01,
  0x6b6b51f4, 0x1c6c6162, 0x856530d8, 0xf262004e, 0x6c0695ed, 0x1b01a57b, 0x8208f4c1, 0xf50fc457,
  0x65b0d9c6, 0x12b7e950, 0x8bbeb8ea, 0xfcb9887c, 0x62dd1ddf, 0x15da2d49, 0x8cd37cf3, 0xfbd44c65,
  0x4db26158, 0x3ab551ce, 0xa3bc0074, 0xd4bb30e2, 0x4adfa541, 0x3dd895d7, 0xa4d1c46d, 0xd3d6f4fb,
  0x4369e96a, 0x346ed9fc, 0xad678846, 0xda60b8d0, 0x44042d73, 0x33031de5, 0xaa0a4c5f, 0xdd0d7cc9,
  0x5005713c, 0x270241aa, 0xbe0b1010, 0xc90c2086, 0x5768b525, 0x206f85b3, 0xb966d409, 0xce61e49f,
  0x5edef90e, 0x29d9c998, 0xb0d09822, 0xc7d7a8b4, 0x59b33d17, 0x2eb40d81, 0xb7bd5c3b, 0xc0ba6cad,
  0xedb88320, 0x9abfb3b6, 0x03b6e20c, 0x74b1d29a, 0xead54739, 0x9dd277af, 0x04db2615, 0x73dc1683,
  0xe3630b12, 0x94643b84, 0x0d6d6a3e, 0x7a6a5aa8, 0xe40ecf0b, 0x9309ff9d, 0x0a00ae27, 0x7d079eb1,
  0xf00f9344, 0x8708a3d2, 0x1e01f268, 0x6906c2fe, 0xf762575d, 0x806567cb, 0x196c3671, 0x6e6b06e7,
  0xfed41b76, 0x89d32be0, 0x10da7a5a, 0x67dd4acc, 0xf9b9df6f, 0x8ebeeff9, 0x17b7be43, 0x60b08ed5,
  0xd6d6a3e8, 0xa1d1937e, 0x38d8c2c4, 0x4fdff252, 0xd1bb67f1, 0xa6bc5767, 0x3fb506dd, 0x48b2364b,
  0xd80d2bda, 0xaf0a1b4c, 0x36034af6, 0x41047a60, 0xdf60efc3, 0xa867df55, 0x316e8eef, 0x4669be79,
  0xcb61b38c, 0xbc66831a, 0x256fd2a0, 0x5268e236, 0xcc0c7795, 0xbb0b4703, 0x220216b9, 0x5505262f,
  0xc5ba3bbe, 0xb2bd0b28, 0x2bb45a92, 0x5cb36a04, 0xc2d7ffa7, 0xb5d0cf31, 0x2cd99e8b, 0x5bdeae1d,
  0x9b64c2b0, 0xec63f226, 0x756aa39c, 0x026d930a, 0x9c0906a9, 0xeb0e363f, 0x72076785, 0x05005713,
  0x95bf4a82, 0xe2b87a14, 0x7bb12bae, 0x0cb61b38, 0x92d28e9b, 0xe5d5be0d, 0x7cdcefb7, 0x0bdbdf21,
  0x86d3d2d4, 0xf1d4e242, 0x68ddb3f8, 0x1fda836e, 0x81be16cd, 0xf6b9265b, 0x6fb077e1, 0x18b74777,
  0x88085ae6, 0xff0f6a70, 0x66063bca, 0x11010b5c, 0x8f659eff, 0xf862ae69, 0x616bffd3, 0x166ccf45,
  0xa00ae278, 0xd70dd2ee, 0x4e048354, 0x3903b3c2, 0xa7672661, 0xd06016f7, 0x4969474d, 0x3e6e77db,
  0xaed16a4a, 0xd9d65adc, 0x40df0b66, 0x37d83bf0, 0xa9bcae53, 0xdebb9ec5, 0x47b2cf7f, 0x30b5ffe9,
  0xbdbdf21c, 0xcabac28a, 0x53b39330, 0x24b4a3a6, 0xbad03605, 0xcdd70693, 0x54de5729, 0x23d967bf,
  0xb3667a2e, 0xc4614ab8, 0x5d681b02, 0x2a6f2b94, 0xb40bbe37, 0xc30c8ea1, 0x5a05df1b, 0x2d02ef8d,
];

/// UTF-16 协议字符串容器 (v8/src/inspector/string-16.h:24)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct String16 {
  data: Vec<u16>,
}

impl String16 {
  /// 创建空 String16
  #[inline]
  pub const fn new() -> Self {
    Self { data: Vec::new() }
  }

  /// 从 Rust &str 构造
  #[inline]
  pub fn from_str_slice(s: &str) -> Self {
    Self {
      data: s.encode_utf16().collect(),
    }
  }

  /// 从 &[u16] 构造
  #[inline]
  pub fn from_u16_slice(slice: &[u16]) -> Self {
    Self {
      data: slice.to_vec(),
    }
  }

  /// 获取底层 UTF-16 切片
  #[inline]
  pub fn characters16(&self) -> &[u16] {
    &self.data
  }

  /// 字符单元长度
  #[inline]
  pub fn length(&self) -> usize {
    self.data.len()
  }

  /// 是否为空
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  /// 转换为 Rust String
  #[inline]
  pub fn to_string_lossy(&self) -> String {
    String::from_utf16_lossy(&self.data)
  }

  /// 截取子串 (pos, len)
  pub fn substring(&self, start: usize, len: usize) -> Self {
    if start >= self.data.len() {
      return Self::new();
    }
    let end = start.saturating_add(len).min(self.data.len());
    Self {
      data: self.data[start..end].to_vec(),
    }
  }

  /// 从 start 截取到结尾
  pub fn substring_from(&self, start: usize) -> Self {
    if start >= self.data.len() {
      return Self::new();
    }
    Self {
      data: self.data[start..].to_vec(),
    }
  }

  /// 正向查找单字符
  pub fn find(&self, ch: u16, start: usize) -> usize {
    if start >= self.data.len() {
      return K_STRING16_NOT_FOUND;
    }
    for (i, &item) in self.data.iter().enumerate().skip(start) {
      if item == ch {
        return i;
      }
    }
    K_STRING16_NOT_FOUND
  }

  /// 正向查找子串
  pub fn find_str(&self, sub: &str, start: usize) -> usize {
    let sub_u16: Vec<u16> = sub.encode_utf16().collect();
    if sub_u16.is_empty() {
      return if start <= self.data.len() {
        start
      } else {
        K_STRING16_NOT_FOUND
      };
    }
    if start >= self.data.len() || self.data.len() - start < sub_u16.len() {
      return K_STRING16_NOT_FOUND;
    }
    for i in start..=(self.data.len() - sub_u16.len()) {
      if self.data[i..i + sub_u16.len()] == sub_u16[..] {
        return i;
      }
    }
    K_STRING16_NOT_FOUND
  }

  /// 反向查找子串
  pub fn reverse_find(&self, sub: &str, start: usize) -> usize {
    let sub_u16: Vec<u16> = sub.encode_utf16().collect();
    if sub_u16.is_empty() {
      return start.min(self.data.len());
    }
    if sub_u16.len() > self.data.len() {
      return K_STRING16_NOT_FOUND;
    }
    let max_start = (self.data.len() - sub_u16.len()).min(start);
    for i in (0..=max_start).rev() {
      if self.data[i..i + sub_u16.len()] == sub_u16[..] {
        return i;
      }
    }
    K_STRING16_NOT_FOUND
  }

  /// 去除首尾空白字符 (对齐 V8 isSpaceOrNewLine 0x09..=0x0d 与 0x20)
  pub fn strip_white_space(&self) -> Self {
    let is_ws = |c: u16| c == 0x20 || (0x09..=0x0d).contains(&c);
    let mut start = 0;
    while start < self.data.len() && is_ws(self.data[start]) {
      start += 1;
    }
    let mut end = self.data.len();
    while end > start && is_ws(self.data[end - 1]) {
      end -= 1;
    }
    Self {
      data: self.data[start..end].to_vec(),
    }
  }

  /// 拼接多串
  pub fn concat(parts: &[&Self]) -> Self {
    let total_len: usize = parts.iter().map(|p| p.length()).sum();
    let mut data = Vec::with_capacity(total_len);
    for p in parts {
      data.extend_from_slice(&p.data);
    }
    Self { data }
  }

  /// 从 i32 构造
  pub fn from_integer(val: i32) -> Self {
    Self::from_str_slice(&val.to_string())
  }

  /// 从 i64 构造
  pub fn from_integer64(val: i64) -> Self {
    Self::from_str_slice(&val.to_string())
  }

  /// 解析为 i32
  pub fn to_integer(&self) -> Result<i32> {
    let s = self.to_string_lossy();
    s.trim()
      .parse::<i32>()
      .map_err(|e| Error::InvalidParameter(e.to_string()))
  }

  /// 解析为 i64
  pub fn to_integer64(&self) -> Result<i64> {
    let s = self.to_string_lossy();
    s.trim()
      .parse::<i64>()
      .map_err(|e| Error::InvalidParameter(e.to_string()))
  }
}

impl From<&str> for String16 {
  fn from(s: &str) -> Self {
    Self::from_str_slice(s)
  }
}

/// String16 缓冲区构建器 (v8/src/inspector/string-16.h:130)
#[derive(Debug, Default)]
pub struct String16Builder {
  buffer: Vec<u16>,
}

impl String16Builder {
  /// 创建空构建器
  pub fn new() -> Self {
    Self { buffer: Vec::new() }
  }

  /// 追加单字符
  pub fn append_char(&mut self, ch: u16) {
    self.buffer.push(ch);
  }

  /// 追加 &str
  pub fn append_str(&mut self, s: &str) {
    self.buffer.extend(s.encode_utf16());
  }

  /// 追加 String16
  pub fn append_string16(&mut self, s: &String16) {
    self.buffer.extend_from_slice(s.characters16());
  }

  /// 追加十六进制格式无符号整数 (appendUnsignedAsHex)
  pub fn append_unsigned_as_hex(&mut self, val: u32) {
    let hex = format!("{val:08x}");
    self.append_str(&hex);
  }

  /// 转换为 String16
  pub fn to_string16(&self) -> String16 {
    String16::from_u16_slice(&self.buffer)
  }
}

/// 计算 UTF-16 字节流的标准 IEEE 802.3 CRC32 值 (v8/src/inspector/crc32.cc)
pub fn compute_crc32(text: &String16) -> i32 {
  let mut checksum: u32 = 0;
  // 每个 u16 拆分为低位与高位字节
  for &unit in text.characters16() {
    let low = (unit & 0xff) as u8;
    let high = ((unit >> 8) & 0xff) as u8;
    let idx0 = ((checksum ^ (low as u32)) & 0xff) as usize;
    checksum = (checksum >> 8) ^ CRC32_TABLE[idx0];
    let idx1 = ((checksum ^ (high as u32)) & 0xff) as usize;
    checksum = (checksum >> 8) ^ CRC32_TABLE[idx1];
  }
  checksum as i32
}

/// 源码魔术注释定位核心 (v8/src/inspector/search-util.cc:findMagicComment)
pub fn find_magic_comment(content: &String16, name: &str, multiline: bool) -> String16 {
  let chars = content.characters16();
  let len = chars.len();
  let mut pos = len;
  let mut equal_pos = 0;
  let mut closing_pos = 0;
  let name_len = name.encode_utf16().count();

  loop {
    let found = content.reverse_find(name, pos);
    if found == K_STRING16_NOT_FOUND {
      return String16::new();
    }
    if found < 4 {
      return String16::new();
    }
    let p = found - 4;
    if chars[p] != b'/' as u16 {
      if found == 0 {
        break;
      }
      pos = found - 1;
      continue;
    }
    if (!multiline && chars[p + 1] != b'/' as u16) || (multiline && chars[p + 1] != b'*' as u16) {
      if found == 0 {
        break;
      }
      pos = found - 1;
      continue;
    }
    if chars[p + 2] != b'#' as u16 && chars[p + 2] != b'@' as u16 {
      if found == 0 {
        break;
      }
      pos = found - 1;
      continue;
    }
    if chars[p + 3] != b' ' as u16 && chars[p + 3] != b'\t' as u16 {
      if found == 0 {
        break;
      }
      pos = found - 1;
      continue;
    }
    let eq = p + 4 + name_len;
    if eq >= len || chars[eq] != b'=' as u16 {
      if found == 0 {
        break;
      }
      pos = found - 1;
      continue;
    }
    if multiline {
      let end_comment = content.find_str("*/", eq + 1);
      if end_comment == K_STRING16_NOT_FOUND {
        return String16::new();
      }
      closing_pos = end_comment;
    }
    equal_pos = eq;
    break;
  }

  if equal_pos == 0 {
    return String16::new();
  }

  let url_start = equal_pos + 1;
  let mut matched = if multiline {
    if closing_pos <= url_start {
      return String16::new();
    }
    content.substring(url_start, closing_pos - url_start)
  } else {
    content.substring_from(url_start)
  };

  let nl_pos = matched.find(b'\n' as u16, 0);
  if nl_pos != K_STRING16_NOT_FOUND {
    matched = matched.substring(0, nl_pos);
  }
  let trimmed = matched.strip_white_space();
  for &c in trimmed.characters16() {
    if c == b'"' as u16 || c == b'\'' as u16 || c == b' ' as u16 || c == b'\t' as u16 {
      return String16::new();
    }
  }
  trimmed
}

/// 定位 sourceURL 魔术注释 (v8/src/inspector/search-util.cc:163)
pub fn find_source_url(content: &String16, multiline: bool) -> String16 {
  find_magic_comment(content, "sourceURL", multiline)
}

/// 定位 sourceMappingURL 魔术注释 (v8/src/inspector/search-util.cc:167)
pub fn find_source_map_url(content: &String16, multiline: bool) -> String16 {
  find_magic_comment(content, "sourceMappingURL", multiline)
}

/// 定位 debugId 魔术注释 (v8/src/inspector/search-util.cc:171)
pub fn find_debug_id(content: &String16, multiline: bool) -> String16 {
  find_magic_comment(content, "debugId", multiline)
}

/// 二进制数据承载容器 (v8/src/inspector/string-util.h:49)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Binary {
  data: Vec<u8>,
}

impl Binary {
  /// 从字节切片构造
  pub fn from_bytes(bytes: &[u8]) -> Self {
    Self {
      data: bytes.to_vec(),
    }
  }

  /// 字节切片引用
  pub fn bytes(&self) -> &[u8] {
    &self.data
  }

  /// 字节长度
  pub fn size(&self) -> usize {
    self.data.len()
  }
}

/// 调用栈 ID 转字符串 (v8/src/inspector/string-util.h:60)
pub fn stack_trace_id_to_string(id: usize) -> String16 {
  String16::from_str_slice(&id.to_string())
}
