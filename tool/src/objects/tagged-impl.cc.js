#!/usr/bin/env bun
// tool/src/objects/tagged-impl.cc.js — TaggedImpl 实现文件面生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/tagged-impl.cc + tagged-impl-inl.h → v8-objects-core/src/tagged_impl.rs
//   (tagged-impl.h 声明面归 tool/src/objects/tagged-impl.h.js → tagged.rs.skel; 本脚本只登记 -inl 与 .cc)
// 规则面: -inl 的 17 个 GetHeapObject/ToSmi 家族定义 + .cc 的打印面与显式实例化;
//   压缩分支 (kIsFull=false) 语义留 doc; 打印函数留签名骨架 todo!
import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/objects/tagged-impl.cc");
const inl = readV8("src/objects/tagged-impl-inl.h");

let checks = 0;
const need = (cond, msg) => { checks++; if (!cond) throw new Error(msg); };

// —— -inl 定义清单 (定义顺序) ——
const inlDefs = [...inl.matchAll(/TaggedImpl<kRefType, StorageType>::(\w+)\(/g)].map((m) => m[1]);
// GetHeapObject 4 重载 + IfStrong 2 + AssumeStrong 2 + IfWeak 2 + AssumeWeak 2 + GetHeapObjectOrSmi 2 + ToSmi 2 + cage 判定 2 (压缩分支)
const EXPECT = ["IsInMainCageBase", "IsInTrustedCageBase", "ToSmi", "ToSmi", "GetHeapObject", "GetHeapObject",
  "GetHeapObject", "GetHeapObject", "GetHeapObjectIfStrong", "GetHeapObjectIfStrong", "GetHeapObjectAssumeStrong",
  "GetHeapObjectAssumeStrong", "GetHeapObjectIfWeak", "GetHeapObjectIfWeak", "GetHeapObjectAssumeWeak",
  "GetHeapObjectAssumeWeak", "GetHeapObject", "GetHeapObject", "GetHeapObjectOrSmi", "GetHeapObjectOrSmi"];
if (inlDefs.join() !== EXPECT.join())
  throw new Error(`tagged-impl-inl.h 定义序漂移 (应 ${EXPECT.length}):\n  ${inlDefs}`);
// 压缩分支守卫计数
const comprGuards = (inl.match(/#ifdef V8_COMPRESS_POINTERS/g) || []).length;
need(comprGuards >= 2, `V8_COMPRESS_POINTERS 守卫应 ≥2, 实得 ${comprGuards}`);

// —— -inl 位运算锚 (非压缩主路径) ——
need(/return Cast<HeapObject>\(Tagged<Object>\(ptr_ & ~kWeakHeapObjectMask\)\);/.test(inl), "GetHeapObject 弱位清除位运算缺失");
need(/CHECK\(kIsFull\);/.test(inl), "kIsFull CHECK 守卫缺失");
need(/\*reference_type = IsWeakOrCleared\(\) \? HeapObjectReferenceType::WEAK\s*\n\s*: HeapObjectReferenceType::STRONG;/.test(inl),
  "reference_type 判定漂移");

// —— .cc 面 ——
need(cc.includes("bool CheckObjectComparisonAllowed(Address a, Address b)"), "CheckObjectComparisonAllowed 缺失");
need(/#if defined\(V8_EXTERNAL_CODE_SPACE\) \|\| defined\(V8_ENABLE_SANDBOX\)/.test(cc), "比较守卫双条件缺失");
need(cc.includes("TrustedHeapLayout::InCodeSpace(obj_a)") && cc.includes("TrustedHeapLayout::InCodeSpace(obj_b)"), "InCodeSpace 对称检查缺失");
// 打印模板与显式实例化计数
const shortPrints = (cc.match(/void ShortPrint\(/g) || []).length;       // 1 模板 + 3 重载声明体内? 实际: 模板定义 3 个 (FILE*/StringStream/ostream)
const exportInst = (cc.match(/EXPORT_TEMPLATE_DEFINE\(V8_EXPORT_PRIVATE\)/g) || []).length;
need(shortPrints === 9, `ShortPrint 出现应 9 (3 重载定义 × (1 定义 + 2 实例化)), 实得 ${shortPrints}`);
need(exportInst === 10, `显式实例化声明应 10 (ShortPrint 6 + Print 4), 实得 ${exportInst}`);
const printInst = (cc.match(/template class TaggedImpl</g) || []).length;
need(printInst === 2, `模板类显式实例化应 2 (STRONG/WEAK), 实得 ${printInst}`);
// 打印语义锚
need(/os << "Smi: " << std::hex << "0x" << smi\.value\(\);/.test(cc), "Smi 打印格式锚缺失");
need(/os << "\[cleared\]";/.test(cc), "cleared 打印锚缺失");
need(/os << "\[weak\] ";/.test(cc), "weak 打印锚缺失");

const body = `${banner("src/objects/tagged-impl.cc.js", "src/objects/tagged-impl.cc + tagged-impl-inl.h", "非压缩 kIsFull=true 主路径; 声明面在 tagged.rs.skel")}
//! TaggedImpl 实现面 (原版 tagged-impl.cc + tagged-impl-inl.h)
//!
//! 非压缩基线: StorageType=Address, kIsFull=true, 全部 GetHeapObject*
//! 走 CHECK(kIsFull) 主路径; 压缩分支 (DecompressTagged) 登记为 todo!.

// ===== -inl: 取值家族 (定义 ${inlDefs.length} 个, 重载去重见下) =====
/// ToSmi 带出参版 (原版 ToSmi(Tagged<Smi>*): HAS_SMI_TAG 则写值返回 true)
pub fn to_smi_out(/* this, out: &mut Tagged<Smi> */) -> bool {
  todo!("就地修订: HAS_SMI_TAG(ptr_) 判定")
}

/// ToSmi 直取 (非压缩原值包 Tagged<Smi>; V8_ASSUME 已是 Smi)
pub fn to_smi(/* this */) /* -> Tagged<Smi> */ {
  todo!("就地修订: 非压缩 Tagged<Smi>(ptr_); 压缩走 DecompressTaggedSigned")
}

/// GetHeapObject 可失败版 (非强非弱返回 false)
pub fn get_heap_object(/* this, out */) -> bool {
  todo!("就地修订: CHECK(kIsFull); IsStrongOrWeak() 判定")
}

/// GetHeapObject 带引用类型出参 (WEAK 判定: IsWeakOrCleared)
pub fn get_heap_object_with_ref_type(/* this, out, ref_type: &mut HeapObjectReferenceType */) -> bool {
  todo!("就地修订: IsWeakOrCleared ? WEAK : STRONG")
}

/// GetHeapObjectIfStrong (仅强引用时写出)
pub fn get_heap_object_if_strong(/* this, out */) -> bool {
  todo!("就地修订: IsStrong() 判定")
}

/// GetHeapObjectAssumeStrong (弱位直接清零: ptr_ & ~kWeakHeapObjectMask)
pub fn get_heap_object_assume_strong(/* this */) /* -> Tagged<HeapObject> */ {
  todo!("就地修订: kCanBeWeak 时 DCHECK(!IsCleared) 且清弱位")
}

/// GetHeapObjectIfWeak (kCanBeWeak=false 恒 false)
pub fn get_heap_object_if_weak(/* this, out */) -> bool {
  todo!("就地修订: kCanBeWeak && IsWeak() 判定")
}

/// GetHeapObjectAssumeWeak
pub fn get_heap_object_assume_weak(/* this */) /* -> Tagged<HeapObject> */ {
  todo!("就地修订: DCHECK(IsWeak) 后走 get_heap_object")
}

/// GetHeapObject 主形态 (可弱: 清弱位; 不可弱: DCHECK 无弱标记)
pub fn get_heap_object(/* this */) /* -> Tagged<HeapObject> */ {
  todo!("就地修订: kCanBeWeak ? ptr_ & ~kWeakHeapObjectMask : ptr_")
}

/// GetHeapObjectOrSmi (Smi 直返, 否则走 GetHeapObject)
pub fn get_heap_object_or_smi(/* this */) /* -> Tagged<Object> */ {
  todo!("就地修订: IsSmi() ? Tagged<Object>(ptr_) : get_heap_object()")
}

/// 压缩 cage 判定 (原版 IsInMainCageBase/IsInTrustedCageBase, 仅 V8_COMPRESS_POINTERS)
pub fn cage_base_checks(/* this */) -> (bool, bool) {
  todo!("就地修订: 压缩开时比较 GetPtrComprCageBaseAddress(ptr_) 与 scheme::base()")
}

// ===== .cc: 比较合法性检查 =====
/// 对象比较合法性 (原版 CheckObjectComparisonAllowed: 非强标记直放;
/// 双方须同处 code space; 沙箱开时还须同处 trusted space)
///
/// 守卫: \`#if defined(V8_EXTERNAL_CODE_SPACE) || defined(V8_ENABLE_SANDBOX)\`
pub fn check_object_comparison_allowed(a: usize, b: usize) -> bool {
  let _ = (a, b);
  todo!("就地修订: HAS_STRONG_HEAP_OBJECT_TAG 双侧 + TrustedHeapLayout::InCodeSpace 对称 + 沙箱加验 InTrustedSpace")
}

// ===== .cc: 打印面 (3 个 ShortPrint 重载 + OBJECT_PRINT 下 2 个 Print) =====
/// ShortPrint → FILE* (原版经 OFStream << Brief)
pub fn short_print_file(/* ptr, out: *mut FILE */) {
  todo!("就地修订: Brief 单行打印")
}

/// ShortPrint → StringStream 累加器
pub fn short_print_string_stream(/* ptr, accumulator */) {
  todo!("就地修订: ostringstream << Brief 后 Add")
}

/// ShortPrint → std::ostream
pub fn short_print_ostream(/* ptr, os */) {
  todo!("就地修订: os << Brief")
}

/// Print 全打印 (OBJECT_PRINT: Smi "Smi: 0x.. (..)" / "[cleared]" / "[weak] " + HeapObjectPrint / 强引用直印)
pub fn print(/* ptr, os */) {
  todo!("就地修订: ToSmi→IsCleared→IfWeak→IfStrong 四段判定打印")
}

/// .cc 显式实例化登记 (EXPORT_TEMPLATE_DEFINE ${exportInst} 处 + 模板类实例化 STRONG/WEAK ×1)
pub const TAGGED_IMPL_INSTANTIATIONS: [&str; 2] = ["STRONG,Address", "WEAK,Address"];
`;

const p = writeSrc("v8-objects-core", "tagged_impl.rs", body);
console.log(`tagged-impl.cc.js: 已生成 ${p} (must ${checks}: inl 定义 ${inlDefs.length}, ShortPrint ${shortPrints}, 实例化 ${exportInst}+${printInst})`);
