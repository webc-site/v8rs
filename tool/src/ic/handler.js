#!/usr/bin/env bun
// tool/src/ic/handler.js — LoadHandler/StoreHandler Kind 位编码表校验
// 覆盖: src/ic/handler-configuration.h + handler-configuration.cc +
//       handler-configuration-inl.h + handler-configuration.tq
// 规则: LoadHandler::Kind 16 变体 + StoreHandler::Kind 12 变体 (kKindsNumber 守尾)
import { readV8 } from "../../lib.js";

const FILES = [
  "src/ic/handler-configuration.h",
  "src/ic/handler-configuration.cc",
  "src/ic/handler-configuration-inl.h",
  "src/ic/handler-configuration.tq",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => {
  throw new Error(`[handler.js] ${m}`);
};
const count = (s, re) => (s.match(re) ?? []).length;

const h = txt["src/ic/handler-configuration.h"];
const loadRegion = h.slice(
  h.indexOf("class LoadHandler"),
  h.indexOf("class StoreHandler"),
);
const storeRegion = h.slice(h.indexOf("class StoreHandler"));

// ① LoadHandler::Kind 16 变体
const loadKinds = [
  "kElement",
  "kElementWithTransition",
  "kIndexedString",
  "kNormal",
  "kGlobal",
  "kField",
  "kConstantFromPrototype",
  "kAccessorFromPrototype",
  "kNativeDataProperty",
  "kApiGetter",
  "kInterceptor",
  "kSlow",
  "kProxy",
  "kNonExistent",
  "kModuleExport",
  "kGeneric",
];
for (const k of loadKinds) {
  if (count(loadRegion, new RegExp(`\\b${k},`)) !== 1) {
    die(`LoadHandler::Kind 缺/重 ${k}`);
  }
}

// ② StoreHandler::Kind 12 变体 + kKindsNumber 守尾
const storeKinds = [
  "kField",
  "kConstField",
  "kAccessorFromPrototype",
  "kNativeDataProperty",
  "kSharedStructField",
  "kApiSetter",
  "kGlobalProxy",
  "kNormal",
  "kInterceptor",
  "kSlow",
  "kProxy",
  "kGeneric",
];
for (const k of storeKinds) {
  if (count(storeRegion, new RegExp(`\\b${k},`)) !== 1) {
    die(`StoreHandler::Kind 缺/重 ${k}`);
  }
}
if (!storeRegion.includes("kKindsNumber")) die("StoreHandler 缺 kKindsNumber 守尾");

// ③ 位段面针 + .tq 针
for (const needle of [
  "using KindBits = base::BitField<Kind, 0, 4>;",
  "static_assert(DescriptorBits::kLastUsedBit < kSmiValueSize)",
]) {
  if (!h.includes(needle)) die(`缺针 ${needle}`);
}
const tq = txt["src/ic/handler-configuration.tq"];
if (
  count(
    tq.split("\n").filter((l) => l.startsWith("extern class")).length ? tq : "",
    /extern class/g,
  ) !== 2
) {
  die(".tq extern 声明非 2 行");
}
for (const needle of [
  "extern class LoadHandler extends DataHandler;",
  "extern class StoreHandler extends DataHandler;",
]) {
  if (!tq.includes(needle)) die(`.tq 缺针 ${needle}`);
}

console.log(
  `OK handler.js 针核验通过: Load 16 + Store 12 + 守尾 1 + 位段针 2 + tq 针 2 全过`,
);
