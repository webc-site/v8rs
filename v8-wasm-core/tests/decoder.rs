//! Decoder 测试: 顺序读取/越界错误/pc 推进/consume_bytes/LEB 族
use aok::{OK, Void};
use v8_wasm_core::{
  decoder::{Decoder, FullValidation, NoValidation},
  error::{Error, WasmError},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 定宽小端顺序读取
#[test]
fn read_fixed_width() -> Void {
  let bytes = [0x01, 0xef, 0xbe, 0xad, 0xde];
  let mut d = Decoder::new(&bytes);
  assert_eq!(d.read_u8::<FullValidation>(0), Ok(0x01));
  assert_eq!(d.read_u32::<FullValidation>(1), Ok(0xdead_beef)); // 小端
  assert_eq!(d.position(), 0); // read 族不推进 pc
  OK
}

/// 小端语义: read_u32 = LE 字节序
#[test]
fn read_u32_little_endian() -> Void {
  let bytes = [0x78, 0x56, 0x34, 0x12];
  let mut d = Decoder::new(&bytes);
  assert_eq!(d.read_u32::<FullValidation>(0), Ok(0x1234_5678));
  let b8 = [0xef, 0xbe, 0xad, 0xde, 0x00, 0x00, 0x00, 0x00];
  let mut d8 = Decoder::new(&b8);
  assert_eq!(d8.read_u64::<FullValidation>(0), Ok(0x0000_0000_dead_beef));
  OK
}

/// 越界: FullValidation 报错并记录首错; 只报首错
#[test]
fn out_of_bounds_error() -> Void {
  let bytes = [0x01, 0x02];
  let mut d = Decoder::new(&bytes);
  assert!(d.read_u32::<FullValidation>(0).is_err());
  assert!(d.failed());
  let err = d.error().map(|e| e.offset);
  assert_eq!(err, Some(0));
  // 首错已记 → 第二次错误不覆盖
  let first_msg = d.error().map(|e| e.message.clone());
  let _ = d.read_u8::<FullValidation>(99);
  assert_eq!(d.error().map(|e| e.message.clone()), first_msg);
  OK
}

/// pc 推进: consume 族成功前进, 失败跳到 end
#[test]
fn consume_pc_advance() -> Void {
  let bytes = [0x2a, 0xff, 0x00, 0x00, 0x00, 0x80, 0x02]; // u32v: 0x100 (0x80 0x02)
  let mut d = Decoder::new(&bytes);
  assert_eq!(d.position(), 0);
  assert_eq!(d.consume_u8("x"), Ok(0x2a));
  assert_eq!(d.position(), 1);
  assert_eq!(d.consume_u32("y"), Ok(0xff));
  assert_eq!(d.position(), 5);
  assert_eq!(d.consume_u32v("v"), Ok(0x100));
  assert_eq!(d.position(), 7);
  assert!(!d.more());
  // 耗尽后 consume 失败且 pc = end
  assert!(d.consume_u8("z").is_err());
  assert_eq!(d.position() as usize, d.end());
  OK
}

/// consume_bytes: 跳过与不足
#[test]
fn consume_bytes_test() -> Void {
  let bytes = [0u8; 10];
  let mut d = Decoder::new(&bytes);
  assert!(d.consume_bytes(4, "skip").is_ok());
  assert_eq!(d.position(), 4);
  assert!(d.consume_bytes(6, "skip").is_ok());
  assert_eq!(d.position(), 10);
  // 不足 → pc = end + Err
  let mut d2 = Decoder::new(&bytes);
  assert!(d2.consume_bytes(11, "skip").is_err());
  assert_eq!(d2.position(), 10);
  OK
}

/// LEB 读取族: u32v/i32v/u64v/i33v
#[test]
fn read_leb_family() -> Void {
  // 624485 = 0xE5 0x8E 0x26
  let mut d = Decoder::new(&[0xe5, 0x8e, 0x26]);
  assert_eq!(d.read_u32v::<FullValidation>(0), Ok((624_485, 3)));
  // 单字节快路径
  let mut d = Decoder::new(&[0x7f]);
  assert_eq!(d.read_u32v::<FullValidation>(0), Ok((127, 1)));
  // 有符号: -1 → 0x7f; -123456 → c0 bb 78
  let mut d = Decoder::new(&[0xc0, 0xbb, 0x78]);
  assert_eq!(d.read_i32v::<FullValidation>(0), Ok((-123_456, 3)));
  // i64 大值: u64::MAX
  let big: Vec<u8> = [0xff; 9].iter().copied().chain([0x01]).collect();
  let mut d = Decoder::new(&big);
  assert_eq!(d.read_u64v::<FullValidation>(0), Ok((u64::MAX, 10)));
  // i33: 33 位有符号, 范围 [-2^32, 2^32-1]; 符号位在第 5 字节 bit4
  // 2^31 = 0x8000_0000 → LEB33: 0x80 0x80 0x80 0x80 0x08
  let mut d = Decoder::new(&[0x80, 0x80, 0x80, 0x80, 0x08]);
  assert_eq!(d.read_i33v::<FullValidation>(0), Ok((1 << 31, 5)));
  // 2^32 - 1 (尾字节 0x0F: 符号位 0)
  let mut d = Decoder::new(&[0xff, 0xff, 0xff, 0xff, 0x0f]);
  assert_eq!(
    d.read_i33v::<FullValidation>(0),
    Ok((((1u64 << 32) - 1) as i64, 5))
  );
  // 尾字节 0x7F: 符号位 1 全 1 → -1
  let mut d = Decoder::new(&[0xff, 0xff, 0xff, 0xff, 0x7f]);
  assert_eq!(d.read_i33v::<FullValidation>(0), Ok((-1, 5)));
  // 2^32 越界 (尾字节 0x10: 符号位 1 但高位 0 → extra bits 非法)
  let mut d = Decoder::new(&[0x80, 0x80, 0x80, 0x80, 0x10]);
  assert!(d.read_i33v::<FullValidation>(0).is_err());
  // 负 i33: -1 → 0x7f
  let mut d = Decoder::new(&[0x7f]);
  assert_eq!(d.read_i33v::<FullValidation>(0), Ok((-1, 1)));
  OK
}

/// LEB 错误形态: 长度溢出 / extra bits / 截断
#[test]
fn read_leb_errors() -> Void {
  // u32 长度溢出: 6 字节全 continuation
  let mut d = Decoder::new(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x00]);
  let err = d.read_u32v::<FullValidation>(0);
  assert!(err.is_err());
  assert!(
    d.error()
      .map(|e| e.message.contains("length overflow"))
      .unwrap_or(false)
  );
  // extra bits: u32 第 5 字节有效位外有值 (0x10 → bit4 越界)
  let mut d = Decoder::new(&[0x80, 0x80, 0x80, 0x80, 0x10]);
  assert!(d.read_u32v::<FullValidation>(0).is_err());
  assert!(
    d.error()
      .map(|e| e.message.contains("extra bits"))
      .unwrap_or(false)
  );
  // 截断: continuation 处源耗尽
  let mut d = Decoder::new(&[0x80, 0x80]);
  assert!(d.read_u32v::<FullValidation>(0).is_err());
  assert!(
    d.error()
      .map(|e| e.message.contains("reached end"))
      .unwrap_or(false)
  );
  OK
}

