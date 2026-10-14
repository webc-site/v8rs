#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/liveness-broker.js — v8/src/heap/cppgc-internal/liveness-broker.h + .cc → v8-heap-cppgc/src/liveness_broker.rs.skel
// 存活代理工厂: LivenessBrokerFactory::Create + IsHeapObjectAliveImpl (标记位查询)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/liveness-broker.h";
const REL_CC = "src/heap/cppgc-internal/liveness-broker.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE LivenessBrokerFactory {", "LivenessBrokerFactory 声明");
need(src, "static LivenessBroker Create();", "Create 声明");
need(srcCc, "bool LivenessBroker::IsHeapObjectAliveImpl(const void* payload) const {", "IsHeapObjectAliveImpl");
need(srcCc, "return internal::HeapObjectHeader::FromObject(payload).IsMarked();", "IsMarked 委托");
need(srcCc, "cppgc::LivenessBroker LivenessBrokerFactory::Create() {", "Create 实体");
need(srcCc, "return cppgc::LivenessBroker();", "默认构造返回");

const body = `${banner("src/heap/cppgc-internal/liveness-broker.js", `${REL} + ${REL_CC}`, "存活查询: 标记位即存活")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/liveness-broker.cc LivenessBroker::IsHeapObjectAliveImpl
pub fn is_heap_object_alive_impl(payload: *const core::ffi::c_void) -> bool {
    // v8: HeapObjectHeader::FromObject(payload).IsMarked()
    HeapObjectHeader::from_object(payload).is_marked(AccessMode::NonAtomic)
}

/// v8: src/heap/cppgc-internal/liveness-broker.h LivenessBrokerFactory
pub struct LivenessBrokerFactory;

impl LivenessBrokerFactory {
    /// v8: src/heap/cppgc-internal/liveness-broker.cc LivenessBrokerFactory::Create
    pub fn create() -> LivenessBroker {
        LivenessBroker::default()
    }
}
`;

writeSrc("v8-heap-cppgc", "liveness_broker.rs.skel", body);
console.log(`liveness-broker.js: 对数 ${n} 项 (.h 2 + .cc 4) → v8-heap-cppgc/src/liveness_broker.rs.skel`);
