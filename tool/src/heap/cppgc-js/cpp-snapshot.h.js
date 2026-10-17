#!/usr/bin/env bun
// tool/src/heap/cppgc-js/cpp-snapshot.h.js — v8/src/heap/cppgc-js/{cpp-snapshot.h,cpp-snapshot.cc} → v8-heap-cppgc/src/cpp_snapshot.rs.skel
// 职责: CppGraphBuilder — C++ 对象图并入堆快照; ParentScope 双型 + 7 访问者/迭代器协作 + Run 3 段管线
// 规则:
//   ① 门面 Run: CHECK 生成器非空 → Impl 构造 → Run
//   ② Impl.Run 3 段: (1) 标记期 Pass 收集弱容器/包装 (2) 根遍历 (Persistent/GlobalHandle/跨线程) (3) 活对象遍历建节点边 + 包装对象连边
//   ③ 双映射: wrapper_map_ (header→WrapperInfo{entry, v8_object, detachedness}) / weak_containers_ (header→weak TraceDescriptor)
//   ④ GraphBuildingStackVisitor: 三基类 (ConservativeTracingVisitor + StackVisitor + Visitor); VisitPointer → 保守 trace + TracedNode 扫描
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-js/cpp-snapshot.h";
const REL_CC = "src/heap/cppgc-js/cpp-snapshot.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE CppGraphBuilder final {", "门面类");
need(src, "static void Run(v8::internal::CppHeap& cpp_heap,", "Run 声明");
need(src, "CppHeapWrapperSet&& cpp_heap_wrappers);", "包装集参数");
need(src, "CppGraphBuilder() = delete;", "删除默认构造");
need(cc, "class ParentScope final {", "父域类");
need(cc, "enum class Type {", "父域类型枚举");
need(cc, "kRoot,", "根型");
need(cc, "bool IsRoot() const { return type_ == Type::kRoot; }", "根判定内联");
need(cc, "void* ExtractEmbedderDataBackref(Isolate* isolate, CppHeap& cpp_heap,", "嵌入数据回溯函数");
need(cc, "MayHaveEmbedderFields()", "嵌入字段判定");
need(cc, "GetCppHeapWrappable(\n      isolate, kAnyCppHeapPointer);", "取 cpp 可包装对象");
need(cc, "class CppGraphBuilderImpl final {", "Impl 类");
need(cc, "struct WrapperInfo {", "包装信息结构");
need(cc, "v8::EmbedderGraph::Node::Detachedness detachedness;", "分离度字段");
need(cc, "wrapper_map_.emplace(header, info);", "包装映射插入");
need(cc, "weak_containers_.find(&header);", "弱容器查找");
need(cc, "absl::flat_hash_map<const HeapObjectHeader*, WrapperInfo> wrapper_map_;", "包装映射成员");
need(cc, "class GraphBuildingVisitor final : public JSVisitor {", "建图访问者");
need(cc, "class PrePassVisitor final : public JSVisitor {", "预遍访问者");
need(cc, "class PrePassIterator final", "预遍迭代器");
need(cc, "friend class cppgc::internal::HeapVisitor<PrePassIterator>;", "HeapVisitor 友元");
need(cc, "class LiveObjectsIterator final", "活对象迭代器");
need(cc, "class GraphBuildingRootVisitor final : public cppgc::internal::RootVisitorBase {", "根访问者");
need(cc, "class GraphBuildingStackVisitor", "栈访问者");
need(cc, ": public cppgc::internal::ConservativeTracingVisitor,", "保守基类");
need(cc, "public ::heap::base::StackVisitor,", "栈基类");
need(cc, "public cppgc::Visitor {", "cppgc Visitor 基类");
need(cc, "cppgc::internal::VisitorFactory::CreateKey()),", "钥匙构造");
need(cc, "TraceConservativelyIfNeeded(address);", "保守 trace 入口");
need(cc, "traced_handles_scanner_.TryFindNode(address)", "TracedNode 扫描");
need(cc, "RecordTracedReferenceOnStack(", "栈上引用记录");
need(cc, "void VisitConservatively(const HeapObjectHeader& header) {", "统一保守访");
need(cc, "AddRootNode(\"C++ CrossThreadPersistent roots\");", "跨线程根节点");
need(cc, "GetStrongCrossThreadPersistentRegion().Iterate(", "跨线程强持久迭代");
need(cc, "LiveObjectsIterator visitor(*this);", "活对象迭代构造");
need(cc, "visitor.Traverse(cpp_heap_.raw_heap());", "全堆遍历");
need(cc, "for (Tagged<CppHeapPointerWrapperObjectT> object : cpp_heap_wrappers_) {", "包装连边循环");
need(cc, "AddEdgeForCppHeapWrapper(object);", "包装连边");
need(cc, "CHECK_NOT_NULL(generator);", "生成器非空检查");
need(cc, "CppGraphBuilderImpl graph_builder(cpp_heap, generator,", "Impl 构造调用");

