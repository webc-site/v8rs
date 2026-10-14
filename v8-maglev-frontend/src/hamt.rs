// 由 tool/src/maglev/hamt.h.js 从 v8/src/maglev/hamt.h 生成; 修订就地改, 复跑覆盖
// HAMT: 持久化 (不可变) 哈希阵列映射 trie; 5-bit/层 32 叉, 位图压缩, 碰撞链。
// 破环: 原版 Zone*+base::hash 面 → zone 参数 + HamtHashable trait (整数键恒等哈希, 锚 v8/src/base/hashing.h:218 hash_value 恒等)。
// 内存模型: 节点全部落 v8-zone; 发布后只读 (路径复制 COW); 分支子区柔性数组以裸偏移读写 (BRANCH_HEADER_BYTES 后致密 8 字节槽)。

use core::{
  iter,
  marker::PhantomData,
  mem::size_of,
  ptr::{copy_nonoverlapping, from_mut, null},
};

use v8_base::pointer_with_payload::PointerWithPayload;

/// 节点打包句柄底层类型 (v8: src/maglev/hamt.h:125 `base::PointerWithPayload<void, NodeType, 1>`)
type Payload = PointerWithPayload<1>;

/// v8: src/maglev/hamt.h NodeType (枚举 :88, kLeaf=0 由 IsNull static_assert 锚 :102-105)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NodeType {
  /// v8: src/maglev/hamt.h:88 kLeaf
  Leaf = 0,
  /// v8: src/maglev/hamt.h:88 kBranch
  Branch = 1,
}

/// v8: src/maglev/hamt.h:279 kBits — 每层取哈希位数
pub const HAMT_BITS: usize = 5;
/// v8: src/maglev/hamt.h:280 kWidth — 分支宽度 1<<kBits
pub const HAMT_WIDTH: usize = 1 << HAMT_BITS;
/// v8: src/maglev/hamt.h:281 kMask
pub const HAMT_MASK: usize = HAMT_WIDTH - 1;
/// v8: src/maglev/hamt.h:283 kMaxShift — 末层最大 shift (64 位哈希; 32 位系统系 30, 本移植按 64 位)
pub const HAMT_MAX_SHIFT: usize = 64 - HAMT_BITS;
/// 分支头字节数 (v8: src/maglev/hamt.h:131-133 alignas(8) sizeof(Branch)==8, children 起于偏移 8)
const BRANCH_HEADER_BYTES: usize = size_of::<BranchHeader>();
/// 子节点槽字节数 (v8: src/maglev/hamt.h:133 Node children[], Node 即 8 字节打包指针)
const NODE_BYTES: usize = size_of::<Payload>();

/// 整数键恒等哈希 (v8: src/maglev/hamt.h:172 `base::hash<Key>()(key)`;
/// 整数特化恒等锚 src/base/hashing.h:218-221 hash_value)
pub trait HamtHashable {
  /// v8: src/maglev/hamt.h:172/177 base::hash<Key>()(key)
  fn hamt_hash(&self) -> usize;
}

/// v8: src/base/hashing.h:218-221 — 整数哈希恒等 (size_t 截接)
macro_rules! ident_hamt_hash {
  ($($t:ty),* $(,)?) => {
    $(impl HamtHashable for $t {
      #[inline]
      fn hamt_hash(&self) -> usize {
        *self as usize
      }
    })*
  };
}
ident_hamt_hash!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

/// Node 句柄: 指针低位 1 bit 存 NodeType payload
/// (v8: src/maglev/hamt.h:93-126 struct Node)
#[derive(Clone, Copy)]
pub struct Node<K, V> {
  /// 打包裸地址 + tag (raw()==0 即空; 原版 IsNull static_assert(kLeaf==0) :102-104)
  payload: Payload,
  marker: PhantomData<(K, V)>,
}

/// Leaf 节点: key/value/hash/next 碰撞链 (v8: src/maglev/hamt.h:154-162)
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct Leaf<K, V> {
  /// v8: src/maglev/hamt.h:155 Key key
  pub key: K,
  /// v8: src/maglev/hamt.h:156 Value value
  pub value: V,
  /// v8: src/maglev/hamt.h:157 Hash hash
  pub hash: usize,
  /// v8: src/maglev/hamt.h:158 Leaf* next (碰撞链, nullptr 表尾)
  pub next: *const Leaf<K, V>,
}

