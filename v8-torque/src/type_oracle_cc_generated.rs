// 由 tool/src/torque/type-oracle.cc.js 从 v8/src/torque/type-oracle.{h,cc} 生成; 修订就地改, 复跑覆盖
// TypeOracle 方法面对数清单

/// TypeOracle 静态方法面 (原版 type-oracle.h)
pub mod type_oracle_surface {
  /// 工厂 (Get* 前缀, 74 项)
  pub const FACTORIES: [&str; 74] = [
    "GetAbstractType",
    "GetStructType",
    "GetBitFieldStructType",
    "GetClassType",
    "GetBuiltinPointerType",
    "GetGenericTypeInstance",
    "GetReferenceGeneric",
    "GetConstReferenceGeneric",
    "GetMutableReferenceGeneric",
    "GetMutableSliceGeneric",
    "GetConstSliceGeneric",
    "GetWeakGeneric",
    "GetSmiTaggedGeneric",
    "GetLazyGeneric",
    "GetReferenceType",
    "GetConstReferenceType",
    "GetMutableReferenceType",
    "GetMutableSliceType",
    "GetConstSliceType",
    "GetUnionType",
    "GetUnionType",
    "GetTopType",
    "GetArgumentsType",
    "GetBoolType",
    "GetConstexprBoolType",
    "GetConstexprStringType",
    "GetConstexprIntPtrType",
    "GetConstexprInstanceTypeType",
    "GetVoidType",
    "GetRawPtrType",
    "GetExternalPointerType",
    "GetCppHeapPointerType",
    "GetTrustedPointerType",
    "GetProtectedPointerType",
    "GetDispatchHandleType",
    "GetMapType",
    "GetObjectType",
    "GetHeapObjectType",
    "GetTaggedZeroPatternType",
    "GetJSAnyType",
    "GetJSObjectType",
    "GetTaggedType",
    "GetStrongTaggedType",
    "GetUninitializedHeapObjectType",
    "GetSmiType",
    "GetConstStringType",
    "GetStringType",
    "GetNumberType",
    "GetIntPtrType",
    "GetUIntPtrType",
    "GetInt64Type",
    "GetUint64Type",
    "GetInt32Type",
    "GetUint32Type",
    "GetUint31Type",
    "GetInt16Type",
    "GetUint16Type",
    "GetInt8Type",
    "GetUint8Type",
    "GetFloat64Type",
    "GetFloat64OrUndefinedOrHoleType",
    "GetConstFloat64Type",
    "GetIntegerLiteralType",
    "GetNeverType",
    "GetConstInt31Type",
    "GetConstInt32Type",
    "GetContextType",
    "GetNoContextType",
    "GetNativeContextType",
    "GetJSFunctionType",
    "GetUninitializedIteratorType",
    "GetFixedArrayBaseType",
    "GetAggregateTypes",
    "GetClasses",
  ];
  /// 非工厂静态面 (Create*/Match* 等, 2 项)
  pub const OTHERS: [&str; 2] = [
    "MatchReferenceGeneric",
    "CreateGenericTypeInstantiationNamespace",
  ];
}

/// type-oracle.cc 方法定义头清单 (对数基准)
/// void TypeOracle::FinalizeAggregateTypes()
/// const Type* TypeOracle::GetGenericTypeInstance(GenericType* generic_type, TypeVector arg_types)
/// Namespace* TypeOracle::CreateGenericTypeInstantiationNamespace()
/// std::vector<const ClassType*> TypeOracle::GetClasses()
/// std::optional<const Type*> TypeOracle::MatchReferenceGeneric( const Type* reference_type, bool* is_const)
pub mod type_oracle_cc_skel {
  pub const DEF_COUNT: usize = 5;
}
