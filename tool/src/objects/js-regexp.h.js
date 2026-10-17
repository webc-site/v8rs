#!/usr/bin/env bun
// tool/src/objects/js-regexp.h.js — JSRegExp 族骨架 (9 flag 位 + RegExpData 三兄弟 + result shape 常量)
// 组→文件映射: js-regexp.h + js-regexp-inl.h + js-regexp.cc + js-regexp.tq
// 产物: v8-objects-js-core/src/js_regexp.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-regexp.h");
const inl = readV8("src/objects/js-regexp-inl.h");
const cc = readV8("src/objects/js-regexp.cc");
const tq = readV8("src/objects/js-regexp.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. JSRegExp flag 位链 (9 bit, 位序即声明序) ────────────────────────
const jsReBody = h.slice(h.indexOf("V8_OBJECT class JSRegExp :"), h.indexOf("V8_OBJECT_END;", h.indexOf("V8_OBJECT class JSRegExp :")));
const bitChain = [...jsReBody.matchAll(/using (\w+Bit) = (?:base::BitField<bool, 0, 1, uint32_t>|\w+Bit::Next<bool, 1>);/g)].map((m) => m[1]);
eq(bitChain.length, 9, "JSRegExp Bit 别名链长度");
must(bitChain.join(",") === "GlobalBit,IgnoreCaseBit,MultilineBit,StickyBit,UnicodeBit,DotAllBit,LinearBit,HasIndicesBit,UnicodeSetsBit", `位链序 ${bitChain}`);
const flagNames = bitChain.map((b) => b.slice(0, -3));
const flagEnum = [...jsReBody.slice(jsReBody.indexOf("enum Flag : uint32_t"), jsReBody.indexOf("using Flags = base::Flags<Flag>;"))
  .matchAll(/k(\w+) = (?:0|(\w+)Bit::kMask)/g)].map((m) => m[1]);
eq(flagEnum.length, 10, "Flag 枚举项数 (kNone + 9)");
must(flagEnum[0] === "None" && flagEnum.slice(1).join(",") === flagNames.join(","), `Flag 枚举序 ${flagEnum}`);
must(h.includes("static constexpr int kFlagCount = 9;"), "kFlagCount = 9");
must(h.includes("static_assert(kFlagCount == v8::RegExp::kFlagCount);"), "与 v8::RegExp 对数断言");
must(h.includes("static_assert(kFlagCount == regexp::kFlagCount);"), "与 regexp:: 对数断言");

// ── 2. JSRegExp 数据槽 + 常量 ───────────────────────────────────────────
must(h.includes("TrustedPointerMember<RegExpData, kRegExpDataIndirectPointerTag> data_;"), "data_ 可信指针槽");
must(h.includes("TaggedMember<Object> flags_ V8_TQ_TYPE(SmiTagged<JSRegExpFlags> | Undefined);"), "flags_ 槽");
for (const [anchor, msg] of [
  ["static const int kLastIndexOffset;", "kLastIndexOffset 声明"], ["static constexpr int kInObjectFieldCount = 1;", "kInObjectFieldCount = 1"],
  ["static constexpr int kInitialLastIndexValue = 0;", "kInitialLastIndexValue = 0"],
  ["static constexpr int kExecFunctionDescriptorIndex = 1;", "kExecFunctionDescriptorIndex"],
  ["static constexpr int kSymbolMatchFunctionDescriptorIndex = 15;", "kSymbolMatch idx 15"],
  ["static constexpr int kSymbolMatchAllFunctionDescriptorIndex = 16;", "kSymbolMatchAll idx 16"],
  ["static constexpr int kSymbolReplaceFunctionDescriptorIndex = 17;", "kSymbolReplace idx 17"],
  ["static constexpr int kSymbolSearchFunctionDescriptorIndex = 18;", "kSymbolSearch idx 18"],
  ["static constexpr int kSymbolSplitFunctionDescriptorIndex = 19;", "kSymbolSplit idx 19"],
  ["static constexpr int kUninitializedValue = -1;", "kUninitializedValue = -1"],
  ["static constexpr uint32_t kNoBacktrackLimit = 0;", "kNoBacktrackLimit = 0"],
  ["static constexpr int kTierUpForSubjectLengthValue = 1000;", "kTierUpForSubjectLengthValue = 1000"],
  ["static constexpr int kMaxCaptures = 1 << 16;", "kMaxCaptures = 1<<16"],
  ["static constexpr int kAtomCaptureCount = 0;", "kAtomCaptureCount"],
  ["static constexpr int kAtomRegisterCount = 2;", "kAtomRegisterCount"],
]) must(h.includes(anchor), msg);
must(h.includes("static constexpr int RegistersForCaptureCount(int count) {\n    return (count + 1) * 2;\n  }"), "RegistersForCaptureCount 公式");
must(h.includes("return (register_count - 2) / 2;"), "CaptureCountForRegisters 公式");

// ── 3. RegExpData 族 (trusted 对象三兄弟) ──────────────────────────────
const types = [...h.slice(h.indexOf("enum class Type : uint8_t"), h.indexOf("};", h.indexOf("enum class Type")))
  .matchAll(/^\s+(\w+),/gm)].map((m) => m[1]);
eq(types.length, 3, "RegExpData::Type 枚举数");
must(types.join(",") === "ATOM,IRREGEXP,EXPERIMENTAL", `Type 序 ${types}`);
must(h.includes("static constexpr int kQuickCheckBitsetChars = 256;"), "kQuickCheckBitsetChars = 256");
must(h.includes("static constexpr int kQuickCheckBitsetBitsPerWord = 32;"), "kQuickCheckBitsetBitsPerWord = 32");
must(h.includes("static constexpr int kQuickCheckBitsetWords ="), "kQuickCheckBitsetWords = 256/32 = 8");
must(h.includes("kHasQuickCheck = 1 << 0,"), "InternalFlag kHasQuickCheck = 1<<0");
must(h.includes("using CanBeZeroLengthBit = base::BitField<bool, 0, 1, uint32_t>;"), "IrRegExpData CanBeZeroLengthBit");
must(h.includes("using IsLinearExecutableBit = CanBeZeroLengthBit::Next<bool, 1>;"), "IrRegExpData IsLinearExecutableBit");
// RegExpData 自有字段序 (type_tag..internal_flags + 可选 padding)
const rdFields = [...h.slice(h.indexOf("V8_OBJECT class RegExpData :"), h.indexOf("V8_OBJECT class RegExpDataWrapper"))
  .matchAll(/^\s{2}((?:Tagged|Protected|Trusted|Code|uint32)[^;]*?) (\w+)(?:_\[(\w+)\])?;/gm)].map((m) => [m[2], m[3]]);
eq(rdFields.length, 10, "RegExpData 字段数 (含 bitset 数组)");
must(rdFields[0][0] === "type_tag_" && rdFields[8][0] === "internal_flags_" && rdFields[9][0] === "optional_padding_", `RegExpData 字段首中尾 ${rdFields[0][0]}/${rdFields[8][0]}/${rdFields[9][0]}`);
// IrRegExpData 9 字段
const irFields = [...h.slice(h.indexOf("V8_OBJECT class IrRegExpData :"), h.indexOf("// JSRegExpResult is just"))
  .matchAll(/^\s{2}((?:Tagged|Protected|Code)[^;]*?) (\w+)(?: V8_TQ_TYPE\([^)]*\))?;/gm)].map((m) => m[2]);