/// Branch 头: bitmap + 致密子节点区 (v8: src/maglev/hamt.h:131-151)
/// 原版柔性数组 children[] 就地分配; Rust 侧以头指针 + 偏移 BRANCH_HEADER_BYTES 子区基址承接。
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct BranchHeader {
  /// v8: src/maglev/hamt.h:132 uint32_t bitmap
  pub bitmap: u32,
  /// 4 字节补齐 (alignas(8) 下 children 起于偏移 8, 同 C++ sizeof(Branch)==8)
  pub _pad: u32,
}

/// 持久化 HAMT (v8: src/maglev/hamt.h:82-521 class HAMT)
/// 树不可变; 插入/合并返回共享结构新树 (路径复制)。
/// 安全前提: Hamt 不得活过其 zone (同 C++ Zone 存活期假设; 节点裸指针不跟踪借用)。
#[derive(Clone, Copy)]
pub struct Hamt<K, V> {
  /// v8: src/maglev/hamt.h:274 Node root_
  root: Node<K, V>,
}

// ── 分支子区裸偏移访问 (v8: src/maglev/hamt.h:133 Node children[] 柔性数组) ──

/// children 基址 const 侧 (原版 branch->children)
#[inline]
fn children_ptr(branch: *const BranchHeader) -> *const Payload {
  (branch as *const u8)
    .wrapping_add(BRANCH_HEADER_BYTES)
    .cast()
}

/// children 基址 mut 侧 (发布前写)
#[inline]
fn children_mut_ptr(branch: *const BranchHeader) -> *mut Payload {
  (branch as *mut u8).wrapping_add(BRANCH_HEADER_BYTES).cast()
}

// ── 公开面 ──

impl<K: Copy + PartialEq + HamtHashable, V: Copy + PartialEq> Hamt<K, V> {
  /// v8: src/maglev/hamt.h:164 HAMT() = default (空树)
  #[must_use]
  pub const fn new() -> Self {
    Self { root: Node::null() }
  }

  /// v8: src/maglev/hamt.h:171-174 HAMT::insert(zone, key, value)
  /// 键已存在且值相等 → 原树返回 (无变化, 共享 root); 否则路径复制新树。
  pub fn insert(&self, zone: &v8_zone::Zone<'_>, key: K, value: V) -> crate::Result<Self> {
    let hash = key.hamt_hash();
    Ok(Self::from_root(
      self.insert_rec(zone, self.root, key, value, hash, 0)?,
    ))
  }

  /// v8: src/maglev/hamt.h:176-179 HAMT::find(key)
  /// 返回 zone 内存中的值借用; 调用方保证 Hamt 不活过 zone。
  #[must_use]
  pub fn find(&self, key: K) -> Option<&V> {
    let hash = key.hamt_hash();
    self.find_helper(self.root, key, hash, 0)
  }

  /// v8: src/maglev/hamt.h:181-191 HAMT::merge_into (不对称左连接: 仅更新 this 键,
  /// 冲突走 f(this_value, other_value); root 相等/任一空 → *this)
  pub fn merge_into<F>(&self, zone: &v8_zone::Zone<'_>, other: &Self, f: F) -> crate::Result<Self>
  where
    F: Fn(V, V) -> V,
  {
    if self.root == other.root || self.root.is_null() || other.root.is_null() {
      return Ok(*self);
    }
    Ok(Self::from_root(
      self.merge_into_rec(zone, self.root, other.root, 0, &f)?,
    ))
  }

  /// v8: src/maglev/hamt.h:168 begin() (DFS 迭代器起点)
  #[must_use]
  pub fn begin(&self) -> Iterator<'_, K, V> {
    Iterator::from_root(self.root)
  }

