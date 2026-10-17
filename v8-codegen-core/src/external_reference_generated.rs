// 由 tool/src/codegen/external-reference.h.js 从 v8/src/codegen/external-reference.{h,cc} 生成; 修订就地改, 复跑覆盖
// EXTERNAL_REFERENCE_LIST X-macro 全量展开

// 展开规则: 递归拼接 EXTERNAL_REFERENCE_LIST* 宏; 条件宏 (#ifdef 分支) 全部
// 并入并以 doc 注明触发条件, 运行时门控由 Developer 接入时裁定。
// 语义面: ExternalReference 类方法实现登记跳过。

/// 外部引用名册 (原 EXTERNAL_REFERENCE_LIST(V) 展开, 191 条)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExternalReferenceName {
  /// abort_with_reason
  Abort_With_Reason,
  /// abort_with_sandbox_violation
  Abort_With_Sandbox_Violation,
  /// v8_flags.log_or_trace_osr
  Address_Of_Log_Or_Trace_Osr,
  /// v8_flags.builtin_subclassing
  Address_Of_Builtin_Subclassing_Flag,
  /// double_absolute_constant
  Address_Of_Double_Abs_Constant,
  /// double_negate_constant
  Address_Of_Double_Neg_Constant,
  /// address_of_enable_experimental_regexp_engine
  Address_Of_Enable_Experimental_Regexp_Engine,
  /// fp16_absolute_constant
  Address_Of_Fp16_Abs_Constant,
  /// fp16_negate_constant
  Address_Of_Fp16_Neg_Constant,
  /// float_absolute_constant
  Address_Of_Float_Abs_Constant,
  /// float_negate_constant
  Address_Of_Float_Neg_Constant,
  /// log10_offset_table
  Address_Of_Log10_Offset_Table,
  /// LDoubleConstant::min_int
  Address_Of_Min_Int,
  /// v8_flags.js_pr_3883
  Address_Of_Js_Pr_3883_Flag,
  /// v8_flags.mock_arraybuffer_allocator
  Address_Of_Mock_Arraybuffer_Allocator_Flag,
  /// LDoubleConstant::one_half
  Address_Of_One_Half,
  /// TracingFlags::runtime_stats
  Address_Of_Runtime_Stats_Flag,
  /// v8_flags.shared_string_table
  Address_Of_Shared_String_Table_Flag,
  /// the_hole_nan
  Address_Of_The_Hole_Nan,
  /// v8_flags.track_array_buffer_view
  Address_Of_Track_Array_Buffer_Views_Flag,
  /// uint32_bias
  Address_Of_Uint32_Bias,
  /// AllocateAndInitializeYoungExternalPointerTableEntry
  Allocate_And_Initialize_Young_External_Pointer_Table_Entry,
  /// BaselinePCForNextExecutedBytecode
  Baseline_Pc_For_Next_Executed_Bytecode,
  /// Bytecodes::bytecode_size_table_address
  Bytecode_Size_Table_Address,
  /// check_object_type
  Check_Object_Type,
  /// ComputeSeededHash
  Compute_Integer_Hash,
  /// Deoptimizer::ComputeOutputFrames()
  Compute_Output_Frames_Function,
  /// copy_fast_number_jsarray_elements_to_typed_array
  Copy_Fast_Number_Jsarray_Elements_To_Typed_Array,
  /// copy_typed_array_elements_slice
  Copy_Typed_Array_Elements_Slice,
  /// copy_typed_array_elements_to_typed_array
  Copy_Typed_Array_Elements_To_Typed_Array,
  /// cpu_features
  Cpu_Features,
  /// DebugBreakAtEntry
  Debug_Break_At_Entry_Function,
  /// DebugGetCoverageInfo
  Debug_Get_Coverage_Info_Function,
  /// HandleScope::DeleteExtensions
  Delete_Handle_Scope_Extensions,
  /// Heap::EphemeronKeyWriteBarrierFromCode
  Ephemeron_Key_Write_Barrier_Function,
  /// JSDate::GetField
  Get_Date_Field_Function,
  /// get_or_create_hash_raw
  Get_Or_Create_Hash_Raw,
  /// GsabByteLength
  Gsab_Byte_Length,
  /// ieee754_fp64_to_fp16_raw_bits
  Ieee754_Fp64_To_Fp16_Raw_Bits,
  /// ieee754_fp64_raw_bits_to_fp16_raw_bits_for_32bit_arch
  Ieee754_Fp64_Raw_Bits_To_Fp16_Raw_Bits_For_32bit_Arch,
  /// ieee754_fp16_raw_bits_to_fp32_raw_bits
  Ieee754_Fp16_Raw_Bits_To_Fp32_Raw_Bits,
  /// base::ieee754::acos
  Ieee754_Acos_Function,
  /// base::ieee754::acosh
  Ieee754_Acosh_Function,
  /// base::ieee754::asin
  Ieee754_Asin_Function,
  /// base::ieee754::asinh
  Ieee754_Asinh_Function,
  /// base::ieee754::atan
  Ieee754_Atan_Function,
  /// base::ieee754::atan2
  Ieee754_Atan2_Function,
  /// base::ieee754::atanh
  Ieee754_Atanh_Function,
  /// base::ieee754::cbrt
  Ieee754_Cbrt_Function,
  /// base::ieee754::cos
  Ieee754_Cos_Function,
  /// base::ieee754::cosh
  Ieee754_Cosh_Function,
  /// base::ieee754::exp
  Ieee754_Exp_Function,
  /// base::ieee754::expm1
  Ieee754_Expm1_Function,
  /// base::ieee754::log
  Ieee754_Log_Function,
  /// base::ieee754::log10
  Ieee754_Log10_Function,
  /// base::ieee754::log1p
  Ieee754_Log1p_Function,
  /// base::ieee754::log2
  Ieee754_Log2_Function,
  /// math::pow
  Ieee754_Pow_Function,
  /// base::ieee754::sin
  Ieee754_Sin_Function,
  /// base::ieee754::sinh
  Ieee754_Sinh_Function,
  /// base::ieee754::tan
  Ieee754_Tan_Function,
  /// base::ieee754::tanh
  Ieee754_Tanh_Function,
  /// Heap::InsertIntoRememberedSetFromCode
  Insert_Remembered_Set_Function,
  /// JSObject::InvalidatePrototypeChains()
  Invalidate_Prototype_Chains_Function,
  /// InvokeAccessorGetterCallback
  Invoke_Accessor_Getter_Callback,
  /// InvokeNamedInterceptorGetterCallback
  Invoke_Named_Interceptor_Getter_Callback,
  /// InvokeNamedInterceptorSetterCallback
  Invoke_Named_Interceptor_Setter_Callback,
  /// InvokeFunctionCallbackGeneric
  Invoke_Function_Callback_Generic,
  /// InvokeFunctionCallbackOptimized
  Invoke_Function_Callback_Optimized,
  /// jsarray_array_join_concat_to_sequential_string
  Jsarray_Array_Join_Concat_To_Sequential_String,
  /// jsreceiver_create_identity_hash
  Jsreceiver_Create_Identity_Hash,
  /// libc_memchr
  Libc_Memchr_Function,
  /// libc_memcpy
  Libc_Memcpy_Function,
  /// libc_memmove
  Libc_Memmove_Function,
  /// libc_memset
  Libc_Memset_Function,
  /// relaxed_memcpy
  Relaxed_Memcpy_Function,
  /// relaxed_memmove
  Relaxed_Memmove_Function,
  /// mod_two_doubles
  Mod_Two_Doubles_Operation,
  /// MutableBigInt_AbsoluteAddAndCanonicalize
  Mutable_Big_Int_Absolute_Add_And_Canonicalize_Function,
  /// MutableBigInt_AbsoluteSubAndCanonicalize
  Mutable_Big_Int_Absolute_Sub_And_Canonicalize_Function,
  /// MutableBigInt_AbsoluteMulAndCanonicalize
  Mutable_Big_Int_Absolute_Mul_And_Canonicalize_Function,
  /// MutableBigInt_AbsoluteDivAndCanonicalize
  Mutable_Big_Int_Absolute_Div_And_Canonicalize_Function,
  /// MutableBigInt_AbsoluteModAndCanonicalize
  Mutable_Big_Int_Absolute_Mod_And_Canonicalize_Function,
  /// MutableBigInt_BitwiseAndPosPosAndCanonicalize
  Mutable_Big_Int_Bitwise_And_Pp_And_Canonicalize_Function,
  /// MutableBigInt_BitwiseAndNegNegAndCanonicalize
  Mutable_Big_Int_Bitwise_And_Nn_And_Canonicalize_Function,
  /// MutableBigInt_BitwiseAndPosNegAndCanonicalize
  Mutable_Big_Int_Bitwise_And_Pn_And_Canonicalize_Function,
  /// MutableBigInt_BitwiseOrPosPosAndCanonicalize
  Mutable_Big_Int_Bitwise_Or_Pp_And_Canonicalize_Function,
  /// MutableBigInt_BitwiseOrNegNegAndCanonicalize
  Mutable_Big_Int_Bitwise_Or_Nn_And_Canonicalize_Function,
  /// MutableBigInt_BitwiseOrPosNegAndCanonicalize
  Mutable_Big_Int_Bitwise_Or_Pn_And_Canonicalize_Function,
  /// MutableBigInt_BitwiseXorPosPosAndCanonicalize
  Mutable_Big_Int_Bitwise_Xor_Pp_And_Canonicalize_Function,
  /// MutableBigInt_BitwiseXorNegNegAndCanonicalize
  Mutable_Big_Int_Bitwise_Xor_Nn_And_Canonicalize_Function,
  /// MutableBigInt_BitwiseXorPosNegAndCanonicalize
  Mutable_Big_Int_Bitwise_Xor_Pn_And_Canonicalize_Function,
  /// MutableBigInt_LeftShiftAndCanonicalize
  Mutable_Big_Int_Left_Shift_And_Canonicalize_Function,
  /// RightShiftResultLength
  Big_Int_Right_Shift_Result_Length_Function,
  /// MutableBigInt_RightShiftAndCanonicalize
  Mutable_Big_Int_Right_Shift_And_Canonicalize_Function,
  /// Deoptimizer::New()
  New_Deoptimizer_Function,
  /// orderedhashmap_gethash_raw
  Orderedhashmap_Gethash_Raw,
  /// printf
  Printf_Function,
  /// MathRandom::InitializeAndMaybeRefillCache
  Initialize_And_Maybe_Refill_Math_Random,
  /// search_string_raw_one_one
  Search_String_Raw_One_One,
  /// search_string_raw_one_two
  Search_String_Raw_One_Two,
  /// search_string_raw_two_one
  Search_String_Raw_Two_One,
  /// search_string_raw_two_two
  Search_String_Raw_Two_Two,
  /// string_write_to_flat_one_byte
  String_Write_To_Flat_One_Byte,
  /// string_write_to_flat_two_byte
  String_Write_To_Flat_Two_Byte,
  /// v8_flags.additive_safe_int_feedback
  Additive_Safe_Int_Feedback_Flag,
  /// external_one_byte_string_get_chars
  External_One_Byte_String_Get_Chars,
  /// external_two_byte_string_get_chars
  External_Two_Byte_String_Get_Chars,
  /// smi_lexicographic_compare_function
  Smi_Lexicographic_Compare_Function,
  /// String::ToArrayIndex
  String_To_Array_Index_Function,
  /// array_indexof_includes_smi_or_object
  Array_Indexof_Includes_Smi_Or_Object,
  /// array_indexof_includes_double
  Array_Indexof_Includes_Double,
  /// Utf16::HasUnpairedSurrogate
  Has_Unpaired_Surrogate,
  /// Utf16::ReplaceUnpairedSurrogates
  Replace_Unpaired_Surrogates,
  /// try_string_to_index_or_lookup_existing
  Try_String_To_Index_Or_Lookup_Existing,
  /// string_from_forward_table
  String_From_Forward_Table,
  /// raw_hash_from_forward_table
  Raw_Hash_From_Forward_Table,
  /// name_dictionary_lookup_forwarded_string
  Name_Dictionary_Lookup_Forwarded_String,
  /// name_dictionary_find_insertion_entry_forwarded_string
  Name_Dictionary_Find_Insertion_Entry_Forwarded_String,
  /// global_dictionary_lookup_forwarded_string
  Global_Dictionary_Lookup_Forwarded_String,
  /// global_dictionary_find_insertion_entry_forwarded_string
  Global_Dictionary_Find_Insertion_Entry_Forwarded_String,
  /// name_to_index_hashtable_lookup_forwarded_string
  Name_To_Index_Hashtable_Lookup_Forwarded_String,
  /// name_to_index_hashtable_find_insertion_entry_forwarded_string
  Name_To_Index_Hashtable_Find_Insertion_Entry_Forwarded_String,
  /// simple_name_dictionary_lookup_forwarded_string
  Simple_Name_Dictionary_Lookup_Forwarded_String,
  /// simple_name_dictionary_find_insertion_entry_forwarded_string
  Simple_Name_Dictionary_Find_Insertion_Entry_Forwarded_String,
  /// Heap::VerifySkippedIndirectWriteBarrier
  Verify_Skipped_Indirect_Write_Barrier,
  /// Heap::VerifySkippedWriteBarrier
  Verify_Skipped_Write_Barrier,
  /// wasm_i8x16_swizzle_mask
  Address_Of_Wasm_I8x16_Swizzle_Mask,
  /// wasm_i8x16_popcnt_mask
  Address_Of_Wasm_I8x16_Popcnt_Mask,
  /// wasm_i8x16_splat_0x01
  Address_Of_Wasm_I8x16_Splat_0x01,
  /// wasm_i8x16_splat_0x0f
  Address_Of_Wasm_I8x16_Splat_0x0f,
  /// wasm_i8x16_splat_0x33
  Address_Of_Wasm_I8x16_Splat_0x33,
  /// wasm_i8x16_splat_0x55
  Address_Of_Wasm_I8x16_Splat_0x55,
  /// wasm_16x8_splat_0x0001
  Address_Of_Wasm_I16x8_Splat_0x0001,
  /// wasm_f64x2_convert_low_i32x4_u_int_mask
  Address_Of_Wasm_F64x2_Convert_Low_I32x4_U_Int_Mask,
  /// CpuFeatures::supports_simd_128_address
  Supports_Simd_128_Address,
  /// wasm_double_2_power_52
  Address_Of_Wasm_Double_2_Power_52,
  /// wasm_int32_max_as_double
  Address_Of_Wasm_Int32_Max_As_Double,
  /// wasm_uint32_max_as_double
  Address_Of_Wasm_Uint32_Max_As_Double,
  /// wasm_int32_overflow_as_float
  Address_Of_Wasm_Int32_Overflow_As_Float,
  /// wasm_i32x8_int32_overflow_as_float
  Address_Of_Wasm_I32x8_Int32_Overflow_As_Float,
  /// CpuFeatures::supports_cetss_address
  Supports_Cetss_Address,
  /// WriteBarrier::MarkingFromCode
  Write_Barrier_Marking_From_Code_Function,
  /// WriteBarrier::IndirectPointerMarkingFromCode
  Write_Barrier_Indirect_Pointer_Marking_From_Code_Function,
  /// WriteBarrier::SharedMarkingFromCode
  Write_Barrier_Shared_Marking_From_Code_Function,
  /// WriteBarrier::SharedFromCode
  Shared_Barrier_From_Code_Function,
  /// MicrotaskQueue::CallEnqueueMicrotask
  Call_Enqueue_Microtask_Function,
  /// call_enter_context_function
  Call_Enter_Context_Function,
  /// int64_mul_high_function
  Int64_Mul_High_Function,
  /// atomic_pair_load_function
  Atomic_Pair_Load_Function,
  /// atomic_pair_store_function
  Atomic_Pair_Store_Function,
  /// atomic_pair_add_function
  Atomic_Pair_Add_Function,
  /// atomic_pair_sub_function
  Atomic_Pair_Sub_Function,
  /// atomic_pair_and_function
  Atomic_Pair_And_Function,
  /// atomic_pair_or_function
  Atomic_Pair_Or_Function,
  /// atomic_pair_xor_function
  Atomic_Pair_Xor_Function,
  /// atomic_pair_exchange_function
  Atomic_Pair_Exchange_Function,
  /// atomic_pair_compare_exchange_function
  Atomic_Pair_Compare_Exchange_Function,
  /// regexp::RegExpMacroAssembler::CaseInsensitiveCompareUnicode()
  Re_Case_Insensitive_Compare_Unicode,
  /// regexp::RegExpMacroAssembler::CaseInsensitiveCompareNonUnicode()
  Re_Case_Insensitive_Compare_Non_Unicode,
  /// regexp::RegExpMacroAssembler::IsCharacterInRangeArray()
  Re_Is_Character_In_Range_Array,
  /// regexp::RegExpMacroAssembler*::CheckStackGuardState()
  Re_Check_Stack_Guard_State,
  /// NativeRegExpMacroAssembler::GrowStack()
  Re_Grow_Stack,
  /// regexp::RegExpMacroAssembler::word_character_map
  Re_Word_Character_Map,
  /// IrregexpInterpreter::MatchForCallFromJs
  Re_Match_For_Call_From_Js,
  /// ExperimentalRegExp::MatchForCallFromJs
  Re_Experimental_Match_For_Call_From_Js,
  /// RegExp::AtomExecRaw
  Re_Atom_Exec_Raw,
  /// regexp::ResultsCache::EnterRaw
  Re_Split_Cache_Enter,
  /// regexp::ResultVector::Allocate
  Allocate_Regexp_Result_Vector,
  /// regexp::ResultVector::Free
  Free_Regexp_Result_Vector,
  /// v8_flags.trace_regexp_exec (原 #if V8_ENABLE_REGEXP_DIAGNOSTICS)
  Address_Of_Trace_Regexp_Exec,
  /// RegExp::TraceExecutionBegin (原 #if V8_ENABLE_REGEXP_DIAGNOSTICS)
  Address_Of_Regexp_Trace_Begin,
  /// RegExp::TraceExecutionEnd (原 #if V8_ENABLE_REGEXP_DIAGNOSTICS)
  Address_Of_Regexp_Trace_End,
  /// TypedArrayAndRabGsabTypedArrayElementsKindShifts
  Typed_Array_And_Rab_Gsab_Typed_Array_Elements_Kind_Shifts,
  /// TypedArrayAndRabGsabTypedArrayElementsKindSizes
  Typed_Array_And_Rab_Gsab_Typed_Array_Elements_Kind_Sizes,
  /// AllocateBuffer
  Allocate_Buffer,
  /// compare_operation_feedback_transition_table
  Compare_Operation_Feedback_Transition_Table,
  /// compare_operation_feedback_encode_table
  Compare_Operation_Feedback_Encode_Table,
  /// binary_operation_feedback_transition_table
  Binary_Operation_Feedback_Transition_Table,
  /// binary_operation_feedback_encode_table
  Binary_Operation_Feedback_Encode_Table,
  /// intl_convert_one_byte_to_lower (原 #if V8_INTL_SUPPORT)
  Intl_Convert_One_Byte_To_Lower,
  /// intl_to_latin1_lower_table (原 #if V8_INTL_SUPPORT)
  Intl_To_Latin1_Lower_Table,
  /// Intl::AsciiCollationWeightsL1 (原 #if V8_INTL_SUPPORT)
  Intl_Ascii_Collation_Weights_L1,
  /// Intl::AsciiCollationWeightsL3 (原 #if V8_INTL_SUPPORT)
  Intl_Ascii_Collation_Weights_L3,
  /// Sandbox::base() (原 #if V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)
  Sandbox_Base_Address,
  /// Sandbox::end() (原 #if V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)
  Sandbox_End_Address,
  /// EmptyBackingStoreBuffer() (原 #if V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)
  Empty_Backing_Store_Buffer,
  /// BasePage::Table() (原 #if V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)
  Memory_Chunk_Metadata_Table_Address,
  /// SandboxHardwareSupport::sandboxed_mode_pkey_mask() (原 #if V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES)
  Sandboxed_Mode_Pkey_Mask_Address,
  /// v8_flags.cet_compatible (原 #if V8_ENABLE_CET_SHADOW_STACK)
  Address_Of_Cet_Compatible_Flag,
  /// Deoptimizer::EnsureValidReturnAddress() (原 #if V8_ENABLE_CET_SHADOW_STACK)
  Ensure_Valid_Return_Address,
}

