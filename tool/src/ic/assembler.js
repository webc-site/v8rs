#!/usr/bin/env bun
// tool/src/ic/assembler.js — IC 汇编器面校验 (CSA/TSA 生成, 逻辑后置 L5)
// 覆盖: src/ic/accessor-assembler.h + accessor-assembler.cc +
//       binary-op-assembler.h + binary-op-assembler.cc +
//       unary-op-assembler.h + unary-op-assembler.cc +
//       keyed-store-generic.h + keyed-store-generic.cc
import { readV8 } from "../../lib.js";

const FILES = [
  "src/ic/accessor-assembler.h",
  "src/ic/accessor-assembler.cc",
  "src/ic/binary-op-assembler.h",
  "src/ic/binary-op-assembler.cc",
  "src/ic/unary-op-assembler.h",
  "src/ic/unary-op-assembler.cc",
  "src/ic/keyed-store-generic.h",
  "src/ic/keyed-store-generic.cc",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => {
  throw new Error(`[assembler.js] ${m}`);
};

for (const [f, needle] of [
  ["src/ic/accessor-assembler.h", "AccessorAssembler"],
  ["src/ic/binary-op-assembler.h", "BinaryOpAssembler"],
  ["src/ic/unary-op-assembler.h", "UnaryOpAssembler"],
  ["src/ic/keyed-store-generic.h", "KeyedStoreGeneric"],
]) {
  if (!txt[f].includes(needle)) die(`${f} 缺针 ${needle}`);
}

console.log(`OK assembler.js 针核验通过 (类面针 4 全过)`);
