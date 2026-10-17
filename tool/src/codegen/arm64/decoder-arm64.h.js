#!/usr/bin/env bun
// tool/src/codegen/arm64/decoder-arm64.h.js
// 机械展开 v8/src/codegen/arm64/decoder-arm64.{h,cc}: VISITOR_LIST 访问者清单 + 解码树骨架
// → v8-codegen-arm64/src/decoder.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/codegen/arm64/decoder-arm64.h");
const ccSrc = readV8("src/codegen/arm64/decoder-arm64.cc");

const flatH = hSrc.replace(/\\\r?\n/g, " ");

// VISITOR_LIST 展开
const listBody = flatH.match(/#define VISITOR_LIST\(V\) ([\s\S]*?)(?=#define |\/\/ The Visitor)/);
if (!listBody) throw new Error("未找到 VISITOR_LIST");
const visitors = [...listBody[1].matchAll(/V\((\w+)\)/g)].map((x) => x[1]);
// 对数: 原版侧独立计数 (awk 统计 V( 行 = 74)
const EXPECT_VISITORS = 74;
if (visitors.length !== EXPECT_VISITORS)
  throw new Error(`VISITOR_LIST ${visitors.length} != ${EXPECT_VISITORS}`);
if (visitors[0] !== "PCRelAddressing" || visitors.at(-1) !== "Unimplemented")
  throw new Error("VISITOR_LIST 首尾不符");

// 解码树私有方法清点 (对数): 12 个 Decode* 
const decodeFns = [...hSrc.matchAll(/void (Decode\w+)\(Instruction\* instr\);/g)].map((x) => x[1]);
if (decodeFns.length !== 11) throw new Error(`解码树方法 ${decodeFns.length} != 11 (顶层 Decode 外 11 个一级节点)`);

// DispatchingDecoderVisitor 管理方法
for (const f of ["AppendVisitor", "PrependVisitor", "InsertVisitorBefore", "InsertVisitorAfter", "RemoveVisitor"]) {
  if (!hSrc.includes(f)) throw new Error(`缺 ${f}`);
}
// cc: 访问者管理实现在场 (解码树模板在 decoder-arm64-inl.h)
if (!ccSrc.includes("DispatchingDecoderVisitor::AppendVisitor") || !ccSrc.includes("DispatchingDecoderVisitor::RemoveVisitor"))
  throw new Error("decoder-arm64.cc 缺 DispatchingDecoderVisitor 管理");

let rust = banner(
  "src/codegen/arm64/decoder-arm64.h.js",
  "src/codegen/arm64/decoder-arm64.{h,cc}",
  "arm64 解码器: VISITOR_LIST 展开 + 解码树骨架",
);

rust += `
use crate::instructions::Instruction;

/// 原版 VISITOR_LIST 展开的访问者类别 (共 ${visitors.length} 项)。
/// DecoderVisitor 以 trait 表达: 每个类别一个 visit_* 方法。
#[rustfmt::skip]
pub const VISITOR_LIST: &[&str] = &[
${visitors.map((v) => `  ${JSON.stringify(v)},`).join("\n")}
];

/// 原版 DecoderVisitor 接口 (C++ 虚基类 → Rust trait):
/// 实现方为 ${visitors.length} 类别提供 Visit 方法 (反汇编器/模拟器)。默认空实现。
pub trait DecoderVisitor {
${visitors
  .map((v) => {
    const snake = v.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
    return `  /// 原版 Visit${v}\n  fn visit_${snake}(&mut self, _instr: &Instruction) {}`;
  })
  .join("\n")}
}

fn ptr_eq(a: *mut dyn DecoderVisitor, b: *mut dyn DecoderVisitor) -> bool {
  std::ptr::eq(a as *mut (), b as *mut ())
}

/// 原版 DispatchingDecoderVisitor: 访问者链派发
#[derive(Default)]
pub struct DispatchingDecoderVisitor {
  /// 原版 visitors_ (注册顺序即调用顺序)
  visitors_: Vec<*mut dyn DecoderVisitor>,
}

impl DispatchingDecoderVisitor {
  #[must_use]
  pub const fn new() -> Self {
    Self { visitors_: Vec::new() }
  }

