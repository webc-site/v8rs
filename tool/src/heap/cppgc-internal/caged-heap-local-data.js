#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/caged-heap-local-data.js — v8/src/heap/cppgc-internal/caged-heap-local-data.cc → v8-heap-cppgc/src/caged_heap_local_data.rs.skel
// AgeTable 卡表年龄写入门面: 内卡整龄 + 外卡混合龄 (CPPGC_YOUNG_GENERATION)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/caged-heap-local-data.cc";
const src = readV8(REL);

let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`caged-heap-local-data.cc 缺锚 ${label}: ${needle}`);
  n++;
};

need("#if defined(CPPGC_YOUNG_GENERATION)", "年轻代条件");
need("std::is_trivially_default_constructible_v<AgeTable>,", "平凡默认构造断言");
need("void AgeTable::SetAgeForRange(uintptr_t offset_begin, uintptr_t offset_end,", "SetAgeForRange 签名");
need("AdjacentCardsPolicy adjacent_cards_policy) {", "相邻卡策略参数");
need("const uintptr_t inner_card_offset_begin =", "内卡下界");
need("const uintptr_t outer_card_offset_end =", "外卡上界");
need("AdjacentCardsPolicy::kIgnore", "kIgnore 分支");
need("AgeTable::Age::kMixed", "kMixed 混合龄");
need("set_age_for_outer_card(offset_begin);", "外卡回写 begin");
need("set_age_for_outer_card(offset_end);", "外卡回写 end");
need("AgeTable::Age AgeTable::GetAgeForRange(uintptr_t offset_begin,", "GetAgeForRange 签名");
need("if (result != GetAge(offset)) result = Age::kMixed;", "巡卡混合判定");
need("void AgeTable::ResetForTesting() {", "ResetForTesting 签名");
need('std::fill(&table_[0], &table_[CagedHeapBase::GetAgeTableSize()], Age::kOld);', "整表刷 kOld");

const body = `${banner("src/heap/cppgc-internal/caged-heap-local-data.js", REL, "AgeTable 区间年龄操作 (年轻代)")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/caged-heap-local-data.cc AgeTable (CPPGC_YOUNG_GENERATION cfg)
#[cfg(feature = "cppgc_young_generation")]
pub mod age_table {
    /// v8: AgeTable::Age — kOld / kYoung / kMixed (定义于 include/cppgc/internal/caged-heap-local-data.h)
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Age {
        Old,
        Young,
        Mixed,
    }

    /// v8: AgeTable::AdjacentCardsPolicy
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum AdjacentCardsPolicy {
        /// v8: kIgnore — 外卡直接设目标龄
        Ignore,
        /// v8: kConsider — 外卡与现龄不同则设 kMixed
        Consider,
    }

    pub struct AgeTable {
        /// v8: table_ — 按 kCardSizeInBytes 分卡
        pub table: Vec<Age>,
    }

    impl AgeTable {
        /// v8: src/heap/cppgc-internal/caged-heap-local-data.cc AgeTable::SetAgeForRange
        /// 内卡 [RoundUp(begin), RoundDown(end)) 整卡设龄;
        /// 外卡: 对齐则跳过; kIgnore 直设; 否则与现龄不同 → kMixed。
        pub fn set_age_for_range(
            &mut self,
            offset_begin: usize,
            offset_end: usize,
            age: Age,
            adjacent_cards_policy: AdjacentCardsPolicy,
        ) {
            let inner_begin = round_up(offset_begin, K_CARD_SIZE_IN_BYTES);
            let outer_end = round_down(offset_end, K_CARD_SIZE_IN_BYTES);
            let mut off = inner_begin;
            while off < outer_end {
                self.set_age(off, age);
                off += K_CARD_SIZE_IN_BYTES;
            }
            for off in [offset_begin, offset_end] {
                self.set_age_for_outer_card(off, age, adjacent_cards_policy);
            }
        }

        /// v8: set_age_for_outer_card — IsAligned 跳过; kIgnore 直设; 异龄 kMixed
        fn set_age_for_outer_card(&mut self, offset: usize, age: Age, policy: AdjacentCardsPolicy) {
            if is_aligned(offset, K_CARD_SIZE_IN_BYTES) { return; }
            match policy {
                AdjacentCardsPolicy::Ignore => self.set_age(offset, age),
                AdjacentCardsPolicy::Consider if self.get_age(offset) != age =>
                    self.set_age(offset, Age::Mixed),
                AdjacentCardsPolicy::Consider => {}
            }
        }

        /// v8: src/heap/cppgc-internal/caged-heap-local-data.cc AgeTable::GetAgeForRange
        /// 全程一致返回该龄, 出现异龄即 kMixed。
        pub fn get_age_for_range(&self, offset_begin: usize, offset_end: usize) -> Age {
            let mut result = self.get_age(offset_begin);
            let mut off = offset_begin + K_CARD_SIZE_IN_BYTES;
            while off < offset_end {
                if result != self.get_age(off) { result = Age::Mixed; }
                off += K_CARD_SIZE_IN_BYTES;
            }
            result
        }

        /// v8: src/heap/cppgc-internal/caged-heap-local-data.cc AgeTable::ResetForTesting
        pub fn reset_for_testing(&mut self) {
            self.table.fill(Age::Old);
        }

        /// v8: AgeTable::SetAge / GetAge — 单卡读写 (定义于 include 头, 语义锚)
        fn set_age(&mut self, _offset: usize, _age: Age) { todo!("单卡写 (含 cage base 换算)") }
        fn get_age(&self, _offset: usize) -> Age { todo!("单卡读 (含 cage base 换算)") }
    }
}
`;

writeSrc("v8-heap-cppgc", "caged_heap_local_data.rs.skel", body);
console.log(`caged-heap-local-data.js: 对数 ${n} 项 (条件 2 + 签名 3 + 分支逻辑 9) → v8-heap-cppgc/src/caged_heap_local_data.rs.skel`);
