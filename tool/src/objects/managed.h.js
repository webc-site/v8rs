#!/usr/bin/env bun
// tool/src/objects/managed.h.js — 覆盖 v8/src/objects/managed.{h,inl.h,cc,tq}
// 托管资源骨架: CppGCManagedWrapper / CppGCManagedBase / CppGCManaged<T> /
//   ManagedPtrDestructor / Managed<T>(弃用) / TrustedManaged<T>
// → v8-objects-js-async/src/managed.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/managed.h");
const inlSrc = readV8("src/objects/managed-inl.h");
const ccSrc = readV8("src/objects/managed.cc");
const tqSrc = readV8("src/objects/managed.tq");

// 1. CppGCManagedWrapper 字段 (cppgc::GarbageCollected 侧, 非堆对象; 只看 private 段)
const wrapperPriv = hSrc.split("class CppGCManagedWrapper final")[1]?.split(" private:")[1]?.split("\n};")[0];
if (!wrapperPriv) throw new Error("未找到 CppGCManagedWrapper private 段");
const wrapperNames = [...wrapperPriv.matchAll(/\b[a-z][A-Za-z0-9_]*\b/g)].map((m) => m[0]).filter((s) => s.endsWith("_"));
if (wrapperNames.length !== 7) throw new Error(`CppGCManagedWrapper 字段数不符: 期望 7, 得 ${wrapperNames.length} → ${wrapperNames}`);

// 2. CppGCManagedBase: tq 布局 (cpp_gc_wrapper: CppHeapPointer)
const tqClass = tqSrc.match(/extern class (\w+) extends (\w+) \{([\s\S]*?)\}/);
if (!tqClass || tqClass[1] !== "CppGCManagedBase" || tqClass[2] !== "HeapObject") throw new Error("managed.tq CppGCManagedBase 布局失配");
const tqFields = [...tqClass[3].matchAll(/^  (\w+): (\w+);$/gm)].map((m) => [m[1], m[2]]);
if (tqFields.length !== 1 || tqFields[0][0] !== "cpp_gc_wrapper") throw new Error(`managed.tq 字段失配: ${JSON.stringify(tqFields)}`);

// 3. ManagedPtrDestructor 字段 (struct 全公开, 取声明区: struct 头到构造函数前)
const dtorDecl = hSrc.split("struct ManagedPtrDestructor")[1]?.split("\n\n  ManagedPtrDestructor(")[0];
if (!dtorDecl) throw new Error("未找到 ManagedPtrDestructor 声明区");
const dtorNames = [...dtorDecl.matchAll(/\b[a-z][A-Za-z0-9_]*\b/g)].map((m) => m[0]).filter((s) => s.endsWith("_"));
if (dtorNames.length !== 8) throw new Error(`ManagedPtrDestructor 字段数不符: 期望 8, 得 ${dtorNames.length} → ${dtorNames}`);

