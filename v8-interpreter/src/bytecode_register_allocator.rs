// 由 tool/src/interpreter/bytecode-register-allocator.h.js 从 v8/src/interpreter/bytecode-register-allocator.h 生成; 修订就地改, 复跑覆盖

/// 分配观察者 trait 骨架 (原版 class Observer 纯虚四事件)
pub trait RegisterAllocatorObserver {
  fn register_allocate_event(&mut self, reg: i32);
  fn register_list_allocate_event(&mut self, reg_list: (i32, i32));
  fn register_list_free_event(&mut self, reg_list: (i32, i32));
  fn register_free_event(&mut self, reg: i32);
}

/// 原版 class/struct BytecodeRegisterAllocator 成员声明 (机械 1:1 提取, 19 条)
pub const BYTECODE_REGISTER_ALLOCATOR_DECLARATIONS: &[&str] = &[
  "[public/field] class Observer",
  "[public/method] explicit BytecodeRegisterAllocator(int start_index) : next_register_index_(start_index), max_register_count_(start_index), observer_(nullptr)",
  "[public/method] ~BytecodeRegisterAllocator() = default",
  "[public/method] BytecodeRegisterAllocator(const BytecodeRegisterAllocator&) = delete",
  "[public/method] BytecodeRegisterAllocator& operator=(const BytecodeRegisterAllocator&) = delete",
  "[public/method] Register NewRegister()",
  "[public/method] RegisterList NewRegisterList(int count)",
  "[public/method] RegisterList NewGrowableRegisterList()",
  "[public/method] Register GrowRegisterList(RegisterList* reg_list)",
  "[public/method] void ReleaseRegisters(int register_index)",
  "[public/method] void ReleaseRegister(Register reg)",
  "[public/method] bool RegisterIsLive(Register reg) const",
  "[public/method] RegisterList AllLiveRegisters() const",
  "[public/method] void set_observer(Observer* observer)",
  "[public/method] int next_register_index() const",
  "[public/method] int maximum_register_count() const",
  "[private/field] int next_register_index_",
  "[private/field] int max_register_count_",
  "[private/field] Observer* observer_",
];

/// 原版 class/struct Observer 成员声明 (机械 1:1 提取, 5 条)
pub const REGISTER_ALLOCATOR_OBSERVER_DECLARATIONS: &[&str] = &[
  "[public/method] virtual ~Observer() = default",
  "[public/method] virtual void RegisterAllocateEvent(Register reg) = 0",
  "[public/method] virtual void RegisterListAllocateEvent(RegisterList reg_list) = 0",
  "[public/method] virtual void RegisterListFreeEvent(RegisterList reg_list) = 0",
  "[public/method] virtual void RegisterFreeEvent(Register reg_list) = 0",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_REGISTER_ALLOCATOR_DECLARATIONS.len() == 19);
};
