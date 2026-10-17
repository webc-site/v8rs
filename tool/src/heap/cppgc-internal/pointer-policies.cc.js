#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/pointer-policies.cc.js — v8/src/heap/cppgc-internal/pointer-policies.cc → v8-heap-cppgc/src/pointer_policies.rs.skel
// 职责: SameThreadEnabledCheckingPolicyBase::CheckPointerImpl 全量 DEBUG 校验链 + 4 个 Persistent 策略的 region 取用
// 规则:
//   ① CheckPointerImpl 5 段链: 栈/cage/-1 哨兵 → 堆归属 (惰性绑定 heap_) → 头部校验 (payload 去标 vs mixin 内地址) → CPPGC_VERIFY_HEAP 段
//   ② 4 个 GetPersistentRegion: Strong/Weak × 同线程/跨线程, 均 BasePage::FromPayload→heap() 链
//   ③ DEBUG 专属 IsOnStack: 栈区间 [当前 SP, 栈起点)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/pointer-policies.cc";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need("void SameThreadEnabledCheckingPolicyBase::CheckPointerImpl(", "校验函数定义");
need("DCHECK(!IsOnStack(ptr));", "禁栈断言");
need("DCHECK(CagedHeapBase::IsWithinCage(ptr));", "cage 内断言");
need("DCHECK_NE(reinterpret_cast<void*>(-1), ptr);", "-1 哨兵断言");
need("DCHECK_IMPLIES(base_page->is_large(), points_to_payload);", "大对象必 payload 断言");
need("heap_ = &base_page->heap();", "惰性堆绑定");
need("CHECK(!HeapRegistry::TryFromManagedPointer(this));", "堆外引用无注册断言");
need("DCHECK_EQ(heap_, &base_page->heap());", "禁混堆断言");
need("DCHECK(heap_->CurrentThreadIsHeapThread());", "同线程断言");
need("reinterpret_cast<uintptr_t>(ptr) & ~kAllocationMask", "payload 去标掩码");
need("&base_page->ObjectHeaderFromInnerAddress<AccessMode::kAtomic>(ptr));", "头一致性断言");
need("DCHECK(!header->IsFree());", "禁 free 断言");
need("heap_->prefinalizer_handler()->IsInvokingPreFinalizers()", "VERIFY_HEAP 段");
need("DCHECK_IMPLIES(slot_is_live, header->IsMarked());", "slot 活→对象活断言");
// 4 个 GetPersistentRegion
const regions = [...src.matchAll(/(\w*Persistent\w*) (Strong\w*|Weak\w*)PersistentPolicy::GetPersistentRegion\(/g)];
if ((src.match(/GetPersistentRegion\(/g) || []).length !== 4)
  throw new Error(`GetPersistentRegion 计数: got=${(src.match(/GetPersistentRegion\(/g) || []).length} want=4`);
n++;
need("->heap().GetStrongPersistentRegion();", "Strong 同线程链");
need("->heap().GetWeakPersistentRegion();", "Weak 同线程链");
need("GetStrongCrossThreadPersistentRegion();", "Strong 跨线程链");
need("GetWeakCrossThreadPersistentRegion();", "Weak 跨线程链");

const body = `${banner("src/heap/cppgc-internal/pointer-policies.cc.js", REL, "CheckPointerImpl 校验链 + 4 Persistent 策略 region 取用 (声明面在 include/cppgc/internal/pointer-policies.h)")}
// ══ SameThreadEnabledCheckingPolicyBase::CheckPointerImpl (原版 pointer-policies.cc:33-106) ══
// 5 段: 入参形态 → 堆归属 → 混堆/线程 → 头校验双臂 → CPPGC_VERIFY_HEAP (prefinalizer 期 slot 活则对象活)。

/// v8: 原版 pointer-policies.cc:24-29 — DEBUG 专属: ptr ∈ [当前栈指针, 栈起点)
#[cfg(debug_assertions)]
fn is_on_stack(address: *const u8) -> bool {
    let cur = Stack::get_current_stack_position();
    (cur <= address) && (address < Stack::get_stack_start())
}

impl SameThreadEnabledCheckingPolicyBase {
    /// 原版 pointer-policies.cc:33-106 — 同线程强校验链, 全 DEBUG 断言保留为 debug_assert!
    pub fn check_pointer_impl(
        &mut self,
        ptr: *const u8,
        points_to_payload: bool,
        check_off_heap_assignments: bool,
    ) {
        // 段1 入参形态: 禁栈 / cage 内 / 非 -1 哨兵
        #[cfg(debug_assertions)] debug_assert!(!is_on_stack(ptr));
        // v8: CPPGC_CAGED_HEAP 目标配置开
        debug_assert!(CagedHeapBase::is_within_cage(ptr));
        debug_assert_ne!(ptr as usize, usize::MAX); // -1 哨兵
        let base_page = BasePage::from_payload(ptr);
        // 大对象不支持 mixin → 大对象必指向 payload
        debug_assert!(!base_page.is_large() || points_to_payload);

        // 段2 堆归属: 首次惰性绑定; 引用不可换堆
        let mut is_on_heap = true;
        if self.heap_.is_null() {
            self.heap_ = &mut base_page.heap() as *mut _;
            if self.heap_.page_backend().lookup(self as *const _ as usize).is_none() {
                // this 不在 ptr 的堆内 → 栈上/堆外引用, 不应有注册堆
                is_on_heap = false;
                v8_check!(HeapRegistry::try_from_managed_pointer(self as *const _).is_none());
            }
        }
        // 段3 禁混堆 + 同线程
        debug_assert_eq!(self.heap_, &base_page.heap());
        debug_assert!(self.heap_.current_thread_is_heap_thread());

        // 段4 头校验双臂: payload 去标取头 / mixin 内地址并发取头
        let header: *const HeapObjectHeader = if points_to_payload {
            let untagged = (ptr as usize & !K_ALLOCATION_MASK) as *const u8;
            let h = &HeapObjectHeader::from_object(untagged);
            debug_assert_eq!(h as *const _, &base_page.object_header_from_inner_address::<AccessMode::Atomic>(ptr));
            h
        } else {
            let h = &base_page.object_header_from_inner_address::<AccessMode::Atomic>(ptr);
            debug_assert!(h.object_start() <= ptr); // DCHECK_LE
            debug_assert!(h.object_end::<AccessMode::Atomic>() > ptr); // DCHECK_GT
            h
        };
        debug_assert!(!(*header).is_free());

        // 段5 CPPGC_VERIFY_HEAP — prefinalizer 期 slot 活 ⇒ 对象活
        // v8: 目标配置默认关; 原版 pointer-policies.cc:88-105
        #[cfg(feature = "cppgc-verify-heap")]
        if check_off_heap_assignments || is_on_heap {
            if self.heap_.prefinalizer_handler().is_invoking_pre_finalizers() {
                let slot_page = BasePage::from_inner_address(self.heap_, self as *const _ as *mut _);
                let slot_is_live = slot_page.is_none()
                    || slot_page.unwrap().object_header_from_inner_address(self as *const _ as *mut _).is_marked();
                debug_assert!(!slot_is_live || (*header).is_marked());
            }
        }
    }
}

// ══ 4 个 Persistent 策略 region 取用 (原版 pointer-policies.cc:108-130) ══

/// StrongPersistentPolicy — 原版 :108-111
pub fn strong_persistent_region(object: *const u8) -> &'static mut PersistentRegion {
    BasePage::from_payload(object).heap().get_strong_persistent_region()
}
/// WeakPersistentPolicy — 原版 :113-116
pub fn weak_persistent_region(object: *const u8) -> &'static mut PersistentRegion {
    BasePage::from_payload(object).heap().get_weak_persistent_region()
}
/// StrongCrossThreadPersistentPolicy — 原版 :118-123
pub fn strong_cross_thread_persistent_region(object: *const u8) -> &'static mut CrossThreadPersistentRegion {
    BasePage::from_payload(object).heap().get_strong_cross_thread_persistent_region()
}
/// WeakCrossThreadPersistentPolicy — 原版 :125-130
pub fn weak_cross_thread_persistent_region(object: *const u8) -> &'static mut CrossThreadPersistentRegion {
    BasePage::from_payload(object).heap().get_weak_cross_thread_persistent_region()
}
`;
const out = writeSrc("v8-heap-cppgc", "pointer_policies.rs.skel", body);
console.log(`OK pointer-policies.cc.js: 锚 ${n} 条 (校验链14+region 计数1+4链); 4 策略; 产物 ${out}`);