/// 长度溢出 vs extra bits: i32 5 字节带符号合法形态
#[test]
fn leb_signed_last_byte() -> Void {
  // i32::MIN = 0x80 ×4 + 0x78 (高位全 1 符号扩展合法)
  let mut d = Decoder::new(&[0x80, 0x80, 0x80, 0x80, 0x78]);
  assert_eq!(d.read_i32v::<FullValidation>(0), Ok((i32::MIN, 5)));
  // i32::MAX = 0xFF ×4 + 0x07
  let mut d = Decoder::new(&[0xff, 0xff, 0xff, 0xff, 0x07]);
  assert_eq!(d.read_i32v::<FullValidation>(0), Ok((i32::MAX, 5)));
  // 无符号 u32::MAX = 0xFF ×4 + 0x0F
  let mut d = Decoder::new(&[0xff, 0xff, 0xff, 0xff, 0x0f]);
  assert_eq!(d.read_u32v::<FullValidation>(0), Ok((u32::MAX, 5)));
  OK
}

/// LEB 错误 offset: 报在出错字节处 (length overflow → 溢出字节; 截断 → 越界字节)
#[test]
fn read_leb_error_offsets() -> Void {
  // u32 长度溢出: 5 字节均带 continuation
  // 原版 errorf(pc) 于递归后 pc = LEB 起点 + kMaxLength - 1 = 索引 4
  let bytes = [0x80, 0x80, 0x80, 0x80, 0x80, 0x00];
  let mut d = Decoder::new(&bytes);
  let err = d.read_u32v::<FullValidation>(0).unwrap_err();
  assert_eq!(
    err,
    Error::Wasm(WasmError::new(4, "length overflow while decoding LEB32"))
  );
  assert_eq!(
    d.error().map(|x| x.offset),
    Some(4),
    "error_ 与 Err 同 offset"
  );
  // 截断: continuation 处源耗尽 → 报在越界字节位置 (索引 2)
  let mut d = Decoder::new(&[0x80, 0x80]);
  let err = d.read_u32v::<FullValidation>(0).unwrap_err();
  assert_eq!(
    err,
    Error::Wasm(WasmError::new(2, "reached end while decoding LEB32"))
  );
  // extra bits: 报在末字节处 (索引 4)
  let mut d = Decoder::new(&[0x80, 0x80, 0x80, 0x80, 0x10]);
  let err = d.read_u32v::<FullValidation>(0).unwrap_err();
  assert_eq!(err, Error::Wasm(WasmError::new(4, "extra bits in varint")));
  // buffer_offset 叠加: 溢出字节偏移 + buffer_offset
  let mut d = Decoder::with_buffer_offset(&[0x80, 0x80, 0x80, 0x80, 0x80], 100);
  assert!(d.read_u32v::<FullValidation>(0).is_err());
  assert_eq!(d.error().map(|x| x.offset), Some(104));
  OK
}