  /// v8: src/maglev/hamt.h:169 end() (空句柄哨兵)
  #[must_use]
  pub fn end(&self) -> Iterator<'_, K, V> {
    Iterator::from_root(Node::null())
  }

  /// range-for 桥 (原版 :193-211 迭代器文档: 顺序按哈希布局, 不可预期)
  pub fn iter(&self) -> Iterator<'_, K, V> {
    self.begin()
  }

  /// v8: src/maglev/hamt.h:285 explicit HAMT(Node root)
  pub(crate) const fn from_root(root: Node<K, V>) -> Self {
    Self { root }
  }

  /// v8: src/maglev/hamt.h:361-408 InsertRec (路径复制插入)
  fn insert_rec(
    &self,
    zone: &v8_zone::Zone<'_>,
    node: Node<K, V>,
    key: K,
    value: V,
    hash: usize,
    shift: usize,
  ) -> crate::Result<Node<K, V>> {
    if node.is_null() {
      let leaf = self.new_leaf(zone, key, value, hash, null())?;
      return Ok(Node::from_leaf(leaf));
    }

    if node.ty() == NodeType::Leaf {
      return self.insert_in_leaf(zone, node.as_leaf(), key, value, hash, shift);
    }

    // Branch case. (v8: src/maglev/hamt.h:371-407)
    let branch = node.as_branch();
    let bit = (hash >> shift) & HAMT_MASK;
    let head = unsafe { &*branch };
    let idx = head.get_index(bit);

    if head.has_child_in_bit(bit) {
      // Child already exists at this bit.
      let existing_child = Node::branch_child(branch, idx);
      let new_child = self.insert_rec(zone, existing_child, key, value, hash, shift + HAMT_BITS)?;

      // No change.
      if existing_child == new_child {
        return Ok(Node::from_branch(branch));
      }

      let copy = self.clone_branch(zone, branch)?;
      Node::branch_set_child(copy, idx, new_child);
      return Ok(Node::from_branch(copy));
    }

    // New bit. Add a new leaf and insert into a new copy of a branch node.
    let new_leaf = self.new_leaf(zone, key, value, hash, null())?;
    let new_child = Node::from_leaf(new_leaf);

    let new_map = head.bitmap | (1u32 << bit);
    let old_count = head.size();
    let copy = self.new_branch(zone, new_map, old_count + 1)?;

    // Memcpy split around idx. (v8: src/maglev/hamt.h:401-406)
    let src = children_ptr(branch);
    let dst = children_mut_ptr(copy);
    if idx > 0 {
      unsafe { copy_nonoverlapping(src, dst, idx) };
    }
    unsafe { dst.add(idx).write_unaligned(new_child.payload) };
    if idx < old_count {
      unsafe { copy_nonoverlapping(src.add(idx), dst.add(idx + 1), old_count - idx) };
    }
    Ok(Node::from_branch(copy))
  }

  /// v8: src/maglev/hamt.h:325-359 InsertInLeaf (叶分裂/碰撞/覆写)
  fn insert_in_leaf(
    &self,
    zone: &v8_zone::Zone<'_>,
    leaf: *const Leaf<K, V>,
    key: K,
    value: V,
    hash: usize,
    shift: usize,
  ) -> crate::Result<Node<K, V>> {
    let l = unsafe { &*leaf };

    // Same key.
    if l.key == key {
      if l.value == value {
        return Ok(Node::from_leaf(leaf));
      }
      let nl = self.new_leaf(zone, key, value, hash, l.next)?;
      return Ok(Node::from_leaf(nl));
    }

    // Hash collision or max depth.
    if l.hash == hash || shift > HAMT_MAX_SHIFT {
      let merged = self.insert_in_collision_list_rec(zone, leaf, key, value, hash)?;
      return Ok(Node::from_leaf(merged));
    }

    // Partial hash match. Add a new tree level.
    let old_bit = (l.hash >> shift) & HAMT_MASK;
    let new_bit = (hash >> shift) & HAMT_MASK;

    // Bits are different, so we will stop in the next level. Just create the branch directly.
    if old_bit != new_bit {
      let new_leaf = self.new_leaf(zone, key, value, hash, null())?;
      let bitmap = (1u32 << old_bit) | (1u32 << new_bit);
      let branch = self.new_branch(zone, bitmap, 2)?;
      let (first, second) = if old_bit < new_bit {
        (leaf, new_leaf)
      } else {
        (new_leaf, leaf)
      };
      Node::branch_set_child(branch, 0, Node::from_leaf(first));
      Node::branch_set_child(branch, 1, Node::from_leaf(second));
      return Ok(Node::from_branch(branch));
    }

    // Bitmaps are the same. Recurse.
    let child = self.insert_rec(
      zone,
      Node::from_leaf(leaf),
      key,
      value,
      hash,
      shift + HAMT_BITS,
    )?;
    let branch = self.new_branch(zone, 1u32 << old_bit, 1)?;
    Node::branch_set_child(branch, 0, child);
    Ok(Node::from_branch(branch))
  }

  /// v8: src/maglev/hamt.h:307-323 InsertInCollisionListRec (碰撞链路径复制;
  /// leaf 可为 null → 新建链头)
  fn insert_in_collision_list_rec(
    &self,
    zone: &v8_zone::Zone<'_>,
    leaf: *const Leaf<K, V>,
    key: K,
    value: V,
    hash: usize,
  ) -> crate::Result<*const Leaf<K, V>> {
    if leaf.is_null() {
      return self.new_leaf(zone, key, value, hash, null());
    }

    let l = unsafe { &*leaf };
    if l.key == key {
      if l.value == value {
        return Ok(leaf);
      }
      return self.new_leaf(zone, key, value, hash, l.next);
    }

    let new_next = self.insert_in_collision_list_rec(zone, l.next, key, value, hash)?;
    if new_next == l.next {
      return Ok(leaf);
    }

    self.new_leaf(zone, l.key, l.value, l.hash, new_next)
  }

  /// v8: src/maglev/hamt.h:410-425 FindHelper (循环下降)
  fn find_helper(&self, node: Node<K, V>, key: K, hash: usize, shift: usize) -> Option<&V> {
    let mut node = node;
    let mut shift = shift;
    while !node.is_null() {
      if node.ty() == NodeType::Leaf {
        return self.find_in_collision_list(node.as_leaf(), key);
      }
      let branch = node.as_branch();
      let head = unsafe { &*branch };
      let bit = (hash >> shift) & HAMT_MASK;
      if !head.has_child_in_bit(bit) {
        return None;
      }
      let idx = head.get_index(bit);
      node = Node::branch_child(branch, idx);
      shift += HAMT_BITS;
    }
    None
  }

  /// v8: src/maglev/hamt.h:427-434 FindInCollisionList
  fn find_in_collision_list(&self, list: *const Leaf<K, V>, key: K) -> Option<&V> {
    let mut cur = list;
    while !cur.is_null() {
      let l = unsafe { &*cur };
      if l.key == key {
        return Some(&l.value);
      }
      cur = l.next;
    }
    None
  }

  /// v8: src/maglev/hamt.h:439-450 MergeCollisionLists
  /// (list2 并入 list1: 仅 list1 键更新, list2 独有键丢弃; 链尾 null 返回 null)
  fn merge_collision_lists<F>(
    &self,
    zone: &v8_zone::Zone<'_>,
    list1: *const Leaf<K, V>,
    list2: *const Leaf<K, V>,
    f: &F,
  ) -> crate::Result<*const Leaf<K, V>>
  where
    F: Fn(V, V) -> V,
  {
    if list1.is_null() {
      return Ok(list1);
    }
    let l = unsafe { &*list1 };
    let new_next = self.merge_collision_lists(zone, l.next, list2, f)?;
    let merged = match self.find_in_collision_list(list2, l.key) {
      Some(v) => f(l.value, *v),
      None => l.value,
    };
    if new_next == l.next && merged == l.value {
      return Ok(list1);
    }
    self.new_leaf(zone, l.key, merged, l.hash, new_next)
  }

  /// v8: src/maglev/hamt.h:452-467 MergeIntoBranch
  fn merge_into_branch<F>(
    &self,
    zone: &v8_zone::Zone<'_>,
    branch: *const BranchHeader,
    leaf: *const Leaf<K, V>,
    shift: usize,
    f: &F,
  ) -> crate::Result<*const BranchHeader>
  where
    F: Fn(V, V) -> V,
  {
    let bit = (unsafe { &*leaf }.hash >> shift) & HAMT_MASK;
    let head = unsafe { &*branch };
    if !head.has_child_in_bit(bit) {
      return Ok(branch);
    }

    let idx = head.get_index(bit);
    let old_child = Node::branch_child(branch, idx);
    let merged =
      self.merge_into_rec(zone, old_child, Node::from_leaf(leaf), shift + HAMT_BITS, f)?;

    if old_child == merged {
      return Ok(branch);
    }

    let new_branch = self.clone_branch(zone, branch)?;
    Node::branch_set_child(new_branch, idx, merged);
    Ok(new_branch)
  }

  /// v8: src/maglev/hamt.h:469-482 MergeIntoLeaf
  /// (branch 项并入碰撞链 list: 链上无变化复用原叶)
  fn merge_into_leaf<F>(
    &self,
    zone: &v8_zone::Zone<'_>,
    list: *const Leaf<K, V>,
    branch: *const BranchHeader,
    shift: usize,
    f: &F,
  ) -> crate::Result<*const Leaf<K, V>>
  where
    F: Fn(V, V) -> V,
  {
    if list.is_null() {
      return Ok(list);
    }
    let l = unsafe { &*list };
    let new_next = self.merge_into_leaf(zone, l.next, branch, shift, f)?;
    let merged = match self.find_helper(Node::from_branch(branch), l.key, l.hash, shift) {
      Some(v) => f(l.value, *v),
      None => l.value,
    };
    if new_next == l.next && merged == l.value {
      return Ok(list);
    }
    self.new_leaf(zone, l.key, merged, l.hash, new_next)
  }

  /// v8: src/maglev/hamt.h:484-520 MergeIntoRec (双分支共享子树指针跳过)
  fn merge_into_rec<F>(
    &self,
    zone: &v8_zone::Zone<'_>,
    a: Node<K, V>,
    b: Node<K, V>,
    shift: usize,
    f: &F,
  ) -> crate::Result<Node<K, V>>
  where
    F: Fn(V, V) -> V,
  {
    if a == b {
      // Always return the left side! (v8: src/maglev/hamt.h:486)
      return Ok(a);
    }
    if a.is_null() || b.is_null() {
      return Ok(a);
    }

    match (a.ty(), b.ty()) {
      (NodeType::Leaf, NodeType::Leaf) => Ok(Node::from_leaf(self.merge_collision_lists(
        zone,
        a.as_leaf(),
        b.as_leaf(),
        f,
      )?)),
      (NodeType::Leaf, NodeType::Branch) => Ok(Node::from_leaf(self.merge_into_leaf(
        zone,
        a.as_leaf(),
        b.as_branch(),
        shift,
        f,
      )?)),
      (NodeType::Branch, NodeType::Leaf) => Ok(Node::from_branch(self.merge_into_branch(
        zone,
        a.as_branch(),
        b.as_leaf(),
        shift,
        f,
      )?)),
      (NodeType::Branch, NodeType::Branch) => {
        let br_a = a.as_branch();
        let br_b = b.as_branch();
        let mut res: *const BranchHeader = null();
        let mut shared_bits = unsafe { &*br_a }.bitmap & unsafe { &*br_b }.bitmap;
        while shared_bits != 0 {
          // 消最低置位 (v8: src/maglev/hamt.h:504 shared_bits &= shared_bits - 1)
          let bit = shared_bits.trailing_zeros() as usize;
          shared_bits &= shared_bits - 1;
          let idx = unsafe { &*br_a }.get_index(bit);
          let old_child = Node::branch_child(br_a, idx);
          let b_child = Node::branch_child(br_b, unsafe { &*br_b }.get_index(bit));
          let child = self.merge_into_rec(zone, old_child, b_child, shift + HAMT_BITS, f)?;
          if child == old_child {
            continue;
          }
          if res.is_null() {
            res = self.clone_branch(zone, br_a)?;
          }
          Node::branch_set_child(res, idx, child);
        }
        Ok(if res.is_null() {
          a
        } else {
          Node::from_branch(res)
        })
      }
    }
  }

  /// v8: src/maglev/hamt.h:287-290 NewLeaf (zone->New<Leaf>)
  fn new_leaf(
    &self,
    zone: &v8_zone::Zone<'_>,
    key: K,
    value: V,
    hash: usize,
    next: *const Leaf<K, V>,
  ) -> crate::Result<*const Leaf<K, V>> {
    let leaf = zone.make(Leaf {
      key,
      value,
      hash,
      next,
    })?;
    Ok(from_mut(leaf).cast_const())
  }

  /// v8: src/maglev/hamt.h:292-297 NewBranch (DCHECK count==popcount; sizeof(Branch)+count*sizeof(Node))
  fn new_branch(
    &self,
    zone: &v8_zone::Zone<'_>,
    bitmap: u32,
    count: usize,
  ) -> crate::Result<*const BranchHeader> {
    debug_assert_eq!(
      count,
      bitmap.count_ones() as usize,
      "count != popcount(bitmap)"
    );
    debug_assert!(count >= 1, "空分支不合 HAMT 不变式");
    let total_bytes = BRANCH_HEADER_BYTES + count * NODE_BYTES;
    let mem = zone.allocate_bytes_ref(total_bytes)?;
    let branch = mem.as_mut_ptr().cast::<BranchHeader>();
    unsafe { branch.write(BranchHeader { bitmap, _pad: 0 }) };
    Ok(branch as *const BranchHeader)
  }

  /// v8: src/maglev/hamt.h:299-305 CloneBranch (memcpy 子区)
  fn clone_branch(
    &self,
    zone: &v8_zone::Zone<'_>,
    old: *const BranchHeader,
  ) -> crate::Result<*const BranchHeader> {
    let head = unsafe { &*old };
    let size = head.size();
    let new_branch = self.new_branch(zone, head.bitmap, size)?;
    let src = children_ptr(old);
    let dst = children_mut_ptr(new_branch);
    if size > 0 {
      unsafe { copy_nonoverlapping(src, dst, size) };
    }
    Ok(new_branch)
  }
}

