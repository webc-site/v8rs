[English](#en) | [中文](#zh)

---

<a name="en"></a>

#

---

<a name="zh"></a>

# v8-objects-rest 推迟台账

深水面 todo! 登记 (原函数名 / 前置依赖 / 解锁 crate)。解锁条件就位后即填体并删除本表对应行。

基准配置: 非压缩 / 沙箱关 / 恒常池关 / 跳转表关 / gearbox 关。

| # | 文件 | 原版函数 (v8: src/…) | 前置依赖 | 解锁 crate/席 |
|---|---|---|---|---|
| 1 | abstract_code.rs | AbstractCode::contains (Code 支) | Isolate 形参 + Code::contains(isolate, ptr) | execution/heap 席 (v8-execution-frames) |
| 2 | abstract_code.rs | AbstractCode::SourcePosition | Code/BytecodeArray 两支位置表 (SourcePositionTableIterator + 字节码位置表面) | v8-codegen-core |
| 3 | bytecode_array.rs | BytecodeArray::SourcePositionTable | Isolate::Current()->heap()->empty_trusted_byte_array() 空表哨兵 | isolate/heap 席 |
| 4 | bytecode_array.rs | BytecodeArray::MarkVerified | Publish (沙箱支) + BytecodeWrapper::set_bytecode (trusted-object 类本体) | sandbox/trusted-object 席 |
| 5 | code.rs | Code::metadata_start (嵌入内建支) | EmbeddedData::FromBlob().MetadataStartOf(builtin_id) 嵌入 blob 面 | v8-snapshot |
| 6 | code.rs | Code::sandboxing_mode (内建支) | Builtins::SandboxingModeOf 内建表 | v8-init/builtins |
| 7 | code.rs | Code::SourcePosition | SourcePositionTableIterator (src/codegen/source-position-table.cc) | v8-codegen-core |
| 8 | instruction_stream.rs | InstructionStream::Relocate | WritableJitAllocation + RelocIterator 逐条平移 | v8-codegen-core + heap 席 |
| 9 | instruction_stream.rs | InstructionStream::Initialize | CodeDesc / WritableJitAllocation 装配面 | v8-codegen-core |
| 10 | instruction_stream.rs | InstructionStream::Finalize | SetTaggedField 写屏障桩 + RelocateFromDesc | heap 席 + v8-codegen-core |
| 11 | deoptimization_data_inl.rs | DeoptimizationData::GetSharedFunctionInfo | SharedFunctionInfoWrapper 类本体 + shared_info() (元素读面已就位) | shared-function-info 席 |

- [已解锁回填 (本轮 [轮7 W7O3c] 登记)](#已解锁回填-本轮-轮7-w7o3c-登记)

## 已解锁回填 (本轮 [轮7 W7O3c] 登记)

fixed-array / byte-array 尺寸面 (v8-objects-arrays) 交付后, 以下原推迟项就地填体:

- `BytecodeArray::SizeIncludingMetadata` — TrustedFixedArray::Size / Trusted(ByteArray)::AllocatedSize 三支累加;
- `InstructionStream::relocation_start / relocation_end / relocation_size` — TrustedByteArray begin/end/ulength;
- `DeoptimizationData::ulength / literal_array / size_including_header / DeoptCount / GetBytecodeOffsetOrBuiltinContinuationId / SetBytecodeOffset` — ProtectedFixedArray 元素区;
- `DeoptimizationLiteralArray::ulength / get / get_raw / set` — TrustedWeakFixedArray 弱槽区 (三分支换装, 基线沙箱关两支)。

