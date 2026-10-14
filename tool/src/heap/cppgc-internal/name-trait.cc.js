#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/name-trait.cc.js — v8/src/heap/cppgc-internal/name-trait.cc → v8-heap-cppgc/src/name_trait.rs.skel
// 职责: NameProvider 双哨兵名常量落地 + NameTraitBase::GetNameFromTypeSignature 的 "T = " 签名解析骨架
// 规则:
//   ① 2 个 constexpr 常量定义 (kHiddenName/kNoNameDeducible), 值定义在 include/cppgc/name-provider.h, 本文件只落地 ODR 定义
//   ② GetNameFromTypeSignature: 无签名 → kNoNameDeducible; 有签名 → rfind("T = ")+4 起, 去尾 ']' (len-1)
//   ③ 原版 new char[] 拷贝返回 → Rust 侧返回 String 归属权, 免手工释放 (语义面留锚)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/name-trait.cc";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};
const count = (re) => (src.match(re) || []).length;

need('constexpr const char NameProvider::kHiddenName[];', "kHiddenName ODR 定义");
need('constexpr const char NameProvider::kNoNameDeducible[];', "kNoNameDeducible ODR 定义");
need("HeapObjectName NameTraitBase::GetNameFromTypeSignature(const char* signature) {", "解析函数定义");
need('raw.rfind("T = ") + 4', "T = 定位式");
need("raw.length() - start_pos - 1", "去尾 ] 长度式");
need("if (!signature) return {NameProvider::kNoNameDeducible, false};", "空签名臂");
if (count(/\/\/ static/g) !== 3) throw new Error(`// static 计数: got=${count(/\/\/ static/g)} want=3`);
n++;

// 值取声明头 include/cppgc/name-provider.h 的字面量 (只读核对, 不改)
const declRel = "include/cppgc/name-provider.h";
const decl = readV8(declRel);
const hidden = decl.match(/kHiddenName\[\] = "([^"]*)"/);
const noName = decl.match(/kNoNameDeducible\[\] = "([^"]*)"/);
if (!hidden || !noName) throw new Error("声明头哨兵名值未命中");
n += 2;

const body = `${banner("src/heap/cppgc-internal/name-trait.cc.js", REL, "NameProvider 哨兵名 ODR 落地 + T=签名解析骨架 (声明面在 include/cppgc/name-provider.h)")}
// ══ NameProvider 哨兵名 (原版 name-trait.cc:16-19, 值来自 include/cppgc/name-provider.h) ══

/// NameProvider::kHiddenName — 隐藏名哨兵 (脱敏模式用)
pub const K_HIDDEN_NAME: &str = "${hidden[1]}";

/// NameProvider::kNoNameDeducible — 无法推导名哨兵
pub const K_NO_NAME_DEDUCIBLE: &str = "${noName[1]}";

// ══ NameTraitBase::GetNameFromTypeSignature (原版 name-trait.cc:24-38) ══
// 输入形如: static HeapObjectName NameTrait<int>::GetNameFor(...) [T = int]
// 输出 HeapObjectName { name, was_hidden: false }。

/// 解析 "[T = <名>]" 签名 → 拥有权的名字; 原版 new char[] 拷贝, Rust 侧 String 归属权免释放。
/// v8: 原版 DCHECK_NE(npos, start_pos) — 签名必含 "T = ", Rust 侧 panic 对应
pub fn get_name_from_type_signature(signature: Option<&str>) -> (String, bool) {
    let Some(signature) = signature else {
        // 原版 name-trait.cc:27 空/空签名臂
        return (K_NO_NAME_DEDUCIBLE.to_string(), false);
    };
    // 原版 name-trait.cc:30-32: rfind("T = ")+4 起, 长度 = len - start - 1 (剥尾 ']')
    let start = signature.rfind("T = ").expect("签名必含 \\"T = \\"") + 4;
    let end = signature.len() - 1;
    (signature[start..end].to_string(), false)
}
`;
const out = writeSrc("v8-heap-cppgc", "name_trait.rs.skel", body);
console.log(`OK name-trait.cc.js: 锚 ${n} 条 (ODR 定义2+函数定义1+定位式1+长度式1+空臂1+static 计数3+声明头值2); 哨兵 "${hidden[1]}"/"${noName[1]}"; 产物 ${out}`);
