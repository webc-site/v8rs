//! bump 分配内核抽象层
//!
//! 内核评估记录 (bumpalo 3.20.3, macOS aarch64, release, 2026-10 复测,
//! 详见 readme/zh.md): 1,000,000 次分配
//! - 固定 32B: 自研 chunk 0.0022s vs bumpalo 0.0039s (bumpalo 侧含
//!   fill_default 清零开销); 两者 8 字节对齐违例均为 0
//! - 变长 1..=64B: 自研 0.0020s vs bumpalo 0.0107s
//!
//! 语义缺口 (决定性): bumpalo::Bump 无 Segment 级链表 (Snapshot/Restore
//! 需按段归还)、无 TraceAllocateSegment/allocation_size 记账钩子、
//! reset 不保留末段、无 per-zone segment_bytes 口径, 无法满足原版
//! zone.h 外部行为 1:1 红线。
//!
//! 结论: 评估不通过, 采用自研 chunk 内核 (chunk.rs); bumpalo 依赖与
//! 评估代码已移除, 无遗留双轨。

pub(crate) mod chunk;
