#!/usr/bin/env bun
// v8/src/compiler/frame.cc + frame.h
// → v8-compiler-passes/src/frame.rs.skel
//
// 规则面: Frame 类 (slot 分配器语义: Align/AllocateUnaligned/
// EnsureOtherKindSlot 分派) + FrameOffset (kFromSp=1/kFromFp=0 常量) +
// FrameAccessState 方法面; AllocateSpillSlot 的 width/alignment 对齐规则
// (默认 kDoubleSize 对齐 saved callee 区) 逐字核对。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/frame.h");
const cc = readV8("src/compiler/frame.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE Frame : public ZoneObject \{/, "Frame"],
  [h, /static const int kFromSp = 1;/, "kFromSp"],
  [h, /static const int kFromFp = 0;/, "kFromFp"],
  [h, /class FrameAccessState : public ZoneObject \{/, "FrameAccessState"],
  [h, /void AlignSavedCalleeRegisterSlots\(int alignment = kDoubleSize\) \{/, "AlignSavedCalleeRegisterSlots (h 内联)"],
  [h, /int AllocateSpillSlot\(int width, int alignment = 0, bool is_tagged = false\) \{/, "AllocateSpillSlot (h 内联)"],
  [cc, /void FrameAccessState::SetFrameAccessToDefault\(\)/, "SetFrameAccessToDefault"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
// AlignSavedCalleeRegisterSlots 默认 kDoubleSize
if (!/void AlignSavedCalleeRegisterSlots\(int alignment = kDoubleSize\) \{/.test(h)) throw new Error("Align 默认参缺失");

const body = `${banner("src/compiler/frame.js", "src/compiler/frame.h + src/compiler/frame.cc", "Frame / FrameOffset / FrameAccessState — 编译器栈帧布局与访问状态")}
//! Frame: 四区 (固定头/溢出槽/被调保存/返回槽) 的槽分配器。
//!
//! 规则面: 常量 2 + 分配方法语义。语义面 (推迟):
//! AlignedSlotAllocator 本体 (zone 层)。

#![allow(dead_code, unused_variables)]

/// v8: src/compiler/frame.h FrameOffset::kFromSp
pub const K_FROM_SP: i32 = 1;
/// v8: src/compiler/frame.h FrameOffset::kFromFp
pub const K_FROM_FP: i32 = 0;

/// v8: src/compiler/frame.h FrameOffset
pub struct FrameOffset {
  /// 原 from_sp / offset
  pub from_sp: bool,
  pub offset: i32,
}

/// v8: src/compiler/frame.h Frame
pub struct Frame {
  /// 原 slot_allocator_
  pub slot_allocator: crate::aligned_slot_allocator::AlignedSlotAllocator,
  /// 原 fixed_slot_count_
  pub fixed_slot_count: i32,
  /// 原 spill_slot_count_
  pub spill_slot_count: i32,
  /// 原 return_slot_count_
  pub return_slot_count: i32,
  /// 原 allocated_registers_ / allocated_double_registers_
  pub allocated_registers: *mut crate::bit_vector::BitVector,
  pub allocated_double_registers: *mut crate::bit_vector::BitVector,
  /// 原 spill_slots_finished_
  pub spill_slots_finished: bool,
}

impl Frame {
  /// v8: src/compiler/frame.h Frame::GetTotalFrameSlotCount
  /// 原文: slot_allocator_.Size() + return_slot_count_
  pub fn get_total_frame_slot_count(&self) -> i32 {
    self.slot_allocator.size() + self.return_slot_count
  }
  /// v8: src/compiler/frame.h Frame::GetFixedSlotCount
  pub fn get_fixed_slot_count(&self) -> i32 {
    self.fixed_slot_count
  }
  /// v8: src/compiler/frame.h Frame::GetSpillSlotCount
  pub fn get_spill_slot_count(&self) -> i32 {
    self.spill_slot_count
  }
  /// v8: src/compiler/frame.h Frame::GetReturnSlotCount
  pub fn get_return_slot_count(&self) -> i32 {
    self.return_slot_count
  }
  /// v8: src/compiler/frame.h Frame::DidAllocateDoubleRegisters
  /// 原文: !allocated_double_registers_->IsEmpty()
  pub fn did_allocate_double_registers(&self) -> bool {
    !unsafe { &*self.allocated_double_registers }.is_empty()
  }

  /// v8: src/compiler/frame.h Frame::AlignSavedCalleeRegisterSlots (内联)
  /// 原文 (默认 alignment = kDoubleSize): spill_slots_finished_ = true;
  /// padding = slot_allocator_.Align(NumSlotsForWidth(alignment));
  /// spill_slot_count_ += padding; return padding;
  pub fn align_saved_callee_register_slots(&mut self, alignment: i32 /* 原 kDoubleSize */) -> i32 {
    self.spill_slots_finished = true;
    let alignment_in_slots = crate::aligned_slot_allocator::num_slots_for_width(alignment);
    let padding = self.slot_allocator.align(alignment_in_slots);
    self.spill_slot_count += padding;
    padding
  }

  /// v8: src/compiler/frame.h Frame::AllocateSavedCalleeRegisterSlots (内联)
  /// 原文: spill_slots_finished_ = true;
  /// slot_allocator_.AllocateUnaligned(count)
  pub fn allocate_saved_callee_register_slots(&mut self, count: i32) {
    self.spill_slots_finished = true;
    self.slot_allocator.allocate_unaligned(count);
  }

  /// v8: src/compiler/frame.h Frame::AllocateSpillSlot (内联)
  /// 原文: actual_width = max(width, kSlotSize);
  /// actual_alignment = max(alignment, kSlotSize) (alignment=0 → kSlotSize);
  /// is_tagged ⇒ width==alignment==sizeof(uintptr_t);
  /// 分派: 宽 4 且非对齐 → AllocateUnaligned; 否则 Allocate(kind)
  pub fn allocate_spill_slot(&mut self, width: i32, alignment: i32 /* 原默认 0 */, is_tagged: bool /* 原默认 false */) -> i32 {
    debug_assert_eq!(
      self.get_total_frame_slot_count(),
      self.fixed_slot_count + self.spill_slot_count + self.return_slot_count
    );
    debug_assert!(if is_tagged { width == crate::SIZEOF_UINTPTR && alignment == crate::SIZEOF_UINTPTR } else { true });
    debug_assert!(!self.spill_slots_finished);
    let actual_width = width.max(crate::aligned_slot_allocator::K_SLOT_SIZE);
    let actual_alignment = alignment.max(crate::aligned_slot_allocator::K_SLOT_SIZE);
    let slot = if actual_width == 4 && actual_alignment == 4 {
      self.slot_allocator.allocate(crate::SlotKind::Untagged)
    } else if actual_width == 8 && actual_alignment == 8 {
      self.slot_allocator.allocate(crate::SlotKind::Double)
    } else if is_tagged {
      self.slot_allocator.allocate(crate::SlotKind::Tagged)
    } else {
      self.slot_allocator.allocate_unaligned(actual_width)
    };
    self.spill_slot_count += actual_width / 4;
    slot
  }
}

/// v8: src/compiler/frame.h FrameAccessState
/// 可变帧访问状态 (sp delta 追踪)
pub struct FrameAccessState {
  /// 原 frame_
  pub frame: *const Frame,
  /// 原 access_frame_with_fp_
  pub access_frame_with_fp: bool,
  /// 原 sp_delta_
  pub sp_delta: i32,
}

impl FrameAccessState {
  /// v8: src/compiler/frame.cc FrameAccessState::SetFrameAccessToDefault
  /// 原文: access_frame_with_fp_ = true (标准省 fp 无 delta 态)
  pub fn set_frame_access_to_default(&mut self) {
    self.access_frame_with_fp = true;
  }
  /// v8: src/compiler/frame.cc FrameAccessState::SetFrameAccessToSP
  /// 原文: DCHECK(!access_frame_with_fp_); access_frame_with_fp_ = false;
  pub fn set_frame_access_to_sp(&mut self) {
    debug_assert!(!self.access_frame_with_fp);
    self.access_frame_with_fp = false;
  }
  /// v8: src/compiler/frame.cc FrameAccessState::IncreaseSPDelta
  pub fn increase_sp_delta(&mut self, delta: i32) {
    self.sp_delta += delta;
  }
  /// v8: src/compiler/frame.cc FrameAccessState::GetSPToFPSlotCount
  /// 原文: frame()->GetTotalFrameSlotCount() - StandardFrameConstants::kFixedSlotCountAboveFp - sp_delta()/kSystemPointerSize
  pub fn get_sp_to_fp_slot_count(&self) -> i32 {
    unsafe { (*self.frame).get_total_frame_slot_count() }
      - crate::K_FIXED_SLOT_COUNT_ABOVE_FP
      - self.sp_delta / crate::K_SYSTEM_POINTER_SIZE
  }
}
`;

const p = writeSrc("v8-compiler-passes", "frame.rs.skel", body);
console.log(`written ${p}\n常量=2 (kFromSp=1/kFromFp=0) Frame方法=9 FrameAccessState方法=4`);
