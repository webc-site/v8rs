// 由 tool/tool/src/codegen/csa/tnode.js 从 v8/v8/src/codegen/tnode.{h,cc} 生成; 修订就地改, 复跑覆盖
// TNode 强类型 SSA 节点与机器表示标记体系

use core::{
  fmt,
  hash::{Hash, Hasher},
  marker::PhantomData,
  mem::size_of,
};

/// SSA 节点唯一标识符 (基于无符号整数，0 为空/无效节点)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u32);

impl NodeId {
  /// 无效/空节点
  pub const INVALID: Self = Self(0);

  #[inline]
  #[must_use]
  pub const fn is_valid(self) -> bool {
    self.0 != 0
  }

  #[inline]
  #[must_use]
  pub const fn raw(self) -> u32 {
    self.0
  }
}

impl fmt::Display for NodeId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_valid() {
      write!(f, "Node#{}", self.0)
    } else {
      write!(f, "Node#INVALID")
    }
  }
}

/// 强类型 SSA 节点标记特征 (Marker Trait)
pub trait TypeTag: 'static + Copy + fmt::Debug + PartialEq + Eq {
  /// 类型标签名称
  fn type_name() -> &'static str;
  /// 对应的机器表示名称 (若存在)
  fn machine_type() -> Option<&'static str>;
  /// 是否为 Tagged 指针类型
  fn is_tagged() -> bool;
}

