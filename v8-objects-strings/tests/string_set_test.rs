// 迁移规约:
//   string-set.h:16-28 StringSetShape 常量 (kPrefixSize=0/kEntrySize=1/
//     kMatchNeedsHoleCheck=true/kDoHashSpreading=false/kHashBits=0)
//   string-set-inl.h:19-31 IsMatch/Hash/HashForObject (EnsureHash/Equals 契约)
//   StringSet::New/Add/Has (hash-table.h 基建 + isolate): 推迟 → 规约仅落
//     形状常量与判定契约
use aok::{OK, Void};
use v8_objects_strings::string_set::{StringSetEntry, StringSetShape};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// 测试条目 (String 的 EnsureHash/Equals 契约形态)。
#[derive(Debug, Clone, PartialEq, Eq)]
struct Word {
  content: String,
  hash: Option<u32>,
}

impl Word {
  fn new(content: &str) -> Self {
    Self {
      content: content.to_owned(),
      hash: None,
    }
  }
}

impl StringSetEntry for Word {
  fn ensure_hash(&mut self) -> u32 {
    *self.hash.get_or_insert_with(|| {
      // 确定性内容哈希 (fnv1a), 模拟 EnsureHash 惰性计算
      let mut h: u32 = 0x811c_9dc5;
      for &b in self.content.as_bytes() {
        h = (h ^ u32::from(b)).wrapping_mul(0x0100_0193);
      }
      h
    })
  }

  fn equals(&self, other: &Self) -> bool {
    self.content == other.content
  }
}

/// 规约 (string-set.h:23-27): 形状常量 1:1 (布尔位编译期断言, 免运行时废话断言)。
const _: () = assert!(StringSetShape::MATCH_NEEDS_HOLE_CHECK);
const _: () = assert!(!StringSetShape::DO_HASH_SPREADING);

#[test]
fn shape_constants() -> Void {
  assert_eq!(StringSetShape::PREFIX_SIZE, 0);
  assert_eq!(StringSetShape::ENTRY_SIZE, 1);
  assert_eq!(StringSetShape::HASH_BITS, 0);
  OK
}

/// 规约 (string-set-inl.h:19 IsMatch): 键值内容等值判定。
#[test]
fn shape_is_match_by_content() -> Void {
  let foo = Word::new("foo");
  let foo2 = Word::new("foo");
  let bar = Word::new("bar");
  assert!(StringSetShape::is_match(&foo, &foo2));
  assert!(!StringSetShape::is_match(&foo, &bar));
  OK
}

/// 规约 (string-set-inl.h:24/28 Hash/HashForObject): 惰性计算并缓存
/// (EnsureHash 语义: 首次计算后稳定)。
#[test]
fn shape_hash_is_ensure_semantics() -> Void {
  let mut w = Word::new("stable");
  let h1 = StringSetShape::hash(&mut w);
  let h2 = StringSetShape::hash(&mut w);
  assert_eq!(h1, h2, "重复 EnsureHash 返回缓存值");
  // HashForObject 同一契约
  let mut v = Word::new("stable");
  assert_eq!(StringSetShape::hash_for_object(&mut v), h1);
  // 不同内容不同哈希 (fnv1a 性质)
  let mut x = Word::new("other");
  assert_ne!(StringSetShape::hash(&mut x), h1);
  OK
}
