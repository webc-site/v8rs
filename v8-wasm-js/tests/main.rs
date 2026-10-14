use aok::{OK, Void};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_wasm_memory_grow() -> Void {
  use v8_wasm_js::js_wasm_memory_grow;

  // 1. 正常扩容 (触发 V8 min_growth 启发式步进: 1 + 8 + 0 = 9 页)
  let grown = js_wasm_memory_grow(1, 2, 10);
  assert_eq!(grown, Some(9));

  // 2. 扩容至恰好达到上限
  let max_reached = js_wasm_memory_grow(8, 2, 10);
  assert_eq!(max_reached, Some(10));

  // 3. 超过上限失败
  let overflow = js_wasm_memory_grow(5, 6, 10);
  assert_eq!(overflow, None);

  // 4. 加法饱和溢出防护
  let sat_overflow = js_wasm_memory_grow(u32::MAX, 1, 10);
  assert_eq!(sat_overflow, None);

  // 5. 零页扩容防假扩容 (必须返回当前原页数，不得误触发 min_growth 步进)
  let zero_grow = js_wasm_memory_grow(1, 0, 10);
  assert_eq!(zero_grow, Some(1));

  // 6. 最大页数为 u32::MAX 时的溢出防卫 (确保 checked_add 拦截溢出，禁 panic/wrap)
  let max_overflow = js_wasm_memory_grow(u32::MAX, 1, u32::MAX);
  assert_eq!(max_overflow, None);

  // 7. 初始状态已超上限时的零页扩容拒收
  let already_exceeded = js_wasm_memory_grow(11, 0, 10);
  assert_eq!(already_exceeded, None);

  OK
}

#[test]
fn test_wasm_js_callbacks_and_references() -> Void {
  use v8_wasm_js::{
    API_INSTALLS, GLOBAL_VALUE_ACCESSOR_PAIR, OBJECT_NAMES, WASM_JS_EXTERNAL_REFERENCES,
    webassembly_api_callbacks,
  };

  // 1. 跨表一致性验证: API_INSTALLS 注册的核心非条件回调必须存在于外部引用总表
  for (target, prop, callback) in API_INSTALLS {
    if !prop.contains("Streaming") && *prop != "promising" {
      assert!(
        WASM_JS_EXTERNAL_REFERENCES.contains(callback),
        "API callback '{callback}' for {target}.{prop} must exist in WASM_JS_EXTERNAL_REFERENCES"
      );
    }
  }

  // 2. Global.value 的 getter/setter 成对访问器回调有效性
  assert!(WASM_JS_EXTERNAL_REFERENCES.contains(&GLOBAL_VALUE_ACCESSOR_PAIR.0));
  assert!(WASM_JS_EXTERNAL_REFERENCES.contains(&GLOBAL_VALUE_ACCESSOR_PAIR.1));

  // 3. API 回调列表非空且均在总表中建立索引
  let callbacks = webassembly_api_callbacks();
  assert!(!callbacks.is_empty());
  for cb in callbacks {
    assert!(WASM_JS_EXTERNAL_REFERENCES.contains(cb));
  }

  // 4. 对象映射表非空且键值均有效
  for (sym, js_name) in OBJECT_NAMES {
    assert!(!sym.is_empty());
    assert!(js_name.starts_with("WebAssembly."));
  }

  OK
}
