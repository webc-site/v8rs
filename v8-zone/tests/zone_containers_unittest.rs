//! 补充用例 (原版 unittests/zone 无 zone-list/deque/linked 直接单测,
//! 此处按头文件语义自拟, 映射表见 readme/zh.md)

use std::ptr;

use aok::{OK, Void};
use v8_zone::{
  AccountingAllocator, Zone, ZoneAllocationPolicy, ZoneDeque, ZoneForwardList, ZoneLinkedList,
  ZoneList, ZoneQueue, ZoneStack,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// ZoneList: Add/AddBlock/InsertAt/Remove/Rewind/Contains/Sort 语义
#[test]
fn zone_list_add_and_remove() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut list: ZoneList<'_, '_, i32> = ZoneList::new(2, &zone)?;
  assert_eq!(list.length(), 0);
  for i in 1..=10 {
    list.add(&i)?;
  }
  assert_eq!(list.length(), 10);
  let total: i32 = list.iter().sum();
  assert_eq!(total, 55);
  // ResizeAdd 高水位: 1 + 2 * capacity 序列 2→5→11
  assert_eq!(list.capacity(), 11);
  // Remove 左移
  let removed = list.remove(0);
  assert_eq!(removed, 1);
  assert_eq!(*list.first(), 2);
  let removed_last = list.remove_last();
  assert_eq!(removed_last, 10);
  // InsertAt 中段
  list.insert_at(0, &1)?;
  list.insert_at(5, &99)?;
  assert_eq!(*list.first(), 1);
  assert_eq!(*list.at(5), 99);
  // Contains / Rewind
  assert!(list.contains(&99));
  list.rewind(4);
  assert_eq!(list.length(), 4);
  assert!(!list.contains(&99));
  // AddBlock
  {
    let block = list.add_block(&7, 3)?;
    assert_eq!(block.len(), 3);
  }
  assert!(list.contains(&7));
  // Sort (cmp 三态)
  let mut slist: ZoneList<'_, '_, i32> = ZoneList::from_slice(&zone, &[3, 1, 2])?;
  slist.sort_by(|a, b| a.cmp(b));
  assert_eq!(slist.as_slice(), &[1, 2, 3]);
  // Clear 归还存储记账 (原版 DeleteArray 仅记账, allocation_size 口径不变)
  let used_before = zone.allocation_size();
  slist.clear();
  assert_eq!(slist.length(), 0);
  assert_eq!(slist.capacity(), 0);
  assert_eq!(zone.allocation_size(), used_before);
  OK
}

/// ZoneList 移动语义: DropAndClear 与原版 std::move 赋值形态
#[test]
fn zone_list_drop_and_clear() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut list: ZoneList<'_, '_, i32> = ZoneList::from_slice(&zone, &[1, 2, 3])?;
  list.drop_and_clear();
  assert_eq!(list.length(), 0);
  assert_eq!(list.capacity(), 0);
  assert!(list.is_empty());
  // 重新填充可用
  list.add(&42)?;
  assert_eq!(*list.first(), 42);
  OK
}

/// ZoneDeque/ZoneQueue/ZoneStack: 双端与适配器语义
#[test]
fn zone_deque_queue_stack() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut deque: ZoneDeque<'_, '_, i32> = ZoneDeque::new(&zone);
  for i in 1..=4 {
    deque.push_back(i)?;
  }
  for i in [0, -1, -2] {
    deque.push_front(i)?;
  }
  assert_eq!(deque.size(), 7);
  assert_eq!(
    deque.iter().copied().collect::<Vec<_>>(),
    vec![-2, -1, 0, 1, 2, 3, 4]
  );
  assert_eq!(*deque.at(3), 1);
  assert_eq!(deque.pop_front()?.unwrap(), -2);
  assert_eq!(deque.pop_back()?.unwrap(), 4);
  // 双端交替触发整体搬运 (front/back 侧交换), 内容序不变
  let rest: Vec<i32> = deque.iter().copied().collect();
  assert_eq!(rest, vec![-1, 0, 1, 2, 3]);
  deque.clear();
  assert!(deque.is_empty());

  let mut queue: ZoneQueue<'_, '_, i32> = ZoneQueue::new(&zone);
  for i in 1..=5 {
    queue.push(i)?;
  }
  assert_eq!(queue.front().copied().unwrap(), 1);
  for i in 1..=5 {
    assert_eq!(queue.pop()?.unwrap(), i);
  }
  assert!(queue.is_empty());

  let mut stack: ZoneStack<'_, '_, i32> = ZoneStack::new(&zone);
  for i in 1..=3 {
    stack.push(i)?;
  }
  assert_eq!(stack.top().copied().unwrap(), 3);
  for i in (1..=3).rev() {
    assert_eq!(stack.pop()?.unwrap(), i);
  }
  OK
}

