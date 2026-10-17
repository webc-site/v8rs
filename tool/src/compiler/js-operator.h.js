#!/usr/bin/env bun
// tool/src/compiler/js-operator.h.js
// 从 v8/src/compiler/js-operator.h + js-operator.cc
// 提取 JS 算子常量与签名，生成 v8-compiler-ops/src/js_operator.rs
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-operator.h");
const cc = readV8("src/compiler/js-operator.cc");

// 提取 CACHED_OP_LIST
const cachedMatch = cc.match(/#define CACHED_OP_LIST\(V\)\s*\\([\s\S]*?)\n\n/);
if (!cachedMatch) throw new Error("未找到 CACHED_OP_LIST");
const cachedOps = [];
for (const line of cachedMatch[1].split("\n")) {
  const m = line.match(/V\((\w+),\s*([^,]+),\s*(\d+),\s*(\d+)\)/);
  if (m) {
    const propStr = m[2].trim();
    let prop = "Properties::NONE";
    if (propStr !== "Operator::kNoProperties") {
      const parts = propStr.split("|").map((p) => {
        const t = p.trim().replace(/^Operator::k/, "");
        return `Properties::${t.replace(/^No/, "NO_").toUpperCase()}`;
      });
      prop = parts.join(" | ");
    }
    cachedOps.push({
      name: m[1],
      prop,
      value_in: Number(m[3]),
      value_out: Number(m[4]),
    });
  }
}

const toSnakeCase = (str) =>
  str.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const rustSrc = `${banner("src/compiler/js-operator.h.js", "src/compiler/js-operator.h + js-operator.cc")}
//! JSOperatorBuilder 与 JS 算子族
//!
//! 对应原版 js-operator.h / js-operator.cc。
//! 包含 JSAdd, JSSubtract, JSCall, JSCreate, JSLoadProperty 等。

use std::sync::LazyLock;
use v8_compiler_graph::{
  FeedbackSource, IrOpcode, Operator, Operator1, OperatorCounts, Properties,
};
use v8_zone::Zone;

use crate::Error;

/// 调用算子参数 (CallParameters)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CallParameters {
  pub arity: usize,
  pub frequency: f32,
  pub feedback: FeedbackSource,
}

impl CallParameters {
  pub const fn new(arity: usize, frequency: f32, feedback: FeedbackSource) -> Self {
    Self {
      arity,
      frequency,
      feedback,
    }
  }
}

impl Eq for CallParameters {}

impl core::hash::Hash for CallParameters {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.arity.hash(state);
    self.frequency.to_bits().hash(state);
    self.feedback.hash(state);
  }
}

/// 构造调用算子参数 (ConstructParameters, js-operator.h:170-198)
/// JSConstruct / JSConstructWithArrayLike / JSConstructWithSpread 共用
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstructParameters {
  pub arity: usize,
  pub frequency: f32,
  pub feedback: FeedbackSource,
}

impl ConstructParameters {
  /// 原版 kExtraConstructInputCount (js-operator.h:175, = JSConstructNode::kExtraInputCount:
  /// target + new.target + receiver 三个隐式输入)
  pub const EXTRA_CONSTRUCT_INPUT_COUNT: usize = 3;

  /// 原版构造器 (js-operator.h:176-181):
  /// DCHECK_GE(arity, kExtraConstructInputCount) 与 DCHECK(is_int32(arity)) 以断言承接
  pub fn new(arity: usize, frequency: f32, feedback: FeedbackSource) -> Self {
    debug_assert!(arity >= Self::EXTRA_CONSTRUCT_INPUT_COUNT);
    debug_assert!(arity <= i32::MAX as usize);
    Self {
      arity,
      frequency,
      feedback,
    }
  }

  /// 原版 arity_without_implicit_args (js-operator.h:192)
  pub const fn arity_without_implicit_args(&self) -> i32 {
    (self.arity - Self::EXTRA_CONSTRUCT_INPUT_COUNT) as i32
  }
}

impl Eq for ConstructParameters {}

impl core::hash::Hash for ConstructParameters {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.arity.hash(state);
    self.frequency.to_bits().hash(state);
    self.feedback.hash(state);
  }
}

/// 全局共享 JS 缓存算子 (镜像原版 JSOperatorGlobalCache)
pub struct JSOperatorGlobalCache {
${cachedOps.map((op) => `  pub js_${toSnakeCase(op.name)}: Operator,`).join("\n")}
}

#[inline]
fn unwrap_op<T>(res: core::result::Result<T, v8_compiler_graph::Error>) -> T {
  match res {
    Ok(v) => v,
    Err(_) => unreachable!(),
  }
}

impl JSOperatorGlobalCache {
  fn new() -> Self {
    Self {
${cachedOps
  .map(
    (op) => `      js_${toSnakeCase(op.name)}: unwrap_op(Operator::new(
        IrOpcode::JS${op.name} as u16,
        ${op.prop},
        "JS${op.name}",
        OperatorCounts::new(
          ${op.value_in},
          (${op.prop}).zero_if_pure(),
          (${op.prop}).zero_if_eliminatable(),
          ${op.value_out},
          (${op.prop}).zero_if_pure(),
          (${op.prop}).zero_if_no_throw(),
        ),
      )),`,
  )
  .join("\n")}
    }
  }
}

static JS_CACHE: LazyLock<JSOperatorGlobalCache> =
  LazyLock::new(JSOperatorGlobalCache::new);

/// JSOperatorBuilder
pub struct JSOperatorBuilder<'z, 'a> {
  zone: &'z Zone<'a>,
}

impl<'z, 'a> JSOperatorBuilder<'z, 'a> {
  /// 创建构建器
  pub fn new(zone: &'z Zone<'a>) -> Self {
    Self { zone }
  }

  /// 底层 Zone 引用
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  // —— 静态缓存 JS 算子 ——

${cachedOps
  .map(
    (op) => `  /// JS${op.name}
  #[inline]
  pub fn js_${toSnakeCase(op.name)}(&self) -> &'static Operator {
    &JS_CACHE.js_${toSnakeCase(op.name)}
  }`,
  )
  .join("\n\n")}

  /// JSAdd 算子
  pub fn js_add(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::JSAdd as u16,
      Properties::NONE,
      "JSAdd",
      OperatorCounts::new(3, 1, 1, 1, 1, 2),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSSubtract 算子
  pub fn js_subtract(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::JSSubtract as u16,
      Properties::NONE,
      "JSSubtract",
      OperatorCounts::new(3, 1, 1, 1, 1, 2),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSMultiply 算子
  pub fn js_multiply(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::JSMultiply as u16,
      Properties::NONE,
      "JSMultiply",
      OperatorCounts::new(3, 1, 1, 1, 1, 2),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSDivide 算子
  pub fn js_divide(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::JSDivide as u16,
      Properties::NONE,
      "JSDivide",
      OperatorCounts::new(3, 1, 1, 1, 1, 2),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSCall 算子
  pub fn js_call(
    &self,
    arity: usize,
    frequency: f32,
    feedback: FeedbackSource,
  ) -> Result<&'z Operator, Error> {
    let params = CallParameters::new(arity, frequency, feedback);
    let op = Operator1::new(
      IrOpcode::JSCall as u16,
      Properties::NONE,
      "JSCall",
      OperatorCounts::new(arity as u32, 1, 1, 1, 1, 2),
      params,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSConstruct 算子 (js-operator.cc:1134): value_in = arity, control_out = 2
  pub fn js_construct(
    &self,
    arity: usize,
    frequency: f32,
    feedback: FeedbackSource,
  ) -> Result<&'z Operator, Error> {
    let params = ConstructParameters::new(arity, frequency, feedback);
    let op = Operator1::new(
      IrOpcode::JSConstruct as u16,
      Properties::NONE,
      "JSConstruct",
      OperatorCounts::new(arity as u32, 1, 1, 1, 1, 2),
      params,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSLoadProperty 算子
  pub fn js_load_property(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::JSLoadProperty as u16,
      Properties::NONE,
      "JSLoadProperty",
      OperatorCounts::new(3, 1, 1, 1, 1, 2),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSStoreProperty 算子
  pub fn js_store_property(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::JSSetKeyedProperty as u16,
      Properties::NONE,
      "JSSetKeyedProperty",
      OperatorCounts::new(4, 1, 1, 0, 1, 2),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }
}
`;

writeSrc("v8-compiler-ops", "js_operator.rs", rustSrc);
console.log(`written js_operator.rs (${cachedOps.length} cached ops)`);
