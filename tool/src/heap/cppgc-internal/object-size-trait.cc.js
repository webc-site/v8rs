#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/object-size-trait.cc.js — v8/src/heap/cppgc-internal/object-size-trait.cc → v8-heap-cppgc/src/object_size_trait.rs.skel
// 职责: BaseObjectSizeTrait 两个静态查询 — GC 对象尺寸 / mixin 内地址回溯尺寸
// 规则:
//   ① GetObjectSizeForGarbageCollected: ObjectView<kAtomic>(HeapObjectHeader::FromObject).Size()
//   ② GetObjectSizeForGarbageCollectedMixin: BasePage::FromPayload → ObjectHeaderFromInnerAddress<kAtomic>, DCHECK 非大对象, 取 ObjectSize<kAtomic>
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/object-size-trait.cc";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need("size_t BaseObjectSizeTrait::GetObjectSizeForGarbageCollected(", "GC 对象尺寸定义");
need("size_t BaseObjectSizeTrait::GetObjectSizeForGarbageCollectedMixin(", "mixin 尺寸定义");
need("ObjectView<AccessMode::kAtomic>(HeapObjectHeader::FromObject(object))", "GC 臂视图链");
need("BasePage::FromPayload(address)", "mixin 臂页回溯");
need("->ObjectHeaderFromInnerAddress<AccessMode::kAtomic>(address);", "内地址取头");
need("DCHECK(!header.IsLargeObject<AccessMode::kAtomic>());", "非大对象断言");
need("return header.ObjectSize<AccessMode::kAtomic>();", "mixin 臂取尺寸");
// 大对象 mixin 不支持 — 注释锚
need("large object mixins\n  // are not supported", "mixin 不支持大对象注释");

const body = `${banner("src/heap/cppgc-internal/object-size-trait.cc.js", REL, "BaseObjectSizeTrait 两静态尺寸查询 (声明面在 include/cppgc/object-size-trait.h)")}
// ══ BaseObjectSizeTrait (原版 object-size-trait.cc:15-31) ══
// 两臂均 AccessMode::kAtomic (并发安全读)。

pub struct BaseObjectSizeTrait;

impl BaseObjectSizeTrait {
    /// 原版 object-size-trait.cc:15-19 — GC 对象: 头 → 视图 → 尺寸
    pub fn get_object_size_for_garbage_collected(object: *const u8) -> usize {
        let header = HeapObjectHeader::from_object(object);
        ObjectView::<AccessMode::Atomic>::new(&header).size()
    }

    /// 原版 object-size-trait.cc:22-31 — mixin 内地址: 页回溯头, 断言非大对象
    /// v8: 原版 DCHECK(!header.IsLargeObject<kAtomic>()) — 大对象 mixin 不支持
    pub fn get_object_size_for_garbage_collected_mixin(address: *const u8) -> usize {
        let page = BasePage::from_payload(address);
        let header = page.object_header_from_inner_address::<AccessMode::Atomic>(address);
        debug_assert!(!header.is_large_object::<AccessMode::Atomic>());
        header.object_size::<AccessMode::Atomic>()
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "object_size_trait.rs.skel", body);
console.log(`OK object-size-trait.cc.js: 锚 ${n} 条 (定义2+视图链1+页回溯1+内地址1+断言1+取尺寸1+注释1); 产物 ${out}`);