/// v8: src/maglev/hamt.h:164 HAMT() = default
impl<K: Copy + PartialEq + HamtHashable, V: Copy + PartialEq> Default for Hamt<K, V> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

/// Branch 位图运算 (v8: src/maglev/hamt.h:131-151 Branch 方法)
impl BranchHeader {
  /// v8: src/maglev/hamt.h:138 size() = popcount(bitmap)
  #[inline]
  #[must_use]
  pub const fn size(&self) -> usize {
    self.bitmap.count_ones() as usize
  }

  /// v8: src/maglev/hamt.h:140 has_child_in_bit(bit)
  #[inline]
  #[must_use]
  pub const fn has_child_in_bit(&self, bit: usize) -> bool {
    self.bitmap & (1u32 << bit) != 0
  }

  /// v8: src/maglev/hamt.h:142-144 get_index(bit) = popcount(bitmap & ((1<<bit)-1))
  #[inline]
  #[must_use]
  pub const fn get_index(&self, bit: usize) -> usize {
    (self.bitmap & ((1u32 << bit) - 1)).count_ones() as usize
  }
}

/// DFS 迭代器 (v8: src/maglev/hamt.h:212-271 class Iterator)
/// 栈式非递归深度优先; 顺序由哈希布局决定, 不可预期 (原版 :207-210 注)。
/// 'a 名义锚定 Hamt 借用期 (原版 const Leaf* 活在 begin() 宿主树上)。
pub struct Iterator<'a, K, V> {
  /// v8: src/maglev/hamt.h:256 const Leaf* current_
  current: *const Leaf<K, V>,
  /// v8: src/maglev/hamt.h:257 std::vector<StackEntry> stack_
  stack: Vec<StackEntry<K, V>>,
  marker: PhantomData<&'a ()>,
}

