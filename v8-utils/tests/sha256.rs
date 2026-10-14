// 上游 v8/test/unittests/utils 无 sha-256 单测 (实现被 snapshot 源哈希间接使用);
// 此处以 FIPS 180-2 / RFC 6234 标准向量补测关键路径, 分块边界 (55/56/63/64/128) 覆盖。
use aok::{OK, Void};
use v8_utils::sha256::{SIZE_OF_SHA256_DIGEST, Sha256, sha256_hash};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn hex(b: &[u8]) -> String {
  b.iter().map(|x| format!("{x:02x}")).collect()
}

fn assert_digest(input: &[u8], expected: &str) {
  let digest = sha256_hash(input);
  assert_eq!(hex(&digest), expected, "input len {}", input.len());
}

// 标准向量: 空 / "abc" / 长串 / 分块边界
#[test]
fn rfc_vectors() -> Void {
  assert_digest(
    b"",
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  );
  assert_digest(
    b"abc",
    "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
  );
  assert_digest(
    b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
    "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
  );
  // 分块边界: 55/56/63/64/128 字节 (填充路径 1 块/2 块全覆盖)
  for len in [55, 56, 63, 64, 128] {
    let input = vec![b'a'; len];
    let digest = sha256_hash(&input);
    assert_eq!(digest.len(), SIZE_OF_SHA256_DIGEST);
    // 与增量 update 等价
    let mut ctx = Sha256::new();
    ctx.update(&input[..len / 2]);
    ctx.update(&input[len / 2..]);
    assert_eq!(hex(&ctx.finalize()), hex(&digest), "len {len}");
  }
  // RFC 6234 一百万个 'a' (大输入正确性)
  let mut ctx = Sha256::new();
  let chunk = [b'a'; 1000];
  for _ in 0..1000 {
    ctx.update(&chunk);
  }
  assert_eq!(
    hex(&ctx.finalize()),
    "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
  );
  OK
}
