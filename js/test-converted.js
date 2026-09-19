#!/usr/bin/env -S bun

// 运行从 v8 复刻过来的转换测试（example/tests/converted），汇总结果
import { join } from "node:path";

const root_dir = join(import.meta.dirname, "..");

const proc = Bun.spawn(["./test.sh", "converted"], {
  cwd: root_dir,
  stdout: "inherit",
  stderr: "inherit",
});
const code = await proc.exited;

if (code !== 0) {
  console.error("复刻测试未通过，退出码 " + code);
  process.exit(code);
}
console.log("converted 测试全部通过");