/// v8: src/maglev/hamt.h:214-217 Iterator::StackEntry { Branch* node; int idx; }
#[derive(Clone, Copy)]
pub struct StackEntry<K, V> {
  /// v8: src/maglev/hamt.h:215 Branch* node
  pub node: *const BranchHeader,
  /// v8: src/maglev/hamt.h:216 int idx (致密子区当前下标)
  pub idx: usize,
  marker: PhantomData<(*const Leaf<K, V>,)>,
}

impl<K: Copy, V: Copy> Iterator<'_, K, V> {
  /// v8: src/maglev/hamt.h:219 explicit Iterator(Node root) : current_(nullptr) { Push(root); }
  pub(crate) fn from_root(root: Node<K, V>) -> Self {
    let mut it = Self {
      current: null(),
      stack: Vec::with_capacity(16),
      marker: PhantomData,
    };
    it.push(root);
    it
  }

  /// v8: src/maglev/hamt.h:221-223 operator== (current_ 指针相等)
  #[must_use]
  pub fn same(&self, other: &Self) -> bool {
    self.current == other.current
  }

  /// v8: src/maglev/hamt.h:226-249 operator++ (链内前进 → 回溯找兄弟下降)
  pub fn advance(&mut self) {
    if self.current.is_null() {
      return;
    }

    // Move to next collision in chain.
    let next = unsafe { &*self.current }.next;
    if !next.is_null() {
      self.current = next;
      return;
    }

    // Backtrack up the tree.
    while let Some(entry) = self.stack.last_mut() {
      entry.idx += 1;
      let size = unsafe { &*entry.node }.size();
      if entry.idx < size {
        // Found a sibling, descend.
        let child = Node::<K, V>::branch_child(entry.node, entry.idx);
        self.push(child);
        return;
      }
      self.stack.pop();
    }

    // Done.
    self.current = null();
  }

  /// v8: src/maglev/hamt.h:251-253 operator* → std::pair<Key,Value> (空位 None)
  #[must_use]
  pub fn item(&self) -> Option<(K, V)> {
    if self.current.is_null() {
      return None;
    }
    let l = unsafe { &*self.current };
    Some((l.key, l.value))
  }

  /// v8: src/maglev/hamt.h:259-270 Push (降入最左叶)
  fn push(&mut self, node: Node<K, V>) {
    let mut node = node;
    while !node.is_null() {
      match node.ty() {
        NodeType::Leaf => {
          self.current = node.as_leaf();
          return;
        }
        NodeType::Branch => {
          let branch = node.as_branch();
          self.stack.push(StackEntry {
            node: branch,
            idx: 0,
            marker: PhantomData,
          });
          node = Node::branch_child(branch, 0);
        }
      }
    }
  }
}

