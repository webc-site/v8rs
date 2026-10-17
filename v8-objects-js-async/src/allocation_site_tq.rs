// 由 tool/src/objects/allocation-site.tq.js 从 v8/src/objects/allocation-site.tq 生成; 修订就地改, 复跑覆盖
// 席位表差集: tq 声明面 (.h 常量/位域归既有 allocation-site.h.js)
//! AllocationSite/AllocationMemento tq 差集。
//!
//! AllocationSite 布局与嵌套链 (nested_site) 在 .h 定义 (V8_OBJECT 非 tq 生成);
//! AllocationMemento 为字面量分配后追加的 memento 槽 (指向所属 site)。

/// AllocationSite: tq 侧仅外部声明, 布局归 .h (既有 allocation-site.h.js 产物)。
pub const ALLOCATION_SITE_TQ_NEW_FIELDS: usize = 0;

// ── AllocationMemento 布局 (tq extern class extends Struct) ──
pub const ALLOCATION_MEMENTO_FIELDS: [(&str, &str); 1] = [("allocation_site", "AllocationSite")];

// ── 提取不变式断言 ── (const 上下文无 PartialEq<&str>, 用字节循环 const fn)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}
const _: () = {
  assert!(ALLOCATION_SITE_TQ_NEW_FIELDS == 0);
  assert!(ALLOCATION_MEMENTO_FIELDS.len() == 1);
  assert!(str_eq(ALLOCATION_MEMENTO_FIELDS[0].0, "allocation_site"));
};
