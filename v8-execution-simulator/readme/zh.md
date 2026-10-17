# v8-execution-simulator

规则面基线 `src/simulator_cc.rs` (由 `tool/src/execution/simulator-cc.js` 生成, 复跑覆盖) 的推迟台账。

## 推迟台账 (轮7 W7LIT)

深水面 todo! 保留登记 (依赖未就位上层类型, 禁伪装填体):

- `simulator_cc.rs::initialize_once_per_process` — 对位 `simulator-base.cc:27-38`。
  依赖 `SimulatorBase` 进程级单例状态 (`base::Mutex` × 2 + `base::CustomMatcherHashMap` / `Simulator::ICacheMatch`)。
  本模块仅持语义名常量 (`REDIRECTION_MUTEX`/`I_CACHE` 等), 真实单例归 `simulator.rs` 上层; OnceLock 收敛待 SimulatorBase 状态面就位后填。
- `simulator_cc.rs::global_tear_down` — 对位 `simulator-base.cc:40-58`。
  依赖同上单例 + `Redirection::DeleteChain` 重定向链与逐 `CachePage` 释放; 归 SimulatorBase 生命周期轮。

## 已就地填体 (轮7 W7LIT, 小面 1:1)

- `K_FP16_DEFAULT_NAN` = 0x7E00 (`instructions-arm64-constants.cc`), `K_H_QUIET_NAN_MASK` = 1<<9 = 0x200 (`constants-arm64.h`)。
- `Half::from_f32` / `Half::to_f32` — 1:1 对位 `fp16_ieee_from_fp32_value` / `fp16_ieee_to_fp32_value` (镜像 CSA `TruncateFloat32ToFloat16` / `ChangeFloat16ToFloat32`); 全 65536 位模式往返 0 失配。