/// range-for 桥: 协议为 "取当前 → 前进" (等价 C++ begin..end 解引用序)
impl<K: Copy, V: Copy> iter::Iterator for Iterator<'_, K, V> {
  type Item = (K, V);

  fn next(&mut self) -> Option<(K, V)> {
    let item = self.item();
    if item.is_some() {
      self.advance();
    }
    item
  }
}

impl<K: Copy, V: Copy> Node<K, V> {
  /// v8: src/maglev/hamt.h:94 空构造 (payload_=nullptr, kLeaf; raw==0)
  #[must_use]
  pub const fn null() -> Self {
    Self {
      payload: Payload::new(),
      marker: PhantomData,
    }
  }

  /// v8: src/maglev/hamt.h:95 Node(Leaf* leaf)
  #[must_use]
  pub fn from_leaf(leaf: *const Leaf<K, V>) -> Self {
    debug_assert_eq!(leaf as usize & 1, 0, "Leaf alignas(8) 低位必须空闲");
    Self {
      payload: Payload::from_pointer_and_payload(leaf as *mut Leaf<K, V>, NodeType::Leaf as usize),
      marker: PhantomData,
    }
  }

  /// v8: src/maglev/hamt.h:96 Node(Branch* branch)
  #[must_use]
  pub fn from_branch(branch: *const BranchHeader) -> Self {
    debug_assert_eq!(branch as usize & 1, 0, "Branch alignas(8) 低位必须空闲");
    Self {
      payload: Payload::from_pointer_and_payload(
        branch as *mut BranchHeader,
        NodeType::Branch as usize,
      ),
      marker: PhantomData,
    }
  }