// -----------------------------------------------------------------------------
// 1. Untagged 基本机器类型标记
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UntaggedT;
impl TypeTag for UntaggedT {
  #[inline]
  fn type_name() -> &'static str {
    "UntaggedT"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    None
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntegralT;
impl TypeTag for IntegralT {
  #[inline]
  fn type_name() -> &'static str {
    "IntegralT"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    None
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WordT;
impl TypeTag for WordT {
  #[inline]
  fn type_name() -> &'static str {
    "WordT"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Pointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawPtrT;
impl TypeTag for RawPtrT {
  #[inline]
  fn type_name() -> &'static str {
    "RawPtrT"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Pointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SandboxedPtrT;
impl TypeTag for SandboxedPtrT {
  #[inline]
  fn type_name() -> &'static str {
    "SandboxedPtrT"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("SandboxedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Word32T;
impl TypeTag for Word32T {
  #[inline]
  fn type_name() -> &'static str {
    "Word32T"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Word32")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Int32T;
impl TypeTag for Int32T {
  #[inline]
  fn type_name() -> &'static str {
    "Int32T"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Int32")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uint32T;
impl TypeTag for Uint32T {
  #[inline]
  fn type_name() -> &'static str {
    "Uint32T"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Uint32")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Int16T;
impl TypeTag for Int16T {
  #[inline]
  fn type_name() -> &'static str {
    "Int16T"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Int16")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uint16T;
impl TypeTag for Uint16T {
  #[inline]
  fn type_name() -> &'static str {
    "Uint16T"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Uint16")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Int8T;
impl TypeTag for Int8T {
  #[inline]
  fn type_name() -> &'static str {
    "Int8T"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Int8")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uint8T;
impl TypeTag for Uint8T {
  #[inline]
  fn type_name() -> &'static str {
    "Uint8T"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Uint8")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Word64T;
impl TypeTag for Word64T {
  #[inline]
  fn type_name() -> &'static str {
    "Word64T"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Word64")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Int64T;
impl TypeTag for Int64T {
  #[inline]
  fn type_name() -> &'static str {
    "Int64T"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Int64")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uint64T;
impl TypeTag for Uint64T {
  #[inline]
  fn type_name() -> &'static str {
    "Uint64T"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Uint64")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntPtrT;
impl TypeTag for IntPtrT {
  #[inline]
  fn type_name() -> &'static str {
    "IntPtrT"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("IntPtr")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UintPtrT;
impl TypeTag for UintPtrT {
  #[inline]
  fn type_name() -> &'static str {
    "UintPtrT"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("UintPtr")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Float32T;
impl TypeTag for Float32T {
  #[inline]
  fn type_name() -> &'static str {
    "Float32T"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Float32")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Float64T;
impl TypeTag for Float64T {
  #[inline]
  fn type_name() -> &'static str {
    "Float64T"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Float64")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoolT;
impl TypeTag for BoolT {
  #[inline]
  fn type_name() -> &'static str {
    "BoolT"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Int32")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalPointerT;
impl TypeTag for ExternalPointerT {
  #[inline]
  fn type_name() -> &'static str {
    "ExternalPointerT"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Pointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

// -----------------------------------------------------------------------------
// 2. Tagged JS 对象类型标记
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Object;
impl TypeTag for Object {
  #[inline]
  fn type_name() -> &'static str {
    "Object"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("AnyTagged")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Smi;
impl TypeTag for Smi {
  #[inline]
  fn type_name() -> &'static str {
    "Smi"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedSigned")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeapObject;
impl TypeTag for HeapObject {
  #[inline]
  fn type_name() -> &'static str {
    "HeapObject"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaggedIndex;
impl TypeTag for TaggedIndex {
  #[inline]
  fn type_name() -> &'static str {
    "TaggedIndex"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("Pointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    false
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Map;
impl TypeTag for Map {
  #[inline]
  fn type_name() -> &'static str {
    "Map"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringTag;
impl TypeTag for StringTag {
  #[inline]
  fn type_name() -> &'static str {
    "String"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JSReceiver;
impl TypeTag for JSReceiver {
  #[inline]
  fn type_name() -> &'static str {
    "JSReceiver"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JSObject;
impl TypeTag for JSObject {
  #[inline]
  fn type_name() -> &'static str {
    "JSObject"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JSArray;
impl TypeTag for JSArray {
  #[inline]
  fn type_name() -> &'static str {
    "JSArray"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextTag;
impl TypeTag for ContextTag {
  #[inline]
  fn type_name() -> &'static str {
    "Context"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedArray;
impl TypeTag for FixedArray {
  #[inline]
  fn type_name() -> &'static str {
    "FixedArray"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorArray;
impl TypeTag for DescriptorArray {
  #[inline]
  fn type_name() -> &'static str {
    "DescriptorArray"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Oddball;
impl TypeTag for Oddball {
  #[inline]
  fn type_name() -> &'static str {
    "Oddball"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;
impl TypeTag for Null {
  #[inline]
  fn type_name() -> &'static str {
    "Null"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Undefined;
impl TypeTag for Undefined {
  #[inline]
  fn type_name() -> &'static str {
    "Undefined"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct True;
impl TypeTag for True {
  #[inline]
  fn type_name() -> &'static str {
    "True"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct False;
impl TypeTag for False {
  #[inline]
  fn type_name() -> &'static str {
    "False"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hole;
impl TypeTag for Hole {
  #[inline]
  fn type_name() -> &'static str {
    "Hole"
  }
  #[inline]
  fn machine_type() -> Option<&'static str> {
    Some("TaggedPointer")
  }
  #[inline]
  fn is_tagged() -> bool {
    true
  }
}

// -----------------------------------------------------------------------------
// 3. TNode<T> 强类型节点封装
// -----------------------------------------------------------------------------

/// 强类型 SSA 节点包装 (包装内部 NodeId)
#[repr(transparent)]
pub struct TNode<T: TypeTag> {
  node_id: NodeId,
  _marker: PhantomData<T>,
}

impl<T: TypeTag> Copy for TNode<T> {}

impl<T: TypeTag> Clone for TNode<T> {
  #[inline]
  fn clone(&self) -> Self {
    *self
  }
}

impl<T: TypeTag> PartialEq for TNode<T> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.node_id == other.node_id
  }
}

impl<T: TypeTag> Eq for TNode<T> {}

impl<T: TypeTag> Hash for TNode<T> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.node_id.hash(state);
  }
}

impl<T: TypeTag> fmt::Debug for TNode<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "TNode<{}>({})", T::type_name(), self.node_id)
  }
}

impl<T: TypeTag> fmt::Display for TNode<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}:{}", self.node_id, T::type_name())
  }
}

impl<T: TypeTag> TNode<T> {
  /// 从原生 NodeId 构造强类型节点
  #[inline]
  #[must_use]
  pub const fn new(id: NodeId) -> Self {
    Self {
      node_id: id,
      _marker: PhantomData,
    }
  }

  /// 构造空/无效节点
  #[inline]
  #[must_use]
  pub const fn invalid() -> Self {
    Self {
      node_id: NodeId::INVALID,
      _marker: PhantomData,
    }
  }

  /// 获取底层 NodeId
  #[inline]
  #[must_use]
  pub const fn node_id(self) -> NodeId {
    self.node_id
  }

  /// 检查是否为有效节点
  #[inline]
  #[must_use]
  pub const fn is_valid(self) -> bool {
    self.node_id.is_valid()
  }

  /// 强行类型重解释转换 (原版 UncheckedCast)
  #[inline]
  #[must_use]
  pub fn unchecked_cast<U: TypeTag>(self) -> TNode<U> {
    TNode::new(self.node_id)
  }

  /// 转换为通用 Object 节点 (如果自身为堆对象或 Smi)
  #[inline]
  #[must_use]
  pub fn to_object(self) -> TNode<Object> {
    TNode::new(self.node_id)
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(size_of::<TNode<Word32T>>() == size_of::<NodeId>());
  assert!(size_of::<TNode<Object>>() == 4);
};
