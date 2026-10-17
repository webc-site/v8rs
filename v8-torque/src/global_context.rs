// 由 tool/src/torque/misc.js 从 v8/src/torque/global-context.h 生成; 修订就地改, 复跑覆盖
// 全局上下文 + 目标架构 (上下文单例 → 驱动持有结构体)

use std::collections::{BTreeMap, BTreeSet};

use crate::{
  ast::{Ast, IncludeSelector},
  context::{CurrentScopeGuard, SourcePositionScope, current_source_file},
  cpp_builder::File,
  declarable::{
    Declarable, DeclarableData, DeclarableId, DeclarableKind, NamespaceData, make_declarable,
  },
  source_positions::{LineAndColumn, SourceId, SourcePosition},
  utils::K_BASE_NAMESPACE_NAME,
};

/// 原版 CppInclude 1:1
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CppInclude {
  pub include_path: String,
  pub include_selector: IncludeSelector,
}

impl CppInclude {
  #[must_use]
  pub fn csa_selected(&self) -> bool {
    matches!(
      self.include_selector,
      IncludeSelector::Csa | IncludeSelector::Any
    )
  }

  #[must_use]
  pub fn tsa_selected(&self) -> bool {
    matches!(
      self.include_selector,
      IncludeSelector::Tsa | IncludeSelector::Any
    )
  }
}

/// 原版 GlobalContext::KytheInlineMetadata 1:1
#[derive(Debug, Clone, Default)]
pub struct KytheInlineMetadata {
  pub begin: usize,
  pub end: usize,
  pub vname_sig: String,
  pub vname_path: String,
  pub source_begin: usize,
  pub source_end: usize,
  pub is_call_anchor: bool,
}

impl KytheInlineMetadata {
  /// 原版 Generates
  #[must_use]
  pub fn generates(begin: usize, end: usize, vname_sig: String, vname_path: String) -> Self {
    Self {
      begin,
      end,
      vname_sig,
      vname_path,
      source_begin: 0,
      source_end: 0,
      is_call_anchor: false,
    }
  }

  /// 原版 CallAnchor
  #[must_use]
  pub fn call_anchor(
    begin: usize,
    end: usize,
    source_begin: usize,
    source_end: usize,
    vname_path: String,
  ) -> Self {
    Self {
      begin,
      end,
      vname_sig: String::new(),
      vname_path,
      source_begin,
      source_end,
      is_call_anchor: true,
    }
  }
}

/// 原版 PerFileStreams — 单文件生成物集合
/// 登记偏差: 原版 csa_headerfile/csa_ccfile/class_definition_ccfile 三个 stringstream
/// 与 cpp::File 双层封装, Rust 版 File 直接持有 String 缓冲, 三字段并入 File.buf。
#[derive(Debug, Default)]
pub struct PerFileStreams {
  pub file: Option<SourceId>,
  pub csa_header: File,
  pub csa_header_rules: Vec<KytheInlineMetadata>,
  pub csa_cc: File,
  pub csa_cc_rules: Vec<KytheInlineMetadata>,
  pub class_definition_cc: File,
  pub required_builtin_includes: BTreeSet<SourceId>,
}

/// 原版 GlobalContext (ContextualClass 单例 → 编译驱动持有的结构体)
/// 声明ables 存储与 Ast 一样改为 arena; 指针字段 → DeclarableId。
#[derive(Debug, Default)]
pub struct GlobalContext {
  pub collect_language_server_data: bool,
  pub collect_kythe_data: bool,
  pub force_assert_statements: bool,
  pub annotate_ir: bool,
  pub torque_dwarf: bool,
  pub has_kythe_inline_metadata: bool,
  pub kythe_default_corpus: String,
  default_namespace: Option<DeclarableId>,
  pub ast: Ast,
  declarables: Vec<Declarable>,
  cpp_includes: BTreeSet<CppInclude>,
  generated_per_file: BTreeMap<SourceId, PerFileStreams>,
  fresh_ids: BTreeMap<String, usize>,
  macros_for_cc_debug_output: Vec<(DeclarableId, SourceId)>,
  macros_for_cc_debug_output_set: BTreeSet<(DeclarableId, SourceId)>,
  instance_types_initialized: bool,
}

