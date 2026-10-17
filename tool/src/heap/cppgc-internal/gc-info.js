#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/gc-info.js — v8/src/heap/cppgc-internal/gc-info.cc → v8-heap-cppgc/src/gc_info.rs.skel
// EnsureGCInfoIndexTrait::EnsureGCInfoIndex 四重载: 按 (finalization, name) 参数组合分派 GCInfo 构造
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/gc-info.cc";
const src = readV8(REL);

let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`gc-info.cc 缺锚 ${label}: ${needle}`);
  n++;
};

need("#if !defined(CPPGC_ENABLE_OBJECT_SECTION_GCINFO)", "对象段条件");
need("GCInfoIndex EnsureGCInfoIndexTrait::EnsureGCInfoIndex(", "重载声明");
// 四重载逐个对锚: (trace, finalization, name) / (trace, finalization) / (trace, name) / (trace)
const OVERLOADS = [
  ["std::atomic<GCInfoIndex>& registered_index, TraceCallback trace_callback,\n    FinalizationCallback finalization_callback, NameCallback name_callback",
    "GCInfo(finalization_callback, trace_callback, name_callback)"],
  ["std::atomic<GCInfoIndex>& registered_index, TraceCallback trace_callback,\n    FinalizationCallback finalization_callback)",
    "GCInfo(finalization_callback, trace_callback, GetHiddenName)"],
  ["std::atomic<GCInfoIndex>& registered_index, TraceCallback trace_callback,\n    NameCallback name_callback)",
    "GCInfo(nullptr, trace_callback, name_callback)"],
  ["std::atomic<GCInfoIndex>& registered_index, TraceCallback trace_callback)",
    "GCInfo(nullptr, trace_callback, GetHiddenName)"],
];
for (const [sig, ctor] of OVERLOADS) {
  need(`EnsureGCInfoIndex(\n    ${sig}`, "重载签名");
  need(ctor, "GCInfo 构造实参");
}
need("return GlobalGCInfoTable::GetMutable().RegisterNewGCInfo(", "注册入口");

const body = `${banner("src/heap/cppgc-internal/gc-info.js", REL, "EnsureGCInfoIndex 四重载骨架 (全部委托 GlobalGCInfoTable::RegisterNewGCInfo)")}
#![allow(dead_code)]

use crate::gc_info_table::GlobalGcInfoTable;

/// v8: src/heap/cppgc-internal/gc-info.cc EnsureGCInfoIndexTrait
pub mod ensure_gc_info_index_trait {
    use super::*;

    /// v8: src/heap/cppgc-internal/gc-info.cc EnsureGCInfoIndex(trace, finalization, name)
    pub fn ensure_gc_info_index_full(
        table: &GlobalGcInfoTable,
        registered_index: &AtomicU16,
        trace_callback: TraceCallback,
        finalization_callback: FinalizationCallback,
        name_callback: NameCallback,
    ) -> GcInfoIndex {
        table.register_new_gc_info(
            registered_index,
            GcInfo { finalize: finalization_callback, trace: trace_callback, name: name_callback },
        )
    }

    /// v8: src/heap/cppgc-internal/gc-info.cc EnsureGCInfoIndex(trace, finalization) — 隐名
    pub fn ensure_gc_info_index_fin(
        table: &GlobalGcInfoTable,
        registered_index: &AtomicU16,
        trace_callback: TraceCallback,
        finalization_callback: FinalizationCallback,
    ) -> GcInfoIndex {
        table.register_new_gc_info(
            registered_index,
            GcInfo { finalize: finalization_callback, trace: trace_callback, name: NameCallback::HIDDEN },
        )
    }

    /// v8: src/heap/cppgc-internal/gc-info.cc EnsureGCInfoIndex(trace, name) — 无析构
    pub fn ensure_gc_info_index_name(
        table: &GlobalGcInfoTable,
        registered_index: &AtomicU16,
        trace_callback: TraceCallback,
        name_callback: NameCallback,
    ) -> GcInfoIndex {
        table.register_new_gc_info(
            registered_index,
            GcInfo { finalize: None, trace: trace_callback, name: name_callback },
        )
    }

    /// v8: src/heap/cppgc-internal/gc-info.cc EnsureGCInfoIndex(trace) — 无析构 + 隐名
    pub fn ensure_gc_info_index(
        table: &GlobalGcInfoTable,
        registered_index: &AtomicU16,
        trace_callback: TraceCallback,
    ) -> GcInfoIndex {
        table.register_new_gc_info(
            registered_index,
            GcInfo { finalize: None, trace: trace_callback, name: NameCallback::HIDDEN },
        )
    }
}
`;

writeSrc("v8-heap-cppgc", "gc_info.rs.skel", body);
console.log(`gc-info.js: 对数 ${n} 项 (条件 1 + 重载签名 4 + 构造实参 4 + 入口 1) → v8-heap-cppgc/src/gc_info.rs.skel`);