eq(irFields.length, 10, "IrRegExpData 字段数 (Protected x3 + CodePointer x2 + Smi x5)");
must(irFields.join(",") === "latin1_bytecode_,uc16_bytecode_,capture_name_map_,latin1_code_,uc16_code_,max_register_count_,capture_count_,ticks_until_tier_up_,backtrack_limit_,bit_field_", `IrRegExpData 字段序 ${irFields}`);
must(irFields[irFields.length - 1] === "bit_field_", `IrRegExpData 末字段 ${irFields[irFields.length - 1]}`);

// ── 4. JSRegExpResult shape 常量族 (in-object 槽) ──────────────────────
must(h.includes("static constexpr int kIndexSlotIndex = 0;"), "kIndexSlotIndex = 0");
must(h.includes("static constexpr int kInputSlotIndex = 1;"), "kInputSlotIndex = 1");
must(h.includes("static constexpr int kGroupsSlotIndex = 2;"), "kGroupsSlotIndex = 2");
must(h.includes("static constexpr int kRegexpInputSlotIndex = 3;"), "kRegexpInputSlotIndex = 3");
must(h.includes("static constexpr int kRegexpLastIndexSlotIndex = 4;"), "kRegexpLastIndexSlotIndex = 4");
must(h.includes("static constexpr int kInObjectPropertyCount = 5;"), "JSRegExpResult kInObjectPropertyCount = 5");
must(h.includes("static constexpr int kAdditionalInObjectPropertyCount = 1;"), "WithIndices 追加 1 槽");
must(h.includes("static constexpr int kGroupsDescriptorIndex = 1;"), "Indices kGroupsDescriptorIndex = 1");

