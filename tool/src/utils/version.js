#!/usr/bin/env bun
// v8/src/utils/version.{h,cc} + v8/include/v8-version*.h → v8-utils/src/version.rs
// 确定性转换: 版本四元组/候选位从 v8-version.h 宏提取; GetString/GetSONAME/Hash
// 为格式化直译; 语义修订就地改 src/, 复跑覆盖
import { readV8, writeSrc, banner } from "../../lib.js";

const vh = readV8("include/v8-version.h");
const num = (name) => {
  const m = vh.match(new RegExp(`#define ${name} (\\d+)`));
  if (!m) throw new Error(`v8-version.h: ${name} 未找到`);
  return m[1];
};
const cand = vh.match(/#define V8_IS_CANDIDATE_VERSION (\d+)/);
if (!cand) throw new Error("v8-version.h: V8_IS_CANDIDATE_VERSION 未找到");

// version.cc 中 SONAME 宏默认空串 (构建系统可覆盖)
const vc = readV8("src/utils/version.cc");
const sonameDef = vc.match(/#define SONAME "(.*)"/);
if (!sonameDef) throw new Error("version.cc: SONAME 未找到");

const body = `${banner("src/utils/version.js", "src/utils/version.h;src/utils/version.cc;include/v8-version.h;include/v8-version-string.h")}
//! V8 版本号与版本串, 移植自 v8/src/utils/version。
//!
//! 上游为静态成员类; Rust 版 \`Version\` 为值类型, \`Version::get()\` 返回构建期版本,
//! 测试可通过构造实例改写四元组 (对应上游 test-version.cc 的 SetVersion 友元)。

use super::utils::Hasher;

/// 主版本号 (V8_MAJOR_VERSION)
pub const V8_MAJOR_VERSION: i32 = ${num("V8_MAJOR_VERSION")};
/// 次版本号 (V8_MINOR_VERSION)
pub const V8_MINOR_VERSION: i32 = ${num("V8_MINOR_VERSION")};
/// 构建号 (V8_BUILD_NUMBER)
pub const V8_BUILD_NUMBER: i32 = ${num("V8_BUILD_NUMBER")};
/// 补丁级 (V8_PATCH_LEVEL)
pub const V8_PATCH_LEVEL: i32 = ${num("V8_PATCH_LEVEL")};
/// 嵌入方后缀 (V8_EMBEDDER_STRING, 默认空)
pub const V8_EMBEDDER_STRING: &str = "";
/// 是否候选版本 (V8_IS_CANDIDATE_VERSION)
pub const V8_IS_CANDIDATE_VERSION: bool = ${cand[1]} != 0;
/// 构建 SONAME 覆盖 (version.cc 的 SONAME 宏, 默认空 → 生成通用 SONAME)
pub const SONAME: &str = "${sonameDef[1]}";

/// 版本描述值对象 (上游 Version 类的实例化形态)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
  pub major: i32,
  pub minor: i32,
  pub build: i32,
  pub patch: i32,
  pub embedder: &'static str,
  pub candidate: bool,
  pub soname: &'static str,
}

impl Default for Version {
  fn default() -> Self {
    Version::get()
  }
}

impl Version {
  /// 构建期版本
  pub fn get() -> Self {
    Version {
      major: V8_MAJOR_VERSION,
      minor: V8_MINOR_VERSION,
      build: V8_BUILD_NUMBER,
      patch: V8_PATCH_LEVEL,
      embedder: V8_EMBEDDER_STRING,
      candidate: V8_IS_CANDIDATE_VERSION,
      soname: SONAME,
    }
  }

  /// Version::Hash: Hasher 依次混入四元组 (int→hash32) 与 embedder 逐字节
  pub fn hash(&self) -> u32 {
    let mut h = Hasher::new();
    h.add_i32(self.major).add_i32(self.minor).add_i32(self.build).add_i32(self.patch);
    h.add_bytes(self.embedder.as_bytes());
    h.hash() as u32
  }

  /// Version::GetString: "major.minor.build[.patch]embedder[ (candidate)]"
  pub fn get_string(&self) -> String {
    let candidate = if self.candidate { " (candidate)" } else { "" };
    if self.patch > 0 {
      format!("{}.{}.{}.{}{}{}", self.major, self.minor, self.build, self.patch, self.embedder, candidate)
    } else {
      format!("{}.{}.{}{}{}", self.major, self.minor, self.build, self.embedder, candidate)
    }
  }

  /// Version::GetSONAME: 空 SONAME → "libv8-x.y.z[.p]embedder[-candidate].so"
  pub fn get_soname(&self) -> String {
    if self.soname.is_empty() {
      let candidate = if self.candidate { "-candidate" } else { "" };
      if self.patch > 0 {
        format!("libv8-{}.{}.{}.{}{}{}.so", self.major, self.minor, self.build, self.patch, self.embedder, candidate)
      } else {
        format!("libv8-{}.{}.{}{}{}.so", self.major, self.minor, self.build, self.embedder, candidate)
      }
    } else {
      self.soname.to_owned()
    }
  }
}
`;

writeSrc("v8-utils", "version.rs", body);
console.log(
  `version.rs: ${num("V8_MAJOR_VERSION")}.${num("V8_MINOR_VERSION")}.${num("V8_BUILD_NUMBER")}.${num("V8_PATCH_LEVEL")} candidate=${cand[1]} ✓`,
);
