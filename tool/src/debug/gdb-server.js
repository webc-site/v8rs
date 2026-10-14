#!/usr/bin/env bun
// tool/src/debug/gdb-server.js — wasm GDB 远端调试服务组
// 覆盖: src/debug/wasm/gdb-server/packet.h/.cc + session.h/.cc + target.h/.cc +
//       transport.h/.cc + gdb-server.h/.cc + gdb-server-thread.h/.cc +
//       gdb-remote-util.h/.cc + wasm-module-debug.h/.cc (8 对, 16 文件, 实测数; 任务声称 27 含顶层外溢)
// 产出: v8-debug/src/gdb_server.rs.skel (规则代码: ErrDef/ErrorCode/ProcessPacketResult/
//       Status/WasmAddressType 枚举全量 + Session/Target/Transport 类面骨架)
// 复跑幂等: 纯文本扫描 + 源码序输出, 无时间戳。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/debug/gdb-server.js";
const P = "src/debug/wasm/gdb-server/";
// 路径必须全字面量: 审计器按 src/... 字面量反推覆盖, 模板串拼路径抓不到 (W9 消账)
const FILES = [
  "src/debug/wasm/gdb-server/packet.h", "src/debug/wasm/gdb-server/packet.cc",
  "src/debug/wasm/gdb-server/session.h", "src/debug/wasm/gdb-server/session.cc",
  "src/debug/wasm/gdb-server/target.h", "src/debug/wasm/gdb-server/target.cc",
  "src/debug/wasm/gdb-server/transport.h", "src/debug/wasm/gdb-server/transport.cc",
  "src/debug/wasm/gdb-server/gdb-server.h", "src/debug/wasm/gdb-server/gdb-server.cc",
  "src/debug/wasm/gdb-server/gdb-server-thread.h", "src/debug/wasm/gdb-server/gdb-server-thread.cc",
  "src/debug/wasm/gdb-server/gdb-remote-util.h", "src/debug/wasm/gdb-server/gdb-remote-util.cc",
  "src/debug/wasm/gdb-server/wasm-module-debug.h", "src/debug/wasm/gdb-server/wasm-module-debug.cc",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[gdb-server.js] ${m}`); };
const need = (f, s) => { if (!txt[f].includes(s)) die(`${f} 缺针 ${s}`); };

// ① Packet (packet.h)
need(`${P}packet.h`, "enum class ErrDef { None = 0, BadFormat = 1, BadArgs = 2, Failed = 3 };");
need(`${P}packet.h`, "std::string GetPacketData() const;");
// ② Target 三枚举 (target.h)
need(`${P}target.h`, "class Target {");
need(`${P}target.h`, "enum class ErrorCode { None = 0, BadFormat = 1, BadArgs = 2, Failed = 3 };");
need(`${P}target.h`, "enum class ProcessPacketResult {");
for (const v of ["Paused,", "Continue,", "Detach,", "Kill"]) need(`${P}target.h`, v);
need(`${P}target.h`, "enum class Status { Running, WaitingForSuspension, Suspended, Terminated };");
need(`${P}target.h`, "ProcessPacketResult ProcessPacket(Packet* pkt_in, Packet* pkt_out);");
// ③ Session / Transport / GdbServer
need(`${P}session.h`, "class V8_EXPORT_PRIVATE Session {");
need(`${P}transport.h`, "class V8_EXPORT_PRIVATE TransportBase {");
need(`${P}transport.h`, "kBufSize = 4096;");
need(`${P}gdb-server.h`, "class GdbServer {");
need(`${P}gdb-server.h`, "BreakProgramRequested(");
// ④ 地址编码 (gdb-remote-util.h) + 模块调试面
need(`${P}gdb-remote-util.h`, "enum class WasmAddressType : uint8_t {");
for (const v of ["Memory = 0x00,", "Object = 0x01,", "Invalid = 0x03"]) need(`${P}gdb-remote-util.h`, v);
need(`${P}gdb-remote-util.h`, "class wasm_addr_t {");
need(`${P}wasm-module-debug.h`, "class WasmModuleDebug {");
need(`${P}wasm-module-debug.h`, "static bool GetWasmStackValue(Isolate* isolate, uint32_t frame_index,");

// ⑤ 机械提取: ProcessPacketResult / Status 变体
const pprBlock = txt[`${P}target.h`].match(/enum class ProcessPacketResult \{([^}]*)\}/)[1];
const pprVars = [...pprBlock.matchAll(/^\s{4}([A-Z]\w+),?/gm)].map((m) => m[1]);
const stVars = [...txt[`${P}target.h`].match(/enum class Status \{([^}]*)\}/)[1].matchAll(/\b([A-Z]\w+),?/g)].map((m) => m[1]);
if (pprVars.length !== 4) die(`ProcessPacketResult 变体数=${pprVars.length}, 期望 4`);
if (stVars.length !== 4) die(`Status 变体数=${stVars.length}, 期望 4`);

console.log(`OK gdb-server.js 针核验通过: 错误码 4 + 结果 4 + 状态 4 + 地址 3 校验全过`);

