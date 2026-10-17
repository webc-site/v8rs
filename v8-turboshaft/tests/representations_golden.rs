//! representations 映射与尺寸金标准 — 穷举 FromMachineRepresentation + SizeInBytes
//! v8: src/compiler/turboshaft/representations.h L278 (Register From) / L882 (Memory From) / SizeInBytesLog2
//! 逐例断言具体映射与字节数 (round1 仅测 Word32/Word64/整型尺寸; 此处补齐指针/压缩/tagged 面)

use aok::{OK, Void};
use log::info;
use v8_compiler_graph::MachineRepresentation;
use v8_compiler_ops::LoadRepresentation;
use v8_turboshaft::{MemoryRepresentation, RegisterRepresentation};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// RegisterRepresentation::FromMachineRepresentation 穷举金标准 (原版 representations.h L278)
/// 覆盖 19 个可达映射 (排除 kNone/kFloat16RawBits 的 UNREACHABLE)
#[test]
fn register_from_machine_representation_exhaustive() -> Void {
  info!("> RegisterRepresentation::from_machine_representation 穷举金标准");
  // 整型窄表示 → Word32 (原版 4 case 聚合)
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Bit),
    RegisterRepresentation::Word32
  );
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Word8),
    RegisterRepresentation::Word32
  );
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Word16),
    RegisterRepresentation::Word32
  );
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Word32),
    RegisterRepresentation::Word32
  );
  // Word64 直映
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Word64),
    RegisterRepresentation::Word64
  );
  // tagged 族聚合 (原版 kTaggedSigned/kTaggedPointer/kTagged/kProtectedPointer)
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::TaggedSigned),
    RegisterRepresentation::Tagged
  );
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::TaggedPointer),
    RegisterRepresentation::Tagged
  );
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Tagged),
    RegisterRepresentation::Tagged
  );
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::ProtectedPointer),
    RegisterRepresentation::Tagged
  );
  // map packing 关闭: MapWord → Tagged (原版 DCHECK + 返回 Tagged)
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::MapWord),
    RegisterRepresentation::Tagged
  );
  // 压缩指针族 → Compressed
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::CompressedPointer),
    RegisterRepresentation::Compressed
  );
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Compressed),
    RegisterRepresentation::Compressed
  );
  // 浮点族 (Float16 视为 Float32 寄存器)
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Float16),
    RegisterRepresentation::Float32
  );
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Float32),
    RegisterRepresentation::Float32
  );
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Float64),
    RegisterRepresentation::Float64
  );
  // SIMD
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Simd128),
    RegisterRepresentation::Simd128
  );
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Simd256),
    RegisterRepresentation::Simd256
  );
  // 指针特殊: Indirect/Sandboxed → WordPtr (64 位目标即 Word64)
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::IndirectPointer),
    RegisterRepresentation::Word64
  );
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::SandboxedPointer),
    RegisterRepresentation::Word64
  );
  OK
}

/// MemoryRepresentation::FromMachineRepresentation 穷举金标准 (原版 L882)
/// 覆盖 13 个可达映射; 整型默认无符号 (原版仅返回 Uint*)
#[test]
fn memory_from_machine_representation_exhaustive() -> Void {
  info!("> MemoryRepresentation::from_machine_representation 穷举金标准");
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::Word8),
    MemoryRepresentation::Uint8
  );
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::Word16),
    MemoryRepresentation::Uint16
  );
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::Word32),
    MemoryRepresentation::Uint32
  );
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::Word64),
    MemoryRepresentation::Uint64
  );
  // tagged 三态保留具体名
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::TaggedSigned),
    MemoryRepresentation::TaggedSigned
  );
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::TaggedPointer),
    MemoryRepresentation::TaggedPointer
  );
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::Tagged),
    MemoryRepresentation::AnyTagged
  );
  // 浮点 (Float16 保留)
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::Float16),
    MemoryRepresentation::Float16
  );
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::Float32),
    MemoryRepresentation::Float32
  );
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::Float64),
    MemoryRepresentation::Float64
  );
  // SandboxedPointer 唯一入格
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::SandboxedPointer),
    MemoryRepresentation::SandboxedPointer
  );
  // SIMD
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::Simd128),
    MemoryRepresentation::Simd128
  );
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::Simd256),
    MemoryRepresentation::Simd256
  );
  OK
}

/// SizeInBytes 压缩指针开 (kTaggedSizeLog2=2, kSystemPointerSizeLog2=3) 关键金标准
/// 覆盖 round1 未测的 tagged/compressed/pointer/Simd 尺寸分岔
#[test]
fn memory_size_in_bytes_compressed_pointer() -> Void {
  info!("> MemoryRepresentation size_in_bytes 压缩指针关键面");
  // kTaggedSizeLog2=2 (压缩指针开): AnyTagged/TaggedPointer/TaggedSigned/ProtectedPointer → 4
  assert_eq!(MemoryRepresentation::AnyTagged.size_in_bytes(), 4);
  assert_eq!(MemoryRepresentation::TaggedPointer.size_in_bytes(), 4);
  assert_eq!(MemoryRepresentation::TaggedSigned.size_in_bytes(), 4);
  assert_eq!(MemoryRepresentation::ProtectedPointer.size_in_bytes(), 4);
  // kSystemPointerSizeLog2=3: AnyUncompressed*/UncompressedTagged* → 8
  assert_eq!(
    MemoryRepresentation::AnyUncompressedTagged.size_in_bytes(),
    8
  );
  assert_eq!(
    MemoryRepresentation::UncompressedTaggedPointer.size_in_bytes(),
    8
  );
  assert_eq!(
    MemoryRepresentation::UncompressedTaggedSigned.size_in_bytes(),
    8
  );
  // 间接/可信指针 → 4 (kTaggedSizeLog2=2), 沙箱指针 → 8 (kSystemPointerSizeLog2=3)
  assert_eq!(MemoryRepresentation::IndirectPointer.size_in_bytes(), 4);
  assert_eq!(MemoryRepresentation::TrustedPointer.size_in_bytes(), 4);
  assert_eq!(MemoryRepresentation::SandboxedPointer.size_in_bytes(), 8);
  // Float16 → 2 (与 Int16/Uint16 同 log2=1)
  assert_eq!(MemoryRepresentation::Float16.size_in_bytes(), 2);
  assert_eq!(MemoryRepresentation::Float32.size_in_bytes(), 4);
  assert_eq!(MemoryRepresentation::Float64.size_in_bytes(), 8);
  // 整型窄位 (round1 已测, 保留交叉验证)
  assert_eq!(MemoryRepresentation::Int8.size_in_bytes(), 1);
  assert_eq!(MemoryRepresentation::Uint64.size_in_bytes(), 8);

  // 桥接验证: machine_rep_to_memory_rep
  let mem_rep = v8_turboshaft::machine_rep_to_memory_rep(LoadRepresentation::int32());
  assert_eq!(mem_rep, MemoryRepresentation::Int32);
  OK
}
