#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/member-storage.js — v8/src/heap/cppgc-internal/member-storage.h + .cc → v8-heap-cppgc/src/member_storage.rs.skel
// 压缩指针笼基: CageBaseGlobalUpdater (整表 CPPGC_POINTER_COMPRESSION) + 调试解压出口
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/member-storage.h";
const REL_CC = "src/heap/cppgc-internal/member-storage.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "#if defined(CPPGC_POINTER_COMPRESSION)", "压缩指针条件");
need(src, "class CageBaseGlobalUpdater final {", "CageBaseGlobalUpdater 声明");
need(src, "CageBaseGlobalUpdater() = delete;", "构造删除");
need(src, "static void UpdateCageBase(uintptr_t cage_base) {", "UpdateCageBase");
need(src, "CPPGC_DCHECK(CageBaseGlobal::IsBaseConsistent());", "基一致性检查");
need(src, "CPPGC_DCHECK(0u == (cage_base & CageBaseGlobal::kLowerHalfWordMask));", "低半字掩码对齐断言");
need(src, "cage_base | CageBaseGlobal::kLowerHalfWordMask;", "置低位标签");
need(src, "static uintptr_t GetCageBase() {", "GetCageBase");
need(src, "return CageBaseGlobal::g_base_.base & ~CageBaseGlobal::kLowerHalfWordMask;", "去标签还原");
// .cc
need(srcCc, "alignas(api_constants::kCachelineSize) CageBaseGlobal::Base", "缓存行对齐定义");
need(srcCc, "CageBaseGlobal::g_base_ = {CageBaseGlobal::kLowerHalfWordMask};", "初值带标签");
need(srcCc, "void*\n_cppgc_internal_Decompress_Compressed_Pointer(uint32_t cmprsd) {", "C 解压出口");
need(srcCc, "return CompressedPointer::Decompress(cmprsd);", "解压委托");
need(srcCc, "class MemberDebugHelper final {", "MemberDebugHelper");
need(srcCc, "return const_cast<void*>(m->GetRaw());", "GetRaw 裸值");
need(srcCc, "_cppgc_internal_Uncompress_Member(", "C 成员解压出口");

const body = `${banner("src/heap/cppgc-internal/member-storage.js", `${REL} + ${REL_CC}`, "压缩指针笼基全局 + 调试出口")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/member-storage.h CageBaseGlobalUpdater (CPPGC_POINTER_COMPRESSION cfg)
#[cfg(feature = "cppgc_pointer_compression")]
pub mod cage_base_global_updater {
    /// v8: UpdateCageBase — 写 g_base_: base | kLowerHalfWordMask (低位标签防 0 判空)
    pub fn update_cage_base(cage_base: usize) {
        debug_assert!(CageBaseGlobal::is_base_consistent());
        debug_assert_eq!(0, cage_base & CageBaseGlobal::K_LOWER_HALF_WORD_MASK);
        CageBaseGlobal::g_base().base = cage_base | CageBaseGlobal::K_LOWER_HALF_WORD_MASK;
    }

    /// v8: GetCageBase — 读 g_base_ 去标签
    pub fn get_cage_base() -> usize {
        debug_assert!(CageBaseGlobal::is_base_consistent());
        CageBaseGlobal::g_base().base & !CageBaseGlobal::K_LOWER_HALF_WORD_MASK
    }
}

/// v8: src/heap/cppgc-internal/member-storage.cc CageBaseGlobal::g_base_
/// alignas(kCachelineSize), 初值 {kLowerHalfWordMask}。
/// v8: src/heap/cppgc-internal/member-storage.cc _cppgc_internal_Decompress_Compressed_Pointer
/// extern "C" 调试出口 → CompressedPointer::Decompress。
/// v8: src/heap/cppgc-internal/member-storage.cc MemberDebugHelper::Uncompress
/// extern "C" _cppgc_internal_Uncompress_Member → MemberBase::GetRaw。
#[cfg(feature = "cppgc_pointer_compression")]
pub mod debug_exports {
    /// v8: _cppgc_internal_Decompress_Compressed_Pointer
    pub fn decompress_compressed_pointer(cmprsd: u32) -> *mut core::ffi::c_void {
        CompressedPointer::decompress(cmprsd)
    }
    /// v8: _cppgc_internal_Uncompress_Member
    pub fn uncompress_member(m: &MemberBase) -> *mut core::ffi::c_void {
        m.get_raw() as *mut core::ffi::c_void
    }
}
`;

writeSrc("v8-heap-cppgc", "member_storage.rs.skel", body);
console.log(`member-storage.js: 对数 ${n} 项 (.h 9 + .cc 6) → v8-heap-cppgc/src/member_storage.rs.skel`);
