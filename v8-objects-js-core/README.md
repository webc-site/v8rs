[English](#en) | [中文](#zh)

---

<a name="en"></a>

#

---

<a name="zh"></a>

# v8-objects-js-core

JSArrayBuffer 族差集骨架 `src/js_array_buffer.rs.skel` (由 `tool/src/objects/js-array-buffer.h.js` 生成, 复跑覆盖) 的推迟台账。

- [推迟台账 (轮7 W7LIT)](#推迟台账-轮7-w7lit)

## 推迟台账 (轮7 W7LIT)

深水面 todo! 保留登记 (`.skel` 未入编译图, 依赖未就位上层类型, 禁伪装填体):

- `js_array_buffer.rs.skel::type_and_element_size_for` — 对位 `js-array-buffer-inl.h:380` `JSTypedArray::TypeAndElementSizeFor`。
  X-macro `TYPED_ARRAYS` / `RAB_GSAB_TYPED_ARRAYS_WITH_TYPED_ARRAY_TYPE` 展开 `ElementsKind → (ExternalArrayType, sizeof(ctype))`。
  本 crate 尚未引入 `ElementsKind` 与 `ExternalArrayType` 判别式枚举 (骨架签名暂以裸 `u8` 占位); 填体需上层枚举就位, 归语义 Developer 轮。

(其余位标志/布局常量、`Age`/`GcState`/`accounting_state`、`TYPED_ARRAY_CONSTRUCTORS` 均为已就位声明, 无 todo。)

