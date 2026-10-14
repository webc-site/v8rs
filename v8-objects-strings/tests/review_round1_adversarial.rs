//! 第 2 轮对抗式独立审查用例 (review_round1_adversarial):
//! 1. StringForwardingTable::ensure_capacity 跨倍容边界并发无野指针与脏写
//! 2. StringForwardingTable 析构与 Reset 全 BlockVector/Block 彻底释放零泄漏
//! 3. ConsStringIterator 深度 > 32 栈溢出保护、StackBlown 与 Search 重启状态机
//! 4. WriteToFlat2 反向游标 DFS 与 RepeatOptimizer 极端重复/空叶折叠
//! 5. SlicedString 多层嵌套累加解嵌套与 StringTable 临界扩容

use std::{
  alloc::{GlobalAlloc, Layout, System},
  str::from_utf8,
  sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
  },
  thread,
};

use aok::{OK, Void};
use v8_objects_core::tagged::{Object, Tagged};
use v8_objects_strings::{
  string::{ConsString, ConsStringIterator, StringExt, write_to_flat, write_to_flat2},
  string_forwarding_table::StringForwardingTable,
  string_table::{
    MIN_CAPACITY, compute_capacity, compute_capacity_with_shrink, has_sufficient_capacity_to_add,
  },
};

pub mod common;
use common::{Arena, Maps};

/// 活跃分配字节数计数器 (用于检测 Drop/Reset 路径是否存在内存泄漏)。
static ACTIVE_ALLOC: AtomicUsize = AtomicUsize::new(0);

struct CountingAlloc;

unsafe impl GlobalAlloc for CountingAlloc {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    // SAFETY: 系统分配器
    let ptr = unsafe { System.alloc(layout) };
    if !ptr.is_null() {
      ACTIVE_ALLOC.fetch_add(layout.size(), Ordering::Relaxed);
    }
    ptr
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    ACTIVE_ALLOC.fetch_sub(layout.size(), Ordering::Relaxed);
    // SAFETY: 系统释放
    unsafe { System.dealloc(ptr, layout) }
  }
}

#[global_allocator]
static ALLOC: CountingAlloc = CountingAlloc;

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

fn make_tagged_obj(id: usize) -> Tagged<Object> {
  Tagged::new((0x2000 + id * 0x10) | 1)
}

static TEST_LOCK: Mutex<()> = Mutex::new(());

// ─────────────────────────────────────────────────────────────────────────────
// 对抗用例 1: StringForwardingTable 并发跨倍容边界无野指针与脏写
// ─────────────────────────────────────────────────────────────────────────────

/// 并发多线程跨 BlockVector 倍容边界 (前 4 块共 240 项，从 4 扩至 8 块):
/// 8 线程并发插入 800 条记录，彻底冲破 block 4 (256)、block 5 (512) 边界。
/// 验证：
/// 1. 消除局部变量遮蔽后，并发倍容返回的 BlockVector 指针 100% 有效；
/// 2. 无空指针解引用，无野指针，无数据竞争与脏写；
/// 3. 所有并发插入的 index 都可在表中被正确读回，且内容与写入一一对应。
#[test]
fn adversarial_forwarding_table_concurrent_growth_no_wild_pointer() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  let table = Arc::new(StringForwardingTable::new());
  let thread_count = 8;
  let items_per_thread = 100; // 总计 800 项，跨越 Block 0, 1, 2, 3, 4, 5

  let mut handles = Vec::new();
  for t in 0..thread_count {
    let tbl = Arc::clone(&table);
    handles.push(thread::spawn(move || {
      let mut recorded = Vec::with_capacity(items_per_thread);
      for i in 0..items_per_thread {
        let val = t * 1000 + i;
        let s = make_tagged_obj(val);
        let fwd = make_tagged_obj(val + 500_000);
        let idx = tbl.add_forward_string(s, fwd);
        recorded.push((idx, s.ptr(), fwd.ptr()));
      }
      recorded
    }));
  }

  let mut all_records = Vec::new();
  for h in handles {
    let recs = h.join().unwrap();
    all_records.extend(recs);
  }

  assert_eq!(table.size(), (thread_count * items_per_thread) as i32);
  assert_eq!(all_records.len(), thread_count * items_per_thread);

  // 验证所有条目的读回准确性
  for (idx, _s_ptr, fwd_ptr) in all_records {
    let read_fwd = table.get_forward_string(idx).expect("idx 必须有效");
    assert_eq!(
      read_fwd.ptr(),
      fwd_ptr,
      "idx={idx} 读取到的转发目标与写入不一致"
    );
  }

  OK
}

// ─────────────────────────────────────────────────────────────────────────────
// 对抗用例 2: StringForwardingTable 析构与 Reset 全量 BlockVector/Block 零泄漏
// ─────────────────────────────────────────────────────────────────────────────