/// 原版描述串表 (与 ExternalReferenceName 判别值同序)
#[rustfmt::skip]
pub static EXTERNAL_REFERENCE_DESCS: [&str; 191] = [
  "abort_with_reason",
  "abort_with_sandbox_violation",
  "v8_flags.log_or_trace_osr",
  "v8_flags.builtin_subclassing",
  "double_absolute_constant",
  "double_negate_constant",
  "address_of_enable_experimental_regexp_engine",
  "fp16_absolute_constant",
  "fp16_negate_constant",
  "float_absolute_constant",
  "float_negate_constant",
  "log10_offset_table",
  "LDoubleConstant::min_int",
  "v8_flags.js_pr_3883",
  "v8_flags.mock_arraybuffer_allocator",
  "LDoubleConstant::one_half",
  "TracingFlags::runtime_stats",
  "v8_flags.shared_string_table",
  "the_hole_nan",
  "v8_flags.track_array_buffer_view",
  "uint32_bias",
  "AllocateAndInitializeYoungExternalPointerTableEntry",
  "BaselinePCForNextExecutedBytecode",
  "Bytecodes::bytecode_size_table_address",
  "check_object_type",
  "ComputeSeededHash",
  "Deoptimizer::ComputeOutputFrames()",
  "copy_fast_number_jsarray_elements_to_typed_array",
  "copy_typed_array_elements_slice",
  "copy_typed_array_elements_to_typed_array",
  "cpu_features",
  "DebugBreakAtEntry",
  "DebugGetCoverageInfo",
  "HandleScope::DeleteExtensions",
  "Heap::EphemeronKeyWriteBarrierFromCode",
  "JSDate::GetField",
  "get_or_create_hash_raw",
  "GsabByteLength",
  "ieee754_fp64_to_fp16_raw_bits",
  "ieee754_fp64_raw_bits_to_fp16_raw_bits_for_32bit_arch",
  "ieee754_fp16_raw_bits_to_fp32_raw_bits",
  "base::ieee754::acos",
  "base::ieee754::acosh",
  "base::ieee754::asin",
  "base::ieee754::asinh",
  "base::ieee754::atan",
  "base::ieee754::atan2",
  "base::ieee754::atanh",
  "base::ieee754::cbrt",
  "base::ieee754::cos",
  "base::ieee754::cosh",
  "base::ieee754::exp",
  "base::ieee754::expm1",
  "base::ieee754::log",
  "base::ieee754::log10",
  "base::ieee754::log1p",
  "base::ieee754::log2",
  "math::pow",
  "base::ieee754::sin",
  "base::ieee754::sinh",
  "base::ieee754::tan",
  "base::ieee754::tanh",
  "Heap::InsertIntoRememberedSetFromCode",
  "JSObject::InvalidatePrototypeChains()",
  "InvokeAccessorGetterCallback",
  "InvokeNamedInterceptorGetterCallback",
  "InvokeNamedInterceptorSetterCallback",
  "InvokeFunctionCallbackGeneric",
  "InvokeFunctionCallbackOptimized",
  "jsarray_array_join_concat_to_sequential_string",
  "jsreceiver_create_identity_hash",
  "libc_memchr",
  "libc_memcpy",
  "libc_memmove",
  "libc_memset",
  "relaxed_memcpy",
  "relaxed_memmove",
  "mod_two_doubles",
  "MutableBigInt_AbsoluteAddAndCanonicalize",
  "MutableBigInt_AbsoluteSubAndCanonicalize",
  "MutableBigInt_AbsoluteMulAndCanonicalize",
  "MutableBigInt_AbsoluteDivAndCanonicalize",
  "MutableBigInt_AbsoluteModAndCanonicalize",
  "MutableBigInt_BitwiseAndPosPosAndCanonicalize",
  "MutableBigInt_BitwiseAndNegNegAndCanonicalize",
  "MutableBigInt_BitwiseAndPosNegAndCanonicalize",
  "MutableBigInt_BitwiseOrPosPosAndCanonicalize",
  "MutableBigInt_BitwiseOrNegNegAndCanonicalize",
  "MutableBigInt_BitwiseOrPosNegAndCanonicalize",
  "MutableBigInt_BitwiseXorPosPosAndCanonicalize",
  "MutableBigInt_BitwiseXorNegNegAndCanonicalize",
  "MutableBigInt_BitwiseXorPosNegAndCanonicalize",
  "MutableBigInt_LeftShiftAndCanonicalize",
  "RightShiftResultLength",
  "MutableBigInt_RightShiftAndCanonicalize",
  "Deoptimizer::New()",
  "orderedhashmap_gethash_raw",
  "printf",
  "MathRandom::InitializeAndMaybeRefillCache",
  "search_string_raw_one_one",
  "search_string_raw_one_two",
  "search_string_raw_two_one",
  "search_string_raw_two_two",
  "string_write_to_flat_one_byte",
  "string_write_to_flat_two_byte",
  "v8_flags.additive_safe_int_feedback",
  "external_one_byte_string_get_chars",
  "external_two_byte_string_get_chars",
  "smi_lexicographic_compare_function",
  "String::ToArrayIndex",
  "array_indexof_includes_smi_or_object",
  "array_indexof_includes_double",
  "Utf16::HasUnpairedSurrogate",
  "Utf16::ReplaceUnpairedSurrogates",
  "try_string_to_index_or_lookup_existing",
  "string_from_forward_table",
  "raw_hash_from_forward_table",
  "name_dictionary_lookup_forwarded_string",
  "name_dictionary_find_insertion_entry_forwarded_string",
  "global_dictionary_lookup_forwarded_string",
  "global_dictionary_find_insertion_entry_forwarded_string",
  "name_to_index_hashtable_lookup_forwarded_string",
  "name_to_index_hashtable_find_insertion_entry_forwarded_string",
  "simple_name_dictionary_lookup_forwarded_string",
  "simple_name_dictionary_find_insertion_entry_forwarded_string",
  "Heap::VerifySkippedIndirectWriteBarrier",
  "Heap::VerifySkippedWriteBarrier",
  "wasm_i8x16_swizzle_mask",
  "wasm_i8x16_popcnt_mask",
  "wasm_i8x16_splat_0x01",
  "wasm_i8x16_splat_0x0f",
  "wasm_i8x16_splat_0x33",
  "wasm_i8x16_splat_0x55",
  "wasm_16x8_splat_0x0001",
  "wasm_f64x2_convert_low_i32x4_u_int_mask",
  "CpuFeatures::supports_simd_128_address",
  "wasm_double_2_power_52",
  "wasm_int32_max_as_double",
  "wasm_uint32_max_as_double",
  "wasm_int32_overflow_as_float",
  "wasm_i32x8_int32_overflow_as_float",
  "CpuFeatures::supports_cetss_address",
  "WriteBarrier::MarkingFromCode",
  "WriteBarrier::IndirectPointerMarkingFromCode",
  "WriteBarrier::SharedMarkingFromCode",
  "WriteBarrier::SharedFromCode",
  "MicrotaskQueue::CallEnqueueMicrotask",
  "call_enter_context_function",
  "int64_mul_high_function",
  "atomic_pair_load_function",
  "atomic_pair_store_function",
  "atomic_pair_add_function",
  "atomic_pair_sub_function",
  "atomic_pair_and_function",
  "atomic_pair_or_function",
  "atomic_pair_xor_function",
  "atomic_pair_exchange_function",
  "atomic_pair_compare_exchange_function",
  "regexp::RegExpMacroAssembler::CaseInsensitiveCompareUnicode()",
  "regexp::RegExpMacroAssembler::CaseInsensitiveCompareNonUnicode()",
  "regexp::RegExpMacroAssembler::IsCharacterInRangeArray()",
  "regexp::RegExpMacroAssembler*::CheckStackGuardState()",
  "NativeRegExpMacroAssembler::GrowStack()",
  "regexp::RegExpMacroAssembler::word_character_map",
  "IrregexpInterpreter::MatchForCallFromJs",
  "ExperimentalRegExp::MatchForCallFromJs",
  "RegExp::AtomExecRaw",
  "regexp::ResultsCache::EnterRaw",
  "regexp::ResultVector::Allocate",
  "regexp::ResultVector::Free",
  "v8_flags.trace_regexp_exec",
  "RegExp::TraceExecutionBegin",
  "RegExp::TraceExecutionEnd",
  "TypedArrayAndRabGsabTypedArrayElementsKindShifts",
  "TypedArrayAndRabGsabTypedArrayElementsKindSizes",
  "AllocateBuffer",
  "compare_operation_feedback_transition_table",
  "compare_operation_feedback_encode_table",
  "binary_operation_feedback_transition_table",
  "binary_operation_feedback_encode_table",
  "intl_convert_one_byte_to_lower",
  "intl_to_latin1_lower_table",
  "Intl::AsciiCollationWeightsL1",
  "Intl::AsciiCollationWeightsL3",
  "Sandbox::base()",
  "Sandbox::end()",
  "EmptyBackingStoreBuffer()",
  "BasePage::Table()",
  "SandboxHardwareSupport::sandboxed_mode_pkey_mask()",
  "v8_flags.cet_compatible",
  "Deoptimizer::EnsureValidReturnAddress()",
];