  /// 原版 AppendVisitor: 尾插 (重复注册 = 移位到尾部)
  pub fn append_visitor(&mut self, visitor: *mut dyn DecoderVisitor) {
    self.remove_visitor(visitor);
    self.visitors_.push(visitor);
  }

  /// 原版 PrependVisitor: 头插
  pub fn prepend_visitor(&mut self, visitor: *mut dyn DecoderVisitor) {
    self.remove_visitor(visitor);
    self.visitors_.insert(0, visitor);
  }

  /// 原版 InsertVisitorBefore
  pub fn insert_visitor_before(
    &mut self,
    new_visitor: *mut dyn DecoderVisitor,
    registered: *mut dyn DecoderVisitor,
  ) {
    self.remove_visitor(new_visitor);
    if let Some(pos) = self.visitors_.iter().position(|&v| ptr_eq(v, registered)) {
      self.visitors_.insert(pos, new_visitor);
    } else {
      self.visitors_.push(new_visitor);
    }
  }

  /// 原版 InsertVisitorAfter
  pub fn insert_visitor_after(
    &mut self,
    new_visitor: *mut dyn DecoderVisitor,
    registered: *mut dyn DecoderVisitor,
  ) {
    self.remove_visitor(new_visitor);
    if let Some(pos) = self.visitors_.iter().position(|&v| ptr_eq(v, registered)) {
      self.visitors_.insert(pos + 1, new_visitor);
    } else {
      self.visitors_.push(new_visitor);
    }
  }

  /// 原版 RemoveVisitor
  pub fn remove_visitor(&mut self, visitor: *mut dyn DecoderVisitor) {
    self.visitors_.retain(|&v| !ptr_eq(v, visitor));
  }

  #[inline]
  #[must_use]
  pub fn visitors_count(&self) -> usize {
    self.visitors_.len()
  }

  pub fn decode(&mut self, instr: &Instruction) {
    Decoder::new(self).decode(instr);
  }
}

impl DecoderVisitor for DispatchingDecoderVisitor {
${visitors
  .map((v) => {
    const snake = v.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
    return `  fn visit_${snake}(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_${snake}(instr) };
    }
  }`;
  })
  .join("\n")}
}

impl<T: DecoderVisitor + ?Sized> DecoderVisitor for &mut T {
${visitors
  .map((v) => {
    const snake = v.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
    return `  fn visit_${snake}(&mut self, instr: &Instruction) {
    (**self).visit_${snake}(instr);
  }`;
  })
  .join("\n")}
}

/// 解码树一级分支 (原版 Decoder<V> 私有 Decode* 方法, 按 bits 27:24 分派)
#[rustfmt::skip]
pub const DECODE_TREE_NODES: &[(&str, &str)] = &[
${decodeFns.map((f) => `  (${JSON.stringify(f)}, ${JSON.stringify(f.replace(/^Decode/, ""))}),`).join("\n")}
];

/// 解码器核心 (参数化 V 访问者)
pub struct Decoder<V: DecoderVisitor> {
  pub visitor: V,
}

impl<V: DecoderVisitor> Decoder<V> {
  pub const fn new(visitor: V) -> Self {
    Self { visitor }
  }

  /// 顶层指令解码分派 (对齐 decoder-arm64-inl.h: Decode)
  pub fn decode(&mut self, instr: &Instruction) {
    if instr.bit_range(28, 27) == 0 {
      if instr.bit_range(31, 24) == 0x45
        && instr.bit(21) == 0
        && instr.bit_range(15, 12) == 0xB
        && instr.bit_range(11, 10) != 3
      {
        self.visitor.visit_svebit_perm(instr);
      } else {
        self.visitor.visit_unallocated(instr);
      }
    } else {
      match instr.bit_range(27, 24) {
        0x0 => self.decode_pcrel_addressing(instr),
        0x1 => self.decode_data_processing_immediate(instr),
        0xA | 0xB => self.decode_data_processing(instr),
        0x2 => self.decode_logical(instr),
        0x3 => self.decode_bitfield_extract(instr),
        0x4..=0x7 => self.decode_branch_system_exception(instr),
        0x8 | 0x9 | 0xC | 0xD => self.decode_load_store(instr),
        0xE | 0xF => self.decode_fp(instr),
        _ => self.visitor.visit_unallocated(instr),
      }
    }
  }

