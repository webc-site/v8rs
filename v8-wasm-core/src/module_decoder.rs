// 由 tool/src/wasm/module-decoder.h.js 从 v8/src/wasm/module-decoder.{h,cc} 生成; 修订就地改, 复跑覆盖
// 模块解码入口: DecodingMethod 枚举 + 特殊段名映射表 + SectionName 段码表
// 深水推迟 (见 readme/zh.md 台账): DecodeWasmModule 三重载 / ValidateFunctions / DecodeCustomSections
// 依赖 v8-wasm-objects 的 WasmModule 构建面, 骨架函数不入挂载面

use crate::{
  table_eq::str_eq,
  wasm_constants::{
    K_BRANCH_HINTS_STRING, K_BUILD_ID_STRING, K_COMPILATION_PRIORITY_STRING, K_DEBUG_INFO_STRING,
    K_EXTERNAL_DEBUG_INFO_STRING, K_INST_TRACE_STRING, K_NAME_STRING, K_SOURCE_MAPPING_URL_STRING,
    SectionCode,
  },
};

/// 解码方式 (对应 enum class DecodingMethod)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DecodingMethod {
  /// kSync
  Sync = 0,
  /// kAsync
  Async = 1,
  /// kSyncStream
  SyncStream = 2,
  /// kAsyncStream
  AsyncStream = 3,
  /// kDeserialize
  Deserialize = 4,
}

/// 特殊自定义段名 → 段码映射 (对应 kSpecialSections, 10 条)
/// 段码记号保留原名 (SectionCode 定义在 wasm-constants 侧)
#[rustfmt::skip]
pub static SPECIAL_SECTIONS: &[(&str, &str)] = &[
  ("Name", "kNameSectionCode"),
  ("SourceMappingURL", "kSourceMappingURLSectionCode"),
  ("InstTrace", "kInstTraceSectionCode"),
  ("BranchHints", "kBranchHintsSectionCode"),
  ("CompilationPriority", "kCompilationPrioritySectionCode"),
  ("InstructionFrequencies", "kInstFrequenciesSectionCode"),
  ("CallTargets", "kCallTargetsSectionCode"),
  ("DebugInfo", "kDebugInfoSectionCode"),
  ("ExternalDebugInfo", "kExternalDebugInfoSectionCode"),
  ("BuildId", "kBuildIdSectionCode"),
];

/// 识别特殊自定义段 (对应 IdentifyUnknownSectionInternal 的表驱动部分)
#[must_use]
pub const fn identify_special_section(name: &str) -> Option<&'static str> {
  let mut i = 0;
  while i < SPECIAL_SECTIONS.len() {
    let (n, code) = SPECIAL_SECTIONS[i];
    if str_eq(n, name) {
      return Some(code);
    }
    i += 1;
  }
  None
}

/// 段码 → 段名 (对应 SectionName, module-decoder.cc:25)
#[must_use]
pub const fn section_name(code: SectionCode) -> &'static str {
  match code {
    SectionCode::UnknownSection => "Unknown",
    SectionCode::TypeSection => "Type",
    SectionCode::ImportSection => "Import",
    SectionCode::FunctionSection => "Function",
    SectionCode::TableSection => "Table",
    SectionCode::MemorySection => "Memory",
    SectionCode::GlobalSection => "Global",
    SectionCode::ExportSection => "Export",
    SectionCode::StartSection => "Start",
    SectionCode::ElementSection => "Element",
    SectionCode::CodeSection => "Code",
    SectionCode::DataSection => "Data",
    SectionCode::DataCountSection => "DataCount",
    SectionCode::TagSection => "Tag",
    SectionCode::StringRefSection => "StringRef",
    SectionCode::NameSection => K_NAME_STRING,
    SectionCode::SourceMappingURLSection => K_SOURCE_MAPPING_URL_STRING,
    SectionCode::DebugInfoSection => K_DEBUG_INFO_STRING,
    SectionCode::ExternalDebugInfoSection => K_EXTERNAL_DEBUG_INFO_STRING,
    SectionCode::BuildIdSection => K_BUILD_ID_STRING,
    SectionCode::InstTraceSection => K_INST_TRACE_STRING,
    SectionCode::BranchHintsSection => K_BRANCH_HINTS_STRING,
    SectionCode::CompilationPrioritySection => K_COMPILATION_PRIORITY_STRING,
    // 原版 default: "<unknown>" (inst_frequencies/call_targets/descriptors 未列出)
    SectionCode::InstFrequenciesSection
    | SectionCode::CallTargetsSection
    | SectionCode::DescriptorsSection => "<unknown>",
  }
}

// —— 提取不变式断言 ——
const _: () = {
  use crate::table_eq::str_eq;
  assert!(DecodingMethod::Sync as u8 == 0);
  assert!(DecodingMethod::AsyncStream as u8 == 3);
  assert!(DecodingMethod::Deserialize as u8 == 4);
  assert!(SPECIAL_SECTIONS.len() == 10);
  assert!(str_eq(SPECIAL_SECTIONS[0].0, "Name"));
  assert!(str_eq(SPECIAL_SECTIONS[0].1, "kNameSectionCode"));
  assert!(match identify_special_section("Name") {
    Some(s) => str_eq(s, "kNameSectionCode"),
    None => false,
  });
  assert!(str_eq(section_name(SectionCode::TypeSection), "Type"));
  assert!(str_eq(section_name(SectionCode::NameSection), "name"));
};