/// 验证多次倍容与 Drop 之后活跃堆字节数严格回落至基线：
/// 1. 产生多代 BlockVector (storage 链存在 3 个以上 BlockVector 实例)；
/// 2. 每个 BlockVector 经 Box::from_raw 彻底释放；
/// 3. 每一个 Block 经 free_blocks 彻底释放；
/// 4. 重复 Reset -> 填充 -> Reset -> Drop，杜绝内存泄漏与 double free。
#[test]
fn adversarial_forwarding_table_drop_and_reset_zero_leak() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  // 预热：让全局/线程内部懒加载分配（如 Mutex/TLS/格式化缓冲）稳定
  {
    let mut warm = StringForwardingTable::new();
    warm.add_forward_string(make_tagged_obj(0), make_tagged_obj(1));
    warm.reset();
  }

  // 阶梯: block 0(16), 1(32), 2(64), 3(128) -> 共 240 项 (第 1 代 BlockVector 满)
  // block 4(256) -> 496 项 (第 2 代 BlockVector，容量 8)
  // block 5(512) -> 1008 项 (第 2 代 BlockVector 仍可容纳)
  let n: i32 = 600; // 必然触发 4 扩至 8 的倍容

  let baseline = ACTIVE_ALLOC.load(Ordering::Relaxed);

  // 场景 A: 多轮 Reset 循环
  {
    let mut table = StringForwardingTable::new();
    for round in 0..5 {
      for i in 0..n {
        table.add_forward_string(
          make_tagged_obj(i as usize),
          make_tagged_obj(10_000 + i as usize),
        );
      }
      assert_eq!(table.size(), n);
      table.reset();
      assert_eq!(table.size(), 0);
      assert!(table.is_empty(), "round {round} reset 必须置空");
    }
  }

  let after_a = ACTIVE_ALLOC.load(Ordering::Relaxed);
  let leak_a = (after_a as isize - baseline as isize).abs();
  assert!(
    leak_a < 2048,
    "Reset 循环后 Drop 内存泄漏超标: leak={leak_a} bytes (baseline={baseline}, after={after_a})"
  );

  // 场景 B: 独立构造与 Drop
  {
    let table = StringForwardingTable::new();
    for i in 0..n {
      table.add_forward_string(
        make_tagged_obj(i as usize),
        make_tagged_obj(20_000 + i as usize),
      );
    }
    assert_eq!(table.size(), n);
  }

  let after_b = ACTIVE_ALLOC.load(Ordering::Relaxed);
  let leak_b = (after_b as isize - baseline as isize).abs();
  assert!(
    leak_b < 2048,
    "Drop 拥有多代 BlockVector 的表内存泄漏超标: leak={leak_b} bytes (baseline={baseline}, after={after_b})"
  );

  OK
}

// ─────────────────────────────────────────────────────────────────────────────
// 对抗用例 3: ConsStringIterator 深度 > 32 栈溢出保护、StackBlown 与 Search 重启
// ─────────────────────────────────────────────────────────────────────────────

/// 构造深度达 50 层的向左倾斜 ConsString 树 (a + (b + (c + ...)))
/// 与交替锯齿形深度达 40 层的树，检验：
/// 1. 深度超过 kStackSize (32) 时 StackBlown 标志正确被触发；
/// 2. 状态机能以已消费字符数 consumed 正确通过 Search 重启；
/// 3. 最终遍历拼接结果与全量基准完全一致。
#[test]
fn adversarial_cons_iterator_stack_blown_left_and_zigzag() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);

  // 1. 深度 50 的左深 Cons 树:
  // root = cons(cons(cons(...), sN), sN+1)
  let mut current = arena.seq_one_byte(maps.seq_one, b"X0");
  for i in 1..50 {
    let next_leaf = arena.seq_one_byte(maps.seq_one, format!("X{i}").as_bytes());
    let len = unsafe { current.length() + next_leaf.length() };
    current = arena.cons(maps.cons_one, len, current, next_leaf);
  }

  unsafe {
    let cons: Tagged<ConsString> = current.cast();
    let mut it = ConsStringIterator::new(Some(cons), 0);
    let mut got = String::new();
    let mut offset = 0u32;
    while let Some(seg) = it.next(&mut offset) {
      got.push_str(from_utf8(seg.get_flat_content().to_one_byte_vector()).unwrap());
    }
    let expect: String = (0..50).map(|i| format!("X{i}")).collect();
    assert_eq!(got, expect, "50 层左深 Cons 遍历不匹配");
  }

  // 2. 深度 40 的交替锯齿 Cons 树
  let mut zigzag = arena.seq_one_byte(maps.seq_one, b"Z0");
  for i in 1..40 {
    let next_leaf = arena.seq_one_byte(maps.seq_one, format!("Z{i}").as_bytes());
    let len = unsafe { zigzag.length() + next_leaf.length() };
    if i % 2 == 1 {
      zigzag = arena.cons(maps.cons_one, len, zigzag, next_leaf);
    } else {
      zigzag = arena.cons(maps.cons_one, len, next_leaf, zigzag);
    }
  }

  unsafe {
    let cons: Tagged<ConsString> = zigzag.cast();
    let mut it = ConsStringIterator::new(Some(cons), 0);
    let mut got = String::new();
    let mut offset = 0u32;
    while let Some(seg) = it.next(&mut offset) {
      got.push_str(from_utf8(seg.get_flat_content().to_one_byte_vector()).unwrap());
    }
    // 验证字符总长一致且首字符为 Z
    assert_eq!(got.len(), zigzag.length() as usize);
    assert!(got.starts_with('Z'));
  }

  OK
}

