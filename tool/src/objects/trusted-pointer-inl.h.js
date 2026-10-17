#!/usr/bin/env bun
// tool/src/objects/trusted-pointer-inl.h.js — trusted.rs 差集骨架生成 (源 4/4)
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/trusted-pointer-inl.h → v8-objects-arrays/src/trusted.rs (实现语义锚点)
// 产物: v8-objects-arrays/src/trusted_pointer_inl_h.rs.skel (-inl 骨架; 最终以就地修订 src 为准)
// 断言: 每个方法体 sandbox 分支守卫计数; 空 handle → Smi::zero(); acquire/release 内存序全集;
//       非 sandbox 分支委托 TaggedField/TaggedMember; TPT 表查找调用链
import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/trusted-pointer-inl.h");

let checks = 0;
const need = (cond, msg) => {
  checks++;
  if (!cond) throw new Error(msg);
};
const count = (re) => [...inl.matchAll(re)].length;

// —— sandbox 分支守卫: 13 处 #ifdef (每个方法体一处 + storage_address 守卫) ——
need(count(/^\#ifdef V8_ENABLE_SANDBOX$/gm) === 13, `sandbox 守卫应 13, 实得 ${count(/^\#ifdef V8_ENABLE_SANDBOX$/gm)}`);

// —— 字段地址与 handle 语义 ——
need(count(/host->ptr\(\) \+ offset - kHeapObjectTag/g) === 5, "field_address = ptr+offset-kHeapObjectTag 应 5 处 (read/write/empty/unpublished/clear)");
need(count(/kNullIndirectPointerHandle/g) === 7, "空 handle 引用应 7 处");
need(count(/Smi::zero/g) === 2, "Smi::zero 应 2 处 (maybe-empty 返回 + 非 sandbox Clear 写入)");
need(count(/return Smi::zero\(\);/g) === 1, "Acquire_Load_maybe_empty 空 handle 返回 Smi::zero()");
need(count(/IsSmi\(/g) === 1, "非 sandbox IsTrustedPointerFieldEmpty 走 IsSmi()");

// —— 内存序全集 (acquire 读 4 / release 写 2; 注释称 load/store 恒 acquire/release) ——
need(count(/std::memory_order::acquire/g) === 2, "std::memory_order::acquire 应 2");
need(count(/std::memory_order_acquire/g) === 2, "std::memory_order_acquire 应 2");
need(count(/std::memory_order_release/g) === 2, "std::memory_order_release 应 2");
need(count(/Acquire_Load/g) === 11, "Acquire_Load (定义/委托/AsAtomic32/TaggedField) 应 11");

// —— TPT 间接读链 (sandbox) ——
need(count(/ReadIndirectPointerField<tag_range>/g) === 1, "maybe-empty 字段读应委托 ReadIndirectPointerField");
need(count(/WriteIndirectPointerField<tag_range>/g) === 1, "字段写应委托 WriteIndirectPointerField<kReleaseStore>");
need(count(/ReadIndirectPointerHandle<kTagRange>/g) === 2, "成员读应委托 ReadIndirectPointerHandle");
need(count(/GetTrustedPointerTableFor/g) === 2, "is_unpublished/字段 unpublished 两处取 TPT");
need(count(/table\.IsUnpublished\(handle\)/g) === 2, "IsUnpublished 表调用应 2");
need(/handle_\.store\(\s*Cast<ExposedTrustedObject>\(value\)->self_indirect_pointer_handle\(\),/.test(inl), "Release_Store 应存 value 的 self handle");
need(/WriteBarrier::ForIndirectPointer\(host, this, value, mode\)/.test(inl), "sandbox Release_Store 应走 ForIndirectPointer 屏障");

// —— TrustedCast 收口 ——
need(count(/TrustedCast<ReturnType>/g) === 1, "ReadTrustedPointerField 出口 TrustedCast<ReturnType>");
need(count(/TrustedCast<Union<Smi, TrustedTypeFor<tag_range>>>/g) === 1, "maybe-empty 出口 TrustedCast<Union<Smi,...>>");
need(count(/TrustedCast<T>/g) === 1, "成员 Acquire_Load 出口 TrustedCast<T>");

// —— 非 sandbox 分支委托 TaggedMember 基类 ——
need(count(/Base::Acquire_Load/g) === 3, "非 sandbox 读委托 Base(TaggedMember)::Acquire_Load x3");
need(count(/Base::Release_Store_no_write_barrier/g) === 2, "非 sandbox 无屏障写/clear 委托 Base x2");
need(count(/TaggedField<Smi>::Release_Store/g) === 1, "非 sandbox Clear 应写 Smi::zero 经 TaggedField<Smi>");

// —— ctor 委托链 (AllocationWitness, 推迟登记但语义须留档) ——
need(count(/TrustedPointerMember\(/g) === 6, "ctor 定义+委托应 6 处");
need(/TryCast\(maybe_value, &value\)/.test(inl), "maybe_value ctor 应 TryCast 分流 (失败支 DCHECK IsUndefined + clear)");
need(/IsUndefined\(maybe_value\)/.test(inl), "maybe_value ctor 失败支语义缺失");

const skel = `${banner("src/objects/trusted-pointer-inl.h.js", "src/objects/trusted-pointer-inl.h", "trusted.rs 差集骨架 (inl 语义), 合并后就地修订; -inl 骨架仅存档")}
//! 差集骨架: TrustedPointerField/Member 实现语义 (sandbox 开为默认配置)

// - 字段访问: field_address = host.address() + offset (Tagged::address 已去 kHeapObjectTag)
// - handle 槽为 u32 原子; 空判定 handle == kNullIndirectPointerHandle; 空 → Smi::zero()
// - load 恒 acquire / store 恒 release (上游注释: 底层间接指针本即 acquire/release)
// - sandbox 成员读: TrustedCast<T>(table.read(handle)); 写: 存 value.self handle + ForIndirectPointer
// - 非 sandbox 分支: 委托 TaggedMember/Base 与 TaggedField (本 crate 配置不启用, 注释留档)
// - is_unpublished: 空 handle → false, 否则 table.IsUnpublished(handle)
// - clear(sandbox): store kNullIndirectPointerHandle release; 非 sandbox: Base 空 tagged release 写
`;

const p = writeSrc("v8-objects-arrays", "trusted_pointer_inl_h.rs.skel", skel);
console.log(`trusted-pointer-inl.h.js: 骨架 ${p} (对数断言 ${checks} 项通过; sandbox 守卫=13, 内存序 acquire ${count(/std::memory_order::acquire/g) + count(/std::memory_order_acquire/g)}/release 2)`);
