#!/usr/bin/env bun
// tool/src/strings/owning-external-string-resource.h.js
// 机械提取 v8/src/strings/owning-external-string-resource.h:
// 独占外部字符串资源模板的存储策略与类型别名

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/strings/owning-external-string-resource.h");

// 对数守卫
const must = [
  "class OwningExternalStringResourceImpl",
  "CreateUninitializedStorage",
  "SealIfSupported",
  "V8_ENABLE_MEMORY_CORRUPTION_API",
  "using OwningExternalOneByteStringResource =",
  "using OwningExternalStringResource =",
];
for (const k of must) {
  if (!hSrc.includes(k)) throw new Error(`owning-external-string-resource.h 未找到 ${k}`);
}
// static_assert 对数: 字宽一致 ×2
const asserts = (hSrc.match(/static_assert\(sizeof\(CharT\) == sizeof\(\w+\)\);/g) || []).length;
if (asserts !== 2) throw new Error(`字宽断言应 2, 实得 ${asserts}`);

let rust = banner("src/strings/owning-external-string-resource.h.js", "src/strings/owning-external-string-resource.h", "独占外部字符串资源骨架");

rust += `
/// 独占式外部字符串资源 (原版 OwningExternalStringResourceImpl 模板)。
/// 存储策略分支 (原版 #ifdef V8_ENABLE_MEMORY_CORRUPTION_API):
///   关闭 → std::unique_ptr<CharT[]> (默认, x64/arm64 常规构建)
///   开启 → ExternalStringsCage 池分配 + Seal 封印
pub mod owning_resource_skel {
  /// 资源骨架: data() + length() 两访问器 (原版 override data/length)
  pub struct OwningExternalStringResourceSkel<CharT> {
    /// 只读长度 (原版 const size_t length_)
    pub length: usize,
    /// 独占存储 (原版 const Storage storage_)
    pub storage: Box<[CharT]>,
  }

  impl<CharT> OwningExternalStringResourceSkel<CharT> {
    /// 未初始化存储分配 (原版 CreateUninitializedStorage, cage 关闭路径)
    fn create_uninitialized_storage(_len: usize) -> Box<[CharT]> {
      unimplemented!("骨架")
    }

    /// 构造后封印 (原版 SealIfSupported; cage 开启路径)
    fn seal_if_supported(&mut self) {
      unimplemented!("骨架")
    }

    /// 数据指针 (原版 data())
    pub fn data(&self) -> &[CharT] {
      &self.storage
    }
  }

  /// 一字节别名 (原版 OwningExternalOneByteStringResource: char/char/uint8_t)
  pub type OwningExternalOneByteStringResource = OwningExternalStringResourceSkel<u8>;
  /// 双字节别名 (原版 OwningExternalStringResource: uc16/char16_t/uc16)
  pub type OwningExternalStringResource = OwningExternalStringResourceSkel<u16>;

  /// 字宽断言对数守卫 (原版 static_assert ×2)
  pub const WIDTH_ASSERTS: usize = 2;
}
`;

const p = writeSrc("v8-strings", "owning_external_string_resource_generated.rs", rust);
console.log(`written ${p}; 断言=${asserts}`);
