#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/compaction-worklists.js — v8/src/heap/cppgc-internal/compaction-worklists.h + .cc → v8-heap-cppgc/src/compaction_worklists.rs.skel
// 压缩工作列表: MovableReference 别名 + 256 槽位 worklist + ClearForTesting
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/compaction-worklists.h";
const REL_CC = "src/heap/cppgc-internal/compaction-worklists.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "class CompactionWorklists final {", "CompactionWorklists 声明");
need(src, "using MovableReference = const void*;", "MovableReference");
need(src, "heap::base::Worklist<MovableReference*, 256 /* local entries */>;", "Worklist 模板实参 256");
need(src, "CompactionWorklists(const CompactionWorklists&) = delete;", "拷贝删除");
need(src, "MovableReferencesWorklist* movable_slots_worklist() {", "movable_slots_worklist 访问器");
need(src, "void ClearForTesting();", "ClearForTesting 声明");
need(src, "MovableReferencesWorklist movable_slots_worklist_;", "成员字段");
// .cc 侧: 唯一方法实体
need(srcCc, "void CompactionWorklists::ClearForTesting() { movable_slots_worklist_.Clear(); }", "ClearForTesting 实体");
need(srcCc, '#include "src/heap/cppgc-internal/compaction-worklists.h"', "include 自引");

const body = `${banner("src/heap/cppgc-internal/compaction-worklists.js", `${REL} + ${REL_CC}`, "可移动引用槽位工作列表")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/compaction-worklists.h CompactionWorklists::MovableReference
pub type MovableReference = *const core::ffi::c_void;

/// v8: src/heap/cppgc-internal/compaction-worklists.h CompactionWorklists::MovableReferencesWorklist
/// heap::base::Worklist<MovableReference*, 256> — 每本地段 256 项
pub type MovableReferencesWorklist = Worklist<*mut MovableReference, 256>;

/// v8: src/heap/cppgc-internal/compaction-worklists.h CompactionWorklists
#[derive(Default)]
pub struct CompactionWorklists {
    /// v8: movable_slots_worklist_
    movable_slots_worklist: MovableReferencesWorklist,
}

impl CompactionWorklists {
    /// v8: src/heap/cppgc-internal/compaction-worklists.h CompactionWorklists::movable_slots_worklist
    pub fn movable_slots_worklist(&mut self) -> &mut MovableReferencesWorklist {
        &mut self.movable_slots_worklist
    }

    /// v8: src/heap/cppgc-internal/compaction-worklists.cc CompactionWorklists::ClearForTesting
    pub fn clear_for_testing(&mut self) {
        self.movable_slots_worklist.clear();
    }
}
`;

writeSrc("v8-heap-cppgc", "compaction_worklists.rs.skel", body);
console.log(`compaction-worklists.js: 对数 ${n} 项 (.h 8 + .cc 2) → v8-heap-cppgc/src/compaction_worklists.rs.skel`);