impl GlobalContext {
  /// 原版 GlobalContext(Ast) 构造 (global-context.cc:11-26)
  #[must_use]
  pub fn new(ast: Ast) -> Self {
    // v8: src/torque/global-context.cc GlobalContext::GlobalContext
    let mut cx = Self {
      ast,
      ..Self::default()
    };
    // CurrentScope::Scope(nullptr): 根作用域为空
    let _scope_guard = CurrentScopeGuard::new(None);
    // CurrentSourcePosition::Scope({CurrentSourceFile::Get(), invalid, invalid})
    let position = SourcePosition {
      source: current_source_file(),
      start: LineAndColumn::invalid(),
      end: LineAndColumn::invalid(),
    };
    let _position_guard = SourcePositionScope::new(position);
    // default_namespace_ = RegisterDeclarable(Namespace(kBaseNamespaceName))
    let base = make_declarable(
      DeclarableKind::Namespace,
      DeclarableData::Namespace(NamespaceData {
        name: K_BASE_NAMESPACE_NAME.to_owned(),
      }),
      true,
    );
    cx.default_namespace = Some(cx.register_declarable(base));
    cx
  }

  /// 原版 RegisterDeclarable: 登记 arena, 返回稳定下标
  pub fn register_declarable(&mut self, d: Declarable) -> DeclarableId {
    self.declarables.push(d);
    (self.declarables.len() - 1) as DeclarableId
  }

  #[must_use]
  pub fn all_declarables(&self) -> &[Declarable] {
    &self.declarables
  }

  #[must_use]
  pub fn declarable(&self, id: DeclarableId) -> &Declarable {
    &self.declarables[id as usize]
  }

  #[must_use]
  pub fn declarable_mut(&mut self, id: DeclarableId) -> &mut Declarable {
    &mut self.declarables[id as usize]
  }

  /// 原版 GetDefaultNamespace
  #[must_use]
  pub const fn default_namespace(&self) -> Option<DeclarableId> {
    self.default_namespace
  }

  /// 原版 AddCppInclude
  pub fn add_cpp_include(&mut self, include_path: String, include_selector: IncludeSelector) {
    self.cpp_includes.insert(CppInclude {
      include_path,
      include_selector,
    });
  }

  #[must_use]
  pub fn cpp_includes(&self) -> &BTreeSet<CppInclude> {
    &self.cpp_includes
  }

  /// 原版 GeneratedPerFile
  pub fn generated_per_file_mut(&mut self, file: SourceId) -> &mut PerFileStreams {
    let s = self.generated_per_file.entry(file).or_default();
    s.file = Some(file);
    s
  }

  #[must_use]
  pub fn generated_per_file(&self) -> &BTreeMap<SourceId, PerFileStreams> {
    &self.generated_per_file
  }

  /// 原版 MakeUniqueName
  #[must_use]
  pub fn make_unique_name(&mut self, base: &str) -> String {
    let n = self.fresh_ids.entry(base.to_owned()).or_default();
    *n += 1;
    format!("{base}_{}", *n - 1)
  }

  /// 原版 SetInstanceTypesInitialized
  pub fn set_instance_types_initialized(&mut self) {
    debug_assert!(!self.instance_types_initialized);
    self.instance_types_initialized = true;
  }

  #[must_use]
  pub const fn is_instance_types_initialized(&self) -> bool {
    self.instance_types_initialized
  }

  /// 原版 EnsureInCCDebugOutputList
  pub fn ensure_in_cc_debug_output_list(&mut self, macro_: DeclarableId, source: SourceId) {
    let item = (macro_, source);
    if self.macros_for_cc_debug_output_set.insert(item) {
      self.macros_for_cc_debug_output.push(item);
    }
  }

  #[must_use]
  pub fn all_macros_for_cc_debug_output(&self) -> &[(DeclarableId, SourceId)] {
    &self.macros_for_cc_debug_output
  }
}

/// 原版 TargetArchitecture (ContextualClass → 结构体, 编译选项 force_32bit 决定)
#[derive(Debug, Clone, Copy)]
pub struct TargetArchitecture {
  tagged_size: usize,
  raw_ptr_size: usize,
  smi_tag_and_shift_size: i32,
  external_ptr_size: usize,
  cppheap_ptr_size: usize,
  trusted_ptr_size: usize,
}

