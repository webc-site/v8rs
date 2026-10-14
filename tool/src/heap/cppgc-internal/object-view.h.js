#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/object-view.h.js — v8/src/heap/cppgc-internal/object-view.h → v8-heap-cppgc/src/object_view.rs.skel
// 职责: ObjectView<AccessMode> — 对象边界内访问头, Start/End/Size 三查询单一所有者字段三元组
// 规则:
//   ① 模板参数 AccessMode 默认 kNonAtomic → Rust const generic, 3 私有字段 (header/base_page/is_large_object)
//   ② End/Size 双臂: 大对象走 LargePage::From(base_page_), 否则走 header (End=ObjectEnd, Size=ObjectSize)
//   ③ 构造 DCHECK_EQ(Start()+Size(), End()) 不变式
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/object-view.h";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};
const count = (re) => (src.match(re) || []).length;

need("template <AccessMode = AccessMode::kNonAtomic>", "默认模板参数");
need("class ObjectView final {", "类声明");
need("V8_INLINE explicit ObjectView(const HeapObjectHeader& header);", "构造声明");
need("V8_INLINE Address Start() const;", "Start 声明");
need("V8_INLINE ConstAddress End() const;", "End 声明");
need("V8_INLINE size_t Size() const;", "Size 声明");
need("is_large_object_ ? LargePage::From(base_page_)->PayloadEnd()", "End 大对象臂");
need(": header_.ObjectEnd<access_mode>();", "End 普通臂");
need("is_large_object_ ? LargePage::From(base_page_)->ObjectSize()", "Size 大对象臂");
need(": header_.ObjectSize<access_mode>();", "Size 普通臂");
need("DCHECK_EQ(Start() + Size(), End());", "构造不变式");
if (count(/^template <AccessMode access_mode>$/gm) !== 4)
  throw new Error(`成员模板定义计数: got=${count(/^template <AccessMode access_mode>$/gm)} want=4`);
n++;

const body = `${banner("src/heap/cppgc-internal/object-view.h.js", REL, "ObjectView<AccessMode> 对象边界访问器 — Start/End/Size 三查询")}
// ══ ObjectView (原版 object-view.h:19-57) ══
// 模板默认 AccessMode::kNonAtomic; 不保活对象, 只在对象边界内取头。
// 4 个成员 (ctor/Start/End/Size) 均 V8_INLINE → Rust 侧直接内联语义。

pub struct ObjectView<const MODE: AccessMode = AccessMode::NonAtomic> {
    header: &'static HeapObjectHeader,
    base_page: *const BasePage,
    is_large_object: bool, // header_.IsLargeObject<access_mode>()
}

impl<const MODE: AccessMode> ObjectView<MODE> {
    /// 原版 object-view.h:33-40 — 构造: 页回溯 + 大对象判定, 不变式 Start+Size==End
    pub fn new(header: &HeapObjectHeader) -> Self {
        let base_page = BasePage::from_payload(header as *const HeapObjectHeader as *mut _);
        let is_large_object = header.is_large_object::<MODE>();
        let v = Self { header, base_page, is_large_object };
        debug_assert_eq!(v.start() + v.size(), v.end() as usize);
        v
    }

    /// 原版 object-view.h:42-45 — Start = header_.ObjectStart()
    pub fn start(&self) -> *mut u8 {
        self.header.object_start()
    }

    /// 原版 object-view.h:47-51 — End: 大对象 PayloadEnd / 普通对象 ObjectEnd
    pub fn end(&self) -> *const u8 {
        if self.is_large_object {
            LargePage::from(self.base_page).payload_end()
        } else {
            self.header.object_end::<MODE>()
        }
    }

    /// 原版 object-view.h:53-57 — Size: 大对象 LargePage::ObjectSize / 普通对象 ObjectSize
    pub fn size(&self) -> usize {
        if self.is_large_object {
            LargePage::from(self.base_page).object_size()
        } else {
            self.header.object_size::<MODE>()
        }
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "object_view.rs.skel", body);
console.log(`OK object-view.h.js: 锚 ${n} 条 (签名11+成员定义计数1); 双臂 2×(End/Size); 产物 ${out}`);