const body = `${banner("src/heap/cppgc-js/cpp-snapshot.h.js", REL_H, "CppGraphBuilder — C++ 对象图并入堆快照 (7 协作访问者, cc 实现一并落地)")}
// ══ CppGraphBuilder 门面 (原版 cpp-snapshot.h:17-26, 实现 cc:677-683) ══

/// 静态入口: 算法内部见 CppGraphBuilderImpl
pub struct CppGraphBuilder;
impl CppGraphBuilder {
    /// 原版 cc:677-683 — CHECK 生成器 → Impl 构造 → Run
    pub fn run(cpp_heap: &mut CppHeap, generator: *mut HeapSnapshotGenerator, cpp_heap_wrappers: CppHeapWrapperSet) {
        v8_check_not_null(generator);
        let mut graph_builder = CppGraphBuilderImpl::new(cpp_heap, generator, cpp_heap_wrappers);
        graph_builder.run();
    }
}

// ══ ParentScope (原版 cc:44-67) ══

/// 双型父域: 常规 / 根
pub struct ParentScope {
    entry: *mut HeapEntry,
    type_: ParentScopeType,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentScopeType { Regular, Root }
impl ParentScope {
    /// 原版 :52-56 — DCHECK entry 非空
    pub fn new(entry: *mut HeapEntry, type_: ParentScopeType) -> Self {
        debug_assert!(!entry.is_null());
        Self { entry, type_ }
    }
    pub fn entry(&self) -> *mut HeapEntry { self.entry } // :59
    pub fn is_root(&self) -> bool { self.type_ == ParentScopeType::Root } // :60
}

// ══ ExtractEmbedderDataBackref (原版 cc:69-84) ══
/// V8 值 → JSObject → JSApiWrapper → cpp_heap_wrappable 字段回溯; 非对象/无嵌入字段/非 wrapper 三段退
pub fn extract_embedder_data_backref(isolate: *mut Isolate, v8_value: Local<v8::Data>) -> *mut u8 { unimplemented!() }

// ══ WrapperInfo (原版 cc:88-92) ══
pub struct WrapperInfo {
    pub entry: *mut HeapEntry,
    pub v8_object: Tagged<HeapObject>,
    pub detachedness: v8::EmbedderGraphNodeDetachedness,
}

// ══ CppGraphBuilderImpl (原版 cc:86-273) ══
pub struct CppGraphBuilderImpl {
    cpp_heap: *mut CppHeap,
    generator: *mut HeapSnapshotGenerator,
    cpp_heap_wrappers: CppHeapWrapperSet,
    /// header → WrapperInfo
    wrapper_map: std::collections::HashMap<*const HeapObjectHeader, WrapperInfo>,
    /// header → weak TraceDescriptor (标记期收集)
    weak_containers: std::collections::HashMap<*const HeapObjectHeader, TraceDescriptor>,
    // v8: 其余收集状态 (stack reachability, entries_map 等, cc:150-268)
}
impl CppGraphBuilderImpl {
    pub fn new(cpp_heap: &mut CppHeap, generator: *mut HeapSnapshotGenerator, cpp_heap_wrappers: CppHeapWrapperSet) -> Self { unimplemented!() }

    /// 原版 Run (cc 尾段) — 3 段:
    /// (1) 预遍 (标记期 GraphBuildingVisitor) 收集弱容器与包装
    /// (2) 根遍历: Persistent 域 / GlobalHandles / 跨线程强持久 (锁内) — 各挂 ParentScope{Root}
    /// (3) LiveObjectsIterator 全堆遍历建节点边 + 逐包装对象连边
    pub fn run(&mut self) {
        // (2) 样例: 跨线程强持久根 (原版 cc:655-662)
        // AddRootNode("C++ CrossThreadPersistent roots") + PersistentRegionLock + Iterate
        // (3) 活对象遍历 (原版 cc:667-669)
        let mut visitor = LiveObjectsIterator::new(self);
        visitor.traverse(unsafe { &mut *self.cpp_heap }.raw_heap());
        // 包装连边 (原版 cc:672-675)
        for object in &self.cpp_heap_wrappers {
            self.add_edge_for_cpp_heap_wrapper(object);
        }
    }

    /// 原版 cc:105 — 包装映射插入
    pub fn record_wrapper(&mut self, header: *const HeapObjectHeader, info: WrapperInfo) {
        self.wrapper_map.insert(header, info);
    }
    /// 原版 cc:108-112 — 弱容器描述符查表 (无则 null)
    pub fn get_weak_container_trace_descriptor(&self, header: &HeapObjectHeader) -> *const TraceDescriptor {
        match self.weak_containers.get(&(header as *const _)) {
            Some(d) => d as *const _,
            None => std::ptr::null(),
        }
    }
    pub fn add_root_edge(&mut self, _e: *mut HeapEntry, _h: &HeapObjectHeader, _l: cppgc::SourceLocation) { unimplemented!() }
    pub fn record_object_reachable_from_stack(&mut self, _h: &HeapObjectHeader) { unimplemented!() }
    pub fn record_weak_container(&mut self, header: &HeapObjectHeader, weak_desc: TraceDescriptor) { self.weak_containers.insert(header, weak_desc); } // cc:498
    fn add_edge_for_cpp_heap_wrapper(&mut self, _o: Tagged<CppHeapPointerWrapperObjectT>) { unimplemented!() }
}

// ══ 协作访问者 7 件 (原版 cc:275-675) ══

/// GraphBuildingVisitor — JSVisitor: 标记期主访问者 (节点/边生成)
pub struct GraphBuildingVisitor; // cc:275
/// PrePassVisitor — JSVisitor: 预遍 (弱容器发现)
pub struct PrePassVisitor; // cc:343
/// PrePassIterator — HeapVisitor CRTP: 预遍页迭代
pub struct PrePassIterator; // cc:401
/// LiveObjectsIterator — HeapVisitor CRTP: 活对象全堆遍历 (建图主循环)
pub struct LiveObjectsIterator; // cc:424
impl LiveObjectsIterator {
    pub fn new(_b: &mut CppGraphBuilderImpl) -> Self { Self }
    pub fn traverse(&mut self, _raw_heap: &mut RawHeap) { unimplemented!() }
}
/// GraphBuildingRootVisitor — RootVisitorBase: 持久根遍历
pub struct GraphBuildingRootVisitor; // cc:463

/// GraphBuildingStackVisitor (原版 cc:547-639) — 三基类:
/// ConservativeTracingVisitor + heap::base::StackVisitor + cppgc::Visitor
pub struct GraphBuildingStackVisitor {
    graph_builder: *mut CppGraphBuilderImpl,
    root_visitor: *mut GraphBuildingRootVisitor,
    traced_handles_parent_scope: ParentScope,
    isolate: *mut Isolate,
    traced_handles_scanner: TracedHandlesScanner,
}
impl GraphBuildingStackVisitor {
    /// 原版 :567-577 — 栈走入口: 保守 trace + TracedNode 扫描双路
    pub fn visit_pointer(&mut self, address: *const u8) {
        // 完建 → VisitFullyConstructedConservatively; 在建 → VisitInConstructionConservatively
        self.trace_conservatively_if_needed(address);
        if let Some(node) = self.traced_handles_scanner.try_find_node(address) {
            let object = node.object();
            if is_heap_object(object) {
                unsafe { &mut *self.graph_builder }
                    .record_traced_reference_on_stack(self.traced_handles_parent_scope.entry(), cast_heap_object(object));
            }
        }
    }
    /// 原版 :579-581 — 完建 → 统一保守访
    pub fn visit_fully_constructed_conservatively(&mut self, header: &mut HeapObjectHeader) {
        self.visit_conservatively(header);
    }
    /// 原版 :583-586 — 在建 → 统一保守访
    pub fn visit_in_construction_conservatively(&mut self, header: &mut HeapObjectHeader, _cb: TraceConservativelyCallback) {
        self.visit_conservatively(header);
    }
    /// 原版 :590-596 — 根访 (无 trace 回调) + 栈可达记录
    fn visit_conservatively(&mut self, header: &HeapObjectHeader) {
        // v8: root_visitor_.VisitRoot(header.ObjectStart(), {start, nullptr}, SourceLocation())
        unsafe { &mut *self.graph_builder }.record_object_reachable_from_stack(header);
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "cpp_snapshot.rs.skel", body);
console.log(`OK cpp-snapshot.h.js: 锚 ${n} 条 (h4+cc34); 7 访问者 + 3 段管线; 产物 ${out}`);
