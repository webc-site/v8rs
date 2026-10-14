//! 原版 v8/src/base/types/is-instantiation.h
//! C++ 概念 `is_instantiation<T, C>` 判断 `T` 是否为模板 `C` 的实例化。
//! Rust 的类型系统在单态化时天然区分实例化, 转写时以下游泛型约束 (`Vec<T>` 等具体型构造)
//! 直接表达; 此模块提供 marker trait 以保留 1:1 对照点。

/// marker: 泛型模板的"构造器"身份标记 (对应 C++ 的 `template <typename...> class C`)
pub trait TemplateCtor {}

/// `T: IsInstantiationOf<C>` 当且仅当 `T` 是模板 `C` 的实例化
/// (对应 `is_instantiation_v<T, C>`)
pub trait IsInstantiationOf<C: TemplateCtor> {}

/// 示例登记: `Vec<T>` 是 `Vec` 模板的实例化
pub struct VecCtor;
impl TemplateCtor for VecCtor {}
impl<T> IsInstantiationOf<VecCtor> for Vec<T> {}

/// 示例登记: `Option<T>` 是 `Option` 模板的实例化
pub struct OptionCtor;
impl TemplateCtor for OptionCtor {}
impl<T> IsInstantiationOf<OptionCtor> for Option<T> {}
