#!/usr/bin/env bun
// tool/src/compiler/turboshaft/zone-with-name.h.js
// 机械提取 v8/src/compiler/turboshaft/zone-with-name.h
// → v8-turboshaft/src/zone_with_name.rs.skel
//
// 提取面: ZoneWithName/ZoneWithNamePointer 类型身份机制 (DEBUG 强化)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/zone-with-name.h");

// 1. 哨兵
for (const [re, name] of [
  [/class ZoneWithNamePointerImpl final \{/, "DEBUG 指针包装"],
  [/template <typename T, auto>\s*\nusing ZoneWithNamePointer = T\*;/, "release 退化为裸指针"],
  [/class ZoneWithName final \{/, "ZoneWithName"],
  [/DCHECK_EQ\(std::strcmp\(name, Name\.c_str\(\)\), 0\);/, "名字一致性校验"],
  [/ZoneStats::Scope scope_;/, "ZoneStats::Scope"],
  [/ZoneWithNamePointer<T, Name> New\(Args&&\.\.\. args\)/, "New 转发"],
  [/ZoneWithNamePointer<T, Name> AllocateArray\(size_t length\)/, "AllocateArray"],
  [/void Destroy\(\) \{ scope_\.Destroy\(\); \}/, "Destroy"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. 条件面
const debugTyped = /#if defined\(DEBUG\) && defined\(HAS_CPP_CLASS_TYPES_AS_TEMPLATE_ARGS\)/.test(h);

let rust = banner(
  "src/compiler/turboshaft/zone-with-name.h.js",
  "src/compiler/turboshaft/zone-with-name.h",
  "具名 zone: 类型级 zone 身份编码 (防误用)",
);

rust += `
/// 原版 ZoneWithNamePointer<T, Name> — DEBUG + 类类型模板参可用时是
/// 带 zone 名的类型化指针包装 (空基优化挂 KeyData), release 退化为 T*。
/// Rust 侧以 newtype + phantom 承载同等身份编码:
/// v8: src/compiler/turboshaft/zone-with-name.h ZoneWithNamePointerImpl
#[derive(Debug)]
pub struct ZoneWithNamePointer<T, const NAME: &'static str> {
  pub ptr: *mut T,
  _name: PhantomData<[(); NAME.len()]>,
}

impl<T, const NAME: &'static str> ZoneWithNamePointer<T, NAME> {
  /// v8: operator pointer_type / get / operator* / operator->
  pub fn get(&self) -> *mut T { self.ptr }
}

/// 原版 ZoneWithName<Name> — ZoneStats::Scope 持有真 zone;
/// 构造时 DCHECK 传入 name 与类型名一致。
/// 方法面: New<T>(args...) / AllocateArray<T>(len) (均返回具名指针),
/// get/operator Zone*/operator->/Destroy()。
/// v8: zone-with-name.h ZoneWithName
pub struct ZoneWithName<const NAME: &'static str> {
  /// NOTE: ZoneStats::Scope 实际分配新 zone
  pub scope: ZoneStatsScope,
}

impl<const NAME: &'static str> ZoneWithName<NAME> {
  /// v8: New — zone 分配 + 具名指针包装
  pub fn new<T>(&mut self) -> ZoneWithNamePointer<T, NAME> { todo!() }
  /// v8: AllocateArray
  pub fn allocate_array<T>(&mut self, len: usize) -> ZoneWithNamePointer<T, NAME> { todo!() }
  /// v8: Destroy — scope_.Destroy()
  pub fn destroy(&mut self) { todo!() }
}

/// DEBUG 类型化面: ${debugTyped ? "启用 (DEBUG && HAS_CPP_CLASS_TYPES_AS_TEMPLATE_ARGS)" : "未启用"}
`;

const p = writeSrc("v8-turboshaft", "zone_with_name.rs.skel", rust);
console.log(`written ${p}`);
console.log(`类型=2 debugTyped=${debugTyped}`);
