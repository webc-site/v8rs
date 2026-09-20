use std::{
  cell::RefCell,
  collections::BTreeMap,
  io::{Read, Seek, Write, prelude::*},
  os::fd::AsFd,
  rc::{Rc, Weak},
};

use crate::*;
thread_local!(
  pub static kSizeOfSha256Digest_0: Value<usize> = Rc::new(RefCell::new(32_usize));
);
thread_local!(
  pub static kSizeOfFormattedSha256Digest_1: Value<usize> = Rc::new(RefCell::new(
    (((*kSizeOfSha256Digest_0.with(Value::clone).borrow()).wrapping_mul(2_usize)) as usize)
      .wrapping_add(1_usize),
  ));
);
#[derive()]
pub struct v8_internal_HASH_VTAB {
  pub init: Value<FnPtr<fn(Ptr<v8_internal_HASH_CTX>)>>,
  pub update: Value<FnPtr<fn(Ptr<v8_internal_HASH_CTX>, AnyPtr, usize)>>,
  pub final_: Value<FnPtr<fn(Ptr<v8_internal_HASH_CTX>) -> Ptr<u8>>>,
  pub hash: Value<FnPtr<fn(AnyPtr, usize, Ptr<u8>) -> Ptr<u8>>>,
  pub size: Value<u32>,
}
impl Clone for v8_internal_HASH_VTAB {
  fn clone(&self) -> Self {
    let __this: Value<v8_internal_HASH_VTAB> = Rc::new(RefCell::new(Self {
      init: Rc::new(RefCell::new((*self.init.borrow()).clone())),
      update: Rc::new(RefCell::new((*self.update.borrow()).clone())),
      final_: Rc::new(RefCell::new((*self.final_.borrow()).clone())),
      hash: Rc::new(RefCell::new((*self.hash.borrow()).clone())),
      size: Rc::new(RefCell::new((*self.size.borrow()))),
    }));
    let this: Ptr<v8_internal_HASH_VTAB> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl Default for v8_internal_HASH_VTAB {
  fn default() -> Self {
    v8_internal_HASH_VTAB {
      init: Rc::new(RefCell::new(FnPtr::<fn(Ptr<v8_internal_HASH_CTX>)>::null())),
      update: Rc::new(RefCell::new(FnPtr::<
        fn(Ptr<v8_internal_HASH_CTX>, AnyPtr, usize),
      >::null())),
      final_: Rc::new(RefCell::new(FnPtr::<
        fn(Ptr<v8_internal_HASH_CTX>) -> Ptr<u8>,
      >::null())),
      hash: Rc::new(RefCell::new(
        FnPtr::<fn(AnyPtr, usize, Ptr<u8>) -> Ptr<u8>>::null(),
      )),
      size: <Value<u32>>::default(),
    }
  }
}
impl ByteRepr for v8_internal_HASH_VTAB {
  fn byte_size() -> usize {
    40
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    (*self.init.borrow()).to_bytes(&mut buf[0..8]);
    (*self.update.borrow()).to_bytes(&mut buf[8..16]);
    (*self.final_.borrow()).to_bytes(&mut buf[16..24]);
    (*self.hash.borrow()).to_bytes(&mut buf[24..32]);
    (*self.size.borrow()).to_bytes(&mut buf[32..36]);
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self {
      init: Rc::new(RefCell::new(
        <FnPtr<fn(Ptr<v8_internal_HASH_CTX>)>>::from_bytes(&buf[0..8]),
      )),
      update: Rc::new(RefCell::new(<FnPtr<
        fn(Ptr<v8_internal_HASH_CTX>, AnyPtr, usize),
      >>::from_bytes(&buf[8..16]))),
      final_: Rc::new(RefCell::new(<FnPtr<
        fn(Ptr<v8_internal_HASH_CTX>) -> Ptr<u8>,
      >>::from_bytes(&buf[16..24]))),
      hash: Rc::new(RefCell::new(
        <FnPtr<fn(AnyPtr, usize, Ptr<u8>) -> Ptr<u8>>>::from_bytes(&buf[24..32]),
      )),
      size: Rc::new(RefCell::new(<u32>::from_bytes(&buf[32..36]))),
    }
  }
}
#[derive()]
pub struct v8_internal_HASH_CTX {
  pub f: Value<Ptr<v8_internal_HASH_VTAB>>,
  pub count: Value<u64>,
  pub buf: Value<Box<[u8]>>,
  pub state: Value<Box<[u32]>>,
}
impl Clone for v8_internal_HASH_CTX {
  fn clone(&self) -> Self {
    let __this: Value<v8_internal_HASH_CTX> = Rc::new(RefCell::new(Self {
      f: Rc::new(RefCell::new((*self.f.borrow()).clone())),
      count: Rc::new(RefCell::new((*self.count.borrow()))),
      buf: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 64, _>(
        |__i: usize| (*self.buf.borrow())[(__i) as usize],
      )))),
      state: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 8, _>(
        |__i: usize| (*self.state.borrow())[(__i) as usize],
      )))),
    }));
    let this: Ptr<v8_internal_HASH_CTX> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl Default for v8_internal_HASH_CTX {
  fn default() -> Self {
    v8_internal_HASH_CTX {
      f: Rc::new(RefCell::new(Ptr::<v8_internal_HASH_VTAB>::null())),
      count: <Value<u64>>::default(),
      buf: Rc::new(RefCell::new(
        (0..64).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
      )),
      state: Rc::new(RefCell::new(
        (0..8).map(|_| <u32>::default()).collect::<Box<[u32]>>(),
      )),
    }
  }
}
impl ByteRepr for v8_internal_HASH_CTX {
  fn byte_size() -> usize {
    112
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    (*self.f.borrow()).to_bytes(&mut buf[0..8]);
    (*self.count.borrow()).to_bytes(&mut buf[8..16]);
    (*self.buf.borrow()).to_bytes(&mut buf[16..80]);
    (*self.state.borrow()).to_bytes(&mut buf[80..112]);
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self {
      f: Rc::new(RefCell::new(<Ptr<v8_internal_HASH_VTAB>>::from_bytes(
        &buf[0..8],
      ))),
      count: Rc::new(RefCell::new(<u64>::from_bytes(&buf[8..16]))),
      buf: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[16..80]))),
      state: Rc::new(RefCell::new(<Box<[u32]>>::from_bytes(&buf[80..112]))),
    }
  }
}
thread_local!(
  pub static K_2: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
    1116352408_u32,
    1899447441_u32,
    3049323471_u32,
    3921009573_u32,
    961987163_u32,
    1508970993_u32,
    2453635748_u32,
    2870763221_u32,
    3624381080_u32,
    310598401_u32,
    607225278_u32,
    1426881987_u32,
    1925078388_u32,
    2162078206_u32,
    2614888103_u32,
    3248222580_u32,
    3835390401_u32,
    4022224774_u32,
    264347078_u32,
    604807628_u32,
    770255983_u32,
    1249150122_u32,
    1555081692_u32,
    1996064986_u32,
    2554220882_u32,
    2821834349_u32,
    2952996808_u32,
    3210313671_u32,
    3336571891_u32,
    3584528711_u32,
    113926993_u32,
    338241895_u32,
    666307205_u32,
    773529912_u32,
    1294757372_u32,
    1396182291_u32,
    1695183700_u32,
    1986661051_u32,
    2177026350_u32,
    2456956037_u32,
    2730485921_u32,
    2820302411_u32,
    3259730800_u32,
    3345764771_u32,
    3516065817_u32,
    3600352804_u32,
    4094571909_u32,
    275423344_u32,
    430227734_u32,
    506948616_u32,
    659060556_u32,
    883997877_u32,
    958139571_u32,
    1322822218_u32,
    1537002063_u32,
    1747873779_u32,
    1955562222_u32,
    2024104815_u32,
    2227730452_u32,
    2361852424_u32,
    2428436474_u32,
    2756734187_u32,
    3204031479_u32,
    3329325298_u32,
  ])));
);
pub fn SHA256_Transform_3(ctx: Ptr<v8_internal_HASH_CTX>) {
  let ctx: Value<Ptr<v8_internal_HASH_CTX>> = Rc::new(RefCell::new(ctx));
  let W: Value<Box<[u32]>> = Rc::new(RefCell::new(
    (0..64).map(|_| <u32>::default()).collect::<Box<[u32]>>(),
  ));
  let A: Value<u32> = <Value<u32>>::default();
  let B: Value<u32> = <Value<u32>>::default();
  let C: Value<u32> = <Value<u32>>::default();
  let D: Value<u32> = <Value<u32>>::default();
  let E: Value<u32> = <Value<u32>>::default();
  let F: Value<u32> = <Value<u32>>::default();
  let G: Value<u32> = <Value<u32>>::default();
  let H: Value<u32> = <Value<u32>>::default();
  let p: Value<Ptr<u8>> = Rc::new(RefCell::new(
    ((*(*ctx.borrow()).upgrade().deref()).buf.as_pointer() as Ptr<u8>),
  ));
  let t: Value<i32> = <Value<i32>>::default();
  (*t.borrow_mut()) = 0;
  'loop_: while ((*t.borrow()) < 16) {
    let tmp: Value<u32> = Rc::new(RefCell::new(
      ((((*p.borrow_mut()).postfix_inc().read()) as u32) << 24),
    ));
    (*tmp.borrow_mut()) |= ((((*p.borrow_mut()).postfix_inc().read()) as u32) << 16);
    (*tmp.borrow_mut()) |= ((((*p.borrow_mut()).postfix_inc().read()) as u32) << 8);
    (*tmp.borrow_mut()) |= (((*p.borrow_mut()).postfix_inc().read()) as u32);
    (*W.borrow_mut())[(*t.borrow()) as usize] = (*tmp.borrow());
    (*t.borrow_mut()).prefix_inc();
  }
  'loop_: while ((*t.borrow()) < 64) {
    let s0: Value<u32> = Rc::new(RefCell::new(
      ((((((*W.borrow())[((*t.borrow()) - 15) as usize]) >> (7))
        | (((*W.borrow())[((*t.borrow()) - 15) as usize]) << (32 - (7))))
        ^ ((((*W.borrow())[((*t.borrow()) - 15) as usize]) >> (18))
          | (((*W.borrow())[((*t.borrow()) - 15) as usize]) << (32 - (18)))))
        ^ (((*W.borrow())[((*t.borrow()) - 15) as usize]) >> (3))),
    ));
    let s1: Value<u32> = Rc::new(RefCell::new(
      ((((((*W.borrow())[((*t.borrow()) - 2) as usize]) >> (17))
        | (((*W.borrow())[((*t.borrow()) - 2) as usize]) << (32 - (17))))
        ^ ((((*W.borrow())[((*t.borrow()) - 2) as usize]) >> (19))
          | (((*W.borrow())[((*t.borrow()) - 2) as usize]) << (32 - (19)))))
        ^ (((*W.borrow())[((*t.borrow()) - 2) as usize]) >> (10))),
    ));
    let __rhs = ((((*W.borrow())[((*t.borrow()) - 16) as usize]).wrapping_add((*s0.borrow())))
      .wrapping_add((*W.borrow())[((*t.borrow()) - 7) as usize]))
    .wrapping_add((*s1.borrow()));
    (*W.borrow_mut())[(*t.borrow()) as usize] = __rhs;
    (*t.borrow_mut()).postfix_inc();
  }
  (*A.borrow_mut()) = (*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(0) as usize];
  (*B.borrow_mut()) = (*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(1) as usize];
  (*C.borrow_mut()) = (*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(2) as usize];
  (*D.borrow_mut()) = (*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(3) as usize];
  (*E.borrow_mut()) = (*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(4) as usize];
  (*F.borrow_mut()) = (*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(5) as usize];
  (*G.borrow_mut()) = (*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(6) as usize];
  (*H.borrow_mut()) = (*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(7) as usize];
  (*t.borrow_mut()) = 0;
  'loop_: while ((*t.borrow()) < 64) {
    let s0: Value<u32> = Rc::new(RefCell::new(
      (((((*A.borrow()) >> (2)) | ((*A.borrow()) << (32 - (2))))
        ^ (((*A.borrow()) >> (13)) | ((*A.borrow()) << (32 - (13)))))
        ^ (((*A.borrow()) >> (22)) | ((*A.borrow()) << (32 - (22))))),
    ));
    let maj: Value<u32> = Rc::new(RefCell::new(
      ((((*A.borrow()) & (*B.borrow())) ^ ((*A.borrow()) & (*C.borrow())))
        ^ ((*B.borrow()) & (*C.borrow()))),
    ));
    let t2: Value<u32> = Rc::new(RefCell::new((*s0.borrow()).wrapping_add((*maj.borrow()))));
    let s1: Value<u32> = Rc::new(RefCell::new(
      (((((*E.borrow()) >> (6)) | ((*E.borrow()) << (32 - (6))))
        ^ (((*E.borrow()) >> (11)) | ((*E.borrow()) << (32 - (11)))))
        ^ (((*E.borrow()) >> (25)) | ((*E.borrow()) << (32 - (25))))),
    ));
    let ch: Value<u32> = Rc::new(RefCell::new(
      (((*E.borrow()) & (*F.borrow())) ^ ((!(*E.borrow())) & (*G.borrow()))),
    ));
    let t1: Value<u32> = Rc::new(RefCell::new(
      ((((*H.borrow()).wrapping_add((*s1.borrow()))).wrapping_add((*ch.borrow())))
        .wrapping_add((*K_2.with(Value::clone).borrow())[(*t.borrow()) as usize]))
      .wrapping_add((*W.borrow())[(*t.borrow()) as usize]),
    ));
    (*H.borrow_mut()) = (*G.borrow());
    (*G.borrow_mut()) = (*F.borrow());
    (*F.borrow_mut()) = (*E.borrow());
    (*E.borrow_mut()) = (*D.borrow()).wrapping_add((*t1.borrow()));
    (*D.borrow_mut()) = (*C.borrow());
    (*C.borrow_mut()) = (*B.borrow());
    (*B.borrow_mut()) = (*A.borrow());
    (*A.borrow_mut()) = (*t1.borrow()).wrapping_add((*t2.borrow()));
    (*t.borrow_mut()).postfix_inc();
  }
  {
    let rhs_0 = ((*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(0) as usize])
      .wrapping_add((*A.borrow()));
    (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(0) as usize] = rhs_0
  };
  {
    let rhs_0 = ((*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(1) as usize])
      .wrapping_add((*B.borrow()));
    (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(1) as usize] = rhs_0
  };
  {
    let rhs_0 = ((*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(2) as usize])
      .wrapping_add((*C.borrow()));
    (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(2) as usize] = rhs_0
  };
  {
    let rhs_0 = ((*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(3) as usize])
      .wrapping_add((*D.borrow()));
    (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(3) as usize] = rhs_0
  };
  {
    let rhs_0 = ((*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(4) as usize])
      .wrapping_add((*E.borrow()));
    (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(4) as usize] = rhs_0
  };
  {
    let rhs_0 = ((*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(5) as usize])
      .wrapping_add((*F.borrow()));
    (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(5) as usize] = rhs_0
  };
  {
    let rhs_0 = ((*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(6) as usize])
      .wrapping_add((*G.borrow()));
    (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(6) as usize] = rhs_0
  };
  {
    let rhs_0 = ((*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(7) as usize])
      .wrapping_add((*H.borrow()));
    (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(7) as usize] = rhs_0
  };
}
thread_local!(
  pub static SHA256_VTAB_4: Value<v8_internal_HASH_VTAB> =
    Rc::new(RefCell::new(v8_internal_HASH_VTAB {
      init: Rc::new(RefCell::new(FnPtr::<fn(Ptr<v8_internal_HASH_CTX>)>::new(
        SHA256_init_5,
      ))),
      update: Rc::new(RefCell::new(FnPtr::<
        fn(Ptr<v8_internal_HASH_CTX>, AnyPtr, usize),
      >::new(SHA256_update_6))),
      final_: Rc::new(RefCell::new(FnPtr::<
        fn(Ptr<v8_internal_HASH_CTX>) -> Ptr<u8>,
      >::new(SHA256_final_7))),
      hash: Rc::new(RefCell::new(
        FnPtr::<fn(AnyPtr, usize, Ptr<u8>) -> Ptr<u8>>::new(SHA256_hash_8),
      )),
      size: Rc::new(RefCell::new(
        ((*kSizeOfSha256Digest_0.with(Value::clone).borrow()) as u32),
      )),
    }));
);
pub fn SHA256_init_5(ctx: Ptr<v8_internal_HASH_CTX>) {
  let ctx: Value<Ptr<v8_internal_HASH_CTX>> = Rc::new(RefCell::new(ctx));
  (*(*(*ctx.borrow()).upgrade().deref()).f.borrow_mut()) =
    (SHA256_VTAB_4.with(Value::clone).as_pointer());
  (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(0) as usize] = 1779033703_u32;
  (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(1) as usize] = 3144134277_u32;
  (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(2) as usize] = 1013904242_u32;
  (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(3) as usize] = 2773480762_u32;
  (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(4) as usize] = 1359893119_u32;
  (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(5) as usize] = 2600822924_u32;
  (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(6) as usize] = 528734635_u32;
  (*(*(*ctx.borrow()).upgrade().deref()).state.borrow_mut())[(7) as usize] = 1541459225_u32;
  (*(*(*ctx.borrow()).upgrade().deref()).count.borrow_mut()) = 0_u64;
}
pub fn SHA256_update_6(ctx: Ptr<v8_internal_HASH_CTX>, data: AnyPtr, len: usize) {
  let ctx: Value<Ptr<v8_internal_HASH_CTX>> = Rc::new(RefCell::new(ctx));
  let data: Value<AnyPtr> = Rc::new(RefCell::new(data));
  let len: Value<usize> = Rc::new(RefCell::new(len));
  let i: Value<i32> = Rc::new(RefCell::new(
    (((*(*(*ctx.borrow()).upgrade().deref()).count.borrow()) & 63_u64) as i32),
  ));
  let p: Value<Ptr<u8>> = Rc::new(RefCell::new((*data.borrow()).reinterpret_cast::<u8>()));
  {
    let rhs_0 =
      (*(*(*ctx.borrow()).upgrade().deref()).count.borrow()).wrapping_add(((*len.borrow()) as u64));
    (*(*(*ctx.borrow()).upgrade().deref()).count.borrow_mut()) = rhs_0
  };
  'loop_: while ((*len.borrow_mut()).postfix_dec() != 0) {
    let __rhs = ((*p.borrow_mut()).postfix_inc().read());
    (*(*(*ctx.borrow()).upgrade().deref()).buf.borrow_mut())
      [((*i.borrow_mut()).postfix_inc()) as usize] = __rhs;
    if ((*i.borrow()) == 64) {
      ({ SHA256_Transform_3((*ctx.borrow()).clone()) });
      (*i.borrow_mut()) = 0;
    }
  }
}
pub fn SHA256_final_7(ctx: Ptr<v8_internal_HASH_CTX>) -> Ptr<u8> {
  let ctx: Value<Ptr<v8_internal_HASH_CTX>> = Rc::new(RefCell::new(ctx));
  let p: Value<Ptr<u8>> = Rc::new(RefCell::new(
    ((*(*ctx.borrow()).upgrade().deref()).buf.as_pointer() as Ptr<u8>),
  ));
  let cnt: Value<u64> = Rc::new(RefCell::new(
    ((*(*(*ctx.borrow()).upgrade().deref()).count.borrow()) << (3)),
  ));
  let i: Value<i32> = <Value<i32>>::default();
  let completion: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([128_u8, 0_u8])));
  ({
    SHA256_update_6(
      (*ctx.borrow()).clone(),
      (((completion.as_pointer() as Ptr<u8>).offset(0)) as Ptr<u8>).to_any(),
      1_usize,
    )
  });
  'loop_: while (((*(*(*ctx.borrow()).upgrade().deref()).count.borrow()) & 63_u64) != 56_u64) {
    ({
      SHA256_update_6(
        (*ctx.borrow()).clone(),
        (((completion.as_pointer() as Ptr<u8>).offset(1)) as Ptr<u8>).to_any(),
        1_usize,
      )
    });
  }
  (*i.borrow_mut()) = 0;
  'loop_: while ((*i.borrow()) < 8) {
    let tmp: Value<u8> = Rc::new(RefCell::new((((*cnt.borrow()) >> (56)) as u8)));
    let __rhs = ((*cnt.borrow()) << (8));
    (*cnt.borrow_mut()) = __rhs;
    ({
      SHA256_update_6(
        (*ctx.borrow()).clone(),
        ((tmp.as_pointer()) as Ptr<u8>).to_any(),
        1_usize,
      )
    });
    (*i.borrow_mut()).prefix_inc();
  }
  (*i.borrow_mut()) = 0;
  'loop_: while ((*i.borrow()) < 8) {
    let tmp: Value<u32> = Rc::new(RefCell::new(
      (*(*(*ctx.borrow()).upgrade().deref()).state.borrow())[(*i.borrow()) as usize],
    ));
    let __rhs = (((*tmp.borrow()) >> 24) as u8);
    (*p.borrow_mut()).postfix_inc().write(__rhs);
    let __rhs = (((*tmp.borrow()) >> 16) as u8);
    (*p.borrow_mut()).postfix_inc().write(__rhs);
    let __rhs = (((*tmp.borrow()) >> 8) as u8);
    (*p.borrow_mut()).postfix_inc().write(__rhs);
    let __rhs = (((*tmp.borrow()) >> 0) as u8);
    (*p.borrow_mut()).postfix_inc().write(__rhs);
    (*i.borrow_mut()).postfix_inc();
  }
  return ((*(*ctx.borrow()).upgrade().deref()).buf.as_pointer() as Ptr<u8>);
}
pub fn SHA256_hash_8(data: AnyPtr, len: usize, digest: Ptr<u8>) -> Ptr<u8> {
  let data: Value<AnyPtr> = Rc::new(RefCell::new(data));
  let len: Value<usize> = Rc::new(RefCell::new(len));
  let digest: Value<Ptr<u8>> = Rc::new(RefCell::new(digest));
  let ctx: Value<v8_internal_HASH_CTX> = Rc::new(RefCell::new(<v8_internal_HASH_CTX>::default()));
  ({ SHA256_init_5((ctx.as_pointer())) });
  ({
    SHA256_update_6(
      (ctx.as_pointer()),
      (*data.borrow()).clone(),
      (*len.borrow()),
    )
  });
  {
    ((*digest.borrow()).clone() as Ptr<u8>).to_any().memcpy(
      &(({ SHA256_final_7((ctx.as_pointer())) }) as Ptr<u8>).to_any(),
      (*kSizeOfSha256Digest_0.with(Value::clone).borrow()) as usize,
    );
    ((*digest.borrow()).clone() as Ptr<u8>).to_any()
  };
  return (*digest.borrow()).clone();
}
