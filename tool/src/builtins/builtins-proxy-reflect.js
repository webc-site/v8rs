#!/usr/bin/env bun
// tool/src/builtins/builtins-proxy-reflect.js — v8-builtins-proxy-reflect 基线生成
// 来源: proxy*.tq(12) + reflect.tq + proxy-gen/reflect/accessors .cc/.h
// → v8-builtins-proxy-reflect/src/{proxy, reflect_access}.rs
import { groupRun } from "./builtins-lib.js";
groupRun("builtins-proxy-reflect.js", "v8-builtins-proxy-reflect", [
  { mod: "proxy", files: { cc: ["builtins-proxy-gen.cc"], h: ["builtins-proxy-gen.h"], tq: ["proxy.tq", "proxy-constructor.tq", "proxy-delete-property.tq", "proxy-get-property.tq", "proxy-get-prototype-of.tq", "proxy-has-property.tq", "proxy-is-extensible.tq", "proxy-prevent-extensions.tq", "proxy-revocable.tq", "proxy-revoke.tq", "proxy-set-property.tq", "proxy-set-prototype-of.tq"] }, anchors: [["TF_BUILTIN(CallProxy, ProxiesCodeStubAssembler)", "builtins-proxy-gen.cc"]] },
  { mod: "reflect_access", files: { cc: ["builtins-reflect.cc", "accessors.cc"], h: ["accessors.h"], tq: ["reflect.tq"] }, anchors: [["Reflect", "builtins-reflect.cc"]] },
]);
