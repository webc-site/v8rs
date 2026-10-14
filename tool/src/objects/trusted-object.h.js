#!/usr/bin/env bun
// tool/src/objects/trusted-object.h.js — trusted.rs 差集骨架生成 (源 1/4)
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/trusted-object.h → v8-objects-arrays/src/trusted.rs (差集骨架)
// 产物: v8-objects-arrays/src/trusted_object_h.rs.skel (trusted.rs 已落地实现, 直出会覆盖修订)
// 断言: TrustedObject/ExposedTrustedObject 类拓扑、方法名全集、kHeaderSize 定义、
//       V8_IT_ABSTRACT 数量、self_indirect_pointer_ 字段形态; 不符 throw
// 推迟登记 (详见 trusted.rs 头部): AllocationWitness、TrustedPointerPublishingScope、
//   Isolate/LocalIsolate/IsolateForSandbox 句柄族、DECL_VERIFIER (待 L3 slot/heap/isolate/visitors)
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/trusted-object.h");

let checks = 0;
const need = (cond, msg) => {
  checks++;
  if (!cond) throw new Error(msg);
};

// —— 类拓扑 ——
need(/\bclass TrustedObject : public HeapObject\b/.test(h), "TrustedObject : public HeapObject 缺失");
need(/\bclass ExposedTrustedObject : public TrustedObject\b/.test(h), "ExposedTrustedObject : public TrustedObject 缺失");
const itAbstractCount = [...h.matchAll(/V8_IT_ABSTRACT;/g)].length;
need(itAbstractCount === 2, `V8_IT_ABSTRACT 应为 2 (两抽象类), 实得 ${itAbstractCount}`);

// —— TrustedObject 受保护字段访问器名全集 (方法名按 rust_review 规范转 snake_case) ——
for (const m of ["read_protected_pointer_field", "write_protected_pointer_field", "is_protected_pointer_field_empty", "clear_protected_pointer_field", "raw_protected_pointer_field", "raw_protected_maybe_object_field", "verify_protected_pointer_field"]) {
  const cppName = m.replace(/_(\w)/g, (_, c) => c.toUpperCase());
  const re = new RegExp(`\\b(${cppName[0].toUpperCase()}${cppName.slice(1)}|${cppName})\\s*[<(]`, "g");
  const hits = [...h.matchAll(re)].length;
  const expect = { read_protected_pointer_field: 2, write_protected_pointer_field: 2, is_protected_pointer_field_empty: 2, clear_protected_pointer_field: 2, raw_protected_pointer_field: 1, raw_protected_maybe_object_field: 1, verify_protected_pointer_field: 1 }[m];
  need(hits === expect, `TrustedObject.${cppName} 声明应为 ${expect} 处, 实得 ${hits}`);
}

// —— ExposedTrustedObject 方法全集 ——
const exposed = {
  InitAndPublish: 2, InitDontPublish: 2, Publish: 2, Unpublish: 1,
  IsPublished: 1, self_indirect_pointer_handle: 1, InitSelfIndirectPointerField: 1,
};
for (const [name, count] of Object.entries(exposed)) {
  const hits = [...h.matchAll(new RegExp(`\\b${name}\\s*\\(`, "g"))].length;
  need(hits >= count, `ExposedTrustedObject.${name} 声明缺失 (需 >=${count}, 实得 ${hits})`);
  checks++;
}

// —— kHeaderSize 两处定义 (back-compat 别名 + sizeof 自身) ——
need(/static constexpr int kHeaderSize = sizeof\(HeapObject\);/.test(h), "TrustedObject::kHeaderSize = sizeof(HeapObject) 缺失");
need(/ExposedTrustedObject::kHeaderSize\s*=\s*sizeof\(ExposedTrustedObject\)/.test(h), "ExposedTrustedObject::kHeaderSize = sizeof(ExposedTrustedObject) 缺失");

// —— sandbox 下 self 间接指针字段形态 ——
need(/std::atomic<IndirectPointerHandle> self_indirect_pointer_;/.test(h), "self_indirect_pointer_ 字段形态漂移");

const body = `${banner("src/objects/trusted-object.h.js", "src/objects/trusted-object.h", "trusted.rs 差集骨架 (trusted-object.h 部分), 合并后就地修订")}
//! 差集骨架: TrustedObject / ExposedTrustedObject 基座
// V8_IT_ABSTRACT x${itAbstractCount}: Rust 以 trait TrustedPointee 约束表达"抽象基类不可实例化指向"

#[repr(transparent)]
pub struct TrustedObject {
  pub map: Tagged<Map>,
}
impl TrustedObject {
  pub const K_HEADER_SIZE: usize = size_of::<HeapObject>(); // kHeaderSize = sizeof(HeapObject)
}
impl Tagged<TrustedObject> {
  // 受保护指针字段 (Read/Write/Is/Clear x{2} 重载 + Raw x2 + Verify, VERIFY_HEAP 推迟)
  pub unsafe fn read_protected_pointer_field<T>(this: &Self, offset: usize) -> Tagged<T>;
  pub unsafe fn read_protected_pointer_field_acquire<T>(this: &Self, offset: usize, acquire: AcquireLoadTag) -> Tagged<T>;
  pub unsafe fn write_protected_pointer_field(this: &Self, offset: usize, value: Tagged<TrustedObject>);
  pub unsafe fn write_protected_pointer_field_release(this: &Self, offset: usize, value: Tagged<TrustedObject>, release: ReleaseStoreTag);
  pub unsafe fn is_protected_pointer_field_empty(this: &Self, offset: usize) -> bool;
  pub unsafe fn is_protected_pointer_field_empty_acquire(this: &Self, offset: usize, acquire: AcquireLoadTag) -> bool;
  pub unsafe fn clear_protected_pointer_field(this: &Self, offset: usize);
  pub unsafe fn clear_protected_pointer_field_release(this: &Self, offset: usize, release: ReleaseStoreTag);
  pub fn raw_protected_pointer_field(this: &Self, byte_offset: usize) -> ProtectedPointerSlot;
  pub fn raw_protected_maybe_object_field(this: &Self, byte_offset: usize) -> ProtectedMaybeObjectSlot;
}

#[repr(C)]
pub struct ExposedTrustedObject {
  pub trusted: TrustedObject,
  pub self_indirect_pointer: AtomicU32, // std::atomic<IndirectPointerHandle>, 初值 kNullIndirectPointerHandle
}
impl ExposedTrustedObject {
  pub const K_HEADER_SIZE: usize = size_of::<ExposedTrustedObject>();
  pub fn self_indirect_pointer_handle(&self) -> IndirectPointerHandle;
  // InitAndPublish/InitDontPublish/Publish/Unpublish/InitSelfIndirectPointerField:
  //   依赖 TPT 表项分配与 TrustedPointerPublishingScope → 推迟登记 (v8-sandbox L3)
  // IsPublished: 经 IsTrustedPointerFieldUnpublished 表达 (本 crate 可落地)
}
`;

const p = writeSrc("v8-objects-arrays", "trusted_object_h.rs.skel", body);
console.log(`trusted-object.h.js: 骨架 ${p} (对数断言 ${checks} 项通过; V8_IT_ABSTRACT=${itAbstractCount}, exposed 方法 ${Object.keys(exposed).length} 族)`);
