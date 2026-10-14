#!/usr/bin/env bun
// tool/src/wasm/std-object-sizes.h.js
// 机械提取 v8/src/wasm/std-object-sizes.h 的容器内存估算公式

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/std-object-sizes.h");

// 1. 五个 ContentSize 重载
const overloads = [
  ["std::vector<T>", "vector.capacity() * sizeof(T)"],
  ["base::UniqueArray<T>", "vector.size() * sizeof(T)"],
  ["std::map<Key, T>", "map.size() * (sizeof(Key) + sizeof(T) + 2 * sizeof(void*))"],
  [
    "std::unordered_map<Key, T, Hash>",
    "map.size() * (sizeof(Key) + sizeof(T) + 2 * sizeof(void*))",
  ],
  ["std::unordered_set<T, Hash>", "set.size() * (sizeof(T) + 2 * sizeof(void*))"],
];
let overloadCount = 0;
for (const [sig] of overloads) {
  const m = hSrc.match(new RegExp(`ContentSize\\(const ${sig.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}&`));
  if (m) overloadCount++;
}
if (overloadCount !== 5) throw new Error(`ContentSize 重载数 ${overloadCount} != 5`);

// 2. 哈希容器 75% 填充率 → *4/3
const fillMatches = hSrc.match(/return raw \* 4 \/ 3;/g);
if (!fillMatches || fillMatches.length !== 2) {
  throw new Error(`raw*4/3 出现 ${fillMatches ? fillMatches.length : 0} 次 != 2`);
}

// 3. 每条目两根内部指针近似
if ((hSrc.match(/two internal pointers per entry/g) || []).length !== 3) {
  throw new Error("two internal pointers 注释数 != 3");
}

// 4. UPDATE_WHEN_CLASS_CHANGES 宏条件与展开
if (!hSrc.includes("#define UPDATE_WHEN_CLASS_CHANGES(classname, size)")) {
  throw new Error("未找到 UPDATE_WHEN_CLASS_CHANGES");
}
if (!hSrc.includes("static_assert(sizeof(classname) == size,")) {
  throw new Error("未找到 static_assert 展开");
}
const condAnchors = [
  "V8_TARGET_ARCH_X64",
  "defined(__clang__)",
  "V8_TARGET_OS_LINUX",
  "!V8_USE_ADDRESS_SANITIZER",
  "!V8_USE_MEMORY_SANITIZER",
  "defined(DEBUG)",
  "V8_COMPRESS_POINTERS",
  "!defined(V8_GC_MOLE)",
  "defined(_LIBCPP_VERSION)",
];
for (const c of condAnchors) {
  if (!hSrc.includes(c)) throw new Error(`断言条件缺失: ${c}`);
}

console.log(`std-object-sizes: 重载=5 fill4/3=${fillMatches.length} 宏条件=${condAnchors.length}`);

// 5. 生成 Rust
let rust = banner(
  "src/wasm/std-object-sizes.h.js",
  "src/wasm/std-object-sizes.h",
  "标准容器堆外内存 ContentSize 估算公式",
);

rust += `
//! 容器自身尺寸不计 (由持有方计入), 只算内容占用。
/// v8: src/wasm/std-object-sizes.h ContentSize

/// 每条目的内部指针开销近似 (std::map / unordered_map / unordered_set)
/// v8: src/wasm/std-object-sizes.h "two internal pointers per entry"
pub const INTERNAL_POINTERS_PER_ENTRY: usize = 2;

/// 哈希容器假定填充率 75%, 估算值放大 4/3
/// v8: src/wasm/std-object-sizes.h raw * 4 / 3
pub const HASH_FILL_NUMERATOR: usize = 4;
pub const HASH_FILL_DENOMINATOR: usize = 3;

/// Vec: capacity * elem_size (用容量而非长度, 算真实占用)
/// v8: src/wasm/std-object-sizes.h ContentSize(std::vector<T>&)
pub fn content_size_vec(capacity: usize, elem_size: usize) -> usize {
    capacity * elem_size
}

/// 定长数组 (相当于 UniqueArray): len * elem_size
/// v8: src/wasm/std-object-sizes.h ContentSize(base::UniqueArray<T>&)
pub fn content_size_array(len: usize, elem_size: usize) -> usize {
    len * elem_size
}

/// BTreeMap (对应 std::map): len * (key + val + 2 ptr)
/// v8: src/wasm/std-object-sizes.h ContentSize(std::map<Key, T>&)
pub fn content_size_map(len: usize, key_size: usize, val_size: usize) -> usize {
    len * (key_size + val_size + INTERNAL_POINTERS_PER_ENTRY * size_of::<usize>())
}

/// HashMap (对应 unordered_map): 粗估下界 * 4/3
/// v8: src/wasm/std-object-sizes.h ContentSize(std::unordered_map<...>&)
pub fn content_size_hash_map(len: usize, key_size: usize, val_size: usize) -> usize {
    let raw = content_size_map(len, key_size, val_size);
    raw * HASH_FILL_NUMERATOR / HASH_FILL_DENOMINATOR
}

/// HashSet (对应 unordered_set): len * (val + 2 ptr), 再 * 4/3
/// v8: src/wasm/std-object-sizes.h ContentSize(std::unordered_set<T, Hash>&)
pub fn content_size_hash_set(len: usize, elem_size: usize) -> usize {
    let raw = len * (elem_size + INTERNAL_POINTERS_PER_ENTRY * size_of::<usize>());
    raw * HASH_FILL_NUMERATOR / HASH_FILL_DENOMINATOR
}

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(INTERNAL_POINTERS_PER_ENTRY, 2);
    assert_eq!(HASH_FILL_NUMERATOR, 4);
    assert_eq!(HASH_FILL_DENOMINATOR, 3);
    assert_eq!(content_size_hash_set(3, 8), 3 * (8 + 16) * 4 / 3);
};
`;

const p = writeSrc("v8-wasm-objects", "std_object_sizes.rs.skel", rust);
console.log(`written ${p}`);
