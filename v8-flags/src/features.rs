// 由 tool/src/flags/feature-flags.h.js 从 v8/src/flags/feature-flags.h 生成; 修订就地改, 复跑覆盖
// 特性层: 阶段/种类枚举 + PrintFeatureFlagsJSON 输出 (数据在 features_gen.rs)

use strum::Display;

use crate::features_gen::{FEATURES, Feature};

/// 特性阶段 (原版 FOREACH_<STAGE>_FEATURE_FLAG 宏族; 定义序即阶段序)
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq)]
pub enum FeatureStage {
  #[strum(serialize = "EXPERIMENTAL")]
  Experimental,
  #[strum(serialize = "PRE_STAGED")]
  PreStaged,
  #[strum(serialize = "STAGED")]
  Staged,
  #[strum(serialize = "SHIPPED")]
  Shipped,
}

/// 特性种类 (原版三槽宏 JS_FEATURE / WASM_FEATURE / INTERNAL_FEATURE)
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq)]
pub enum FeatureKind {
  Js,
  Wasm,
  Internal,
}

impl Feature {
  /// 阶段枚举视图
  pub fn stage(&self) -> FeatureStage {
    match self.stage {
      "EXPERIMENTAL" => FeatureStage::Experimental,
      "PRE_STAGED" => FeatureStage::PreStaged,
      "STAGED" => FeatureStage::Staged,
      "SHIPPED" => FeatureStage::Shipped,
      _ => FeatureStage::Experimental, // 生成表内阶段恒在枚举内
    }
  }

  /// 种类枚举视图
  pub fn kind(&self) -> FeatureKind {
    match self.kind {
      "js" => FeatureKind::Js,
      "wasm" => FeatureKind::Wasm,
      "internal" => FeatureKind::Internal,
      _ => FeatureKind::Js,
    }
  }
}

/// 特性 JSON (原版 FlagList::PrintFeatureFlagsJSON; 供脚本清理测试文件中的特性 flag)
/// 输出形状与原版一致: js / harmony (harmony_ 前缀拆组) / wasm 三大组,
/// 各组含 in-progress (EXPERIMENTAL+PRE_STAGED) / staged / shipping 数组; internal 不输出
pub fn feature_flags_json() -> String {
  let mut s = String::from("{\n");
  s.push_str("  \"js\": ");
  feature_group(&mut s, FeatureKind::Js, false);
  s.push_str(",\n");
  s.push_str("  \"harmony\": ");
  feature_group(&mut s, FeatureKind::Js, true);
  s.push_str(",\n");
  s.push_str("  \"wasm\": ");
  feature_group(&mut s, FeatureKind::Wasm, false);
  s.push_str("\n}\n");
  s
}

/// 单组输出; harmony 组仅收 harmony_ 前缀名, 非 harmony 组排除之 (原版 strncmp("harmony_",8) 拆分)
fn feature_group(out: &mut String, kind: FeatureKind, harmony: bool) {
  out.push_str("{\n");
  out.push_str("    \"in-progress\": ");
  names_array(
    out,
    kind,
    harmony,
    &[FeatureStage::Experimental, FeatureStage::PreStaged],
  );
  out.push_str(",\n");
  out.push_str("    \"staged\": ");
  names_array(out, kind, harmony, &[FeatureStage::Staged]);
  out.push_str(",\n");
  out.push_str("    \"shipping\": ");
  names_array(out, kind, harmony, &[FeatureStage::Shipped]);
  out.push_str("\n  }");
}

fn names_array(out: &mut String, kind: FeatureKind, harmony: bool, stages: &[FeatureStage]) {
  let mut first = true;
  out.push('[');
  for f in FEATURES.iter().filter(|f| {
    // internal 特性不进 JSON (原版 IGNORE_FEATURE 槽); 调用方只传 Js/Wasm
    f.kind() == kind && stages.contains(&f.stage()) && f.name.starts_with("harmony_") == harmony
  }) {
    if !first {
      out.push(',');
    }
    out.push_str("\n      \"");
    out.push_str(f.name);
    out.push('"');
    first = false;
  }
  if !first {
    out.push_str("\n    ");
  }
  out.push(']');
}

/// 全部特性 (定义序)
pub fn features() -> &'static [Feature] {
  FEATURES
}
