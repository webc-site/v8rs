#!/usr/bin/env bun
// v8/src/compiler/node.h + node.cc
// → v8-compiler-graph/src/node.rs 基线
//
// Node = zone bump 分配 + Use 前缀布局 (inputs 内联于 Node 后 / 溢出走
// OutOfLineInputs), Use 双向链 1:1:
//
//   inline:   |Use#N|...|Use#0|Node xxxx|I#0|...|I#N|
//   outline:  |Use#N|...|Use#0|OOL hdr  |I#0|...|I#N|  (Node 尾槽存 OOL 指针)
//
// 可变字段包 UnsafeCell — 共享视图 &Node 经 get() 合法写 (arena 所有权,
// 对应原版 const 方法内改内存); repr(C) 下 UnsafeCell<T> 与 T 同布局。
// 对数断言:
// - 结构尺寸 Node=24 / Use=16 / OutOfLineInputs=16 (64 位)
// - kOutlineMarker=15 / kMaxInlineCapacity=14 (位域宽 4)
// - IdField 24 位 → id 域上界 2^24-1 < NodeId::MAX
// - node.cc 转写面 20 函数清点
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/node.h");
const cc = readV8("src/compiler/node.cc");

// —— 原文哨兵核对 (防源漂移) ——
const sentinels = [
  [/struct OutOfLineInputs \{/, "OutOfLineInputs 定义"],
  [/Use\* start = this \+ 1 \+ index;/, "Use 前缀步长公式"],
  [/ptr\[-1 - input_index\];/, "GetUsePtr 逆序公式"],
  [/int capacity = std::max\(1, input_count\);/, "inline capacity 下限 1"],
  [/capacity = std::min\(input_count \+ 3, max\);/, "extensible 内联余量 +3"],
  [/input_count \* 2 \+ 3/, "OOL 扩容 2n+3"],
  [/using IdField = base::BitField<NodeId, 0, 24>;/, "IdField 24 位"],
  [/using InlineCountField = base::BitField<unsigned, 24, 4>;/, "InlineCount 4 位 @24"],
  [/using InlineCapacityField = base::BitField<unsigned, 28, 4>;/, "InlineCapacity 4 位 @28"],
  [/static const int kMaxInlineCapacity = InlineCapacityField::kMax - 1;/, "kMaxInlineCapacity"],
  [/inputs\[0\] == nullptr;/, "IsDead 判据"],
];
for (const [re, name] of sentinels) {
  if (!re.test(h) && !re.test(cc)) throw new Error(`哨兵缺失: ${name}`);
}
const fns = [
  "Node::OutOfLineInputs* Node::OutOfLineInputs::New",
  "void Node::OutOfLineInputs::ExtractFrom",
  "Node* Node::NewImpl",
  "Node* Node::Clone",
  "void Node::Kill",
  "void Node::AppendInput",
  "void Node::InsertInput",
  "void Node::InsertInputs",
  "Node* Node::RemoveInput",
  "void Node::ClearInputs",
  "void Node::NullAllInputs",
  "void Node::TrimInputCount",
  "void Node::EnsureInputCount",
  "int Node::UseCount",
  "int Node::BranchUseCount",
  "void Node::ReplaceUses",
  "bool Node::OwnedBy",
  "void Node::AppendUse",
  "void Node::RemoveUse",
  "void Node::Verify",
];
for (const fn of fns) {
  if (!cc.includes(fn)) throw new Error(`node.cc 缺函数 ${fn}`);
}

const body = `${banner("src/compiler/node.js", "src/compiler/node.h + node.cc")}
//! Node — 图基本原语: 输入/使用链互连 + id (辅助数据索引键) + 可变 operator
//!
//! zone bump 分配 + Use 前缀布局 1:1 (node.h:208-237 内存布局注释):
//! Use 链直接排在 Node (或 OutOfLineInputs 头) 之前, 输入指针数组紧随其后;
//! 每个 Use 记录 input_index, 指针算术在 Use 区与头之间往返。
//! 输入槽为裸指针, 可为 null (原版 InsertInputs 空档; \`input_at\` 返回 Option)。
//!
//! SAFETY 总则:
//! 1. Node/Use/OOL 由 v8-zone 8 对齐 bump 分配, 生命周期同 zone;
//! 2. Node 可变字段包 UnsafeCell — &Node 共享视图下合法写 (arena 所有权,
//!    对应原版 const 方法改内存); Use/OOL 区仅经分配返回的裸指针访问;
//! 3. type_ 字段、Print(std::ostream) 深度展示、NodeMarker 友元面推迟 (README)。

use std::cell::UnsafeCell;
use std::fmt;
use std::marker::PhantomData;
use std::ptr;

use v8_base::bit_field::BitField32;
use v8_zone::Zone;

use crate::opcodes::IrOpcode;
use crate::operator::Operator;
use crate::Error;

/// 原版 Mark — 遍历标记 (NodeMarker 单调递增域)
pub type Mark = u32;
/// 原版 NodeId — 辅助数据索引键
pub type NodeId = u32;

/// 输入槽 (原版 ZoneNodePtr; 可为 null)
type NodeSlot = *mut Node;

/// IdField: NodeId @ [0,24)
type IdField = BitField32<NodeId, 0, 24>;
/// InlineCountField: @ [24,28)
type InlineCountField = BitField32<u32, 24, 4>;
/// InlineCapacityField: @ [28,32)
type InlineCapacityField = BitField32<u32, 28, 4>;
/// Use::InputIndexField: @ [1,32) (Use::InlineField 占 bit0)
type UseIndexField = BitField32<u32, 1, 31>;

/// 原版 kOutlineMarker — InlineCount 的外联哨兵值 (InlineCountField 4 位 kMax=15)
pub const K_OUTLINE_MARKER: u32 = 15;
/// 原版 kMaxInlineCapacity — 内联容量上界 (InlineCapacityField 4 位 kMax-1)
pub const K_MAX_INLINE_CAPACITY: u32 = 14;

// ---------------------------------------------------------------------------
// Use — 使用链节点 (输入 i 的 Use 挂在被用节点的双向链上)
// ---------------------------------------------------------------------------

#[repr(C)]
struct Use {
  next: *mut Use,
  prev: *mut Use,
  bit_field: u32,
}

impl Use {
  /// bit0 — 内联布局标记
  const INLINE: u32 = 1;

  fn input_index(&self) -> usize {
    UseIndexField::decode(self.bit_field) as usize
  }

  fn is_inline_use(&self) -> bool {
    self.bit_field & Self::INLINE != 0
  }

  /// 原版 Use::input_ptr() — 由 Use 反查输入槽
  fn input_ptr(&self) -> *mut NodeSlot {
    let index = self.input_index();
    // SAFETY: Use 恒由 Node/OOL 前缀区派生, this+1+index 落回首部 (原版步长公式)
    let start = (ptr::from_ref(self) as *mut Use).wrapping_add(1 + index);
    // SAFETY: Use 属主头与输入区均在位
    unsafe {
      if self.is_inline_use() {
        node_inputs(start.cast::<Node>()).add(index)
      } else {
        ool_inputs(start.cast::<OutOfLineInputs>()).add(index)
      }
    }
  }

  /// 原版 Use::from() — 由 Use 反查属主节点
  fn from(&self) -> NodeSlot {
    let index = self.input_index();
    // SAFETY: 同 input_ptr 的布局不变式
    let start = (ptr::from_ref(self) as *mut Use).wrapping_add(1 + index);
    if self.is_inline_use() {
      start.cast::<Node>()
    } else {
      // SAFETY: start 即 OOL 头, 其 node 指向属主
      unsafe { (*start.cast::<OutOfLineInputs>()).node }
    }
  }
}

// ---------------------------------------------------------------------------
// OutOfLineInputs — 外联输入存储 (输入数超内联容量时)
// ---------------------------------------------------------------------------

#[repr(C)]
struct OutOfLineInputs {
  /// 属主节点
  node: NodeSlot,
  count: i32,
  capacity: i32,
}

impl OutOfLineInputs {
  /// 原版 OutOfLineInputs::New — 布局 [capacity Use][OOL 头][capacity 输入槽]
  fn new(zone: &Zone<'_>, capacity: usize) -> Result<*mut OutOfLineInputs, Error> {
    let size = size_of::<OutOfLineInputs>() + capacity * (size_of::<NodeSlot>() + size_of::<Use>());
    let buf = zone.allocate_bytes_ref(size)?;
    // SAFETY: 区间独占 8 对齐; 头部偏移 capacity*Use 仍对齐; 完成头部构造后才交出
    let outline = unsafe { buf.as_mut_ptr().add(capacity * size_of::<Use>()) }.cast::<OutOfLineInputs>();
    // SAFETY: outline 指向本分配内的头位置, 随即完全初始化
    unsafe {
      ptr::write(
        outline,
        OutOfLineInputs { node: ptr::null_mut(), count: 0, capacity: capacity as i32 },
      );
    }
    Ok(outline)
  }

  /// 原版 ExtractFrom — 旧输入与 Use 迁入本外联区, 尾部回写 count
  fn extract_from(this: *mut OutOfLineInputs, mut old_use_ptr: *mut Use, mut old_input_ptr: *mut NodeSlot, count: usize) {
    // SAFETY: this 为本节点外联头 (仅此处写头部)
    let mut new_use_ptr = (unsafe { &*this }).header_use_base().wrapping_sub(1);
    let mut new_input_ptr = ool_inputs(this);
    let mut current = 0;
    while current < count {
      // SAFETY: 索引 [0,count) 在新旧容量内
      unsafe {
        (*new_use_ptr).bit_field = UseIndexField::encode(current as u32);
        let old_to = *old_input_ptr;
        if !old_to.is_null() {
          *old_input_ptr = ptr::null_mut();
          (*old_to).remove_use(old_use_ptr);
          *new_input_ptr = old_to;
          (*old_to).append_use(new_use_ptr);
        } else {
          *new_input_ptr = ptr::null_mut();
        }
        old_input_ptr = old_input_ptr.add(1);
        new_input_ptr = new_input_ptr.add(1);
        old_use_ptr = old_use_ptr.sub(1);
        new_use_ptr = new_use_ptr.sub(1);
      }
      current += 1;
    }
    // SAFETY: 头部独占写
    unsafe { (*this).count = count as i32; }
  }

  /// Use 区基址 (= 头自身地址)
  fn header_use_base(&self) -> *mut Use {
    ptr::from_ref(self) as *mut Use
  }
}

/// OOL 头后的输入槽区
fn ool_inputs(ool: *mut OutOfLineInputs) -> *mut NodeSlot {
  // SAFETY: 输入槽紧随 OOL 头 (原版 inputs() 公式)
  unsafe { ool.byte_add(size_of::<OutOfLineInputs>()) }.cast()
}

/// Node 尾部输入槽区 (inline: 输入; outline: 存 OOL 指针的单槽)
fn node_inputs(node: *mut Node) -> *mut NodeSlot {
  // SAFETY: 输入槽紧随 Node (原版 inputs_location = this + sizeof(Node))
  unsafe { node.byte_add(size_of::<Node>()) }.cast()
}

// ---------------------------------------------------------------------------
// Node
// ---------------------------------------------------------------------------

/// Node — 图节点 (原版 final class, 拷贝删除)
///
/// 布局 (repr(C), UnsafeCell 同布局): op | mark | bit_field | first_use = 24B
#[repr(C)]
pub struct Node {
  /// op_ — 可变 operator (lowering 阶段换)
  op: UnsafeCell<*const Operator>,
  /// mark_ — NodeMarker 域 (pub(crate): node-marker 层接管)
  mark: UnsafeCell<Mark>,
  /// bit_field_: id | inline_count | inline_capacity
  bit_field: UnsafeCell<u32>,
  /// first_use_ — 使用链头
  first_use: UnsafeCell<*mut Use>,
}

impl Node {
  /// 原版 Node::New — 工厂 (内联 ≤14 / 外联 >14)
  ///
  /// 输入非空由 \`&Node\` 类型保证 (原版 FATAL 分支天然消除)。
  pub fn new<'z>(
    zone: &'z Zone<'_>,
    id: NodeId,
    op: &Operator,
    inputs: &[&Node],
    extensible: bool,
  ) -> Result<&'z Node, Error> {
    if !IdField::is_valid(id) {
      return Err(Error::NodeIdOverflow { id });
    }
    let count = inputs.len();
    if count > 0 && count - 1 > UseIndexField::k_max() as usize {
      return Err(Error::InputTooMany { count });
    }
    let node = if count > K_MAX_INLINE_CAPACITY as usize {
      Self::alloc_outline(zone, id, op, inputs, extensible)?
    } else {
      Self::alloc_inline(zone, id, op, inputs, extensible)?
    };
    // SAFETY: node 由本 zone 分配且已完整构造, 生命周期同 zone
    let node = unsafe { &*node };
    node.verify();
    Ok(node)
  }

  /// 原版 Node::Clone — 同输入新节点 (type_ 复制随 turbofan-types 层推迟)
  pub fn clone_node<'z>(zone: &'z Zone<'_>, id: NodeId, node: &Node) -> Result<&'z Node, Error> {
    let count = node.input_count();
    let root = if node.has_inline_inputs() {
      node_inputs(ptr::from_ref(node) as *mut Node)
    } else {
      ool_inputs(node.outline_ptr())
    };
    let mut inputs = Vec::with_capacity(count);
    for i in 0..count {
      // SAFETY: 槽位 [0,count) 有效
      let slot = unsafe { *root.add(i) };
      // SAFETY: 槽位非空时指向同 zone 存活节点
      let input = unsafe { slot.as_ref() }.ok_or(Error::NullInput { id: node.id(), mnemonic: node.op().mnemonic(), index: i })?;
      inputs.push(input);
    }
    Self::new(zone, id, node.op(), &inputs, false)
  }

  /// 内联分配 (node.cc NewImpl 93-112 分支)
  fn alloc_inline(
    zone: &Zone<'_>,
    id: NodeId,
    op: &Operator,
    inputs: &[&Node],
    extensible: bool,
  ) -> Result<*mut Node, Error> {
    let count = inputs.len();
    // 容量至少 1 — 留外联指针槽 (原版注释)
    let mut capacity = count.max(1);
    if extensible {
      capacity = (count + 3).min(K_MAX_INLINE_CAPACITY as usize);
    }
    let size = size_of::<Node>() + capacity * (size_of::<NodeSlot>() + size_of::<Use>());
    let buf = zone.allocate_bytes_ref(size)?;
    // SAFETY: 区间独占; Node 位于 capacity 个 Use 之后, 偏移保持对齐; 随即完整构造
    let node = unsafe { buf.as_mut_ptr().add(capacity * size_of::<Use>()) }.cast::<Node>();
    Self::init(node, id, op, count as u32, capacity as u32);
    let input_ptr = node_inputs(node);
    let use_ptr = node.cast::<Use>();
    Self::wire_inputs(input_ptr, use_ptr, inputs, true);
    Ok(node)
  }

  /// 外联分配 (node.cc NewImpl 75-92 分支)
  fn alloc_outline(
    zone: &Zone<'_>,
    id: NodeId,
    op: &Operator,
    inputs: &[&Node],
    extensible: bool,
  ) -> Result<*mut Node, Error> {
    let count = inputs.len();
    let capacity = if extensible { count + K_MAX_INLINE_CAPACITY as usize } else { count };
    let outline = OutOfLineInputs::new(zone, capacity)?;
    // Node 带单外联指针槽
    let buf = zone.allocate_bytes_ref(size_of::<Node>() + size_of::<NodeSlot>())?;
    // SAFETY: 区间独占, 随即完整构造
    let node = buf.as_mut_ptr().cast::<Node>();
    Self::init(node, id, op, K_OUTLINE_MARKER, 0);
    // SAFETY: 尾槽与外联头部独占写
    unsafe {
      *node_inputs(node) = outline.cast::<Node>();
      (*outline).node = node;
      (*outline).count = count as i32;
    }
    let input_ptr = ool_inputs(outline);
    let use_ptr = outline.cast::<Use>();
    Self::wire_inputs(input_ptr, use_ptr, inputs, false);
    Ok(node)
  }

  /// 原版 Node 构造函数 — 位域打包
  fn init(node: *mut Node, id: NodeId, op: &Operator, inline_count: u32, inline_capacity: u32) {
    let bit_field =
      IdField::encode(id) | InlineCountField::encode(inline_count) | InlineCapacityField::encode(inline_capacity);
    // SAFETY: node 指向未初始化独占区间, ptr::write 完成构造
    unsafe {
      ptr::write(
        node,
        Node {
          op: UnsafeCell::new(ptr::from_ref(op)),
          mark: UnsafeCell::new(0),
          bit_field: UnsafeCell::new(bit_field),
          first_use: UnsafeCell::new(ptr::null_mut()),
        },
      );
    }
  }

  /// 原版 NewImpl 117-124 — 写输入槽 + 挂 Use
  fn wire_inputs(input_ptr: *mut NodeSlot, use_ptr: *mut Use, inputs: &[&Node], is_inline: bool) {
    let inline_bit = if is_inline { Use::INLINE } else { 0 };
    for (current, to) in inputs.iter().enumerate() {
      // SAFETY: current < capacity (分配保证), Use 区头前递减
      unsafe {
        *input_ptr.add(current) = ptr::from_ref(*to) as *mut Node;
        let use_ = use_ptr.sub(1 + current);
        (*use_).bit_field = UseIndexField::encode(current as u32) | inline_bit;
        (**to).append_use(use_);
      }
    }
  }

  /// 原版 IsDead — 首输入为 null 即死 (InsertInputs 空档)
  pub fn is_dead(&self) -> bool {
    self.input_count() > 0 && self.input_at(0).is_none()
  }

  /// 原版 Kill — 断全部输入 (调用方保证无使用)
  pub fn kill(&self) {
    self.null_all_inputs();
    debug_assert!(self.uses().next().is_none());
  }

  /// 原版 op()
  pub fn op(&self) -> &Operator {
    // SAFETY: op 由 New/Clone/set_op 写入, 生命周期 ≥ 节点 (调用方契约)
    unsafe { &**self.op.get() }
  }

  /// 原版 opcode() — 无检查解码 (Operator 构造已校验域)
  pub fn opcode(&self) -> IrOpcode {
    // SAFETY: opcode ∈ [0, LAST_U16] 由 Operator::new 保证
    unsafe { IrOpcode::from_repr_unchecked(self.op().opcode()) }
  }

  /// 原版 id()
  pub fn id(&self) -> NodeId {
    // SAFETY: 位域只读
    IdField::decode(unsafe { *self.bit_field.get() })
  }

  /// 原版 InputCount
  pub fn input_count(&self) -> usize {
    if self.has_inline_inputs() {
      InlineCountField::decode(unsafe { *self.bit_field.get() }) as usize
    } else {
      // SAFETY: 外联头由尾槽指向, 同 zone 存活
      unsafe { (*self.outline_ptr()).count as usize }
    }
  }

  /// 原版 InputAt — null 输入返回 None
  pub fn input_at(&self, index: usize) -> Option<&Node> {
    debug_assert!(index < self.input_count());
    // SAFETY: index < count 时槽位有效
    unsafe { (*self.input_ptr_at(index)).as_ref() }
  }

  /// 原版 ReplaceInput — 换输入并迁移 Use (new_to 可为 null)
  pub fn replace_input(&self, index: usize, new_to: Option<&Node>) {
    debug_assert!(index < self.input_count());
    // SAFETY: 槽/Use 配对由布局派生
    unsafe {
      let input_ptr = self.input_ptr_at(index);
      let old_to = *input_ptr;
      let new_to = new_to.map_or(ptr::null_mut(), |n| ptr::from_ref(n) as *mut Node);
      if old_to != new_to {
        let use_ = self.use_ptr_at(index);
        if !old_to.is_null() {
          (*old_to).remove_use(use_);
        }
        *input_ptr = new_to;
        if !new_to.is_null() {
          (*new_to).append_use(use_);
        }
      }
    }
  }

  /// 原版 AppendInput — 尾插 (可触发内联→外联迁移或外联扩容)
  pub fn append_input(&self, zone: &Zone<'_>, new_to: &Node) -> Result<(), Error> {
    let bit_field = unsafe { *self.bit_field.get() };
    let inline_count = InlineCountField::decode(bit_field);
    let inline_capacity = InlineCapacityField::decode(bit_field);
    if inline_count < inline_capacity {
      // 内联直插
      let count = inline_count as usize;
      // SAFETY: 内联余量足够; 位域/槽/Use 均在布局内
      unsafe {
        *self.bit_field.get() = InlineCountField::update(bit_field, inline_count + 1);
        *self.input_ptr_at(count) = ptr::from_ref(new_to) as *mut Node;
        let use_ = self.use_ptr_at(count);
        (*use_).bit_field = UseIndexField::encode(inline_count) | Use::INLINE;
        new_to.append_use(use_);
      }
    } else {
      let input_count = self.input_count();
      let outline = if inline_count != K_OUTLINE_MARKER {
        // 内联 → 外联迁移
        let outline = OutOfLineInputs::new(zone, input_count * 2 + 3)?;
        // SAFETY: 外联头独占写
        unsafe {
          (*outline).node = ptr::from_ref(self) as *mut Node;
          OutOfLineInputs::extract_from(outline, self.use_ptr_at(0), self.input_ptr_at(0), input_count);
        }
        // SAFETY: 位域独占写
        unsafe { *self.bit_field.get() = InlineCountField::update(bit_field, K_OUTLINE_MARKER); }
        self.set_outline(outline);
        outline
      } else {
        // 已外联, 容量满则扩容
        // SAFETY: 外联头有效
        let outline = self.outline_ptr();
        let full = input_count >= unsafe { (*outline).capacity } as usize;
        if full {
          let grown = OutOfLineInputs::new(zone, input_count * 2 + 3)?;
          // SAFETY: 同迁移路径
          unsafe {
            (*grown).node = ptr::from_ref(self) as *mut Node;
            OutOfLineInputs::extract_from(grown, self.use_ptr_at(0), self.input_ptr_at(0), input_count);
          }
          self.set_outline(grown);
          grown
        } else {
          outline
        }
      };
      // SAFETY: 外联头有效, count/槽/Use 在容量内
      unsafe {
        (*outline).count += 1;
        *self.input_ptr_at(input_count) = ptr::from_ref(new_to) as *mut Node;
        let use_ = self.use_ptr_at(input_count);
        (*use_).bit_field = UseIndexField::encode(input_count as u32);
        new_to.append_use(use_);
      }
    }
    self.verify();
    Ok(())
  }

  /// 原版 InsertInput — 尾插同值后滚动后移
  pub fn insert_input(&self, zone: &Zone<'_>, index: usize, new_to: &Node) -> Result<(), Error> {
    debug_assert!(index < self.input_count());
    let last_idx = self.input_count() - 1;
    let last = self.input_at(last_idx).ok_or(Error::NullInput { id: self.id(), mnemonic: self.op().mnemonic(), index: last_idx })?;
    self.append_input(zone, last)?;
    let mut i = self.input_count() - 1;
    while i > index {
      let prev = self.input_at(i - 1);
      self.replace_input(i, prev);
      i -= 1;
    }
    self.replace_input(index, Some(new_to));
    self.verify();
    Ok(())
  }

  /// 原版 InsertInputs — 先复制尾部扩 count, 再整体滚动, 末段置 null
  pub fn insert_inputs(&self, zone: &Zone<'_>, index: usize, count: usize) -> Result<(), Error> {
    debug_assert!(index < self.input_count());
    debug_assert!(count > 0);
    for _ in 0..count {
      let src = self.input_count().saturating_sub(count);
      let tail = self.input_at(src).ok_or(Error::NullInput { id: self.id(), mnemonic: self.op().mnemonic(), index: src })?;
      self.append_input(zone, tail)?;
    }
    let mut i = self.input_count() - count - 1;
    while i >= index.max(count) {
      let prev = self.input_at(i - count);
      self.replace_input(i, prev);
      i -= 1;
    }
    for k in 0..count {
      self.replace_input(index + k, None);
    }
    self.verify();
    Ok(())
  }

  /// 原版 RemoveInput — 滚动前移后裁尾, 返回被移除输入
  pub fn remove_input(&self, index: usize) -> Option<&Node> {
    debug_assert!(index < self.input_count());
    let result = self.input_at(index);
    let mut i = index;
    while i + 1 < self.input_count() {
      let next = self.input_at(i + 1);
      self.replace_input(i, next);
      i += 1;
    }
    self.trim_input_count(self.input_count() - 1);
    self.verify();
    result
  }

  /// 原版 ClearInputs — [start, start+count) 清 null 并摘 Use
  fn clear_inputs(&self, start: usize, count: usize) {
    let mut input_ptr = self.input_ptr_at(start);
    let mut use_ptr = self.use_ptr_at(start);
    let mut rest = count;
    while rest > 0 {
      // SAFETY: 槽/Use 在位且配对
      unsafe {
        let input = *input_ptr;
        *input_ptr = ptr::null_mut();
        if !input.is_null() {
          (*input).remove_use(use_ptr);
        }
        input_ptr = input_ptr.add(1);
        use_ptr = use_ptr.sub(1);
      }
      rest -= 1;
    }
    self.verify();
  }

  /// 原版 NullAllInputs
  pub fn null_all_inputs(&self) {
    let count = self.input_count();
    self.clear_inputs(0, count);
  }

  /// 原版 TrimInputCount — 裁到 new_input_count
  pub fn trim_input_count(&self, new_input_count: usize) {
    let current = self.input_count();
    debug_assert!(new_input_count <= current);
    if new_input_count == current {
      return;
    }
    self.clear_inputs(new_input_count, current - new_input_count);
    if self.has_inline_inputs() {
      // SAFETY: 位域独占写
      unsafe { *self.bit_field.get() = InlineCountField::update(*self.bit_field.get(), new_input_count as u32); }
    } else {
      // SAFETY: 外联头独占写
      unsafe { (*self.outline_ptr()).count = new_input_count as i32; }
    }
  }

  /// 原版 EnsureInputCount — 可裁可补 (补尾重复)
  pub fn ensure_input_count(&self, zone: &Zone<'_>, new_input_count: usize) -> Result<(), Error> {
    let mut current = self.input_count();
    debug_assert!(current != 0);
    if current > new_input_count {
      self.trim_input_count(new_input_count);
    } else {
      let dummy_idx = current - 1;
      let dummy = self.input_at(dummy_idx).ok_or(Error::NullInput { id: self.id(), mnemonic: self.op().mnemonic(), index: dummy_idx })?;
      while current < new_input_count {
        self.append_input(zone, dummy)?;
        current += 1;
      }
    }
    Ok(())
  }

  /// 原版 UseCount
  pub fn use_count(&self) -> usize {
    let mut n = 0;
    // SAFETY: Use 链由本节点持有
    let mut use_ = unsafe { *self.first_use.get() };
    while !use_.is_null() {
      // SAFETY: 链上节点有效
      unsafe { use_ = (*use_).next; }
      n += 1;
    }
    n
  }

  /// 原版 BranchUseCount — Branch 使用数
  pub fn branch_use_count(&self) -> usize {
    let mut n = 0;
    // SAFETY: Use 链有效
    let mut use_ = unsafe { *self.first_use.get() };
    while !use_.is_null() {
      // SAFETY: from() 属主节点有效
      unsafe {
        if (*(*use_).from()).opcode() == IrOpcode::Branch {
          n += 1;
        }
        use_ = (*use_).next;
      }
    }
    n
  }

  /// 原版 ReplaceUses — 把本节点全部使用嫁接到 replace_to
  pub fn replace_uses(&self, replace_to: &Node) {
    // SAFETY: 两条 Use 链均无别名; 链表指针独占写
    unsafe {
      let that = ptr::from_ref(replace_to) as *mut Node;
      let self_head = *self.first_use.get();
      let that_head = *(*that).first_use.get();
      debug_assert!(self_head.is_null() || (*self_head).prev.is_null());
      debug_assert!(that_head.is_null() || (*that_head).prev.is_null());
      let mut last_use: *mut Use = ptr::null_mut();
      let mut use_ = *self.first_use.get();
      while !use_.is_null() {
        *(*use_).input_ptr() = that;
        last_use = use_;
        use_ = (*use_).next;
      }
      if !last_use.is_null() {
        let that_head = *(*that).first_use.get();
        (*last_use).next = that_head;
        if !that_head.is_null() {
          (*that_head).prev = last_use;
        }
        *(*that).first_use.get() = *self.first_use.get();
      }
      *self.first_use.get() = ptr::null_mut();
    }
  }

  /// 原版 OwnedBy(owner) — 有使用且全部来自 owner
  pub fn owned_by(&self, owner: &Node) -> bool {
    let owner = ptr::from_ref(owner) as *mut Node;
    // SAFETY: Use 链有效
    unsafe {
      let first_head = *self.first_use.get();
      let mut use_ = first_head;
      while !use_.is_null() {
        if (*use_).from() != owner {
          return false;
        }
        use_ = (*use_).next;
      }
      !first_head.is_null()
    }
  }

  /// 原版 OwnedBy(owner1, owner2) — 使用恰来自两者
  pub fn owned_by2(&self, owner1: &Node, owner2: &Node) -> bool {
    let o1 = ptr::from_ref(owner1) as *mut Node;
    let o2 = ptr::from_ref(owner2) as *mut Node;
    let mut mask = 0u8;
    // SAFETY: Use 链有效
    unsafe {
      let mut use_ = *self.first_use.get();
      while !use_.is_null() {
        let from = (*use_).from();
        if from == o1 {
          mask |= 1;
        } else if from == o2 {
          mask |= 2;
        } else {
          return false;
        }
        use_ = (*use_).next;
      }
    }
    mask == 3
  }

  /// 原版 input_edges()
  pub fn input_edges(&self) -> InputEdges<'_> {
    if self.has_inline_inputs() {
      let node = ptr::from_ref(self) as *mut Node;
      InputEdges {
        input_ptr: node_inputs(node),
        use_ptr: node.cast::<Use>().wrapping_sub(1),
        rest: self.input_count(),
        _m: PhantomData,
      }
    } else {
      let ool = self.outline_ptr();
      // SAFETY: count 读自外联头
      InputEdges { input_ptr: ool_inputs(ool), use_ptr: ool.cast::<Use>().wrapping_sub(1), rest: unsafe { (*ool).count as usize }, _m: PhantomData }
    }
  }

  /// 原版 inputs() — 输入节点迭代器 (null 槽 → None)
  pub fn inputs(&self) -> Inputs<'_> {
    if self.has_inline_inputs() {
      Inputs { input_ptr: node_inputs(ptr::from_ref(self) as *mut Node), rest: self.input_count(), _m: PhantomData }
    } else {
      let ool = self.outline_ptr();
      // SAFETY: count 读自外联头
      Inputs { input_ptr: ool_inputs(ool), rest: unsafe { (*ool).count as usize }, _m: PhantomData }
    }
  }

  /// 原版 use_edges() — 使用边迭代器 (预取 next, 迭代中摘链安全)
  pub fn use_edges(&self) -> UseEdges<'_> {
    // SAFETY: 链头有效; next 预取自当前 Use
    unsafe {
      let current = *self.first_use.get();
      UseEdges { current, next: if current.is_null() { ptr::null_mut() } else { (*current).next }, _m: PhantomData }
    }
  }

  /// 原版 uses() — 使用方节点迭代器
  pub fn uses(&self) -> Uses<'_> {
    // SAFETY: 链头有效
    Uses { current: unsafe { *self.first_use.get() }, _m: PhantomData }
  }

  // —— 布局访问 (crate 内) ——

  fn has_inline_inputs(&self) -> bool {
    InlineCountField::decode(unsafe { *self.bit_field.get() }) != K_OUTLINE_MARKER
  }

  fn outline_ptr(&self) -> *mut OutOfLineInputs {
    // SAFETY: 外联时尾槽存 OOL 指针 (由 New/迁移写入)
    unsafe { *node_inputs(ptr::from_ref(self) as *mut Node) as *mut OutOfLineInputs }
  }

  fn set_outline(&self, ool: *mut OutOfLineInputs) {
    // SAFETY: 尾槽独占写
    unsafe { *node_inputs(ptr::from_ref(self) as *mut Node) = ool.cast::<Node>(); }
  }

  fn input_ptr_at(&self, index: usize) -> *mut NodeSlot {
    if self.has_inline_inputs() {
      // SAFETY: index < 内联容量 (调用方不变式)
      unsafe { node_inputs(ptr::from_ref(self) as *mut Node).add(index) }
    } else {
      // SAFETY: index < 外联容量
      unsafe { ool_inputs(self.outline_ptr()).add(index) }
    }
  }

  fn use_ptr_at(&self, index: usize) -> *mut Use {
    let base = if self.has_inline_inputs() {
      (ptr::from_ref(self) as *mut Node).cast::<Use>()
    } else {
      self.outline_ptr().cast::<Use>()
    };
    // SAFETY: Use 区在头前按 -1-index 递减排布
    unsafe { base.sub(1 + index) }
  }

  /// 原版 set_op — 仅 NodeProperties 层可用
  pub(crate) fn set_op(&self, op: &Operator) {
    // SAFETY: op 槽独占写
    unsafe { *self.op.get() = ptr::from_ref(op); }
  }

  /// 原版 mark() — NodeMarker 单调递增域 (node-marker 层接管; 值仅升不降)
  pub fn mark(&self) -> Mark {
    // SAFETY: 只读
    unsafe { *self.mark.get() }
  }

  pub fn set_mark(&self, mark: Mark) {
    // SAFETY: mark 槽独占写
    unsafe { *self.mark.get() = mark; }
  }

  /// 原版 Verify (DEBUG 构建) — 输入/Use/迭代三方一致性
  pub fn verify(&self) {
    if !cfg!(debug_assertions) {
      return;
    }
    let count = self.input_count();
    // 原版: 超大节点抽查 (count>200 且非整百跳过)
    if count > 200 && !count.is_multiple_of(100) {
      return;
    }
    for i in 0..count {
      // SAFETY: i < count 时 Use/槽在位
      debug_assert_eq!(i, unsafe { (*self.use_ptr_at(i)).input_index() });
      debug_assert_eq!(self.input_ptr_at(i), unsafe { (*self.use_ptr_at(i)).input_ptr() });
      debug_assert_eq!(count, self.input_count());
    }
    {
      let mut index = 0;
      for input in self.inputs() {
        debug_assert_eq!(self.input_at(index).map(ptr::from_ref), input.map(ptr::from_ref));
        index += 1;
      }
      debug_assert_eq!(count, index);
    }
    {
      let mut index = 0;
      for edge in self.input_edges() {
        debug_assert!(ptr::eq(edge.from(), self));
        debug_assert_eq!(index, edge.index());
        debug_assert_eq!(self.input_at(index).map(ptr::from_ref), edge.to().map(ptr::from_ref));
        index += 1;
      }
      debug_assert_eq!(count, index);
    }
  }

  // —— Use 链原语 (node.cc AppendUse/RemoveUse) ——

  fn append_use(&self, use_: *mut Use) {
    // SAFETY: use_ 归属本节点输入槽; 链头独占写
    unsafe {
      let first = *self.first_use.get();
      (*use_).next = first;
      (*use_).prev = ptr::null_mut();
      if !first.is_null() {
        (*first).prev = use_;
      }
      *self.first_use.get() = use_;
    }
  }

  fn remove_use(&self, use_: *mut Use) {
    // SAFETY: use_ 在本节点链上; 链头独占写
    unsafe {
      if !(*use_).prev.is_null() {
        (*(*use_).prev).next = (*use_).next;
      } else {
        *self.first_use.get() = (*use_).next;
      }
      if !(*use_).next.is_null() {
        (*(*use_).next).prev = (*use_).prev;
      }
    }
  }
}

impl fmt::Display for Node {
  /// 原版 operator<< (node.cc:375): "{id}: {mnemonic}({inputs})"
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}: {}", self.id(), self.op().mnemonic())?;
    if self.input_count() > 0 {
      write!(f, "(")?;
      for i in 0..self.input_count() {
        if i > 0 {
          write!(f, ", ")?;
        }
        match self.input_at(i) {
          Some(n) => write!(f, "{}", n.id())?,
          None => write!(f, "null")?,
        }
      }
      write!(f, ")")?;
    }
    Ok(())
  }
}

impl fmt::Debug for Node {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(self, f)
  }
}

// ---------------------------------------------------------------------------
// Edge — 单条使用边 (Use + 输入槽对)
// ---------------------------------------------------------------------------

/// Edge — 单条使用边 (Use + 输入槽对); 'n 锚定节点域生命周期
#[derive(Clone, Copy)]
pub struct Edge<'n> {
  use_: *mut Use,
  input_ptr: *mut NodeSlot,
  _m: PhantomData<&'n Node>,
}

impl<'n> Edge<'n> {
  /// 原版 from() — 使用方节点
  pub fn from(&self) -> &'n Node {
    // SAFETY: Use 属主恒非空且同 zone 存活
    unsafe { &*(*self.use_).from() }
  }

  /// 原版 to() — 被用节点 (可为 null)
  pub fn to(&self) -> Option<&'n Node> {
    // SAFETY: 槽位有效
    unsafe { (*self.input_ptr).as_ref() }
  }

  /// 原版 index()
  pub fn index(&self) -> usize {
    // SAFETY: Use 有效
    unsafe { (*self.use_).input_index() }
  }

  /// 原版 UpdateTo — 改指向并迁移 Use
  pub fn update_to(&self, new_to: Option<&Node>) {
    // SAFETY: 槽/Use 配对有效
    unsafe {
      let old_to = *self.input_ptr;
      let new_to = new_to.map_or(ptr::null_mut(), |n| ptr::from_ref(n) as *mut Node);
      if old_to != new_to {
        if !old_to.is_null() {
          (*old_to).remove_use(self.use_);
        }
        *self.input_ptr = new_to;
        if !new_to.is_null() {
          (*new_to).append_use(self.use_);
        }
      }
    }
  }

  fn new(use_: *mut Use, input_ptr: *mut NodeSlot) -> Self {
    debug_assert_eq!(input_ptr, unsafe { (*use_).input_ptr() });
    Self { use_, input_ptr, _m: PhantomData }
  }
}

impl PartialEq for Edge<'_> {
  /// 原版 operator== — 输入槽同一即同边
  fn eq(&self, other: &Self) -> bool {
    self.input_ptr == other.input_ptr
  }
}
impl Eq for Edge<'_> {}

// ---------------------------------------------------------------------------
// 迭代器
// ---------------------------------------------------------------------------

/// 原版 Node::InputEdges — 正向边迭代 (输入槽递增 / Use 递减)
pub struct InputEdges<'n> {
  input_ptr: *mut NodeSlot,
  use_ptr: *mut Use,
  rest: usize,
  _m: PhantomData<&'n Node>,
}

impl<'n> Iterator for InputEdges<'n> {
  type Item = Edge<'n>;

  fn next(&mut self) -> Option<Edge<'n>> {
    if self.rest == 0 {
      return None;
    }
    let edge = Edge::new(self.use_ptr, self.input_ptr);
    // SAFETY: 槽/Use 配对排布, 步进在容量内
    unsafe {
      self.input_ptr = self.input_ptr.add(1);
      self.use_ptr = self.use_ptr.sub(1);
    }
    self.rest -= 1;
    Some(edge)
  }
}

impl ExactSizeIterator for InputEdges<'_> {
  fn len(&self) -> usize {
    self.rest
  }
}

/// 原版 Node::Inputs — 输入节点迭代
pub struct Inputs<'n> {
  input_ptr: *mut NodeSlot,
  rest: usize,
  _m: PhantomData<&'n Node>,
}

impl<'n> Iterator for Inputs<'n> {
  type Item = Option<&'n Node>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.rest == 0 {
      return None;
    }
    // SAFETY: 槽位有效, 正向步进
    let n = unsafe { (*self.input_ptr).as_ref() };
    unsafe { self.input_ptr = self.input_ptr.add(1); }
    self.rest -= 1;
    Some(n)
  }
}

impl ExactSizeIterator for Inputs<'_> {
  fn len(&self) -> usize {
    self.rest
  }
}

/// 原版 Node::UseEdges — 使用边迭代 (预取 next, 迭代中摘链安全)
pub struct UseEdges<'n> {
  current: *mut Use,
  next: *mut Use,
  _m: PhantomData<&'n Node>,
}

impl<'n> Iterator for UseEdges<'n> {
  type Item = Edge<'n>;

  fn next(&mut self) -> Option<Edge<'n>> {
    if self.current.is_null() {
      return None;
    }
    // SAFETY: current 在链上, input_ptr 由 Use 反查
    let edge = Edge::new(self.current, unsafe { (*self.current).input_ptr() });
    self.current = self.next;
    // SAFETY: next 预取
    self.next = if self.current.is_null() { ptr::null_mut() } else { unsafe { (*self.current).next } };
    Some(edge)
  }
}

/// 原版 Node::Uses — 使用方节点迭代
pub struct Uses<'n> {
  current: *mut Use,
  _m: PhantomData<&'n Node>,
}

impl<'n> Iterator for Uses<'n> {
  type Item = &'n Node;

  fn next(&mut self) -> Option<Self::Item> {
    if self.current.is_null() {
      return None;
    }
    // SAFETY: from() 属主非空; 链指针有效
    unsafe {
      let node = &*(*self.current).from();
      self.current = (*self.current).next;
      Some(node)
    }
  }
}

// —— 布局不变式 ——

const _: () = {
  assert!(size_of::<Node>() == 24, "Node 布局 24B");
  assert!(size_of::<Use>() == 24, "Use 布局 24B (2 ptr + u32 对齐填充, 同原版 64 位)");
  assert!(size_of::<OutOfLineInputs>() == 16, "OOL 布局 16B");
  assert!(K_OUTLINE_MARKER == 15);
  assert!(K_MAX_INLINE_CAPACITY == 14);
  assert!(K_OUTLINE_MARKER == 15 && K_MAX_INLINE_CAPACITY == 14);
};
`;

const p = writeSrc("v8-compiler-graph", "node.rs", body);
console.log(`written ${p} (哨兵 ${sentinels.length} + 转写面 ${fns.length} 项核对通过)`);