  pub fn decode_pcrel_addressing(&mut self, instr: &Instruction) {
    debug_assert_eq!(0x0, instr.bit_range(27, 24));
    self.visitor.visit_pcrel_addressing(instr);
  }

  pub fn decode_data_processing_immediate(&mut self, instr: &Instruction) {
    debug_assert_eq!(instr.bit(28), 1);
    debug_assert_eq!(0x1, instr.bit_range(27, 24));
    if instr.bit_range(23, 22) == 2 {
      self.visitor.visit_unallocated(instr);
    } else if instr.bit(23) == 1 {
      if instr.bit_range(30, 29) != 0 || instr.bit_range(21, 20) != 0 {
        self.visitor.visit_unallocated(instr);
      } else {
        self.visitor.visit_min_max_immediate(instr);
      }
    } else {
      self.visitor.visit_add_sub_immediate(instr);
    }
  }

  pub fn decode_logical(&mut self, instr: &Instruction) {
    if instr.bit(23) == 1 {
      self.visitor.visit_unallocated(instr);
    } else {
      self.visitor.visit_logical_immediate(instr);
    }
  }

  pub fn decode_bitfield_extract(&mut self, instr: &Instruction) {
    if instr.bit(23) == 0 {
      self.visitor.visit_bitfield(instr);
    } else {
      self.visitor.visit_extract(instr);
    }
  }

  pub fn decode_branch_system_exception(&mut self, instr: &Instruction) {
    match instr.bit_range(31, 29) {
      0 | 4 => self.visitor.visit_unconditional_branch(instr),
      1 | 5 => {
        if instr.bit(25) == 0 {
          self.visitor.visit_compare_branch(instr);
        } else {
          self.visitor.visit_test_branch(instr);
        }
      }
      2 => {
        if instr.bit(25) == 0 && instr.bit(24) == 0 {
          self.visitor.visit_conditional_branch(instr);
        } else {
          self.visitor.visit_unallocated(instr);
        }
      }
      6 => {
        if instr.bit(25) == 0 {
          if instr.bit(24) == 0 {
            if instr.bit_range(4, 2) != 0
              || instr.mask(0x00E0001D) == 0x00200001
              || instr.mask(0x00E0001D) == 0x00400001
              || instr.mask(0x00E0001E) == 0x00200002
              || instr.mask(0x00E0001E) == 0x00400002
              || instr.mask(0x00E0001C) == 0x00600000
              || instr.mask(0x00E0001C) == 0x00800000
              || instr.mask(0x00E0001F) == 0x00A00000
              || instr.mask(0x00C0001C) == 0x00C00000
            {
              self.visitor.visit_unallocated(instr);
            } else {
              self.visitor.visit_exception(instr);
            }
          } else if instr.bit_range(23, 22) == 0 {
            let masked = instr.mask(0x003FF0E0);
            if instr.bit_range(21, 19) == 0x4
              || masked == 0x00033000
              || masked == 0x003FF020
              || masked == 0x003FF060
              || masked == 0x003FF0E0
              || instr.mask(0x00388000) == 0x00008000
              || instr.mask(0x0038E000) == 0x00000000
              || instr.mask(0x0039E000) == 0x00002000
              || instr.mask(0x003AE000) == 0x00002000
              || instr.mask(0x003CE000) == 0x00042000
              || instr.mask(0x0038F000) == 0x00005000
              || instr.mask(0x0038E000) == 0x00006000
            {
              self.visitor.visit_unallocated(instr);
            } else {
              self.visitor.visit_system(instr);
            }
          } else {
            self.visitor.visit_unallocated(instr);
          }
        } else if instr.bit(24) == 1
          || instr.bit_range(20, 16) != 0x1F
          || instr.bit_range(15, 10) != 0
          || instr.bit_range(4, 0) != 0
          || instr.bit_range(24, 21) == 0x3
          || instr.bit_range(24, 22) == 0x3
        {
          self.visitor.visit_unallocated(instr);
        } else {
          self.visitor.visit_unconditional_branch_to_register(instr);
        }
      }
      _ => self.visitor.visit_unallocated(instr),
    }
  }