impl TargetArchitecture {
  /// 原版 TargetArchitecture(bool force_32bit) (global-context.cc:28)
  /// 值域对齐目标配置 (x64/arm64, ptr-compr + sandbox, todo 五):
  ///   tagged = kTaggedSize = 4 (force_32bit 时 sizeof(int32_t) = 4)
  ///   raw_ptr = kSystemPointerSize = 8 (force_32bit 时 4)
  ///   smi_tag_and_shift = kSmiTagSize + kSmiShiftSize = 1 + 0 = 1
  ///     (ptr-compr 下 SmiTagging<4>::kSmiShiftSize = 0, 两支同值)
  ///   external/cppheap/trusted = sandbox 下 slot 均为 4 字节 handle
  #[must_use]
  pub fn new(force_32bit: bool) -> Self {
    let raw_ptr_size = if force_32bit { 4 } else { 8 };
    Self {
      tagged_size: 4,
      raw_ptr_size,
      smi_tag_and_shift_size: 1,
      external_ptr_size: 4,
      cppheap_ptr_size: 4,
      trusted_ptr_size: 4,
    }
  }

  /// 显式尺寸构造 (thread_local 占位与测试注入用; 零值 = 未设置态)
  #[must_use]
  pub const fn with_sizes(
    tagged_size: usize,
    raw_ptr_size: usize,
    smi_tag_and_shift_size: i32,
    external_ptr_size: usize,
    cppheap_ptr_size: usize,
    trusted_ptr_size: usize,
  ) -> Self {
    Self {
      tagged_size,
      raw_ptr_size,
      smi_tag_and_shift_size,
      external_ptr_size,
      cppheap_ptr_size,
      trusted_ptr_size,
    }
  }

  #[must_use]
  pub const fn tagged_size(&self) -> usize {
    self.tagged_size
  }

  #[must_use]
  pub const fn raw_ptr_size(&self) -> usize {
    self.raw_ptr_size
  }

  #[must_use]
  pub const fn external_pointer_size(&self) -> usize {
    self.external_ptr_size
  }

  #[must_use]
  pub const fn cppheap_pointer_size(&self) -> usize {
    self.cppheap_ptr_size
  }

  #[must_use]
  pub const fn trusted_pointer_size(&self) -> usize {
    self.trusted_ptr_size
  }

  /// 原版 ProtectedPointerSize == TaggedSize
  #[must_use]
  pub const fn protected_pointer_size(&self) -> usize {
    self.tagged_size
  }

  /// 原版 MaxHeapAlignment == TaggedSize
  #[must_use]
  pub const fn max_heap_alignment(&self) -> usize {
    self.tagged_size
  }

  #[must_use]
  pub const fn are_pointers_compressed(&self) -> bool {
    self.tagged_size < self.raw_ptr_size
  }

  #[must_use]
  pub const fn smi_tag_and_shift_size(&self) -> i32 {
    self.smi_tag_and_shift_size
  }
}

// ── 原版 src/torque/global-context.cc ──
// 签名清单
// L11: GlobalContext::GlobalContext(Ast ast) : collect_language_server_data_(false), collect_kythe_data_(false), force_assert_statements_(false), annotate_ir_(false), torque_dwarf_(false), has_kythe_inline_metadata_(false), kythe_default_corpus_(""), ast_(std::move(ast))
// L28: TargetArchitecture::TargetArchitecture(bool force_32bit) : tagged_size_(force_32bit ? sizeof(int32_t) : kTaggedSize), raw_ptr_size_(force_32bit ? sizeof(int32_t) : kSystemPointerSize), smi_tag_and_shift_size_( kSmiTagSize + (force_32bit ? SmiTagging<kApiInt32Size>::kSmiShiftSize : kSmiShiftSize)), external_ptr_size_(force_32bit ? sizeof(int32_t) : kExternalPointerSlotSize), cppheap_ptr_size_(force_32bit ? sizeof(int32_t) : kCppHeapPointerSlotSize), trusted_ptr_size_(force_32bit ? sizeof(int32_t) : kTrustedPointerSize)