  /// v8: src/maglev/hamt.h:102-105 IsNull
  #[must_use]
  pub const fn is_null(&self) -> bool {
    self.payload.raw() == 0
  }

  /// v8: src/maglev/hamt.h:107-110 type()
  #[must_use]
  pub fn ty(&self) -> NodeType {
    debug_assert!(!self.is_null(), "ty() on null node");
    if self.payload.get_payload() == NodeType::Branch as usize {
      NodeType::Branch
    } else {
      NodeType::Leaf
    }
  }

  /// v8: src/maglev/hamt.h:112-116 AsLeaf
  #[must_use]
  pub fn as_leaf(&self) -> *const Leaf<K, V> {
    debug_assert_eq!(self.ty(), NodeType::Leaf);
    self
      .payload
      .get_pointer_with_known_payload::<Leaf<K, V>>(NodeType::Leaf as usize)
      as *const Leaf<K, V>
  }

  /// v8: src/maglev/hamt.h:118-122 AsBranch
  #[must_use]
  pub fn as_branch(&self) -> *const BranchHeader {
    debug_assert_eq!(self.ty(), NodeType::Branch);
    self
      .payload
      .get_pointer_with_known_payload::<BranchHeader>(NodeType::Branch as usize)
      as *const BranchHeader
  }

