#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/trace-trait.cc.js — v8/src/heap/cppgc-internal/trace-trait.cc → v8-heap-cppgc/src/trace_trait.rs.skel
// 职责: TraceTraitFromInnerAddressImpl::GetTraceDescriptor — mixin 内地址回溯 trace 描述符
// 规则: 单函数 4 步链 — FromPayload → SynchronizedLoad → ObjectHeaderFromInnerAddressConcurrent<kAtomic> → {ObjectStart, GCInfoFromIndex(GetGCInfoIndex<kAtomic>).trace}
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/trace-trait.cc";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need("TraceDescriptor TraceTraitFromInnerAddressImpl::GetTraceDescriptor(", "函数定义");
need("const void* address) {", "参数");
need("const BasePage* page = BasePage::FromPayload(address);", "页回溯");
need("page->SynchronizedLoad();", "同步载入");
need("ObjectHeaderFromInnerAddressConcurrent<AccessMode::kAtomic>(", "并发内地址取头");
need("return {header.ObjectStart(),", "描述符起点");
need("GlobalGCInfoTable::GCInfoFromIndex(", "GCInfo 表查");
need(".trace};", "取 trace 槽");

const body = `${banner("src/heap/cppgc-internal/trace-trait.cc.js", REL, "TraceTraitFromInnerAddressImpl::GetTraceDescriptor — mixin 内地址 → TraceDescriptor (声明面在 include/cppgc/trace-trait.h)")}
// ══ GetTraceDescriptor (原版 trace-trait.cc:13-26) ══
// 仅 mixin 用 (内地址必在普通页 — 原版注释保证)。
// TraceDescriptor { base_object_payload, trace 回调 }。

/// 原版 trace-trait.cc:13-26 — kAtomic 并发读链
pub fn get_trace_descriptor(address: *const u8) -> TraceDescriptor {
    // v8: 原版注释 — address 保证在普通页 (mixin 专用)
    let page = BasePage::from_payload(address);
    page.synchronized_load();
    let header = page.object_header_from_inner_address_concurrent::<AccessMode::Atomic>(address);
    TraceDescriptor {
        base_object_payload: header.object_start(),
        trace: GlobalGcInfoTable::gc_info_from_index(header.get_gc_info_index::<AccessMode::Atomic>()).trace,
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "trace_trait.rs.skel", body);
console.log(`OK trace-trait.cc.js: 锚 ${n} 条 (定义1+参数1+链4步4+描述符2); 产物 ${out}`);