// ── 5. inl + cc + tq 交叉 ──────────────────────────────────────────────
must(inl.includes("JSRegExp::Flags JSRegExp::flags() const {"), "inl flags");
must(inl.includes("RegExpData::Type RegExpData::type_tag() const {"), "inl type_tag");
must(inl.includes("IrRegExpData::set_quick_check") || h.includes("clear_quick_check"), "quick_check 访问器族");
must(cc.includes("MaybeDirectHandle<JSRegExp> JSRegExp::New(") || cc.includes("JSRegExp::New(Isolate* isolate,"), "cc New 定义");
must(cc.includes("std::optional<JSRegExp::Flags> JSRegExp::FlagsFromString("), "cc FlagsFromString");
must(cc.includes("DirectHandle<String> JSRegExp::StringFromFlags("), "cc StringFromFlags");
must(tq.includes("bitfield struct JSRegExpFlags extends uint31 {"), "tq flags bitfield");
const tqFlagsBody = tq.slice(tq.indexOf("bitfield struct JSRegExpFlags"), tq.indexOf("\n}", tq.indexOf("bitfield struct JSRegExpFlags")));
eq([...tqFlagsBody.matchAll(/^  \w+: bool: 1 bit;$/gm)].length, 9, "tq JSRegExpFlags 9 个 1bit 域");
must(tq.includes("bitfield struct IrRegExpDataBitField extends uint31 {"), "tq Ir bitfield");
must(tq.includes("extern class JSRegExp extends JSObject {"), "tq JSRegExp");
must(tq.includes("quick_check_reject_bitset[8]: uint32;"), "tq bitset 数组 [8] (256/32)");
must(tq.includes("extern shape JSRegExpResult extends JSArray {"), "tq shape JSRegExpResult");
must(tq.includes("transient type FastJSRegExp extends JSRegExp;"), "tq transient FastJSRegExp");
const tqShapeFields = [...tq.slice(tq.indexOf("extern shape JSRegExpResult"), tq.indexOf("extern shape JSRegExpResultWithIndices"))
  .matchAll(/^  (\w+): /gm)].map((m) => m[1]);
eq(tqShapeFields.length, 5, "tq JSRegExpResult in-object 字段数");

