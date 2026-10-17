//! bytes-unittest.cc 1:1 迁移。
use aok::{OK, Void};
use v8_base_platform::TimeDelta;
use v8_heap_base::{
  ByteSize, BytesAndDuration, BytesAndDurationBuffer, SmoothedBytesAndDuration,
  average_speed_unbounded,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn bad(bytes: usize, ms: i64) -> BytesAndDuration {
  BytesAndDuration::new(bytes, TimeDelta::from_milliseconds(ms))
}

// gtest EXPECT_DOUBLE_EQ 等价 (4 ULP)
fn double_eq(a: f64, b: f64) -> bool {
  a == b || (a - b).abs() <= 4.0 * f64::EPSILON * a.abs().max(b.abs())
}

// TEST(BytesAndDurationTest, MakeBytesAndDuration) cc:13-18
#[test]
fn make_bytes_and_duration() -> Void {
  let b = bad(17, 35);
  assert_eq!(b.bytes, 17);
  assert_eq!(b.duration.in_milliseconds(), 35);
  OK
}

// TEST(BytesAndDurationTest, InitialAsAverage) cc:20-28
#[test]
fn initial_as_average() -> Void {
  let buffer = BytesAndDurationBuffer::new();
  assert!(double_eq(
    100.0 / 2.0,
    average_speed_unbounded(&buffer, &bad(100, 2), None).unwrap()
  ));
  OK
}

// TEST(BytesAndDurationTest, SelectedDuration) cc:30-41
#[test]
fn selected_duration() -> Void {
  let mut buffer = BytesAndDurationBuffer::new();
  // 该事件因 2ms 窗口过滤被忽略
  buffer.push(bad(100, 8));
  assert!(double_eq(
    100.0 / 2.0,
    average_speed_unbounded(&buffer, &bad(100, 2), Some(TimeDelta::from_milliseconds(2))).unwrap()
  ));
  OK
}

// TEST(BytesAndDurationTest, Empty) cc:43-47
#[test]
fn empty() -> Void {
  let buffer = BytesAndDurationBuffer::new();
  assert!(average_speed_unbounded(&buffer, &BytesAndDuration::default(), None).is_none());
  OK
}

// TEST(BytesAndDurationTest, Clear) cc:49-57
#[test]
fn clear() -> Void {
  let mut buffer = BytesAndDurationBuffer::new();
  buffer.push(bad(100, 2));
  assert!(double_eq(
    100.0 / 2.0,
    average_speed_unbounded(&buffer, &BytesAndDuration::default(), None).unwrap()
  ));
  buffer.clear();
  assert!(average_speed_unbounded(&buffer, &BytesAndDuration::default(), None).is_none());
  OK
}

// TEST(BytesAndDurationTest, MaxSpeed) cc:59-67
#[test]
fn max_speed() -> Void {
  let mut buffer = BytesAndDurationBuffer::new();
  const K_MAX_BYTES_PER_MS: usize = 1024;
  buffer.push(BytesAndDuration::new(
    K_MAX_BYTES_PER_MS,
    TimeDelta::from_milliseconds_d(0.5),
  ));
  let bounded = v8_heap_base::average_speed(
    &buffer,
    &BytesAndDuration::default(),
    None,
    0,
    K_MAX_BYTES_PER_MS,
  )
  .unwrap();
  assert!(double_eq(K_MAX_BYTES_PER_MS as f64, bounded));
  OK
}

// TEST(BytesAndDurationTest, MinSpeed) cc:69-77
#[test]
fn min_speed() -> Void {
  let mut buffer = BytesAndDurationBuffer::new();
  const K_MIN_BYTES_PER_MS: usize = 1;
  buffer.push(BytesAndDuration::new(
    K_MIN_BYTES_PER_MS,
    TimeDelta::from_milliseconds_d(2.0),
  ));
  let bounded = v8_heap_base::average_speed(
    &buffer,
    &BytesAndDuration::default(),
    None,
    K_MIN_BYTES_PER_MS,
    usize::MAX,
  )
  .unwrap();
  assert!(double_eq(K_MIN_BYTES_PER_MS as f64, bounded));
  OK
}

// TEST(BytesAndDurationTest, RingBufferAverage) cc:79-96
#[test]
fn ring_buffer_average() -> Void {
  let mut buffer = BytesAndDurationBuffer::new();
  let mut sum = 0usize;
  for i in 0..BytesAndDurationBuffer::K_SIZE {
    sum += i + 1;
    buffer.push(BytesAndDuration::new(
      i + 1,
      TimeDelta::from_milliseconds_d(1.0),
    ));
    assert!(double_eq(
      sum as f64 / (i + 1) as f64,
      average_speed_unbounded(&buffer, &BytesAndDuration::default(), None).unwrap()
    ));
  }
  assert!(double_eq(
    sum as f64 / BytesAndDurationBuffer::K_SIZE as f64,
    average_speed_unbounded(&buffer, &BytesAndDuration::default(), None).unwrap()
  ));
  // 溢出环形缓冲: 最旧 (1) 被覆盖
  buffer.push(bad(100, 1));
  assert!(double_eq(
    (sum + 100 - 1) as f64 / BytesAndDurationBuffer::K_SIZE as f64,
    average_speed_unbounded(&buffer, &BytesAndDuration::default(), None).unwrap()
  ));
  OK
}

// TEST(SmoothedBytesAndDuration, ZeroDelta) cc:98-107
#[test]
fn smoothed_zero_delta() -> Void {
  let mut smoothed =
    SmoothedBytesAndDuration::new(TimeDelta::from_seconds(1), TimeDelta::from_seconds(1));

  assert_eq!(smoothed.throughput(), 0.0);

  // NaN 速率被忽略 (时长为零)
  smoothed.update(BytesAndDuration::new(10, TimeDelta::new()));
  assert_eq!(smoothed.throughput(), 0.0);
  OK
}

// TEST(SmoothedBytesAndDuration, Update) cc:109-133
#[test]
fn smoothed_update() -> Void {
  let mut smoothed = SmoothedBytesAndDuration::new(
    TimeDelta::from_milliseconds(1),
    TimeDelta::from_milliseconds(1),
  );

  assert_eq!(smoothed.throughput(), 0.0);

  // 原吞吐的平滑更新, 1ms 半衰期
  smoothed.update(bad(10, 1));
  assert_eq!(smoothed.throughput(), 5.0);

  // 足够长之后收敛
  smoothed.update(bad(1000, 1000));
  assert_eq!(smoothed.throughput(), 1.0);

  // 1ms 半衰期衰减
  assert_eq!(
    smoothed.throughput_for_testing(TimeDelta::from_milliseconds(1)),
    0.5
  );
  smoothed.update(bad(0, 1));
  assert_eq!(smoothed.throughput(), 0.5);
  OK
}

// API 语义补充: RingBuffer 尺寸/清空 + ByteSize newtype
#[test]
fn ring_buffer_size_and_byte_size() -> Void {
  let mut buffer = BytesAndDurationBuffer::new();
  assert!(buffer.empty());
  assert_eq!(buffer.size(), 0);
  buffer.push(bad(1, 1));
  assert_eq!(buffer.size(), 1);
  for i in 1..BytesAndDurationBuffer::K_SIZE {
    buffer.push(bad(i, 1));
  }
  assert_eq!(buffer.size(), BytesAndDurationBuffer::K_SIZE);
  buffer.push(bad(9, 1)); // 满: 覆盖最旧
  assert_eq!(buffer.size(), BytesAndDurationBuffer::K_SIZE);
  buffer.clear();
  assert!(buffer.empty());

  let sz = ByteSize(4096);
  assert_eq!(sz.0, 4096);
  OK
}
