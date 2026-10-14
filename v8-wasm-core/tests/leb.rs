//! leb-helper LEB128 编解码往返测试 (边界: u32/u64 最大值、多字节 continuation)
use aok::{OK, Void};
use v8_wasm_core::leb_helper::{
  LebHelper, MAX_I32V_SIZE, MAX_I64V_SIZE, MAX_U32V_SIZE, MAX_U64V_SIZE, MAX_VAR_INT32_SIZE,
  MAX_VAR_INT64_SIZE, PADDED_VAR_INT32_SIZE,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 无符号 u32 往返 (含 0/单字节界/多字节 continuation/u32::MAX)
#[test]
fn u32v_roundtrip() -> Void {
  let cases = [
    0u32,
    1,
    0x7f,       // 单字节上限
    0x80,       // 双字节下限
    0x3fff,     // 双字节上限
    0x4000,     // 三字节下限
    0x1f_ff_f0, // 四字节
    0x20_0000,  // 四字节上限边界
    0x7fff_ffff,
    u32::MAX, // 五字节满编码
  ];
  let mut buf = [0u8; MAX_U32V_SIZE];
  for v in cases {
    let n = LebHelper::write_u32v(&mut buf, v);
    assert_eq!(LebHelper::read_u32v(&buf[..n]), Some(v), "u32v 往返 {v:#x}");
    assert_eq!(LebHelper::sizeof_u32v(v), n, "sizeof_u32v {v:#x}");
  }
  OK
}

/// 有符号 i32 往返 (负数编码 + 边界)
#[test]
fn i32v_roundtrip() -> Void {
  let cases = [
    0i32,
    1,
    -1,  // 全 1 字节 0x7f
    63,  // 0x3f 单字节正上限
    64,  // 0x40 需防符号扩展 → 双字节
    -64, // 单字节负下限
    -65, // 双字节
    i32::MAX,
    i32::MIN,
  ];
  let mut buf = [0u8; MAX_I32V_SIZE];
  for v in cases {
    let n = LebHelper::write_i32v(&mut buf, v);
    assert!(n <= MAX_I32V_SIZE, "i32v 长度 {n}");
    assert_eq!(LebHelper::sizeof_i32v(v), n, "sizeof_i32v {v}");
  }
  // -1 编码为单字节 0x7f (符号扩展全 1)
  let n = LebHelper::write_i32v(&mut buf, -1);
  assert_eq!(&buf[..n], &[0x7f]);
  // 64 防符号扩展: 0xC0 0x00
  let n = LebHelper::write_i32v(&mut buf, 64);
  assert_eq!(&buf[..n], &[0xc0, 0x00]);
  OK
}

/// 无符号 u64 往返 (u64::MAX 10 字节满编码)
#[test]
fn u64v_roundtrip() -> Void {
  let cases = [0u64, 0x7f, 0x80, (1 << 63) - 1, 1 << 63, u64::MAX];
  let mut buf = [0u8; MAX_U64V_SIZE];
  for v in cases {
    let n = LebHelper::write_u64v(&mut buf, v);
    assert_eq!(LebHelper::sizeof_u64v(v), n, "sizeof_u64v {v:#x}");
    assert!(n <= MAX_VAR_INT64_SIZE);
  }
  // u64::MAX = 9 个 0xFF + 0x01
  let n = LebHelper::write_u64v(&mut buf, u64::MAX);
  assert_eq!(n, 10);
  assert_eq!(buf[..9], [0xff; 9]);
  assert_eq!(buf[9], 0x01);
  OK
}

/// 有符号 i64 边界
#[test]
fn i64v_boundaries() -> Void {
  let mut buf = [0u8; MAX_I64V_SIZE];
  for v in [0i64, -1, i64::MAX, i64::MIN] {
    let n = LebHelper::write_i64v(&mut buf, v);
    assert!(n <= MAX_I64V_SIZE, "i64v 长度 {n}");
  }
  // -1 → 0x7f; i64::MIN → 0x80 ×9 + 0x7f (尾字节符号扩展全 1)
  let n = LebHelper::write_i64v(&mut buf, -1);
  assert_eq!(&buf[..n], &[0x7f]);
  let n = LebHelper::write_i64v(&mut buf, i64::MIN);
  assert_eq!(n, 10);
  assert_eq!(buf[..9], [0x80; 9]);
  assert_eq!(buf[9], 0x7f);
  OK
}

/// read_u32v: 源耗尽 → None (禁 panic)
#[test]
fn read_u32v_truncated() -> Void {
  assert_eq!(LebHelper::read_u32v(&[]), None);
  assert_eq!(LebHelper::read_u32v(&[0x80]), None);
  // 多字节在 continuation 处截断
  assert_eq!(LebHelper::read_u32v(&[0x80, 0x80]), None);
  OK
}

/// read_u32v: 超长 LEB (第 6 字节 shift=35 越过 u32 位宽) → None (禁 panic 回归)
#[test]
fn read_u32v_overlong() -> Void {
  // 6 字节: 前 5 字节均带 continuation, 第 6 字节存在 → 超长路径
  assert_eq!(
    LebHelper::read_u32v(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x00]),
    None
  );
  assert_eq!(LebHelper::read_u32v(&[0x80; 6]), None);
  // 5 字节满编码仍合法 (u32::MAX)
  assert_eq!(
    LebHelper::read_u32v(&[0xff, 0xff, 0xff, 0xff, 0x0f]),
    Some(u32::MAX)
  );
  // 第 5 字节带 continuation 且源耗尽 → 走耗尽路径
  assert_eq!(LebHelper::read_u32v(&[0x80, 0x80, 0x80, 0x80, 0x80]), None);
  OK
}

/// 常量对数 (与 wasm-limits/leb-helper.h 一致)
#[test]
fn leb_size_constants() -> Void {
  assert_eq!(PADDED_VAR_INT32_SIZE, 5);
  assert_eq!(MAX_VAR_INT32_SIZE, 5);
  assert_eq!(MAX_VAR_INT64_SIZE, 10);
  OK
}