// 4. 基类 (沙箱开关二态: ExternalPointerTable::ManagedResource / Malloced)
const dtorBase = hSrc.match(/struct ManagedPtrDestructor\n#ifdef V8_ENABLE_SANDBOX\n    : public (\S+) \{\n#else\n    : public (\S+) \{\n/);
if (!dtorBase || dtorBase[1] !== "ExternalPointerTable::ManagedResource" || dtorBase[2] !== "Malloced") throw new Error("ManagedPtrDestructor 基类形态失配");

// 5. Ptr 内部类操作面 (CppGCManaged 与 Managed 两处同构; 去重保序)
const ptrBody = hSrc.match(/V8_OBJECT_INNER_CLASS class Ptr final \{([\s\S]*?)\} V8_OBJECT_INNER_CLASS_END;/)?.[1];
if (!ptrBody) throw new Error("未找到 CppGCManaged::Ptr");
const ptrOps = [...new Set([...ptrBody.matchAll(/^    V8_INLINE (?:explicit )?(?:[^\n;(]*?)(operator\b[^(]*?|[\w:]+)\s*\(/gm)].map((m) => m[1].trim()))];
const ptrExpect = ["Ptr", "operator=", "operator->", "operator*", "raw", "get", "as_shared_ptr", "Reset", "operator bool", "operator=="];
if (ptrOps.length !== ptrExpect.length || ptrOps.some((n, i) => n !== ptrExpect[i])) {
  throw new Error(`Ptr 公开操作面不符: 期望 ${JSON.stringify(ptrExpect)}, 得 ${JSON.stringify(ptrOps)}`);
}

// 6. 类型注册宏 (TypeIdForManaged / TagForManaged 各一对)
for (const macro of ["ASSIGN_MANAGED_TYPE_ID_FOR_MANAGED", "ASSIGN_EXTERNAL_POINTER_TAG_FOR_MANAGED"]) {
  if (!hSrc.includes(`#define ${macro}`)) throw new Error(`缺少宏 ${macro}`);
}

// 7. inl/cc 侧锚
if (!/ReadCppHeapPointerField\([\s\S]*kCppGCManagedTag\)/.test(inlSrc)) throw new Error("inl 缺 kCppGCManagedTag 锚");
const ccFns = [...ccSrc.matchAll(/^ {0,2}(?:V8_EXPORT_PRIVATE )?(?:void|size_t) (\w+)(?:::)?(\w*)\(/gm)].map((m) => m[2] || m[1]);
if (ccFns.length !== 4 || !ccFns.includes("ManagedObjectFinalizerSecondPass") || !ccFns.includes("ManagedObjectFinalizer")) throw new Error(`managed.cc 顶层函数数不符: 期望 4, 得 ${ccFns.length} → ${ccFns}`);
if (!ccSrc.includes("ManagedObjectFinalizerSecondPass")) throw new Error("缺 SecondPass 终结器");

// 8. 类层级表
if (!hSrc.includes("The use of {Managed<T>} is deprecated")) throw new Error("缺 Managed 弃用注记");
const hierarchy = [
  ["CppGCManagedWrapper", "cppgc::GarbageCollected", "cppgc 侧包装 (非 V8 堆对象)"],
  ["CppGCManagedBase", "HeapObject", "V8_OBJECT; tq @cppObjectLayoutDefinition"],
  ["CppGCManaged", "CppGCManagedBase", "模板; V8_IT_REUSE_PARENT"],
  ["ManagedPtrDestructor", "ExternalPointerTable::ManagedResource | Malloced", "隔离析构双链表节点"],
  ["Managed", "Foreign", "已弃用, 迁 CppGCManaged"],
  ["TrustedManaged", "TrustedForeign", "可信空间等价物, 免 tag"],
];

// rustfmt 排版规则 (rustfmt.toml 无覆盖 → 默认): 数组元素行宽 ≤60 (array_width) 单行,
// 超宽拆行; 一般语句 ≤100 (max_width); 宽度按字符数计 (CJK 亦记 1)
const w = (s) => [...s].length;
const tuple3 = (a, b, c) => {
  const one = `  (${a}, ${b}, ${c}),`;
  return w(one) <= 60 ? one : `  (\n    ${a},\n    ${b},\n    ${c},\n  ),`;
};
const todoBody = (msg) => {
  const one = `    todo!(${msg})`;
  return w(one) <= 100 ? one : `    todo!(\n      ${msg}\n    )`;
};

console.log(`managed 族: wrapper 字段 ${wrapperNames.length}, dtor 字段 ${dtorNames.length}, Ptr ops ${ptrOps.length}, cc 函数 ${ccFns.length}`);

let rust = banner(
  "src/objects/managed.h.js",
  "src/objects/managed.{h,inl.h,cc,tq}",
  "托管资源骨架 (CppGCManaged / ManagedPtrDestructor / Managed / TrustedManaged)",
);

rust += `
/// CppGCManagedWrapper 字段面 (cppgc 分配, 不落 V8 堆)
pub const CPP_GC_MANAGED_WRAPPER_FIELDS: [&str; ${wrapperNames.length}] = [
${wrapperNames.map((n) => `  "${n}",`).join("\n")}
];

/// ManagedPtrDestructor 字段面 (isolate 级析构双链表节点)
pub const MANAGED_PTR_DESTRUCTOR_FIELDS: [&str; ${dtorNames.length}] = [
${dtorNames.map((n) => `  "${n}",`).join("\n")}
];

/// 类层级表 (类, 基类, 注记)
pub const HIERARCHY: [(&str, &str, &str); ${hierarchy.length}] = [
${hierarchy.map(([a, b, c]) => tuple3(`"${a}"`, `"${b}"`, `"${c}"`)).join("\n")}
];
/// CppGCManaged::Ptr 公开操作 (Managed::Ptr 同构)
pub const PTR_OPS: [&str; ${ptrOps.length}] = [
${ptrOps.map((n) => `  "${n}",`).join("\n")}
];

/// cpp 堆指针 tag 锚 (原版 kCppGCManagedTag)
pub const CPP_GC_MANAGED_TAG: &str = "kCppGCManagedTag";

/// 类型注册宏对 (TypeId / ExternalPointerTag 特化入口)
pub const ASSIGN_MACROS: [&str; 2] = [
  "ASSIGN_MANAGED_TYPE_ID_FOR_MANAGED",
  "ASSIGN_EXTERNAL_POINTER_TAG_FOR_MANAGED",
];

/// tq 布局 (CppGCManagedBase extends HeapObject)
pub const TQ_LAYOUT: [(&str, &str); ${tqFields.length}] = [${tqFields.map(([n, t]) => `("${n}", "${t}")`).join(", ")}];

/// CppGCManagedBase 布局标记 (tq @cppObjectLayoutDefinition)
pub struct CppGCManagedBase;

impl CppGCManagedBase {
  /// 原版 kHeaderSize = kSize = sizeof(CppGCManagedBase) (cpp_gc_wrapper_ 单字段)
  ///
  /// v8: managed.h — 布局求解依赖 CppHeapPointerMember 宽度 (沙箱/压缩二态)
  pub fn header_size() -> usize {
    todo!("v8: src/objects/managed.h CppGCManagedBase::kHeaderSize = sizeof(CppGCManagedBase)")
  }

  /// v8: managed-inl.h CppGCManagedBase::GetWrapper — ReadCppHeapPointerField(kCppGCManagedTag)
  pub fn get_wrapper() -> &'static CppGCManagedWrapper {
    todo!("v8: src/objects/managed-inl.h CppGCManagedBase::GetWrapper")
  }

  /// v8: managed-inl.h CppGCManagedBase::estimated_size — GetWrapper()->estimated_size()
  pub fn estimated_size() -> usize {
    todo!("v8: src/objects/managed-inl.h CppGCManagedBase::estimated_size")
  }
}

/// cppgc 侧包装 (持有 shared_ptr 与外部内存计量)
pub struct CppGCManagedWrapper;

impl CppGCManagedWrapper {
  /// v8: managed.cc CppGCManagedWrapper::UpdateEstimatedSize — 差额增减 ExternalMemoryAccounter
  pub fn update_estimated_size(new_estimated_size: usize) {
${todoBody(`"v8: src/objects/managed.cc CppGCManagedWrapper::UpdateEstimatedSize({new_estimated_size})"`)}
  }
}

/// isolate 级析构双链表节点 (Managed<T> 路径)
pub struct ManagedPtrDestructor;

impl ManagedPtrDestructor {
  /// v8: managed.cc ManagedPtrDestructor::UpdateEstimatedSize — 差额增减后回写
  pub fn update_estimated_size(new_estimated_size: usize) {
${todoBody(`"v8: src/objects/managed.cc ManagedPtrDestructor::UpdateEstimatedSize({new_estimated_size})"`)}
  }
}

/// GC 终结器第一遍: 销毁 global handle 并挂第二遍回调
///
/// v8: managed.cc ManagedObjectFinalizer — GlobalHandles::Destroy + SetSecondPassCallback
pub fn managed_object_finalizer() {
  todo!("v8: src/objects/managed.cc ManagedObjectFinalizer")
}

/// GC 终结器第二遍: 注销节点 → 析构 shared_ptr → 扣减外部内存 → 删节点
///
/// v8: managed.cc (匿名空间) ManagedObjectFinalizerSecondPass
pub fn managed_object_finalizer_second_pass() {
  todo!("v8: src/objects/managed.cc ManagedObjectFinalizerSecondPass")
}

/// 语义实现面登记 (Developer 接线):
/// - detail::Destructor<T>(void*) — inl: delete shared_ptr
/// - Managed<T>::From / TrustedManaged<T>::From — New{,Trusted}Foreign + RegisterManagedPtrDestructor
/// - CppGCManaged<T>::Create — MakeGarbageCollected + NewCppGCManagedBase + 写屏障
/// - CppGCManaged<T>::ptr/raw/get/GetSharedPtr/SetManagedObject — CHECK_EQ type_id 锚
/// - Ptr::as_shared_ptr/Reset — 移动语义面
pub const SIGNATURES: [&str; 10] = [
  "destructor_t",
  "managed_from",
  "trusted_managed_from",
  "cpp_gc_managed_create",
  "cpp_gc_managed_ptr",
  "cpp_gc_managed_raw",
  "cpp_gc_managed_get",
  "cpp_gc_managed_get_shared_ptr",
  "set_managed_object",
  "ptr_as_shared_ptr",
];

// —— 提取不变式断言 —— (const 上下文无 str 比较, 用字节循环 const fn)
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
  assert!(str_eq(CPP_GC_MANAGED_WRAPPER_FIELDS[0], "type_id_"));
  assert!(CPP_GC_MANAGED_WRAPPER_FIELDS.len() == ${wrapperNames.length});
  assert!(str_eq(MANAGED_PTR_DESTRUCTOR_FIELDS[0], "estimated_size_"));
  assert!(MANAGED_PTR_DESTRUCTOR_FIELDS.len() == ${dtorNames.length});
  assert!(HIERARCHY.len() == ${hierarchy.length});
  assert!(PTR_OPS.len() == ${ptrOps.length});
  assert!(str_eq(TQ_LAYOUT[0].0, "cpp_gc_wrapper") && str_eq(TQ_LAYOUT[0].1, "CppHeapPointer"));
  assert!(SIGNATURES.len() == 10);
};
`;

const p = writeSrc("v8-objects-js-async", "managed.rs", rust);
console.log(`written ${p}`);
