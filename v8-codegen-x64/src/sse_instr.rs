// 由 tool/src/codegen/x64/sse-instr.h.js 从 v8/src/codegen/x64/sse-instr.h 生成; 修订就地改, 复跑覆盖
// SSE/AVX 指令编码表 (X-macro 机械展开)

// 二元组: (指令助记名, 操作码字节序列, 前 1-2 字节为前缀)。
// 操作码大小写保持原版 (hex 字面)。

/// AVX 双操作数版 SSE 单目指令 (5 条)
#[rustfmt::skip]
pub const SSE_UNOP_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("sqrtps", &["0F", "51"]),
  ("rsqrtps", &["0F", "52"]),
  ("rcpps", &["0F", "53"]),
  ("cvtps2pd", &["0F", "5A"]),
  ("cvtdq2ps", &["0F", "5B"]),
];

/// AVX 三操作数版 SSE 双目指令 (11 条)
#[rustfmt::skip]
pub const SSE_BINOP_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("unpcklps", &["0F", "14"]),
  ("andps", &["0F", "54"]),
  ("andnps", &["0F", "55"]),
  ("orps", &["0F", "56"]),
  ("xorps", &["0F", "57"]),
  ("addps", &["0F", "58"]),
  ("mulps", &["0F", "59"]),
  ("subps", &["0F", "5C"]),
  ("minps", &["0F", "5D"]),
  ("divps", &["0F", "5E"]),
  ("maxps", &["0F", "5F"]),
];

/// 标量单精度指令 (8 条)
#[rustfmt::skip]
pub const SSE_INSTRUCTION_LIST_SS: &[(&str, &[&str])] = &[
  ("sqrtss", &["F3", "0F", "51"]),
  ("addss", &["F3", "0F", "58"]),
  ("mulss", &["F3", "0F", "59"]),
  ("cvtss2sd", &["F3", "0F", "5A"]),
  ("subss", &["F3", "0F", "5C"]),
  ("minss", &["F3", "0F", "5D"]),
  ("divss", &["F3", "0F", "5E"]),
  ("maxss", &["F3", "0F", "5F"]),
];

/// SSE2 打包双精度 (10 条)
#[rustfmt::skip]
pub const SSE2_INSTRUCTION_LIST_PD: &[(&str, &[&str])] = &[
  ("andpd", &["66", "0F", "54"]),
  ("andnpd", &["66", "0F", "55"]),
  ("orpd", &["66", "0F", "56"]),
  ("xorpd", &["66", "0F", "57"]),
  ("addpd", &["66", "0F", "58"]),
  ("mulpd", &["66", "0F", "59"]),
  ("subpd", &["66", "0F", "5C"]),
  ("minpd", &["66", "0F", "5D"]),
  ("divpd", &["66", "0F", "5E"]),
  ("maxpd", &["66", "0F", "5F"]),
];

