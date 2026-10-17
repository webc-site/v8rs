# 术语表 (C++ ↔ Rust / gtest → Rust)

所有 Developer / Reviewer / Test 子代理 prompt 必须注入本表。
新增映射由 crate 开发者追加, Reviewer 核对一致性。

## 基础类型

| C++ (v8) | Rust | 说明 |
|---|---|---|
| `Address` (`uintptr_t`) | `usize` | 平台指针宽度 |
| `Tagged_t` (`uint32_t`/`uint64_t` 压缩指针槽) | `u32` / `u64` (按 kApiTaggedSize) | ptr-compr |
| `uint8_t/int8_t/...` | `u8/i8/...` | 直接映射 |
| `size_t` | `usize` | |
| `base::Optional` / `std::optional` | `Option<T>` | |
| `std::unique_ptr<T>` | `Box<T>` 或所有权 | 不引入裸指针 |
| `std::vector<T>` | `Vec<T>` (zone 语境用 zone 数组) | |
| `std::atomic<T>` | `std::sync::atomic::AtomicXxx` | |
| `base::Mutex/RecursiveMutex` | `parking_lot::Mutex` (tokio 语境除外) | |
| `v8::internal::Handle/T` | L3 前用 `crate` 内句柄类型占位 | objects 层重设计 |
| C++ 枚举 `enum class Foo` | `#[derive(Debug, Clone, Copy, PartialEq, Eq)] #[repr(xxx)] enum Foo` | 配 `strum` |
| `kConstantName` | `CONSTANT_NAME` (SCREAMING_SNAKE) | 常量 |
| `FooBar` 函数/方法 | `foo_bar` (snake_case) | |
| `class Foo` | `struct Foo` / `trait` 按语义 | 虚函数大户用 `enum_dispatch` |
| `#define V8_FOO_LIST(V) ...` X-macro | 展开为显式 `enum` / `const` 表 / `macro_rules!` 1:1 | 禁"理解"宏, 机械展开登记 |
| `#ifdef V8_ENABLE_XXX` | crate `feature` 或编译期 `const` | 先展开默认配置 |
| `DCHECK/CHECK` | `debug_assert!` / `assert!` (禁 panic 路径的用 Result) | |
| `UNREACHABLE()` | `unreachable!()` 仅限 100% 不可达, 否则 `Result<_, Error>` | |
| `std::function` | `impl Fn` / `Box<dyn Fn>` (dyn 最小化) | |

## gtest → Rust 测试写法

| gtest | Rust | 说明 |
|---|---|---|
| `TEST(Suite, Name)` | `#[test] fn suite_name() -> Void` | `aok::Void`, 以 `OK` 结尾 |
| `TEST_F(Fixture, Name)` | 共享构造函数 + 独立 `#[test]` | Rust 无 fixture 继承 |
| `TYPED_TEST` | 泛型 `#[test]` + 宏展开实例 | |
| `EXPECT_EQ(a, b)` | `assert_eq!(a, b)` | |
| `EXPECT_NE/LT/LE/GT/GE` | `assert_ne!` / 手写比较 | |
| `EXPECT_TRUE/FALSE(x)` | `assert!(x)` / `assert!(!x)` | |
| `EXPECT_NEAR(a,b,eps)` | `assert!((a-b).abs() <= eps)` | 浮点 |
| `ASSERT_*` | 同 EXPECT 写法; 中止语义用 `?`/提前 return | Rust assert 失败即中止用例, 语义等价 |
| `EXPECT_DEATH_IF_SUPPORTED` | 跳过 (禁 panic), 写理由登记映射表 | |
| 测试日志初始化 | `#[ctor::ctor(unsafe)] fn _log_init() { log_init::init(); }` | tests/main.rs 模板 |
| 异步用例 | `compio::runtime::Runtime::new()?.block_on(async { ... })` | |

## 模块命名

| C++ 文件 | Rust 模块 |
|---|---|
| `foo-bar.h/cc` | `src/foo_bar/` (或 `src/foo_bar.rs`) |
| `foo-bar-inl.h` | 并入 `src/foo_bar/` 对应实现 (Rust 无 inl, 内联由编译器决定) |
| `class-inl.h` 模板 | 泛型 + `#[inline]` |

## 差分/确定性约定 (7.4)

- 测试禁随机: 固定 seed, 用 `fastrand` 显式播种
- 禁依赖 HashMap 迭代序 (std/gxhash 一律), 语义有序用 `BTreeMap` 或 Vec
- 浮点断言禁 `==`, 除 golden 数据逐字节比对场景
