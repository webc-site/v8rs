#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/persistent-node.cc.js — v8/src/heap/cppgc-internal/persistent-node.cc → v8-heap-cppgc/src/persistent_node.rs.skel
// 职责: PersistentRegionBase 节点池 (slots×freelist) 清扫/填充/迭代 + PersistentRegionLock 全局锁 RAII + CrossThread 变体
// 规则:
//   ① ClearAllUsedNodes<P>: 逐节点 owner()->ClearFromGC + 回收 freelist, 计数归零 DCHECK; 两显式模板实例 (CrossThread/Persistent)
//   ② RefillFreeList: 新 slots 块全数入 freelist, 分配失败走 oom_handler
//   ③ Iterate: freelist 重建 + 空块释放 (first_next 跨块 DCHECK) + nodes_ 压缩
//   ④ PersistentRegionLock: ProcessGlobalLock<kForCrossThreadHandleCreation> RAII; CrossThread 析构持锁清 + Iterate/Clear 断言持锁
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/persistent-node.cc";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need("PersistentRegionBase::PersistentRegionBase(", "基类构造");
need("const FatalOutOfMemoryHandler& oom_handler)", "OOM handler 参数");
need("PersistentRegionBase::~PersistentRegionBase() { ClearAllUsedNodes(); }", "析构清扫");
need("static_cast<PersistentBaseClass*>(node.owner())->ClearFromGC();", "owner 清除调用");
need("node.InitializeAsFreeNode(free_list_head_);", "freelist 回收式");
need("CPPGC_DCHECK(nodes_in_use_ > 0);", "计数正断言");
need("CPPGC_DCHECK(0u == nodes_in_use_);", "归零断言");
need("ClearAllUsedNodes<CrossThreadPersistentBase>();", "CrossThread 实例");
need("ClearAllUsedNodes<PersistentBase>();", "Persistent 实例");
need('oom_handler_("Oilpan: PersistentRegionBase::RefillFreeList()");', "OOM 报告");
need("auto* node = TryAllocateNodeFromFreeList(owner, trace);", "分配链");
need("node.Trace(root_visitor);", "trace 调用");
need("PersistentNode* first_next = (*slots)[0].FreeListNext();", "空块首链");
need("slots.reset();", "空块释放");
need("nodes_.erase(std::remove_if(nodes_.begin(), nodes_.end(),", "容器压缩");
need("return heap_.CurrentThreadIsHeapThread();", "创建线程判定");
need("ProcessGlobalLock::Reason::kForCrossThreadHandleCreation>();", "锁 reason 实参");
need("void PersistentRegionLock::AssertLocked() { ProcessGlobalLock::AssertHeld(); }", "持锁断言转发");
need("PersistentRegionBase::ClearAllUsedNodes<CrossThreadPersistentBase>();", "CrossThread 析构持锁清扫");
need("// This method does not require a lock.", "NodesInUse 免锁注释");

const body = `${banner("src/heap/cppgc-internal/persistent-node.cc.js", REL, "PersistentRegionBase 节点池管理 + PersistentRegionLock RAII + CrossThread 变体 (声明面在 include/cppgc/internal/persistent-node.h)")}
// ══ PersistentRegionBase 节点池 (原版 persistent-node.cc:20-113) ══
// 池 = PersistentNodeSlots 块向量 + 单链 freelist。

impl PersistentRegionBase {
    /// 原版 :20-22 — 存 OOM handler
    pub fn new(oom_handler: &FatalOutOfMemoryHandler) -> Self {
        Self { oom_handler, nodes: Vec::new(), free_list_head: std::ptr::null_mut(), nodes_in_use: 0 }
    }

    /// 原版 :24 — 析构清扫全部在用节点
    pub fn drop(&mut self) {
        self.clear_all_used_nodes_persistent();
    }

    /// 原版 :26-43 — 泛型版: 逐节点 ClearFromGC + 回收 freelist, 计数递减到 0
    /// 两实例: <CrossThreadPersistentBase> (cc:46) / <PersistentBase> (cc:47)
    fn clear_all_used_nodes<P: PersistentBaseClass>(&mut self) {
        for slots in &mut self.nodes {
            for node in slots.iter_mut() {
                if !node.is_used() {
                    continue;
                }
                node.owner::<P>().clear_from_gc();
                node.initialize_as_free_node(self.free_list_head);
                self.free_list_head = node;
                cppgc_dcheck!(self.nodes_in_use > 0);
                self.nodes_in_use -= 1;
            }
        }
        cppgc_dcheck!(self.nodes_in_use == 0);
    }

    /// 原版 :49-51 — 非模板入口固定 <PersistentBase>
    pub fn clear_all_used_nodes_persistent(&mut self) {
        self.clear_all_used_nodes::<PersistentBase>();
    }

    /// 原版 :53-65 — DEBUG: 全量重数与计数对账; 返回 nodes_in_use_
    pub fn nodes_in_use(&self) -> usize {
        // v8: DEBUG — accumulate count_if(node.IsUsed()) == nodes_in_use_
        self.nodes_in_use
    }

    /// 原版 :67-77 — 新块入向量, 全数挂 freelist; 块分配失败 OOM
    pub fn refill_free_list(&mut self) {
        let mut node_slots = Box::new(PersistentNodeSlots::new());
        // v8: 原版 make_unique 失败 → oom_handler_("Oilpan: PersistentRegionBase::RefillFreeList()")
        self.nodes.push(node_slots);
        let last = self.nodes.last_mut().unwrap();
        for node in last.iter_mut() {
            node.initialize_as_free_node(self.free_list_head);
            self.free_list_head = node;
        }
    }

    /// 原版 :79-85 — Refill 后必得节点 (DCHECK)
    pub fn refill_free_list_and_allocate_node(&mut self, owner: *mut u8, trace: TraceRootCallback) -> *mut PersistentNode {
        self.refill_free_list();
        let node = self.try_allocate_node_from_free_list(owner, trace);
        cppgc_dcheck!(!node.is_null());
        node
    }

    /// 原版 :87-113 — 遍历: 在用 trace; 空闲重挂 freelist; 全空块释放 (首链跨块 DCHECK); 尾部压缩
    pub fn iterate(&mut self, root_visitor: &mut RootVisitor) {
        self.free_list_head = std::ptr::null_mut();
        for slots in &mut self.nodes {
            let mut is_empty = true;
            for node in slots.iter_mut() {
                if node.is_used() {
                    node.trace(root_visitor);
                    is_empty = false;
                } else {
                    node.initialize_as_free_node(self.free_list_head);
                    self.free_list_head = node;
                }
            }
            if is_empty {
                // 首链已在上方循环先处理 — 必 null 或指向别块
                let first_next = slots.at(0).free_list_next();
                cppgc_dcheck!(first_next.is_null() || first_next < slots.front() || first_next > slots.back());
                self.free_list_head = first_next;
                slots.reset(); // 整块释放
            }
        }
        self.nodes.retain(|p| p.is_some()); // remove_if null 压缩
    }
}

// ══ PersistentRegion (原版 :115-117) ══
impl PersistentRegion {
    /// 创建线程 = 堆线程
    pub fn is_creation_thread(&self) -> bool {
        self.heap().current_thread_is_heap_thread()
    }
}

// ══ PersistentRegionLock (原版 :119-130) ══

/// RAII: 构造/析构 Lock/Unlock<kForCrossThreadHandleCreation>
pub struct PersistentRegionLock;
impl PersistentRegionLock {
    pub fn new() -> Self {
        ProcessGlobalLock::lock::<ForCrossThreadHandleCreation>();
        Self
    }
}
impl Drop for PersistentRegionLock {
    fn drop(&mut self) {
        ProcessGlobalLock::unlock::<ForCrossThreadHandleCreation>();
    }
}
/// 原版 :130 — AssertLocked = ProcessGlobalLock::AssertHeld
pub fn persistent_region_assert_locked() {
    ProcessGlobalLock::assert_held();
}

// ══ CrossThreadPersistentRegion (原版 :132-156) ══

impl CrossThreadPersistentRegion {
    /// 原版 :136-141 — 析构: 持锁清扫 <CrossThreadPersistentBase> + 清容器 (基类析构变 noop)
    pub fn drop(&mut self) {
        let _guard = PersistentRegionLock::new();
        self.clear_all_used_nodes_cross_thread();
        self.nodes.clear();
    }

    /// 原版 :143-146 — 断言持锁后走基类 Iterate
    pub fn iterate(&mut self, root_visitor: &mut RootVisitor) {
        persistent_region_assert_locked();
        PersistentRegionBase::iterate(self, root_visitor);
    }

    /// 原版 :148-151 — 免锁读 (计数为原子语义)
    pub fn nodes_in_use(&self) -> usize {
        // v8: 原版注释 — This method does not require a lock.
        PersistentRegionBase::nodes_in_use(self)
    }

    /// 原版 :153-156 — 断言持锁后清扫 CrossThread 基类
    pub fn clear_all_used_nodes_cross_thread(&mut self) {
        persistent_region_assert_locked();
        self.clear_all_used_nodes::<CrossThreadPersistentBase>();
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "persistent_node.rs.skel", body);
console.log(`OK persistent-node.cc.js: 锚 ${n} 条; 池 6 方法 + 锁 RAII + CrossThread 4; 产物 ${out}`);
