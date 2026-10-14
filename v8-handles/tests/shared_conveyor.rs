//! SharedObjectConveyorHandles 语义测试: persist 递增 id, 越界错误, 取回保真
use aok::{OK, Void};
use log::info;
use v8_handles::{
  Error, HandleScope, HeapObject, PersistentHandles, PersistentHandlesList,
  SharedObjectConveyorHandles, Tagged,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 规约: Persist 返回递增稳定 id (push 前 size), HasPersisted/GetPersisted 保真
#[test]
fn conveyor_persist_ids() -> Void {
  let list = PersistentHandlesList::new();
  let ph = PersistentHandles::new(&list);
  let mut conveyor = SharedObjectConveyorHandles::new(ph);

  assert!(!conveyor.has_persisted(0));
  let id0 = conveyor.persist(Tagged::<HeapObject>::from_ptr(0x3001));
  let id1 = conveyor.persist(Tagged::<HeapObject>::from_ptr(0x3002));
  let id2 = conveyor.persist(Tagged::<HeapObject>::from_ptr(0x3003));
  assert_eq!((id0, id1, id2), (0, 1, 2));

  assert!(conveyor.has_persisted(0));
  assert!(conveyor.has_persisted(2));
  assert!(!conveyor.has_persisted(3));
  assert_eq!(
    conveyor.get_persisted(0).ok().map(|t| t.ptr()),
    Some(0x3001)
  );
  assert_eq!(
    conveyor.get_persisted(1).ok().map(|t| t.ptr()),
    Some(0x3002)
  );
  assert_eq!(
    conveyor.get_persisted(2).ok().map(|t| t.ptr()),
    Some(0x3003)
  );
  // 越界走错误 (原 DCHECK(HasPersisted) → Result)
  assert!(matches!(
    conveyor.get_persisted(3),
    Err(Error::InvalidObjectId(3))
  ));
  OK
}

/// 规约: 保活通道跨 HandleScope —— 对象经 conveyor 存续于 PersistentHandles
#[test]
fn conveyor_keeps_objects_alive() -> Void {
  let list = PersistentHandlesList::new();
  let ph = PersistentHandles::new(&list);
  let mut conveyor = SharedObjectConveyorHandles::new(ph);
  let mut ids = Vec::new();
  {
    // 模拟 ValueSerializer 侧会话: persist 的对象不随临时作用域失效
    let _scope = HandleScope::new();
    for i in 0..4u32 {
      ids.push(conveyor.persist(Tagged::<HeapObject>::from_ptr(0x4000 + i as usize)));
    }
  }
  for (i, &id) in ids.iter().enumerate() {
    assert_eq!(id, i as u32);
    assert_eq!(
      conveyor.get_persisted(id).ok().map(|t| t.ptr()),
      Some(0x4000 + i)
    );
  }
  info!("conveyor keeps {} objects", ids.len());
  OK
}