  pub fn decode_load_store(&mut self, instr: &Instruction) {
    if instr.bit(28) == 0 && instr.bit(29) == 0 && instr.bit(26) == 1 {
      self.decode_neon_load_store(instr);
      return;
    }

    if instr.bit(24) == 0 {
      if instr.bit(28) == 0 {
        if instr.bit(29) == 0 {
          if instr.bit(26) == 0 {
            if instr.mask(0xA08000) == 0x800000 {
              self.visitor.visit_unallocated(instr);
            } else if instr.mask(0xA08000) == 0 {
              self.visitor.visit_unimplemented(instr);
            } else {
              self.visitor.visit_load_store_acquire_release(instr);
            }
          } else {
            self.decode_neon_load_store(instr);
          }
        } else if instr.bit(23) == 0 {
          self.visitor.visit_load_store_pair_post_index(instr);
        } else {
          self.visitor.visit_load_store_pair_offset(instr);
        }
      } else if instr.bit(29) == 0 {
        self.visitor.visit_load_literal(instr);
      } else if instr.bit(21) == 0 {
        match instr.bit_range(11, 10) {
          0 => self.visitor.visit_load_store_unscaled_offset(instr),
          1 => self.visitor.visit_load_store_post_index(instr),
          3 => self.visitor.visit_load_store_pre_index(instr),
          _ => self.visitor.visit_unimplemented(instr),
        }
      } else if instr.bit_range(11, 10) == 2 {
        self.visitor.visit_load_store_register_offset(instr);
      } else {
        self.visitor.visit_unallocated(instr);
      }
    } else if instr.bit(28) == 0 {
      if instr.bit(29) == 0 {
        self.visitor.visit_unallocated(instr);
      } else if instr.bit(23) == 0 {
        self.visitor.visit_load_store_pair_pre_index(instr);
      } else {
        self.visitor.visit_load_store_pair_offset(instr);
      }
    } else if instr.bit(29) == 0 {
      self.visitor.visit_unallocated(instr);
    } else {
      self.visitor.visit_load_store_unsigned_offset(instr);
    }
  }

  pub fn decode_data_processing(&mut self, instr: &Instruction) {
    if instr.bit(24) == 0 {
      if instr.bit(28) == 0 {
        if instr.mask(0x80008000) == 0x00008000 {
          self.visitor.visit_unallocated(instr);
        } else {
          self.visitor.visit_logical_shifted(instr);
        }
      } else {
        match instr.bit_range(23, 21) {
          0 => self.visitor.visit_add_sub_with_carry(instr),
          2 => {
            if instr.bit(11) == 0 {
              self.visitor.visit_conditional_compare_register(instr);
            } else {
              self.visitor.visit_conditional_compare_immediate(instr);
            }
          }
          4 => {
            if instr.mask(0x20000000) != 0 || instr.mask(0x00000800) != 0 || instr.mask(0x00000010) != 0 {
              self.visitor.visit_unallocated(instr);
            } else {
              self.visitor.visit_conditional_select(instr);
            }
          }
          6 => {
            if instr.bit(30) == 1 {
              self.visitor.visit_unallocated(instr);
            } else if instr.bit(11) == 0 {
              self.visitor.visit_data_processing2_source(instr);
            } else {
              self.visitor.visit_data_processing1_source(instr);
            }
          }
          _ => self.visitor.visit_unallocated(instr),
        }
      }
    } else if instr.bit(28) == 0 {
      if instr.bit(21) == 0 {
        self.visitor.visit_add_sub_shifted(instr);
      } else if instr.bit(23) == 0 {
        self.visitor.visit_add_sub_extended(instr);
      } else {
        self.visitor.visit_unallocated(instr);
      }
    } else {
      self.visitor.visit_data_processing3_source(instr);
    }
  }

