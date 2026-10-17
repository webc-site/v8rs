// 由 tool/src/debug/gdb-server.js 生成
// wasm GDB 远端调试服务组

use strum::{Display, EnumIter, EnumString};

pub const K_TRANSPORT_BUF_SIZE: usize = 4096;

/// GDB 报文错误码 (来源: packet.h:85, target.h:75)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
#[repr(u8)]
pub enum PacketError {
  None = 0,
  BadFormat = 1,
  BadArgs = 2,
  Failed = 3,
}

/// 报文处理结果 (来源: target.h:77, 4 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum ProcessPacketResult {
  Paused,
  Continue,
  Detach,
  Kill,
}

impl ProcessPacketResult {
  pub const fn is_paused(&self) -> bool {
    matches!(self, Self::Paused)
  }

  pub const fn is_continue(&self) -> bool {
    matches!(self, Self::Continue)
  }
}

/// 调试目标状态 (来源: target.h:96, 4 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum TargetStatus {
  Running,
  WaitingForSuspension,
  Suspended,
  Terminated,
}

impl TargetStatus {
  pub const fn is_running(&self) -> bool {
    matches!(self, Self::Running)
  }

  pub const fn is_suspended(&self) -> bool {
    matches!(self, Self::Suspended)
  }
}

/// Wasm 虚拟地址类型 (来源: gdb-remote-util.h:54)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
#[repr(u8)]
pub enum WasmAddressType {
  Memory = 0x00,
  Object = 0x01,
  Invalid = 0x03,
}

/// Wasm GDB 地址打包结构 (来源: gdb-remote-util.h)
/// 编码: offset[0:32] / module_id[32:62, 30 bits] / type[62:64, 2 bits]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WasmAddr {
  pub offset: u32,
  pub module_id: u32,
  pub address_type: u8,
}

impl WasmAddr {
  pub const fn new(offset: u32, module_id: u32, address_type: WasmAddressType) -> Self {
    Self {
      offset,
      module_id: module_id & 0x3fff_ffff,
      address_type: address_type as u8,
    }
  }

  pub const fn encode(&self) -> u64 {
    let off = self.offset as u64;
    let m = ((self.module_id & 0x3fff_ffff) as u64) << 32;
    let t = ((self.address_type & 0x3) as u64) << 62;
    off | m | t
  }

  pub const fn decode(val: u64) -> Self {
    let offset = (val & 0xffff_ffff) as u32;
    let module_id = ((val >> 32) & 0x3fff_ffff) as u32;
    let address_type = ((val >> 62) & 0x3) as u8;
    Self {
      offset,
      module_id,
      address_type,
    }
  }
}

/// GDB RSP 报文辅助工具
pub struct GdbPacket;

impl GdbPacket {
  /// 计算 GDB 报文校验和 ($payload#checksum 中 payload 字节 mod 256 和)
  pub fn calculate_checksum(payload: &[u8]) -> u8 {
    payload.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
  }

  /// 格式化为标准 GDB RSP 报文 ($payload#xx)
  pub fn format_packet(payload: &str) -> String {
    let csum = Self::calculate_checksum(payload.as_bytes());
    format!("${payload}#{csum:02x}")
  }
}