/// SSE2 打包整数 (48 条)
#[rustfmt::skip]
pub const SSE2_INSTRUCTION_LIST_PI: &[(&str, &[&str])] = &[
  ("punpcklbw", &["66", "0F", "60"]),
  ("punpcklwd", &["66", "0F", "61"]),
  ("punpckldq", &["66", "0F", "62"]),
  ("packsswb", &["66", "0F", "63"]),
  ("pcmpgtb", &["66", "0F", "64"]),
  ("pcmpgtw", &["66", "0F", "65"]),
  ("pcmpgtd", &["66", "0F", "66"]),
  ("packuswb", &["66", "0F", "67"]),
  ("punpckhbw", &["66", "0F", "68"]),
  ("punpckhwd", &["66", "0F", "69"]),
  ("punpckhdq", &["66", "0F", "6A"]),
  ("packssdw", &["66", "0F", "6B"]),
  ("punpcklqdq", &["66", "0F", "6C"]),
  ("punpckhqdq", &["66", "0F", "6D"]),
  ("pcmpeqb", &["66", "0F", "74"]),
  ("pcmpeqw", &["66", "0F", "75"]),
  ("pcmpeqd", &["66", "0F", "76"]),
  ("paddq", &["66", "0F", "D4"]),
  ("pmullw", &["66", "0F", "D5"]),
  ("psubusb", &["66", "0F", "D8"]),
  ("psubusw", &["66", "0F", "D9"]),
  ("pminub", &["66", "0F", "DA"]),
  ("pand", &["66", "0F", "DB"]),
  ("paddusb", &["66", "0F", "DC"]),
  ("paddusw", &["66", "0F", "DD"]),
  ("pmaxub", &["66", "0F", "DE"]),
  ("pandn", &["66", "0F", "DF"]),
  ("pavgb", &["66", "0F", "E0"]),
  ("pavgw", &["66", "0F", "E3"]),
  ("pmulhuw", &["66", "0F", "E4"]),
  ("pmulhw", &["66", "0F", "E5"]),
  ("psubsb", &["66", "0F", "E8"]),
  ("psubsw", &["66", "0F", "E9"]),
  ("pminsw", &["66", "0F", "EA"]),
  ("por", &["66", "0F", "EB"]),
  ("paddsb", &["66", "0F", "EC"]),
  ("paddsw", &["66", "0F", "ED"]),
  ("pmaxsw", &["66", "0F", "EE"]),
  ("pxor", &["66", "0F", "EF"]),
  ("pmuludq", &["66", "0F", "F4"]),
  ("pmaddwd", &["66", "0F", "F5"]),
  ("psubb", &["66", "0F", "F8"]),
  ("psubw", &["66", "0F", "F9"]),
  ("psubd", &["66", "0F", "FA"]),
  ("psubq", &["66", "0F", "FB"]),
  ("paddb", &["66", "0F", "FC"]),
  ("paddw", &["66", "0F", "FD"]),
  ("paddd", &["66", "0F", "FE"]),
];

/// SSE2 移位 (XMM/m128 操作数) (8 条)
#[rustfmt::skip]
pub const SSE2_INSTRUCTION_LIST_SHIFT: &[(&str, &[&str])] = &[
  ("psrlw", &["66", "0F", "D1"]),
  ("psrld", &["66", "0F", "D2"]),
  ("psrlq", &["66", "0F", "D3"]),
  ("psraw", &["66", "0F", "E1"]),
  ("psrad", &["66", "0F", "E2"]),
  ("psllw", &["66", "0F", "F1"]),
  ("pslld", &["66", "0F", "F2"]),
  ("psllq", &["66", "0F", "F3"]),
];

