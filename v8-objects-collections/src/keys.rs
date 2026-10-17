// 由 tool/src/objects/keys.cc.js 从 v8/src/objects/keys.cc 生成; 修订就地改, 复跑覆盖
// 键收集器方法面 (配置枚举归 v8-objects-rest/keys.rs)

//! KeyAccumulator / FastKeyAccumulator 的 .cc 方法面。
//!
//! - 配置枚举 (AddKeyConversion/GetKeysConversion/KeyCollectionMode/IndexedOrNamed)
//!   已由 keys.h.js 提取至 v8-objects-rest/src/keys.rs
//! - 快路径 (FastKeyAccumulator::GetKeysFast) 失败回退慢路径 (GetKeysSlow)
//! - 组键规则: 原型链键并入自有键, 去重依据 = receiver map 的描述符键

// ── 匿名辅助规则 (keys.cc:50-98 namespace) ────────────────────────────
/// 键集合有效性: 每项须为 Name 或 Number (ContainsOnlyValidKeys)。
pub const VALID_KEY_PREDICATE: &str = "IsName(e) || IsNumber(e)";
/// CombineKeys 快路径: 无自有描述符且无元素 → 直接复用原型链键数组。
pub const COMBINE_KEYS_FAST_PATH: &str = "nof_descriptors == 0 && !may_have_elements";
/// 组键尾部: 右裁剪到实际长度 (RightTrimOrEmpty)。
pub const COMBINE_KEYS_TRIM: &str = "RightTrimOrEmpty";
/// AddKey 查重: 描述符中已含该键则不计入 (返回 0)。
pub const ADD_KEY_DEDUPE: &str = "descs->GetKey(i) == key";

// ── KeyAccumulator 方法面 (29 项, 顺序登记) ────────────────
/// KeyAccumulator 全部 .cc 定义 (对数基准)。
pub const KEY_ACCUMULATOR_METHODS: [&str; 29] = [
  "AddKey",
  "AddKeys",
  "AddKeysFromJSProxy",
  "AddShadowingKey",
  "CollectAccessCheckInterceptorKeys",
  "CollectInterceptorKeys",
  "CollectInterceptorKeysInternal",
  "CollectKeys",
  "CollectOwnElementIndices",
  "CollectOwnJSProxyKeys",
  "CollectOwnJSProxyTargetKeys",
  "CollectOwnKeys",
  "CollectOwnPropertyNames",
  "CollectPrivateNames",
  "FilterForEnumerableProperties",
  "GetKeys",
  "GetKeysFast",
  "GetKeysSlow",
  "GetKeysWithPrototypeInfoCache",
  "GetOwnEnumPropertyKeys",
  "GetOwnKeysWithUninitializedEnumLength",
  "HasShadowingKeys",
  "InitializeFastPropertyEnumCache",
  "IsShadowed",
  "MayHaveElements",
  "MayHaveSymbols",
  "Prepare",
  "TryPrototypeInfoCache",
  "keys",
];

// ── FastKeyAccumulator 方法面 (10 项) ──────────────────
/// FastKeyAccumulator 全部 .cc 定义 (对数基准)。
pub const FAST_KEY_ACCUMULATOR_METHODS: [&str; 10] = [
  "GetKeys",
  "GetKeysFast",
  "GetKeysSlow",
  "GetKeysWithPrototypeInfoCache",
  "GetOwnKeysWithUninitializedEnumLength",
  "InitializeFastPropertyEnumCache",
  "MayHaveElements",
  "MayHaveSymbols",
  "Prepare",
  "TryPrototypeInfoCache",
];

// ── 主流程签名骨架 (就地修订; 参数为 Address/枚举值占位) ───────────────
/// 键收集入口 (KeyAccumulator::GetKeys; receiver + 配置 → 键数组)。
///
/// v8: src/objects/keys.cc KeyAccumulator::GetKeys
pub fn get_keys(_receiver: u64, _mode: u8, _convert: u8, _filter: u8) -> u64 {
  todo!("L4: 需 JSReceiver/PrototypeIterator + Object::FilterKey + Factory 数组物化 (全 receiver 模型)")
}
/// 逐键添加 (AddKey; CONVERT_TO_ARRAY_INDEX 转换分支)。
///
/// v8: src/objects/keys.cc KeyAccumulator::AddKey
pub fn add_key(_key: u64, _convert: u8) -> u32 {
  todo!("L4: 需 KeyAccumulator 后端可增长向量 (BaseVector/Factory) + 描述符查重 (全对象模型)")
}
/// 批量并入键 (AddKeys; 数组/类数组双形态)。
///
/// v8: src/objects/keys.cc KeyAccumulator::AddKeys
pub fn add_keys(_array_like: u64) -> u32 {
  todo!("L4: 需 JSArray/JSObject 类数组读取 + FixedArray 迭代 (全 receiver 模型)")
}
/// 收集 receiver 原型链键 (CollectKeys; 拦截器/代理/私有名分派)。
///
/// v8: src/objects/keys.cc KeyAccumulator::CollectKeys
pub fn collect_keys(_receiver: u64) -> u32 {
  todo!("L4: 需原型链遍历 + 拦截器/JSProxy/私有名分派 (全 receiver + Isolate 对象模型)")
}
/// 快路径主入口 (FastKeyAccumulator::GetKeys; 失败回退慢路径)。
///
/// v8: src/objects/keys.cc FastKeyAccumulator::GetKeys
pub fn get_keys_fast_entry(_receiver: u64, _mode: u8, _convert: u8, _filter: u8) -> u64 {
  todo!("L4: 需枚举缓存 (enum cache) 快路径 + CombineKeys + Factory FixedArray 物化")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(KEY_ACCUMULATOR_METHODS.len() == 29);
  assert!(FAST_KEY_ACCUMULATOR_METHODS.len() == 10);
  assert!(KEY_ACCUMULATOR_METHODS.contains(&"CollectOwnJSProxyKeys"));
  assert!(FAST_KEY_ACCUMULATOR_METHODS.contains(&"GetKeysFast"));
};
