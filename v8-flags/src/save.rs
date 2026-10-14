// 由 tool/src/flags/save-flags.h.js 从 v8/src/flags/{save-flags.h, save-flags.cc} 生成; 修订就地改, 复跑覆盖
// 快照层: SaveFlags 的 save/restore 对 (RAII → 显式调用)

use crate::{
  defs,
  value::{FlagValue, Flags},
};

/// flag 值快照 (原版 SaveFlags; 仅非只读主 flag)
///
/// 原版为 RAII (构造存/析构回); Rust 侧由调用方掌控恢复时机:
/// `let saved = flags.save(); ... ; flags.restore(&saved);`
#[derive(Debug)]
pub struct SavedFlags {
  /// 快照值 (索引 = defs::FLAGS 主 flag 序)
  values: Box<[Option<FlagValue>]>,
}

impl Flags {
  /// 存当前值 (原版 SaveFlags 构造; 只读 flag 不存 — 其值恒为默认)
  pub fn save(&self) -> SavedFlags {
    SavedFlags {
      values: defs::FLAGS
        .iter()
        .enumerate()
        .map(|(i, d)| {
          if d.readonly {
            None
          } else {
            Some(self.value(i as u32).clone())
          }
        })
        .collect(),
    }
  }

  /// 恢复快照值 (原版 SaveFlags 析构; 直写值不走门禁, set_by/implied_by 保持现状)
  pub fn restore(&mut self, saved: &SavedFlags) {
    for (i, v) in saved.values.iter().enumerate() {
      // 变化才写 (原版为 TSAN 友好; 此处语义同)
      if let Some(v) = v
        && self.value(i as u32) != v
      {
        self.write_value(i as u32, v.clone());
      }
    }
  }
}