/// SSE2 组合表 = PD + PI + SHIFT (66 条)
#[rustfmt::skip]
pub const SSE2_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("andpd", &["66", "0F", "54"]),
  ("andnpd", &["66", "0F", "55"]),
  ("orpd", &["66", "0F", "56"]),
  ("xorpd", &["66", "0F", "57"]),
  ("addpd", &["66", "0F", "58"]),
  ("mulpd", &["66", "0F", "59"]),
  ("subpd", &["66", "0F", "5C"]),
  ("minpd", &["66", "0F", "5D"]),
  ("divpd", &["66", "0F", "5E"]),
  ("maxpd", &["66", "0F", "5F"]),
  ("punpcklbw", &["66", "0F", "60"]),
  ("punpcklwd", &["66", "0F", "61"]),
  ("punpckldq", &["66", "0F", "62"]),
  ("packsswb", &["66", "0F", "63"]),
  ("pcmpgtb", &["66", "0F", "64"]),
  ("pcmpgtw", &["66", "0F", "65"]),
  ("pcmpgtd", &["66", "0F", "66"]),
  ("packuswb", &["66", "0F", "67"]),
  ("punpckhbw", &["66", "0F", "68"]),
  ("punpckhwd", &["66", "0F", "69"]),
  ("punpckhdq", &["66", "0F", "6A"]),
  ("packssdw", &["66", "0F", "6B"]),
  ("punpcklqdq", &["66", "0F", "6C"]),
  ("punpckhqdq", &["66", "0F", "6D"]),
  ("pcmpeqb", &["66", "0F", "74"]),
  ("pcmpeqw", &["66", "0F", "75"]),
  ("pcmpeqd", &["66", "0F", "76"]),
  ("paddq", &["66", "0F", "D4"]),
  ("pmullw", &["66", "0F", "D5"]),
  ("psubusb", &["66", "0F", "D8"]),
  ("psubusw", &["66", "0F", "D9"]),
  ("pminub", &["66", "0F", "DA"]),
  ("pand", &["66", "0F", "DB"]),
  ("paddusb", &["66", "0F", "DC"]),
  ("paddusw", &["66", "0F", "DD"]),
  ("pmaxub", &["66", "0F", "DE"]),
  ("pandn", &["66", "0F", "DF"]),
  ("pavgb", &["66", "0F", "E0"]),
  ("pavgw", &["66", "0F", "E3"]),
  ("pmulhuw", &["66", "0F", "E4"]),
  ("pmulhw", &["66", "0F", "E5"]),
  ("psubsb", &["66", "0F", "E8"]),
  ("psubsw", &["66", "0F", "E9"]),
  ("pminsw", &["66", "0F", "EA"]),
  ("por", &["66", "0F", "EB"]),
  ("paddsb", &["66", "0F", "EC"]),
  ("paddsw", &["66", "0F", "ED"]),
  ("pmaxsw", &["66", "0F", "EE"]),
  ("pxor", &["66", "0F", "EF"]),
  ("pmuludq", &["66", "0F", "F4"]),
  ("pmaddwd", &["66", "0F", "F5"]),
  ("psubb", &["66", "0F", "F8"]),
  ("psubw", &["66", "0F", "F9"]),
  ("psubd", &["66", "0F", "FA"]),
  ("psubq", &["66", "0F", "FB"]),
  ("paddb", &["66", "0F", "FC"]),
  ("paddw", &["66", "0F", "FD"]),
  ("paddd", &["66", "0F", "FE"]),
  ("psrlw", &["66", "0F", "D1"]),
  ("psrld", &["66", "0F", "D2"]),
  ("psrlq", &["66", "0F", "D3"]),
  ("psraw", &["66", "0F", "E1"]),
  ("psrad", &["66", "0F", "E2"]),
  ("psllw", &["66", "0F", "F1"]),
  ("pslld", &["66", "0F", "F2"]),
  ("psllq", &["66", "0F", "F3"]),
];

/// AVX 双操作数版 SSE2 单目 (5 条)
#[rustfmt::skip]
pub const SSE2_UNOP_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("ucomisd", &["66", "0F", "2E"]),
  ("sqrtpd", &["66", "0F", "51"]),
  ("cvtpd2ps", &["66", "0F", "5A"]),
  ("cvtps2dq", &["66", "0F", "5B"]),
  ("cvttpd2dq", &["66", "0F", "E6"]),
];

/// SSE2 立即数移位 (末位为 opcode 扩展) (8 条)
#[rustfmt::skip]
pub const SSE2_INSTRUCTION_LIST_SHIFT_IMM: &[(&str, &[&str])] = &[
  ("psrlw", &["66", "0F", "71", "2"]),
  ("psrld", &["66", "0F", "72", "2"]),
  ("psrlq", &["66", "0F", "73", "2"]),
  ("psraw", &["66", "0F", "71", "4"]),
  ("psrad", &["66", "0F", "72", "4"]),
  ("psllw", &["66", "0F", "71", "6"]),
  ("pslld", &["66", "0F", "72", "6"]),
  ("psllq", &["66", "0F", "73", "6"]),
];

/// 标量双精度指令 (8 条)
#[rustfmt::skip]
pub const SSE2_INSTRUCTION_LIST_SD: &[(&str, &[&str])] = &[
  ("sqrtsd", &["F2", "0F", "51"]),
  ("addsd", &["F2", "0F", "58"]),
  ("mulsd", &["F2", "0F", "59"]),
  ("cvtsd2ss", &["F2", "0F", "5A"]),
  ("subsd", &["F2", "0F", "5C"]),
  ("minsd", &["F2", "0F", "5D"]),
  ("divsd", &["F2", "0F", "5E"]),
  ("maxsd", &["F2", "0F", "5F"]),
];

