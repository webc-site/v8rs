// tool/lib.js — 转换脚本公共件
// 编排者维护, 子代理禁改; 脚本只准 import node 内置 + 本文件
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

export const root = join(import.meta.dir, "..");

// 读 v8/ 原版文件, rel 形如 "src/common/globals.h"
export function readV8(rel) {
  return readFileSync(join(root, "v8", rel), "utf8");
}

// 直出 crate src/: <crate>/src/<rustrel>, 返回写入路径
// 例: writeSrc("v8-common", "checks.rs", body) → v8-common/src/checks.rs
// 注意: 修订就地改 src/, 复跑脚本会覆盖修订 — 复跑仅限重生成基线
export function writeSrc(crate, rustrel, body) {
  const p = join(root, crate, "src", rustrel);
  mkdirSync(dirname(p), { recursive: true });
  writeFileSync(p, body);
  return p;
}

// 生成文件头: generator = 生成脚本相对 tool/ 的路径, v8rel = 原版来源
export function banner(generator, v8rel, note = "") {
  const n = note ? `// ${note}\n` : "";
  return `// 由 tool/${generator} 从 v8/${v8rel} 生成; 修订就地改, 复跑覆盖\n${n}`;
}

// 取原文 1-based 行区间 (含端点), 用于在脚本内圈定规则区
export function lines(src, from, to) {
  return src.split("\n").slice(from - 1, to);
}
