#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/raw-heap.h.js — v8/src/heap/cppgc-internal/{raw-heap.h,raw-heap.cc} → v8-heap-cppgc/src/raw_heap.rs.skel
// 职责: RawHeap 空间管理 — RegularSpaceType 5 值枚举 + 4 常规页空间 1 大页空间构造 + 自定义空间尾部追加
// 规则:
//   ① 枚举 kNormal1..kNormal4, kLarge; kNumberOfRegularSpaces = kLarge+1 = 5
//   ② 构造序: kNormal1..kNormal4 → NormalPageSpace(compactable=false); kLarge → LargePageSpace; custom 尾接 NormalPageSpace(带 IsCompactable)
//   ③ 空间定位: RegularSpaceType → 下标; custom → kNumberOfRegularSpaces + CustomSpaceIndex.value
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-internal/raw-heap.h";
const REL_CC = "src/heap/cppgc-internal/raw-heap.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE RawHeap final {", "类声明");
need(src, "enum class RegularSpaceType : uint8_t {", "枚举声明");
for (const v of ["kNormal1", "kNormal2", "kNormal3", "kNormal4", "kLarge"]) {
  need(src, `${v},`, `枚举值 ${v}`);
}
need(src, "static constexpr size_t kNumberOfRegularSpaces =", "常规空间数常量");
need(src, "static_cast<size_t>(RegularSpaceType::kLarge) + 1;", "常量式");
need(src, "using Spaces = std::vector<std::unique_ptr<BaseSpace>>;", "容器别名");
need(src, "iterator custom_begin() { return std::next(begin(), kNumberOfRegularSpaces); }", "自定义区起点");
need(src, "return kNumberOfRegularSpaces + space_index.value;", "自定义下标式");
need(src, "DCHECK_GT(kNumberOfRegularSpaces, index);", "常规下标界检查");
need(cc, "for (; i < static_cast<size_t>(RegularSpaceType::kLarge); ++i) {", "常规空间循环");
need(cc, "spaces_.push_back(std::make_unique<NormalPageSpace>(this, i, false));", "常规空间构造");
need(cc, "spaces_.push_back(std::make_unique<LargePageSpace>(", "大页空间构造");
need(cc, "DCHECK_EQ(kNumberOfRegularSpaces, spaces_.size());", "构造后数量断言");
need(cc, "this, kNumberOfRegularSpaces + j, custom_spaces[j]->IsCompactable())", "自定义空间构造");
need(cc, "constexpr size_t RawHeap::kNumberOfRegularSpaces;", "常量 ODR");

// 枚举值序核对
const enumVals = [...src.matchAll(/^    (k\w+),$/gm)].map((m) => m[1]);
if (enumVals.join(",") !== "kNormal1,kNormal2,kNormal3,kNormal4,kLarge")
  throw new Error(`枚举值序: got=${enumVals.join(",")}`);
n++;
const NUM = enumVals.length; // 5

const body = `${banner("src/heap/cppgc-internal/raw-heap.h.js", REL_H, "RawHeap 空间管理 — 常规/大页/自定义空间容器与定位 (cc 构造一并落地)")}
// ══ RegularSpaceType (原版 raw-heap.h:35-41) ══
// 常规空间按尺寸分箱: kNormal1 <32B, kNormal2 <64B, kNormal3 <128B, kNormal4 >=128B; 超过 2^16 走大空间。

/// 原版 raw-heap.h:35-41 — uint8_t 底
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RegularSpaceType {
    Normal1 = 0,
    Normal2 = 1,
    Normal3 = 2,
    Normal4 = 3,
    Large = 4,
}

/// kNumberOfRegularSpaces = kLarge + 1 = 5 — 常规空间总数 (含大空间)
pub const K_NUMBER_OF_REGULAR_SPACES: usize = RegularSpaceType::Large as usize + 1;
static_assertions::const_assert_eq!(K_NUMBER_OF_REGULAR_SPACES, ${NUM});

// ══ RawHeap (原版 raw-heap.h:23-106, 构造 raw-heap.cc:15-30) ══

pub struct RawHeap {
    main_heap: *mut HeapBase,
    /// 布局: [0, K_NUMBER_OF_REGULAR_SPACES) 常规, 之后自定义
    spaces: Vec<Box<dyn BaseSpace>>,
}

impl RawHeap {
    /// 原版 raw-heap.cc:15-30 — 先 4 常规 (非紧凑) + 1 大页, DCHECK 5, 再追加自定义 (带紧凑标志)
    pub fn new(heap: *mut HeapBase, custom_spaces: &[Box<dyn CustomSpaceBase>]) -> Self {
        let mut spaces: Vec<Box<dyn BaseSpace>> = Vec::new();
        let mut i = 0usize;
        while i < RegularSpaceType::Large as usize {
            spaces.push(Box::new(NormalPageSpace::new(/*raw_heap*/ unsafe { &*heap }, i, /*is_compactable*/ false)));
            i += 1;
        }
        spaces.push(Box::new(LargePageSpace::new(unsafe { &*heap }, RegularSpaceType::Large as usize)));
        debug_assert_eq!(K_NUMBER_OF_REGULAR_SPACES, spaces.len());
        for (j, custom) in custom_spaces.iter().enumerate() {
            spaces.push(Box::new(NormalPageSpace::new(
                unsafe { &*heap },
                K_NUMBER_OF_REGULAR_SPACES + j,
                custom.is_compactable(),
            )));
        }
        Self { main_heap: heap, spaces }
    }

    /// 原版 raw-heap.h:69-73 — 枚举型定位: DCHECK 界内后转下标
    pub fn space(&self, type_: RegularSpaceType) -> &BaseSpace {
        let index = type_ as usize;
        debug_assert!(K_NUMBER_OF_REGULAR_SPACES > index);
        self.space_at(index)
    }

    /// 原版 raw-heap.h:78-80 — 自定义空间定位: 常规数 + value
    pub fn custom_space(&self, space_index: CustomSpaceIndex) -> &BaseSpace {
        self.space_at(self.space_index_for_custom_space(space_index))
    }

    /// 原版 raw-heap.h:89-92 — DCHECK 自定义区界内
    fn space_index_for_custom_space(&self, space_index: CustomSpaceIndex) -> usize {
        debug_assert!((space_index.value as usize) < self.spaces.len() - K_NUMBER_OF_REGULAR_SPACES);
        K_NUMBER_OF_REGULAR_SPACES + space_index.value as usize
    }

    /// 原版 raw-heap.h:94-99 — 下标型定位: DCHECK 界内 + 非空
    fn space_at(&self, space_index: usize) -> &BaseSpace {
        debug_assert!(self.spaces.len() > space_index);
        self.spaces[space_index].as_ref()
    }

    pub fn heap(&self) -> *mut HeapBase {
        self.main_heap // 原版 raw-heap.h:85
    }

    pub fn size(&self) -> usize {
        self.spaces.len() // 原版 raw-heap.h:67
    }

    /// 原版 raw-heap.h:64 — 自定义区起点 = 常规数
    pub fn custom_begin(&self) -> std::slice::Iter<'_, Box<dyn BaseSpace>> {
        self.spaces[K_NUMBER_OF_REGULAR_SPACES..].iter()
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "raw_heap.rs.skel", body);
console.log(`OK raw-heap.h.js: 锚 ${n} 条 (h12+cc6+枚举序1); ${NUM} 枚举值; 产物 ${out}`);