/// buffer_offset: 流式偏移换算
#[test]
fn buffer_offset_test() -> Void {
  let bytes = [0x01, 0x02];
  let mut d = Decoder::with_buffer_offset(&bytes, 100);
  assert_eq!(d.pc_offset(), 100);
  let _ = d.consume_u8("x");
  assert_eq!(d.pc_offset(), 101);
  assert_eq!(d.position(), 1);
  assert_eq!(d.get_buffer_relative_offset(101), 1);
  OK
}

/// read_prefixed_opcode: 前缀 + LEB 索引
#[test]
fn prefixed_opcode() -> Void {
  // 0xfb 0x10 → (0xfb << 8 | 0x10, 2)
  let mut d = Decoder::new(&[0xfb, 0x10]);
  assert_eq!(
    d.read_prefixed_opcode::<FullValidation>(0),
    Ok((0xfb << 8 | 0x10, 2))
  );
  // 大索引 (>0xff): 0xfb 0x81 0x02 → index=0x101 > 0xff → <<12
  let mut d = Decoder::new(&[0xfb, 0x81, 0x02]);
  assert_eq!(
    d.read_prefixed_opcode::<FullValidation>(0),
    Ok((0xfb << 12 | 0x101, 3))
  );
  // 索引 > 0xfff → 错误路径返回 unreachable
  let mut d = Decoder::new(&[0xfb, 0x8a, 0x8a, 0x8a, 0x0a]);
  assert_eq!(d.read_prefixed_opcode::<FullValidation>(0), Ok((0, 0)));
  OK
}

/// lookahead / reset / available_bytes
#[test]
fn misc_api() -> Void {
  let bytes = [0x00, 0x61, 0x73];
  let mut d = Decoder::new(&bytes);
  assert_eq!(d.available_bytes(), 3);
  assert!(d.lookahead(1, 0x61));
  assert!(!d.lookahead(1, 0x62));
  assert!(!d.lookahead(9, 0x61)); // 越界 false
  // reset 复用
  let bytes2 = [0xffu8];
  d.reset(&bytes2, 0);
  assert_eq!(d.available_bytes(), 1);
  assert_eq!(d.read_u8::<FullValidation>(0), Ok(0xff));
  OK
}

/// NoValidation: 越界同样 Err (禁 panic), 但不污染错误累积
#[test]
fn no_validation_tag() -> Void {
  let bytes = [0x01];
  let mut d = Decoder::new(&bytes);
  assert!(d.read_u32::<NoValidation>(0).is_err());
  assert!(d.ok()); // 未记录 (Full 才记录)
  let mut d2 = Decoder::new(&bytes);
  assert!(d2.read_u32::<FullValidation>(0).is_err());
  assert!(d2.failed());
  OK
}