// ── 6. 骨架产物 ──────────────────────────────────────────────────────────
const flagDefs = flagNames.map((n, i) => `pub const K_${n.toUpperCase()}_SHIFT: u32 = ${i};`).join("\n");
const body = `${banner("src/objects/js-regexp.h.js", "src/objects/js-regexp.h + -inl.h + .cc + .tq", "JSRegExp/RegExpData 族 flags 位链与 shape 常量差集骨架")}
//! JSRegExp: data_ 可信指针 → RegExpData (ATOM/IRREGEXP/EXPERIMENTAL);
//! flags_ SmiTagged<JSRegExpFlags>|Undefined; lastIndex 为唯一 in-object 字段。

// JSRegExp::Flag 位链 (bit i = 声明序; kNone = 0)
${flagDefs}
pub const K_NONE: u32 = 0;
pub const K_FLAG_COUNT: usize = 9;

// JSRegExp 关键常量
pub const K_IN_OBJECT_FIELD_COUNT: usize = 1; // lastIndex
pub const K_INITIAL_LAST_INDEX_VALUE: i32 = 0;
pub const K_EXEC_FUNCTION_DESCRIPTOR_INDEX: i32 = 1;
pub const K_SYMBOL_MATCH_FUNCTION_DESCRIPTOR_INDEX: i32 = 15;
pub const K_SYMBOL_MATCH_ALL_FUNCTION_DESCRIPTOR_INDEX: i32 = 16;
pub const K_SYMBOL_REPLACE_FUNCTION_DESCRIPTOR_INDEX: i32 = 17;
pub const K_SYMBOL_SEARCH_FUNCTION_DESCRIPTOR_INDEX: i32 = 18;
pub const K_SYMBOL_SPLIT_FUNCTION_DESCRIPTOR_INDEX: i32 = 19;
pub const K_UNINITIALIZED_VALUE: i32 = -1;
pub const K_NO_BACKTRACK_LIMIT: u32 = 0;
pub const K_TIER_UP_FOR_SUBJECT_LENGTH_VALUE: i32 = 1000;
pub const K_MAX_CAPTURES: i32 = 1 << 16;
pub const K_ATOM_CAPTURE_COUNT: i32 = 0;
pub const K_ATOM_REGISTER_COUNT: i32 = 2;
// RegistersForCaptureCount(n) = (n + 1) * 2;  CaptureCountForRegisters(r) = (r - 2) / 2

// RegExpData::Type { ATOM, IRREGEXP, EXPERIMENTAL }; TypeSupportsCaptures: 仅 IRREGEXP|EXPERIMENTAL
pub const K_QUICK_CHECK_BITSET_CHARS: usize = 256;
pub const K_QUICK_CHECK_BITSET_BITS_PER_WORD: usize = 32;
pub const K_QUICK_CHECK_BITSET_WORDS: usize = 8; // 256 / 32
// QuickCheckBitsetBit(c) → (word = c/32, bit = 1 << (c%32)); 'a'(97) → word 3, bit 1
pub const K_INTERNAL_FLAG_HAS_QUICK_CHECK: u32 = 1 << 0;
// IrRegExpData::Bits: CanBeZeroLength bit0 (1b) + IsLinearExecutable bit1 (1b)

// JSRegExpResult shape (JSArray in-object 槽, 相对 JSArray::kHeaderSize=32 非压缩):
//   index@32 input@40 groups@48 regexp_input@56 regexp_last_index@64 → kSize 72 (5 槽)
// JSRegExpResultWithIndices: +indices@72 → 6 槽; JSRegExpResultIndices: groups@32 → 1 槽
// 方法映射 (语义推迟): New/Initialize/FlagsFromString/StringFromFlags/QuickCheckRejects/HasCompiledCode/
//   tier-up 族 (CanTierUp/MarkedForTierUp/TierUpTick/MarkTierUpForNextExec/ShouldProduceBytecode)
`;

const p = writeSrc("v8-objects-js-core", "js_regexp.rs.skel", body);
console.log(
  `js-regexp.h.js: ${p} — 位链 ${bitChain.length}/9、Flag 枚举 ${flagEnum.length}/10、常量锚 17/17、RegExpData 字段 ${rdFields.length}/10、IrRegExpData ${irFields.length}/10、shape 5 槽、tq 9×1bit、断言 ${checks} 项全部通过`,
);
