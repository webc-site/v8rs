#!/usr/bin/env bun
// tool/src/objects/trusted-object-inl.h.js — trusted.rs 差集骨架生成 (源 2/4)
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/trusted-object-inl.h → v8-objects-arrays/src/trusted.rs (实现语义锚点)
// 产物: v8-objects-arrays/src/trusted_object_inl_h.rs.skel (-inl 骨架; 最终以就地修订 src 为准)
// 断言: 受保护字段全部走 TaggedField<..., TrustedSpaceCompressionScheme>; 空/清除语义 Smi::zero();
//       Publish/Unpublish/IsPublished 的 tag 计算与 TPT 调用链; 内存序标注; 预处理守卫计数
import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/trusted-object-inl.h");

let checks = 0;
const need = (cond, msg) => {
  checks++;
  if (!cond) throw new Error(msg);
};
const count = (re) => [...inl.matchAll(re)].length;

// —— 受保护字段访问全部经 TrustedSpaceCompressionScheme (8 处: R2/W2/Empty2/Clear2) ——
const schemeHits = count(/TaggedField<[^;]*TrustedSpaceCompressionScheme>::/g);
need(schemeHits === 8, `TrustedSpaceCompressionScheme TaggedField 调用应为 8, 实得 ${schemeHits}`);
need(count(/== Smi::zero\(\);/g) === 2, "IsProtectedPointerFieldEmpty 两处应与 Smi::zero() 比较");
need(count(/Smi::zero\(\)/g) === 4, "Smi::zero() 应 4 处 (Empty 比较 2 + Clear 写入 2)");
need(count(/RawProtected(?:MaybeObject|Pointer)Field/g) === 2, "Raw 槽访问器定义应 2 处");
need(/ProtectedPointerSlot\(field_address\(byte_offset\)\)/.test(inl), "RawProtectedPointerField 应取 field_address");
need(/TaggedField<T, 0, TrustedSpaceCompressionScheme>::load\(this, offset\)/.test(inl), "ReadProtectedPointerField 主体漂移");

// —— 配置守卫计数 (默认 sandbox 开; 本 checkout 恒定) ——
const sandboxGuards = count(/^\#ifdef V8_ENABLE_SANDBOX$/gm);
const sandboxIfs = count(/^\#if V8_ENABLE_SANDBOX$/gm);
need(sandboxGuards === 8 && sandboxIfs === 1, `V8_ENABLE_SANDBOX 守卫应为 8 处 #ifdef + 1 处 #if, 实得 ${sandboxGuards}/${sandboxIfs}`);
const verifyGuards = count(/^\#ifdef VERIFY_HEAP$/gm);
need(verifyGuards === 3, `VERIFY_HEAP 守卫应 3 (VerifyField/InitAndPublish(Isolate)/Publish(Isolate)), 实得 ${verifyGuards}`);
need(/UNREACHABLE\(\);/.test(inl), "self_indirect_pointer_handle 非 sandbox 分支应 UNREACHABLE");

// —— ExposedTrustedObject 发布链 ——
need(count(/IndirectPointerTagFromInstanceType\(instance_type, SharedFlag\{false\}\)/g) === 3, "Publish/Unpublish/IsPublished 三处 tag 计算漂移");
need(count(/IndirectPointerTagFromInstanceType\(instance_type, shared\)/g) === 1, "InitSelfIndirectPointerField 用 SharedFlag(InAnySharedSpace) 路径缺失");
need(/table\.Publish\(handle, tag\);/.test(inl), "Publish 应调 TrustedPointerTable::Publish(handle, tag)");
need(/table\.Unpublish\(handle\);/.test(inl), "Unpublish 应调 TrustedPointerTable::Unpublish(handle)");
need(/IsTrustedPointerFieldUnpublished\(/.test(inl), "IsPublished 应委托 IsTrustedPointerFieldUnpublished");
need(/offsetof\(ExposedTrustedObject, self_indirect_pointer_\)/.test(inl), "IsPublished 用 offsetof(self_indirect_pointer_) 漂移");
need(/self_indirect_pointer_\.load\(std::memory_order::acquire\)/.test(inl), "Publish/Unpublish 应 acquire 读 self handle");
need(/self_indirect_pointer_\.load\(std::memory_order::relaxed\)/.test(inl), "self_indirect_pointer_handle 应 relaxed 读");
need(count(/kUnpublishedIndirectPointerTag/g) === 2, "InitDontPublish 两处 (Isolate/LocalIsolate) 应用 kUnpublishedIndirectPointerTag");
need(count(/InitSelfIndirectPointerField\(/g) === 6, "InitSelfIndirectPointerField 调用+定义应 6 处");
need(/InAnySharedSpace\(this\)/.test(inl), "shared 判定应走 HeapLayout::InAnySharedSpace");

const body = `${banner("src/objects/trusted-object-inl.h.js", "src/objects/trusted-object-inl.h", "trusted.rs 差集骨架 (inl 语义), 合并后就地修订; -inl 骨架仅存档")}
//! 差集骨架: 受保护字段实现语义 + 发布链 (Rust 无 inl, 并入 trusted.rs 实现)

// 1) 受保护字段 (8 访问器): 统一走 TaggedField<T, 0, TrustedSpaceCompressionScheme>
//    本 crate 现状: 全尺寸槽 (tagged_field.rs 压缩抽象点), trusted cage 压缩推迟登记
//    empty/clear 语义: 与 Smi::zero() 比较 / 写 Smi::zero()
// 2) 发布链 (sandbox): tag = IndirectPointerTagFromInstanceType(map().instance_type(), shared)
//    Publish: table.Publish(handle, tag); Unpublish: table.Unpublish(handle)
//    IsPublished: !TrustedPointerField::IsTrustedPointerFieldUnpublished(host, offsetof(self_indirect_pointer_), tag, isolate)
//    InitAndPublish/InitDontPublish/InitSelfIndirectPointerField: TPT 表项分配 → 推迟 (v8-sandbox)
//    self_indirect_pointer_handle: relaxed load (非 sandbox 构建 UNREACHABLE)
`;

const p = writeSrc("v8-objects-arrays", "trusted_object_inl_h.rs.skel", body);
console.log(`trusted-object-inl.h.js: 骨架 ${p} (对数断言 ${checks} 项通过; scheme=${schemeHits}, SANDBOX 守卫=${sandboxGuards}, VERIFY_HEAP=${verifyGuards})`);