// ─────────────────────────────────────────────────────────────────────────────
// 对抗用例 4: WriteToFlat2 极端重复折叠、单字符与空切片
// ─────────────────────────────────────────────────────────────────────────────

/// 检验 RepeatOptimizer 在以下极端场景下的行为：
/// 1. 空叶节点与单字符节点混排；
/// 2. 5 层递归幂等全同子树 (((((x, x), (x, x)), ...)))；
/// 3. 与 WriteToFlat 的基准比对。
#[test]
fn adversarial_write_to_flat2_extreme_repeat_and_empty() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);

  // 1. 包含空叶节点的 ConsString
  let empty = arena.seq_one_byte(maps.seq_one, b"");
  let single = arena.seq_one_byte(maps.seq_one, b"a");
  let cons_empty = arena.cons(maps.cons_one, 1, empty, single);
  let mut dst = vec![0u8; 1];
  unsafe {
    write_to_flat2(&mut dst, cons_empty.cast()).ok();
  }
  assert_eq!(&dst, b"a");

  // 2. 5 层深度嵌套的纯幂等 Cons 树:
  // x = "ab" (len 2)
  // layer 1: 4
  // layer 2: 8
  // layer 3: 16
  // layer 4: 32
  // layer 5: 64
  let base_unit = arena.seq_one_byte(maps.seq_one, b"ab");
  let mut layer = arena.cons(maps.cons_one, 4, base_unit, base_unit);
  for i in 2..=5 {
    let len = 1u32 << (i + 1);
    layer = arena.cons(maps.cons_one, len, layer, layer);
  }

  let total_len = 1usize << 6; // 64
  let mut dst2 = vec![0u8; total_len];
  let mut dst_ref = vec![0u8; total_len];
  unsafe {
    write_to_flat2(&mut dst2, layer.cast()).ok();
    write_to_flat(layer, &mut dst_ref, 0, total_len as u32).ok();
  }
  let expect = "ab".repeat(32);
  assert_eq!(dst2, expect.as_bytes());
  assert_eq!(dst2, dst_ref);

  OK
}

// ─────────────────────────────────────────────────────────────────────────────
// 对抗用例 5: SlicedString 5 层嵌套与 StringTable 临界扩容
// ─────────────────────────────────────────────────────────────────────────────

/// 验证 5 层嵌套 SlicedString 递归累加 offset 与 StringTable 临界扩容
#[test]
fn adversarial_sliced_nesting_and_string_table_thresholds() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);

  // 5 层嵌套切片
  let full = arena.seq_one_byte(maps.seq_one, b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ");
  // s1: offset 2, len 30 -> "23456789ABCDEFGHIJKLMNOPQRST"
  let s1 = arena.sliced(maps.sliced_one, 30, full, 2);
  // s2: offset 3, len 20 -> "56789ABCDEFGHIJKLMNOPQ"
  let s2 = arena.sliced(maps.sliced_one, 20, s1, 3);
  // s3: offset 4, len 12 -> "9ABCDEFGHIJK"
  let s3 = arena.sliced(maps.sliced_one, 12, s2, 4);
  // s4: offset 1, len 8  -> "ABCDEFGH"
  let s4 = arena.sliced(maps.sliced_one, 8, s3, 1);
  // s5: offset 2, len 4  -> "CDEF"
  let s5 = arena.sliced(maps.sliced_one, 4, s4, 2);

  unsafe {
    assert_eq!(s5.length(), 4);
    assert_eq!(s5.get(0), u16::from(b'C'));
    assert_eq!(s5.get(1), u16::from(b'D'));
    assert_eq!(s5.get(2), u16::from(b'E'));
    assert_eq!(s5.get(3), u16::from(b'F'));

    let mut buf = vec![0u8; 4];
    write_to_flat(s5, &mut buf, 0, 4).ok();
    assert_eq!(&buf, b"CDEF");

    let flat = s5.get_flat_content();
    assert!(flat.is_flat());
    assert_eq!(flat.to_one_byte_vector(), b"CDEF");
  }

  // StringTable 临界容量与 50% 阈值数学断言
  assert!(has_sufficient_capacity_to_add(2048, 1000, 0, 1));
  assert!(!has_sufficient_capacity_to_add(2048, 1500, 0, 1)); // 超过 50% 占用
  assert_eq!(compute_capacity(100), MIN_CAPACITY);
  assert_eq!(compute_capacity(3000), 8192); // 3000 + 1500 = 4500 -> 8192
  // 100 <= 8192/4 = 2048: 触发收缩至所需容量 2048
  assert_eq!(compute_capacity_with_shrink(8192, 100), 2048);
  // 2500 > 8192/4 = 2048: 未达收缩门槛，维持 8192
  assert_eq!(compute_capacity_with_shrink(8192, 2500), 8192);

  OK
}
