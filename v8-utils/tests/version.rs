// 迁移自 v8/test/unittests/utils/version-unittest.cc
// 断言规约: TEST_F(VersionTest, X) → fn x; SetVersion → Version 实例构造
// (上游静态成员 + 友元改写); VersionHash 的 default_hash 以 Hasher::combine_i32 复算。
use aok::{OK, Void};
use v8_utils::{utils::Hasher, version::Version};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// CheckVersion: 无/有特定 SONAME 两轮校验
fn check_version(
  (major, minor, build, patch): (i32, i32, i32, i32),
  embedder: &'static str,
  candidate: bool,
  expected_version_string: &str,
  expected_generic_soname: &str,
) {
  // Test version without specific SONAME.
  let v = Version {
    major,
    minor,
    build,
    patch,
    embedder,
    candidate,
    soname: "",
  };
  assert_eq!(expected_version_string, v.get_string());
  assert_eq!(expected_generic_soname, v.get_soname());

  // Test version with specific SONAME.
  let v = Version {
    soname: "libv8.so.1",
    ..v
  };
  assert_eq!(expected_version_string, v.get_string());
  assert_eq!("libv8.so.1", v.get_soname());
}

// VersionString
#[test]
fn version_string() -> Void {
  check_version((0, 0, 0, 0), "", false, "0.0.0", "libv8-0.0.0.so");
  check_version(
    (0, 0, 0, 0),
    "",
    true,
    "0.0.0 (candidate)",
    "libv8-0.0.0-candidate.so",
  );
  check_version((1, 0, 0, 0), "", false, "1.0.0", "libv8-1.0.0.so");
  check_version(
    (1, 0, 0, 0),
    "",
    true,
    "1.0.0 (candidate)",
    "libv8-1.0.0-candidate.so",
  );
  check_version((1, 0, 0, 1), "", false, "1.0.0.1", "libv8-1.0.0.1.so");
  check_version(
    (1, 0, 0, 1),
    "",
    true,
    "1.0.0.1 (candidate)",
    "libv8-1.0.0.1-candidate.so",
  );
  check_version((2, 5, 10, 7), "", false, "2.5.10.7", "libv8-2.5.10.7.so");
  check_version(
    (2, 5, 10, 7),
    "",
    true,
    "2.5.10.7 (candidate)",
    "libv8-2.5.10.7-candidate.so",
  );
  check_version(
    (6, 0, 287, 0),
    "-emb.1",
    false,
    "6.0.287-emb.1",
    "libv8-6.0.287-emb.1.so",
  );
  check_version(
    (6, 0, 287, 0),
    "-emb.1",
    true,
    "6.0.287-emb.1 (candidate)",
    "libv8-6.0.287-emb.1-candidate.so",
  );
  check_version(
    (6, 0, 287, 53),
    "-emb.1",
    false,
    "6.0.287.53-emb.1",
    "libv8-6.0.287.53-emb.1.so",
  );
  check_version(
    (6, 0, 287, 53),
    "-emb.1",
    true,
    "6.0.287.53-emb.1 (candidate)",
    "libv8-6.0.287.53-emb.1-candidate.so",
  );
  OK
}

// VersionHash
#[test]
fn version_hash() -> Void {
  let base = Version {
    major: 1,
    minor: 2,
    build: 3,
    patch: 4,
    embedder: "",
    candidate: false,
    soname: "",
  };
  // base::hash_combine(1, 2, 3, 4) == Hasher 依次 Add int
  let default_hash = Hasher::combine_i32(&[1, 2, 3, 4]) as u32;
  assert_eq!(default_hash, base.hash());

  let emb1 = Version {
    embedder: "-emb.1",
    ..base
  };
  let embedder_hash = emb1.hash();
  assert_ne!(default_hash, embedder_hash);

  let emb1b = Version {
    embedder: "-emb.1",
    ..base
  };
  assert_eq!(embedder_hash, emb1b.hash());

  let emb2 = Version {
    embedder: "-emb.2",
    ..base
  };
  assert_ne!(embedder_hash, emb2.hash());
  OK
}

// 补充: 构建期版本常量与生成脚本一致 (V8 15.7.0.0 candidate)
#[test]
fn build_version_constants() -> Void {
  let v = Version::get();
  assert_eq!(v.major, 15);
  assert_eq!(v.minor, 7);
  assert_eq!(v.build, 0);
  assert_eq!(v.patch, 0);
  assert!(v.candidate);
  assert_eq!(v.get_string(), "15.7.0 (candidate)");
  assert_eq!(v.get_soname(), "libv8-15.7.0-candidate.so");
  OK
}
