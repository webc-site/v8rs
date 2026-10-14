# v8-execution-simulator 开发日志

> 2026-10-12 ARM64 模拟器: SimulatorBase 生命周期规则面 + simulator-logic 指令语义清单 + 寄存器/内存/NZCV。

## 轮7 W7LIT (零头席)

- [轮7 W7LIT] grep `todo!` 计数开工 4 → 收笼 2。规则面 `src/simulator_cc.rs`:
  - 填体 2 (小面 1:1): `Half::from_f32`/`to_f32` 对位 `fp16_ieee_from_fp32_value`/`to_fp32_value` (镜像 CSA `TruncateFloat32ToFloat16`/`ChangeFloat16ToFloat32`); 另修正 2 处占位常量 `K_FP16_DEFAULT_NAN`=0x7E00、`K_H_QUIET_NAN_MASK`=0x200。独立 `rustc -O` 全 65536 位模式往返 0 失配核验。
  - 推迟 2 (深水, 登记 `readme/zh.md`): `initialize_once_per_process`、`global_tear_down` 依赖 SimulatorBase 进程单例 Mutex/CustomMatcherHashMap 状态, 本模块仅持语义名常量, 上层未就位。
  - `simulator_cc.rs` 未入 `lib.rs` 模块图 (生成器规则面基线), 填体不改公共 API。门禁实测 211/211 全绿 (含本 crate)。
