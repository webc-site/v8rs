// v8-objects-maps/src/dependent_code_inl.rs
// 对标 v8/src/objects/dependent-code-inl.h 与 dependent-code.cc
//! DependentCode 内联模板入口: 对象 → 依赖代码失效/标记。
//! 原版为 template <typename ObjectT> 双重载。
//! 实现依赖代码项的迭代、标记与失效通知。

use std::sync::LazyLock;

use gxhash::HashMap;
use parking_lot::Mutex;

/// 依赖组位标志 (对齐原版 enum DependencyGroup)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DependencyGroup {
  TransitionGroup = 1 << 0,
  PrototypeCheckGroup = 1 << 1,
  PropertyCellChangedGroup = 1 << 2,
  FieldTypeGroup = 1 << 3,
  FieldConstGroup = 1 << 4,
  FieldRepresentationGroup = 1 << 5,
  InitialMapChangedGroup = 1 << 6,
  AllocationSiteTenuringChangedGroup = 1 << 7,
  AllocationSiteTransitionChangedGroup = 1 << 8,
  ContextCellChangedGroup = 1 << 9,
  EmptyContextExtensionGroup = 1 << 10,
}

impl DependencyGroup {
  /// 获取依赖组对应位掩码
  #[inline]
  #[must_use]
  pub const fn mask(self) -> u32 {
    self as u32
  }
}

/// 依赖代码项 (对齐原版 DependentCode 中的弱代码条目与依赖组)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DependentCodeEntry {
  /// 依赖代码对象的句柄/指针地址
  pub code: usize,
  /// 依赖组位掩码 (DependencyGroups)
  pub groups: u32,
  /// 是否已被标记为待去优化
  pub marked_for_deopt: bool,
  /// 导致去优化的触发组原因
  pub deopt_reason: Option<u32>,
}

/// 全局依赖代码注册表与去优化协调器
#[derive(Default)]
pub struct DependentCodeRegistry {
  /// 对象地址 -> 依赖代码条目列表
  entries: HashMap<usize, Vec<DependentCodeEntry>>,
  /// 累计去优化通知计数
  deopt_count: usize,
}

impl DependentCodeRegistry {
  /// 注册依赖关系 (对齐 DependentCode::InstallDependency / InsertWeakCode)
  pub fn install_dependency(&mut self, object: usize, code: usize, groups: u32) {
    let list = self.entries.entry(object).or_default();
    // 若已有相同 code，合并 groups
    if let Some(entry) = list.iter_mut().find(|e| e.code == code) {
      entry.groups |= groups;
      return;
    }
    list.push(DependentCodeEntry {
      code,
      groups,
      marked_for_deopt: false,
      deopt_reason: None,
    });
  }

  /// 获取对象的依赖代码条目切片（零堆分配）
  #[must_use]
  pub fn get_entries(&self, object: usize) -> &[DependentCodeEntry] {
    self.entries.get(&object).map_or(&[], Vec::as_slice)
  }

  /// 迭代并压缩条目 (对齐 DependentCode::IterateAndCompact: 前向单次迭代并原地压缩)
  pub fn iterate_and_compact<F>(&mut self, object: usize, mut filter: F)
  where
    F: FnMut(&mut DependentCodeEntry) -> bool,
  {
    if let Some(list) = self.entries.get_mut(&object) {
      list.retain_mut(|entry| !filter(entry));
      if list.is_empty() {
        self.entries.remove(&object);
      }
    }
  }

  /// 标记依赖代码去优化 (对齐 DependentCode::MarkCodeForDeoptimization)
  pub fn mark_code_for_deoptimization(&mut self, object: usize, deopt_groups: u32) -> bool {
    let Some(list) = self.entries.get_mut(&object) else {
      return false;
    };
    let mut marked_something = false;
    for entry in list.iter_mut() {
      if (entry.groups & deopt_groups) != 0 && !entry.marked_for_deopt {
        let matched_groups = entry.groups & deopt_groups;
        let first_group = 1 << matched_groups.trailing_zeros();
        entry.marked_for_deopt = true;
        entry.deopt_reason = Some(first_group);
        marked_something = true;
      }
    }
    marked_something
  }

  /// 失效并去优化依赖组 (对齐 DependentCode::DeoptimizeDependencyGroups)
  pub fn deoptimize_dependency_groups(&mut self, object: usize, groups: u32) -> bool {
    let marked = self.mark_code_for_deoptimization(object, groups);
    if marked {
      self.deopt_count += 1;
      // 模拟 Deoptimizer::DeoptimizeMarkedCode: 清除或收割已标记条目并回收空槽
      if let Some(list) = self.entries.get_mut(&object) {
        list.retain(|e| !e.marked_for_deopt);
        if list.is_empty() {
          self.entries.remove(&object);
        }
      }
    }
    marked
  }

  /// 累计去优化次数
  #[must_use]
  pub fn deopt_count(&self) -> usize {
    self.deopt_count
  }

  /// 清除对象的所有依赖代码
  pub fn clear_object(&mut self, object: usize) {
    self.entries.remove(&object);
  }
}

static REGISTRY: LazyLock<Mutex<DependentCodeRegistry>> =
  LazyLock::new(|| Mutex::new(DependentCodeRegistry::default()));

/// 为对象安装依赖代码
pub fn install_dependency(object: usize, code: usize, groups: u32) {
  REGISTRY.lock().install_dependency(object, code, groups);
}

/// 查询对象的依赖代码条目
#[must_use]
pub fn get_dependent_code(object: usize) -> Vec<DependentCodeEntry> {
  REGISTRY.lock().get_entries(object).to_vec()
}

/// v8: dependent-code-inl.h:22 — template <typename ObjectT>
///   void DependentCode::DeoptimizeDependencyGroups(Isolate*, ObjectT, DependencyGroups)
///
/// 直传重载: 转发到 Tagged 重载 (static_assert kTaggedCanConvertToRawObjects)。
pub fn deoptimize_dependency_groups(isolate: &str, object: usize, groups: u32) {
  deoptimize_dependency_groups_tagged(isolate, object, groups);
}

/// v8: dependent-code-inl.h:24 — template <typename ObjectT>
///   void DependentCode::DeoptimizeDependencyGroups(Isolate*, Tagged<ObjectT>, DependencyGroups)
///
/// 共享对象与只读对象设计上永不失效代码 (原版 DCHECK 双否定)。
pub fn deoptimize_dependency_groups_tagged(_isolate: &str, object: usize, groups: u32) {
  if object == 0 {
    return;
  }
  REGISTRY.lock().deoptimize_dependency_groups(object, groups);
}

/// v8: dependent-code-inl.h:41 — template <typename ObjectT>
///   bool DependentCode::MarkCodeForDeoptimization(Isolate*, Tagged<ObjectT>, DependencyGroups)
///
/// 返回是否命中任一依赖组。
pub fn mark_code_for_deoptimization(_isolate: &str, object: usize, groups: u32) -> bool {
  if object == 0 {
    return false;
  }
  REGISTRY.lock().mark_code_for_deoptimization(object, groups)
}
