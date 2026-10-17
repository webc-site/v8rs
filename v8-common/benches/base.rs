//! v8-common 核心评测操作 trait 与对比实现
//!
//! 评测面 (crate 真实公开 API, L0 工具层热点):
//! - tag_classify — Smi / 堆对象 / 弱堆对象 位标签判定 (ptr_compr)
//! - compress_roundtrip — 指针压缩→解压往返 (V8HeapCompressionScheme)
//! - align_ops — 对齐向上取整 + padding 算术 (globals)
//! - hash_probe — object_hash + 容器建表/探针 (横切: std vs gxhash)
//!
//! 实现方: `V8Common` 走 crate API; `Std` 为 std 等价写法对照;
//! `Gxhash` (feature bench-gxhash) 只替换 hash_probe 容器, 隔离容器变量。

use std::collections::HashMap;

use v8_common::{
  K_CODE_ALIGNMENT_MASK, K_DOUBLE_ALIGNMENT_MASK, K_OBJECT_ALIGNMENT_MASK,
  K_POINTER_ALIGNMENT_MASK, V8HeapCompressionScheme, code_pointer_align, code_pointer_padding,
  double_pointer_align, has_heap_object_tag, has_smi_tag, has_weak_heap_object_tag, object_hash,
  object_pointer_align, object_pointer_padding, pointer_size_align, pointer_size_padding,
};

/// 评测 cage base — 4GB 对齐 (原版 InitBase CHECK 前提)
pub const CAGE_BASE: usize = 0x2000_0000_0000;

/// 核心评测操作 trait — 评测 id 基于 NAME 生成
pub trait BenchOp {
  const NAME: &'static str;

  /// 位标签判定: 每条输入统计 smi/heap/weak 命中
  fn tag_classify(&self, input: &[usize]) -> usize;

  /// 指针压缩→解压往返校验和
  fn compress_roundtrip(&self, input: &[usize]) -> usize;

  /// 对齐取整 + padding 求和
  fn align_ops(&self, input: &[usize]) -> usize;

  /// object_hash 建表 + 探针查询
  fn hash_probe(&self, keys: &[usize], probes: &[usize]) -> usize;
}

// ── v8rs 实现 ────────────────────────────────────────────────────────────

/// v8-common 自身实现
pub struct V8Common;

impl BenchOp for V8Common {
  const NAME: &'static str = "v8-common";

  #[inline]
  fn tag_classify(&self, input: &[usize]) -> usize {
    let mut acc = 0usize;
    for &v in input {
      acc += usize::from(has_smi_tag(v));
      acc += usize::from(has_heap_object_tag(v)) << 1;
      acc += usize::from(has_weak_heap_object_tag(v)) << 2;
    }
    acc
  }

  #[inline]
  fn compress_roundtrip(&self, input: &[usize]) -> usize {
    input
      .iter()
      .map(|&v| {
        V8HeapCompressionScheme::decompress_tagged(V8HeapCompressionScheme::compress_object(v))
      })
      .sum()
  }

  #[inline]
  fn align_ops(&self, input: &[usize]) -> usize {
    let mut acc = 0usize;
    for &v in input {
      acc += object_pointer_align(v) + object_pointer_padding(v);
      acc += pointer_size_align(v) + pointer_size_padding(v);
      acc += code_pointer_align(v) + code_pointer_padding(v);
      acc += double_pointer_align(v);
    }
    acc
  }

  #[inline]
  fn hash_probe(&self, keys: &[usize], probes: &[usize]) -> usize {
    let mut map: HashMap<u32, usize> = HashMap::with_capacity(keys.len());
    for (i, &k) in keys.iter().enumerate() {
      map.insert(object_hash(k), i);
    }
    probes
      .iter()
      .filter_map(|&p| map.get(&object_hash(p)).copied())
      .sum()
  }
}

// ── std 等价写法对照 ─────────────────────────────────────────────────────

/// std 等价实现: 位标签裸位运算 / 截断+或 base / next_multiple_of /
/// 手写右移散列 + std HashMap
pub struct Std;

impl BenchOp for Std {
  const NAME: &'static str = "std";

  #[inline]
  fn tag_classify(&self, input: &[usize]) -> usize {
    let mut acc = 0usize;
    for &v in input {
      acc += usize::from(v & 1 == 0);
      acc += usize::from(v & 3 == 1) << 1;
      acc += usize::from(v & 3 == 3) << 2;
    }
    acc
  }

  #[inline]
  fn compress_roundtrip(&self, input: &[usize]) -> usize {
    // CompressObject = 截断低 32 位; DecompressTagged = base | raw。
    // 对照面直接用常量 base, 不经进程级原子 (std 等价最乐观形态)
    input.iter().map(|&v| CAGE_BASE | (v as u32 as usize)).sum()
  }

  #[inline]
  fn align_ops(&self, input: &[usize]) -> usize {
    // std 等价写法: next_multiple_of 泛型路径 (2 的幂校验 + 通用分支)
    let (obj, ptr, dbl, code) = (
      K_OBJECT_ALIGNMENT_MASK + 1,
      K_POINTER_ALIGNMENT_MASK + 1,
      K_DOUBLE_ALIGNMENT_MASK + 1,
      K_CODE_ALIGNMENT_MASK + 1,
    );
    let mut acc = 0usize;
    for &v in input {
      acc += v.next_multiple_of(obj) + (v.next_multiple_of(obj) - v);
      acc += v.next_multiple_of(ptr) + (v.next_multiple_of(ptr) - v);
      acc += v.next_multiple_of(code) + (v.next_multiple_of(code) - v);
      acc += v.next_multiple_of(dbl);
    }
    acc
  }

  /// 与 object_hash 同语义的手写移位散列 (K_TAGGED_SIZE_LOG2 = 2)
  #[inline]
  fn hash_probe(&self, keys: &[usize], probes: &[usize]) -> usize {
    let hash = |v: &usize| (*v >> 2) as u32;
    let mut map: HashMap<u32, usize> = HashMap::with_capacity(keys.len());
    for (i, k) in keys.iter().enumerate() {
      map.insert(hash(k), i);
    }
    probes
      .iter()
      .filter_map(|p| map.get(&hash(p)).copied())
      .sum()
  }
}

// ── gxhash 对比实现 (feature bench-gxhash) ───────────────────────────────

/// gxhash 对照: 其余操作与 v8-common 实现一致, 仅把 hash_probe 容器
/// 换成 gxhash::HashMap (todo 10.2 横切容器评测)
#[cfg(feature = "bench-gxhash")]
pub struct Gxhash;

#[cfg(feature = "bench-gxhash")]
impl BenchOp for Gxhash {
  const NAME: &'static str = "gxhash";

  #[inline]
  fn tag_classify(&self, input: &[usize]) -> usize {
    V8Common.tag_classify(input)
  }

  #[inline]
  fn compress_roundtrip(&self, input: &[usize]) -> usize {
    V8Common.compress_roundtrip(input)
  }

  #[inline]
  fn align_ops(&self, input: &[usize]) -> usize {
    V8Common.align_ops(input)
  }

  #[inline]
  fn hash_probe(&self, keys: &[usize], probes: &[usize]) -> usize {
    let mut map: gxhash::HashMap<u32, usize> = gxhash::HashMap::default();
    map.reserve(keys.len());
    for (i, &k) in keys.iter().enumerate() {
      map.insert(object_hash(k), i);
    }
    probes
      .iter()
      .filter_map(|&p| map.get(&object_hash(p)).copied())
      .sum()
  }
}
