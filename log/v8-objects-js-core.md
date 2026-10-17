# v8-objects-js-core 开发日志

> 2026-10-12 JS 对象族核心: field_index / js_arguments / js_array / js_object / js_receiver + JSArrayBuffer 族差集骨架。

## 轮7 W7LIT (零头席)

- [轮7 W7LIT] grep `todo!` 计数开工 1 → 收笼 1 (保留): `src/js_array_buffer.rs.skel::type_and_element_size_for`。
  - `.skel` 未入编译图, 深水面: 对位 `js-array-buffer-inl.h:380` X-macro `ElementsKind → (ExternalArrayType, sizeof(ctype))`; 本 crate 尚无 `ElementsKind`/`ExternalArrayType` 判别枚举 (骨架签名裸 `u8` 占位), 上层未就位, 禁伪装填体。
  - 已登记 `readme/zh.md` 推迟台账。门禁实测 211/211 全绿 (含本 crate)。
