[English](#en) | [中文](#zh)

---

<a name="en"></a>

#

---

<a name="zh"></a>

# v8-wasm-interp

Drumbrake 分发表规则骨架 `src/wasm_interpreter_dispatch.rs.skel` (由 `tool/src/wasm/interpreter/wasm-interpreter.cc.js` 生成, 复跑覆盖) 的推迟台账。

- [推迟台账 (轮7 W7LIT)](#推迟台账-轮7-w7lit)

## 推迟台账 (轮7 W7LIT)

深水面 todo! 保留登记 (`.skel` 未入编译图, 依赖未就位上层类型, 禁伪装填体):

- `wasm_interpreter_dispatch.rs.skel::init_instruction_table_rule` — 对位 `wasm-interpreter.cc` `InitInstructionTableOnce` / `InitTrapHandlersOnce`。
  依赖 builtin blob 解包与 trap_handler 注册, 硬耦合 `Isolate` / `EmbeddedData` 上层类型; 归语义 Developer 轮。
- `wasm_interpreter_dispatch.rs.skel::tail_dispatch_rule` — 对位 `wasm-interpreter.cc` `NextOp` 宏族 (MUSTTAIL 尾分派)。
  依赖 `ReadFnId` 编码 (尺寸位 + id) 联动, 需 code/bytecode 上层就位; 归语义 Developer 轮。

(其余规则面常量/枚举/表 (`K_INSTRUCTION_TABLE_*`, `BoundsCheckedHandler`, `TOP_LEVEL_FN_DEFS`, `DISPATCH_MACROS`) 均为已就位声明, 无 todo。)

