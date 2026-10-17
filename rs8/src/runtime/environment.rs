//! 词法执行环境与作用域链
//!
//! 采用 Arc 与 parking_lot::RwLock 组合的高效词法作用域链模型。
//! 变量查找与更新使用迭代遍历取代递归，更新既有变量实现零堆内存分配。

use std::sync::Arc;

use gxhash::HashMap;
use parking_lot::RwLock;

use crate::runtime::value::JsValue;

/// 词法作用域环境节点
#[derive(Clone)]
pub struct Environment {
  /// 当前环境内定义的变量表
  pub(crate) values: Arc<RwLock<HashMap<String, JsValue>>>,
  /// 词法父作用域引用
  pub(crate) parent: Option<Arc<Environment>>,
}

/// 函数调用激活帧初始化配置
pub struct ActivationScopeInit<'a> {
  pub this_opt: Option<JsValue>,
  pub fname_opt: Option<(&'a str, JsValue)>,
  pub arguments: Option<JsValue>,
  pub params: &'a [String],
  pub args: &'a [JsValue],
  /// rest 形参 (12.40): (绑定名, 调用实参尾段切片), 由 setup_activation_env 按 shared.rest_param 切好
  pub rest_param: Option<(&'a str, &'a [JsValue])>,
  pub hoisted_vars: &'a [String],
  pub hoisted_funcs: Vec<(String, JsValue)>,
}

impl Default for Environment {
  fn default() -> Self {
    Self::new()
  }
}

impl Environment {
  /// 创建独立的顶级作用域
  pub fn new() -> Self {
    Self {
      values: Arc::new(RwLock::new(HashMap::default())),
      parent: None,
    }
  }

  /// 基于父作用域创建子词法环境
  pub fn new_child(parent: &Arc<Environment>) -> Self {
    Self {
      values: Arc::new(RwLock::new(HashMap::default())),
      parent: Some(Arc::clone(parent)),
    }
  }

  /// 在当前环境定义变量
  pub fn define(&self, name: String, val: JsValue) {
    self.values.write().insert(name, val);
  }

  /// 批量初始化函数调用激活帧（对照 V8 作用域变量分配，单次锁争用与哈希预留）
  pub fn init_activation_scope(&self, init: ActivationScopeInit<'_>) {
    let mut guard = self.values.write();
    let total_slots = init.this_opt.is_some() as usize
      + init.arguments.is_some() as usize
      + init.fname_opt.is_some() as usize
      + init.params.len()
      + init.rest_param.is_some() as usize
      + init.hoisted_vars.len()
      + init.hoisted_funcs.len();
    guard.reserve(total_slots);

    if let Some(this) = init.this_opt {
      guard.insert("this".to_string(), this);
    }
    if let Some(arguments) = init.arguments {
      guard.insert("arguments".to_string(), arguments);
    }
    if let Some((fname, fval)) = init.fname_opt {
      guard.insert(fname.to_string(), fval);
    }
    for (i, p) in init.params.iter().enumerate() {
      let val = init.args.get(i).cloned().unwrap_or(JsValue::Undefined);
      guard.insert(p.clone(), val);
    }
    // rest 快照排在按位形参之后、hoist 之前 (hoist 遇同名键会跳过, 不会覆盖数组绑定)
    if let Some((rest_name, tail)) = init.rest_param {
      guard.insert(rest_name.to_string(), JsValue::new_array(tail.to_vec()));
    }
    for v in init.hoisted_vars {
      if !guard.contains_key(v) {
        guard.insert(v.clone(), JsValue::Undefined);
      }
    }
    for (name, fval) in init.hoisted_funcs {
      guard.insert(name, fval);
    }
  }

  /// 检查当前作用域是否已定义指定变量
  #[inline]
  pub fn has_own(&self, name: &str) -> bool {
    self.values.read().contains_key(name)
  }

  /// 快速检索原型链根部 Object.prototype (单次锁争用，规避深层词法作用域逐级加锁)
  pub fn get_object_prototype(&self) -> Option<JsValue> {
    let mut cur = self;
    while let Some(ref p) = cur.parent {
      cur = p;
    }
    let obj = cur.values.read().get("Object").cloned()?;
    obj.get_property("prototype").ok()
  }

  /// 迭代沿作用域链向上检索变量
  pub fn get(&self, name: &str) -> Option<JsValue> {
    let mut cur = Some(self);
    while let Some(env) = cur {
      if let Some(val) = env.values.read().get(name).cloned() {
        return Some(val);
      }
      cur = env.parent.as_deref();
    }
    None
  }

  /// 沿作用域链就地更新既有变量值
  pub fn set(&self, name: &str, val: JsValue) -> bool {
    let mut cur = Some(self);
    while let Some(env) = cur {
      let mut guard = env.values.write();
      if let Some(slot) = guard.get_mut(name) {
        *slot = val;
        return true;
      }
      drop(guard);
      cur = env.parent.as_deref();
    }
    false
  }

  /// 扁平化收集完整作用域链内所有变量 (自底向上单次遍历去重，消除中间 Vec 分配与冗余克隆)
  pub fn to_map(&self) -> HashMap<String, JsValue> {
    let mut map = HashMap::default();
    let mut cur = Some(self);
    while let Some(env) = cur {
      for (k, v) in env.values.read().iter() {
        if !map.contains_key(k) {
          map.insert(k.clone(), v.clone());
        }
      }
      cur = env.parent.as_deref();
    }
    map
  }
}