/// ZoneForwardList / ZoneLinkedList: 链表语义
#[test]
fn zone_linked_lists() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");

  let mut fl: ZoneForwardList<'_, '_, i32> = ZoneForwardList::new(&zone);
  assert!(fl.is_empty());
  for i in 1..=5 {
    fl.push_front(i)?;
  }
  assert_eq!(fl.iter().copied().collect::<Vec<_>>(), vec![5, 4, 3, 2, 1]);
  assert_eq!(fl.pop_front(), Some(5));
  assert_eq!(fl.len(), 4);
  fl.clear();
  assert!(fl.is_empty());

  let mut ll: ZoneLinkedList<'_, '_, i32> = ZoneLinkedList::new(&zone);
  for i in 1..=4 {
    ll.push_back(i)?;
  }
  ll.push_front(0)?;
  assert_eq!(ll.iter().copied().collect::<Vec<_>>(), vec![0, 1, 2, 3, 4]);
  assert_eq!(ll.front().copied(), Some(0));
  assert_eq!(ll.back().copied(), Some(4));
  assert_eq!(ll.pop_front(), Some(0));
  assert_eq!(ll.pop_back(), Some(4));
  assert_eq!(ll.len(), 3);
  ll.clear();
  assert!(ll.is_empty());

  // 带 Drop 元素: 链表析构须逐一回收值 (原版 ~std::list)
  {
    let mut dl: ZoneLinkedList<'_, '_, String> = ZoneLinkedList::new(&zone);
    dl.push_back("a".to_string())?;
    dl.push_back("b".to_string())?;
  }
  OK
}

/// AccountingAllocator: current/max 用量与 zone_stats 开关
#[test]
fn accounting_allocator_usage() -> Void {
  let allocator = AccountingAllocator::new();
  assert_eq!(allocator.current_memory_usage(), 0);
  assert_eq!(allocator.max_memory_usage(), 0);
  {
    let zone = Zone::new(&allocator, "test");
    let _a = zone.allocate_bytes_ref(4096)?;
    let used = allocator.current_memory_usage();
    assert!(used >= 4096);
    assert_eq!(allocator.max_memory_usage(), used);
  }
  assert_eq!(allocator.current_memory_usage(), 0);
  // TracingFlags 替身开关 (待回收 v8-tracing)
  assert!(!v8_zone::zone_stats_enabled());
  v8_zone::set_zone_stats_enabled(true);
  assert!(v8_zone::zone_stats_enabled());
  v8_zone::set_zone_stats_enabled(false);
  OK
}

/// ZoneAllocationPolicy / ZoneAllocator / RecyclingZoneAllocator 直用形态
#[test]
fn allocation_policy_and_allocators() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let policy = ZoneAllocationPolicy::new(&zone);
  let storage = policy.allocate_array::<i32>(4)?;
  let ptr = storage.as_mut_ptr().cast::<i32>();
  for i in 0..4 {
    // SAFETY: 槽位为 allocate_array 返回的未初始化存储
    unsafe { ptr::write(ptr.add(i), (i as i32) * (i as i32)) };
  }
  // SAFETY: 上述写入的 4 元素块归还记账
  policy.delete_array(ptr, 4);
  assert!(ptr::eq(policy.zone(), &zone));

  let za = v8_zone::ZoneAllocator::<u8>::new(&zone);
  let p = za.allocate(16)?;
  // SAFETY: 刚分配的 16 字节未再引用
  unsafe { za.deallocate(p, 16) };
  OK
}