/// SSSE3 指令 (8 条)
#[rustfmt::skip]
pub const SSSE3_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("pshufb", &["66", "0F", "38", "00"]),
  ("phaddw", &["66", "0F", "38", "01"]),
  ("phaddd", &["66", "0F", "38", "02"]),
  ("pmaddubsw", &["66", "0F", "38", "04"]),
  ("psignb", &["66", "0F", "38", "08"]),
  ("psignw", &["66", "0F", "38", "09"]),
  ("psignd", &["66", "0F", "38", "0A"]),
  ("pmulhrsw", &["66", "0F", "38", "0B"]),
];

/// AVX 双操作数版 SSSE3 单目 (3 条)
#[rustfmt::skip]
pub const SSSE3_UNOP_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("pabsb", &["66", "0F", "38", "1C"]),
  ("pabsw", &["66", "0F", "38", "1D"]),
  ("pabsd", &["66", "0F", "38", "1E"]),
];

/// SSE4 指令 (12 条)
#[rustfmt::skip]
pub const SSE4_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("pmuldq", &["66", "0F", "38", "28"]),
  ("pcmpeqq", &["66", "0F", "38", "29"]),
  ("packusdw", &["66", "0F", "38", "2B"]),
  ("pminsb", &["66", "0F", "38", "38"]),
  ("pminsd", &["66", "0F", "38", "39"]),
  ("pminuw", &["66", "0F", "38", "3A"]),
  ("pminud", &["66", "0F", "38", "3B"]),
  ("pmaxsb", &["66", "0F", "38", "3C"]),
  ("pmaxsd", &["66", "0F", "38", "3D"]),
  ("pmaxuw", &["66", "0F", "38", "3E"]),
  ("pmaxud", &["66", "0F", "38", "3F"]),
  ("pmulld", &["66", "0F", "38", "40"]),
];

/// AVX 双操作数版 SSE4 单目 (ptest + PMOV 族) (8 条)
#[rustfmt::skip]
pub const SSE4_UNOP_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("ptest", &["66", "0F", "38", "17"]),
  ("pmovsxbw", &["66", "0F", "38", "20"]),
  ("pmovsxwd", &["66", "0F", "38", "23"]),
  ("pmovsxdq", &["66", "0F", "38", "25"]),
  ("pmovzxbw", &["66", "0F", "38", "30"]),
  ("pmovzxbd", &["66", "0F", "38", "31"]),
  ("pmovzxwd", &["66", "0F", "38", "33"]),
  ("pmovzxdq", &["66", "0F", "38", "35"]),
];

/// SSE4 抽取指令 (4 条)
#[rustfmt::skip]
pub const SSE4_EXTRACT_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("extractps", &["66", "0F", "3A", "17"]),
  ("pextrb", &["66", "0F", "3A", "14"]),
  ("pextrw", &["66", "0F", "3A", "15"]),
  ("pextrd", &["66", "0F", "3A", "16"]),
];

/// SSE4.2 指令 (1 条)
#[rustfmt::skip]
pub const SSE4_2_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("pcmpgtq", &["66", "0F", "38", "37"]),
];

/// AVX2 广播指令 (需 AVX2) (4 条)
#[rustfmt::skip]
pub const AVX2_BROADCAST_LIST: &[(&str, &[&str])] = &[
  ("vpbroadcastb", &["66", "0F", "38", "78"]),
  ("vpbroadcastw", &["66", "0F", "38", "79"]),
  ("vpbroadcastd", &["66", "0F", "38", "58"]),
  ("vpbroadcastq", &["66", "0F", "38", "59"]),
];


// —— 提取不变式断言 ——
const _: () = {
  assert!(SSE_UNOP_INSTRUCTION_LIST.len() == 5);
  assert!(SSE_BINOP_INSTRUCTION_LIST.len() == 11);
  assert!(SSE2_INSTRUCTION_LIST_PI.len() == 48);
  assert!(SSE2_INSTRUCTION_LIST.len() == 66);
  assert!(SSE4_INSTRUCTION_LIST.len() == 12);
  assert!(SSE4_2_INSTRUCTION_LIST.len() == 1);
  assert!(SSE_UNOP_INSTRUCTION_LIST[0].0 == "sqrtps");
  assert!(SSE2_INSTRUCTION_LIST_PI.last().unwrap().0 == "paddd");
};