  pub fn decode_fp(&mut self, instr: &Instruction) {
    if instr.bit(28) == 0 {
      self.decode_neon_vector_data_processing(instr);
    } else if instr.bit_range(31, 30) == 0x3 {
      self.visitor.visit_unallocated(instr);
    } else if instr.bit_range(31, 30) == 0x1 {
      self.decode_neon_scalar_data_processing(instr);
    } else if instr.bit(29) == 0 {
      if instr.bit(24) == 0 {
        if instr.bit(21) == 0 {
          self.visitor.visit_fpfixed_point_convert(instr);
        } else if instr.bit_range(15, 10) == 0 {
          self.visitor.visit_fpinteger_convert(instr);
        } else {
          self.visitor.visit_fpdata_processing1_source(instr);
        }
      } else if instr.bit(21) == 0 {
        self.visitor.visit_fpcompare(instr);
      } else {
        self.visitor.visit_fpimmediate(instr);
      }
    } else if instr.bit(24) == 0 {
      self.visitor.visit_fpdata_processing2_source(instr);
    } else if instr.bit(21) == 0 {
      if instr.bit(11) == 0 {
        self.visitor.visit_fpconditional_compare(instr);
      } else {
        self.visitor.visit_fpconditional_select(instr);
      }
    } else {
      self.visitor.visit_fpdata_processing3_source(instr);
    }
  }

  pub fn decode_neon_load_store(&mut self, instr: &Instruction) {
    if instr.bit(24) == 0 {
      if instr.bit(21) == 0 {
        self.visitor.visit_neonload_store_multi_struct(instr);
      } else {
        self.visitor.visit_neonload_store_single_struct(instr);
      }
    } else if instr.bit(23) == 0 {
      if instr.bit(21) == 0 {
        self.visitor.visit_neonload_store_multi_struct_post_index(instr);
      } else {
        self.visitor.visit_neonload_store_single_struct_post_index(instr);
      }
    } else {
      self.visitor.visit_unallocated(instr);
    }
  }

  pub fn decode_neon_vector_data_processing(&mut self, instr: &Instruction) {
    if instr.bit(23) == 1 {
      self.visitor.visit_neonextract(instr);
    } else if instr.bit(24) == 0 {
      if instr.bit(21) == 0 {
        self.visitor.visit_neon3_same(instr);
      } else if instr.bit(19) == 0 {
        self.visitor.visit_neon2_reg_misc(instr);
      } else {
        self.visitor.visit_neoncopy(instr);
      }
    } else if instr.bit(21) == 0 {
      self.visitor.visit_neon3_different(instr);
    } else if instr.bit(15) == 0 {
      self.visitor.visit_neonshift_immediate(instr);
    } else {
      self.visitor.visit_neonby_indexed_element(instr);
    }
  }

  pub fn decode_neon_scalar_data_processing(&mut self, instr: &Instruction) {
    if instr.bit(23) == 1 {
      self.visitor.visit_unallocated(instr);
    } else if instr.bit(24) == 0 {
      if instr.bit(21) == 0 {
        self.visitor.visit_neonscalar3_same(instr);
      } else if instr.bit(19) == 0 {
        self.visitor.visit_neonscalar2_reg_misc(instr);
      } else {
        self.visitor.visit_neonscalar_copy(instr);
      }
    } else if instr.bit(21) == 0 {
      self.visitor.visit_neonscalar3_diff(instr);
    } else if instr.bit(15) == 0 {
      self.visitor.visit_neonscalar_shift_immediate(instr);
    } else {
      self.visitor.visit_neonscalar_by_indexed_element(instr);
    }
  }
}

/// 原版 Decoder<V>::Decode — 顶层分派函数
pub fn decode(instr: &Instruction) {
  struct NullVisitor;
  impl DecoderVisitor for NullVisitor {}
  let mut v = NullVisitor;
  Decoder::new(&mut v).decode(instr);
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(VISITOR_LIST.len() == ${visitors.length});
  assert!(DECODE_TREE_NODES.len() == 11);
};
`;

const p = writeSrc("v8-codegen-arm64", "decoder.rs", rust);
console.log(`decoder-arm64: 访问者 ${visitors.length} / 解码树节点 ${decodeFns.length} / 管理方法 5 → ${p}`);
