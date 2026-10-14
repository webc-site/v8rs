#!/usr/bin/env bun
// tool/src/objects/visitors-inl.h.js — 访客内联实现生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/visitors-inl.h → v8-objects-core/src/visitors_inl.rs
//   (visitors.h/cc 的 Root 表与 trait 面归 tool/src/objects/visitors.h.js → visitors.rs;
//    本脚本登记 -inl 侧: ClientRootVisitor/ClientObjectVisitor 的共享堆过滤语义)
import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/visitors-inl.h");

// —— 构造器面: ObjectVisitorWithCageBases 3 构造 ——
const ctors = [...inl.matchAll(/^ObjectVisitorWithCageBases::ObjectVisitorWithCageBases\(([^)]*)\)/gm)].map((m) => m[1].replace(/\s+/g, " "));
if (ctors.length !== 3) throw new Error(`ObjectVisitorWithCageBases 构造应 3, 实得 ${ctors.length}`);
if (!ctors[0].includes("PtrComprCageBase cage_base, PtrComprCageBase code_cage_base")) throw new Error("双 cage 构造签名漂移");
if (!ctors[1].includes("Isolate* isolate")) throw new Error("Isolate 构造缺失");
if (!ctors[2].includes("Heap* heap")) throw new Error("Heap 构造缺失");

// —— ClientRootVisitor 面 ——
const crv = [...inl.matchAll(/^(?:inline )?void ClientRootVisitor<Visitor>::(\w+)\(/gm)].map((m) => m[1]);
if (crv.join() !== "VisitRunningCode") throw new Error(`ClientRootVisitor 方法面漂移: ${crv}`);
const crvStatic = [...inl.matchAll(/^bool ClientRootVisitor<Visitor>::(\w+)\(/gm)].map((m) => m[1]);
if (crvStatic.join() !== "IsSharedHeapObject") throw new Error(`ClientRootVisitor 静态面漂移: ${crvStatic}`);

// —— ClientObjectVisitor 面 (定义顺序) ——
const cov = [...inl.matchAll(/^(?:inline )?void ClientObjectVisitor<Visitor>::(\w+)\(/gm)].map((m) => m[1]);
const COV_EXPECT = ["VisitMapPointer", "VisitInstructionStreamPointer", "VisitCodeTarget", "VisitEmbeddedPointer"];
if (cov.join() !== COV_EXPECT.join()) throw new Error(`ClientObjectVisitor 方法面漂移: 期望 ${COV_EXPECT}, 实得 ${cov}`);
const covStatic = [...inl.matchAll(/^bool ClientObjectVisitor<Visitor>::(\w+)\(/gm)].map((m) => m[1]);
if (covStatic.join() !== "IsSharedHeapObject") throw new Error(`ClientObjectVisitor 静态面漂移: ${covStatic}`);

// —— 共享堆过滤语义锚 ——
if (!/HeapLayout::InWritableSharedSpace/.test(inl)) throw new Error("InWritableSharedSpace 过滤锚缺失");
// 过滤规则: map 指针/嵌入指针 仅共享对象转发; 代码目标/指令流 仅 DEBUG 断言不转发
if (!/if \(!IsSharedHeapObject\(host->map\(\)\)\) return;/.test(inl)) throw new Error("VisitMapPointer 过滤语义缺失");
if (!/if \(!IsSharedHeapObject\(rinfo->target_object\(\)\)\) return;/.test(inl)) throw new Error("VisitEmbeddedPointer 过滤语义缺失");

const body = `${banner("src/objects/visitors-inl.h.js", "src/objects/visitors-inl.h", "客户端访客共享堆过滤语义; trait/Root 表在 visitors.rs")}
//! 访客内联实现 (原版 visitors-inl.h)
//!
//! 客户端 (client isolate) 访客在多 isolate 共享堆下只把"可共享对象"
//! 转发给真实访客: \`IsSharedHeapObject = IsHeapObject && InWritableSharedSpace\`.

/// cage 基址对 (压缩开: 主 cage + 代码 cage; 基线非压缩为空对)
#[derive(Debug, Clone, Copy, Default)]
pub struct CageBases {
  /// 主指针压缩 cage 基址 (非压缩基线 0)
  pub cage_base: usize,
  /// 代码 cage 基址 (V8_EXTERNAL_CODE_SPACE 开时区别于主 cage)
  pub code_cage_base: usize,
}

impl CageBases {
  /// 双 cage 直构 (原版 ObjectVisitorWithCageBases(cage_base, code_cage_base))
  pub fn new(cage_base: usize, code_cage_base: usize) -> Self {
    Self { cage_base, code_cage_base }
  }

  /// 从 Isolate 取双 cage (原版 ObjectVisitorWithCageBases(Isolate*))
  pub fn from_isolate(/* isolate: &Isolate */) -> Self {
    todo!("就地修订: isolate.cage_base() / isolate.code_cage_base()")
  }

  /// 从 Heap 取 (原版 ObjectVisitorWithCageBases(Heap*): 经 Isolate::FromHeap)
  pub fn from_heap(/* heap: &Heap */) -> Self {
    todo!("就地修订: Isolate::FromHeap(heap) 后走 from_isolate")
  }
}

/// 共享堆对象判定 (原版 ClientRootVisitor/ClientObjectVisitor::IsSharedHeapObject, 静态)
pub fn is_shared_heap_object(/* object: Tagged<Object> */) -> bool {
  todo!("就地修订: IsHeapObject(object) && HeapLayout::InWritableSharedSpace(obj)")
}

/// 运行中代码根访问 (客户端版仅 DEBUG 断言: 代码槽不得在可写共享空间)
pub fn client_root_visitor_visit_running_code(/* code_slot, maybe_istream_slot */) {
  todo!("就地修订: DEBUG 下双槽 InWritableSharedSpace 断言, release 空实现")
}

/// map 指针访问 (客户端版: 宿主 map 非共享则不转发)
pub fn client_object_visitor_visit_map_pointer(/* host: Tagged<HeapObject>, actual_visitor */) {
  todo!("就地修订: !IsSharedHeapObject(host->map()) → return; 否则转发 actual_visitor")
}

/// 指令流指针访问 (客户端版仅 DEBUG 断言: 指令流不可在可写共享空间)
pub fn client_object_visitor_visit_instruction_stream_pointer(/* host: Tagged<Code>, slot: InstructionStreamSlot */) {
  todo!("就地修订: DEBUG 断言 InWritableSharedSpace(istream) 不成立")
}

/// 代码目标访问 (客户端版仅 DEBUG 断言: 重定位目标不可在可写共享空间)
pub fn client_object_visitor_visit_code_target(/* host: Tagged<InstructionStream>, rinfo */) {
  todo!("就地修订: DEBUG 断言 target 不在 InWritableSharedSpace")
}

/// 嵌入指针访问 (客户端版: 目标非共享则不转发)
pub fn client_object_visitor_visit_embedded_pointer(/* host, rinfo, actual_visitor */) {
  todo!("就地修订: !IsSharedHeapObject(rinfo->target_object()) → return; 否则转发")
}
`;

const p = writeSrc("v8-objects-core", "visitors_inl.rs", body);
console.log(`visitors-inl.h.js: 已生成 ${p} (构造 ${ctors.length}, CRV ${crv.length + crvStatic.length}, COV ${cov.length + covStatic.length})`);
