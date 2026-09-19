// 复刻 v8/src/utils/sha-256.cc：v8 无对应 gtest，采用 NIST FIPS 180-4 标准测试向量
use std::{cell::RefCell, rc::Rc};

use aok::{OK, Void};
use v8rs::{
  AsPointer, Ptr, SHA256_final_7, SHA256_hash_8, SHA256_init_5, SHA256_update_6, Value,
  kSizeOfFormattedSha256Digest_1, kSizeOfSha256Digest_0, v8_internal_HASH_CTX,
};

fn hex(bytes: &[u8]) -> String {
  bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn byte_buf(data: &[u8]) -> Rc<RefCell<Box<[u8]>>> {
  Rc::new(RefCell::new(data.to_vec().into_boxed_slice()))
}

fn u8_ptr(buf: &Rc<RefCell<Box<[u8]>>>) -> Ptr<u8> {
  buf.as_pointer()
}

// 一次成型
fn hash_of(data: &[u8]) -> String {
  let src = byte_buf(data);
  let digest = byte_buf(&[0u8; 32]);
  let out = SHA256_hash_8(u8_ptr(&src).to_any(), data.len(), u8_ptr(&digest));
  hex(&out.with_slice(32, |s| s.to_vec()))
}

// init/update*/final 流式
fn hash_streamed(chunks: &[&[u8]]) -> String {
  let ctx: Rc<RefCell<v8_internal_HASH_CTX>> = Rc::new(RefCell::new(Default::default()));
  SHA256_init_5(ctx.as_pointer());
  for c in chunks {
    let buf = byte_buf(c);
    SHA256_update_6(ctx.as_pointer(), u8_ptr(&buf).to_any(), c.len());
  }
  let out = SHA256_final_7(ctx.as_pointer());
  hex(&out.with_slice(32, |s| s.to_vec()))
}

#[test]
fn empty_kat() -> Void {
  assert_eq!(
    hash_of(b""),
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  );
  OK
}

#[test]
fn abc_kat() -> Void {
  assert_eq!(
    hash_of(b"abc"),
    "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
  );
  OK
}

#[test]
fn fifty_six_byte_kat() -> Void {
  let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
  assert_eq!(msg.len(), 56);
  assert_eq!(
    hash_of(msg),
    "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
  );
  OK
}

#[test]
fn streamed_matches_one_shot() -> Void {
  let whole: Vec<u8> = (0..200u16).map(|i| (i % 251) as u8).collect();
  let mut parts: Vec<&[u8]> = Vec::new();
  for c in whole.chunks(13) {
    parts.push(c);
  }
  assert_eq!(hash_streamed(&parts), hash_of(&whole));
  OK
}

#[test]
fn digest_size_constants() -> Void {
  assert_eq!(*kSizeOfSha256Digest_0.with(Value::clone).borrow(), 32);
  assert_eq!(
    *kSizeOfFormattedSha256Digest_1.with(Value::clone).borrow(),
    65
  );
  OK
}