/// 条件分支登记表 (None = 无条件; Some(条件) = 原 #if 门控)
#[rustfmt::skip]
pub static EXTERNAL_REFERENCE_CONDITIONS: [Option<&str>; 191] = [
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  Some("#if V8_ENABLE_REGEXP_DIAGNOSTICS"),
  Some("#if V8_ENABLE_REGEXP_DIAGNOSTICS"),
  Some("#if V8_ENABLE_REGEXP_DIAGNOSTICS"),
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  Some("#if V8_INTL_SUPPORT"),
  Some("#if V8_INTL_SUPPORT"),
  Some("#if V8_INTL_SUPPORT"),
  Some("#if V8_INTL_SUPPORT"),
  Some("#if V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"),
  Some("#if V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"),
  Some("#if V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"),
  Some("#if V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"),
  Some("#if V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"),
  Some("#if V8_ENABLE_CET_SHADOW_STACK"),
  Some("#if V8_ENABLE_CET_SHADOW_STACK"),
];

// 原 .cc 计数锚:
/// 独立 (共享) 外部引用数: undefined
pub const EXTERNAL_REFERENCE_COUNT_ISOLATE_INDEPENDENT: usize = undefined;
/// 依赖 isolate 的外部引用数: undefined
pub const EXTERNAL_REFERENCE_COUNT_ISOLATE_DEPENDENT: usize = undefined;

// —— 提取不变式断言 ——
const _: () = {
  assert!(ExternalReferenceName::COUNT == 191);
  assert!(EXTERNAL_REFERENCE_DESCS.len() == 191);
  assert!(EXTERNAL_REFERENCE_CONDITIONS.len() == 191);
  assert!(EXTERNAL_REFERENCE_DESCS[0].len() > 0);
};
