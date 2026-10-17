#!/usr/bin/env bun
// tool/src/strings/char-predicates.cc.js
// 机械提取 v8/src/strings/char-predicates.cc: INTL 慢路径谓词签名骨架
// (快路径表已由 tool/src/strings/char-predicates.h.js 产出 char_predicates.rs, 不重复)

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/strings/char-predicates.cc");

// 前置: INTL 必开
if (!/#ifndef V8_INTL_SUPPORT\n#error Internationalization is expected to be enabled./.test(ccSrc)) {
  throw new Error("char-predicates.cc 前置断言漂移 (INTL 必开)");
}

// 慢路径谓词清单
const slowFns = [...ccSrc.matchAll(/^bool (Is\w+Slow)\(base::uc32 c\) \{/gm)].map((m) => m[1]);
if (slowFns.join() !== "IsIdentifierStartSlow,IsIdentifierPartSlow,IsWhiteSpaceSlow") {
  throw new Error(`慢路径谓词漂移: ${slowFns.join()}`);
}

// ICU 依赖与魔法字符对数
if (!ccSrc.includes("u_hasBinaryProperty(c, UCHAR_ID_START)")) throw new Error("ID_START ICU 依赖漂移");
if (!ccSrc.includes("u_hasBinaryProperty(c, UCHAR_ID_CONTINUE)")) throw new Error("ID_CONTINUE ICU 依赖漂移");
if (!ccSrc.includes("u_charType(c) == U_SPACE_SEPARATOR")) throw new Error("Zs 空白判定漂移");
// ZWJ/ZWNJ
if (!/c == 0x200C \|\|\s*\n\s*c == 0x200D;/.test(ccSrc)) throw new Error("ZWJ/ZWNJ 漂移");

let rust = banner("src/strings/char-predicates.cc.js", "src/strings/char-predicates.cc", "INTL 慢路径字符谓词骨架");

rust += `
/// 慢路径字符谓词 (原版 char-predicates.cc, 仅 V8_INTL_SUPPORT 构建; x64/arm64 默认开启)。
/// 快路径 (c ≤ 0xFF) 查表见 char_predicates.rs; 慢路径依赖 ICU 二元属性。
pub mod char_predicates_slow_skel {
  /// UnicodeIDStart 或 '$' '_' '\\\\' (c < 0x60 时; ECMA-262 Names and Keywords)
  pub fn is_identifier_start_slow(_c: u32) -> bool {
    unimplemented!("骨架")
  }

  /// UnicodeIDContinue 或 '$' '_' '\\\\' 或 ZWJ(U+200C 前体) 实为
  /// ZWNJ=U+200C, ZWJ=U+200D (c < 0x60 快旁路 + 两个连接符)
  pub fn is_identifier_part_slow(_c: u32) -> bool {
    unimplemented!("骨架")
  }

  /// White Space: gC=Zs 或 U+0009/000B/000C 或 U+FEFF
  pub fn is_white_space_slow(_c: u32) -> bool {
    unimplemented!("骨架")
  }

  /// 慢路径谓词数对数守卫
  pub const SLOW_PREDICATES: [&str; ${slowFns.length}] = [${slowFns.map((f) => `"${f}"`).join(", ")}];

  /// ICU 依赖登记
  pub const ICU_DEPS: [&str; 3] = ["UCHAR_ID_START", "UCHAR_ID_CONTINUE", "U_SPACE_SEPARATOR"];
}
`;

const p = writeSrc("v8-strings", "char_predicates_cc_generated.rs", rust);
console.log(`written ${p}; 慢路径=${slowFns.length}`);