  /// v8: src/maglev/hamt.h:146-148 get_child(idx) (柔性子区读; 非对齐敏感处取 read_unaligned)
  #[must_use]
  pub fn branch_child(branch: *const BranchHeader, idx: usize) -> Self {
    let payload = unsafe { children_ptr(branch).add(idx).read_unaligned() };
    Self {
      payload,
      marker: PhantomData,
    }
  }

  /// v8: src/maglev/hamt.h:150 set_child(idx, node) (仅限发布前写入; COW 新分支专用)
  pub fn branch_set_child(branch: *const BranchHeader, idx: usize, node: Self) {
    debug_assert!(!branch.is_null(), "set_child on null branch");
    unsafe {
      children_mut_ptr(branch)
        .add(idx)
        .write_unaligned(node.payload)
    };
  }
}

/// 测试友元面 (v8: src/maglev/hamt.h:276 `friend HAMTTest`;
/// test/unittests/maglev/hamt-unittest.cc:30-41 InsertWithHash/FindWithHash/SameRoot 借道)
impl<K: Copy + PartialEq + HamtHashable, V: Copy + PartialEq> Hamt<K, V> {
  /// v8: src/maglev/hamt.h:361 InsertRec 外露 (友元测试入口, 语义同 insert 但哈希直给)
  pub fn insert_with_hash(
    &self,
    zone: &v8_zone::Zone<'_>,
    key: K,
    value: V,
    hash: usize,
  ) -> crate::Result<Self> {
    Ok(Self::from_root(
      self.insert_rec(zone, self.root, key, value, hash, 0)?,
    ))
  }

  /// v8: src/maglev/hamt.h:410 FindHelper 外露 (友元测试入口)
  #[must_use]
  pub fn find_with_hash(&self, key: K, hash: usize) -> Option<&V> {
    self.find_helper(self.root, key, hash, 0)
  }

  /// v8: src/maglev/hamt.h:98 root 相等 (原版友元 SameRoot)
  #[must_use]
  pub fn same_root(&self, other: &Self) -> bool {
    self.root == other.root
  }

  /// v8: src/maglev/hamt.h:274 root_ 只读 (原版友元访问)
  #[must_use]
  pub const fn root(&self) -> Node<K, V> {
    self.root
  }
}

/// v8: src/maglev/hamt.h:98-100 Node::operator== (raw 位相等)
impl<K: Copy, V: Copy> PartialEq for Node<K, V> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.payload.raw() == other.payload.raw()
  }
}

/// v8: src/maglev/hamt.h:98-100 等价关系继承 raw 位序
impl<K: Copy, V: Copy> Eq for Node<K, V> {}
