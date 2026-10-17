#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/task-handle.h.js — v8/src/heap/cppgc-internal/task-handle.h → v8-heap-cppgc/src/task_handle.rs.skel
// 职责: SingleThreadedHandle — shared_ptr<bool> 单线程任务取消句柄, 空/非空/取消三态
// 规则:
//   ① 空 = 默认构造 (null shared_ptr); 非空 = NonEmptyTag 构造 (make_shared<bool>(false))
//   ② Cancel/CancelIfNonEmpty/IsCanceled/operator bool 四方法语义差异: Cancel 与 IsCanceled DCHECK 非空
//   ③ shared_ptr<bool> → Arc<AtomicBool> 对应 (单线程语义面留锚)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/task-handle.h";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
 };
const count = (re) => (src.match(re) || []).length;

need("struct SingleThreadedHandle {", "结构声明");
need("struct NonEmptyTag {};", "非空标签类型");
need("SingleThreadedHandle() = default;", "默认空构造");
need("explicit SingleThreadedHandle(NonEmptyTag)", "非空构造");
need(": is_cancelled_(std::make_shared<bool>(false)) {}", "初始 false");
need("void Cancel() {", "Cancel 声明");
need("void CancelIfNonEmpty() {", "CancelIfNonEmpty 声明");
need("bool IsCanceled() const {", "IsCanceled 声明");
need("explicit operator bool() const {", "bool 转换声明");
need("return is_cancelled_.get() && !*is_cancelled_.get();", "活跃判定式");
if (count(/DCHECK\(is_cancelled_\);/g) !== 2)
  throw new Error(`非空 DCHECK 计数: got=${count(/DCHECK\(is_cancelled_\);/g)} want=2`);
n++;

const body = `${banner("src/heap/cppgc-internal/task-handle.h.js", REL, "SingleThreadedHandle 单线程任务取消句柄")}
// ══ SingleThreadedHandle (原版 task-handle.h:16-48) ══
// 空/非空 + 取消位; 原版 shared_ptr<bool> → Rust Arc<bool>。
// v8: 原版注释 "A handle is active if it is non-empty and not cancelled"

/// 原版 task-handle.h:17 NonEmptyTag — 类型级非空证明
pub struct NonEmptyTag;

#[derive(Default)]
pub struct SingleThreadedHandle {
    /// Option None = 空句柄; Some = 共享取消位
    is_cancelled: Option<std::sync::Arc<bool>>,
}

impl SingleThreadedHandle {
    /// 原版 task-handle.h:22-23 — NonEmptyTag 构造, 取消位 false
    pub fn new_non_empty(_: NonEmptyTag) -> Self {
        Self { is_cancelled: Some(std::sync::Arc::new(false)) }
    }

    /// 原版 task-handle.h:25-28 — Cancel: DCHECK 非空后置位
    /// v8: 原版 DCHECK(is_cancelled_) — 空句柄 Cancel 是契约违规
    pub fn cancel(&mut self) {
        let cell = self.is_cancelled.as_mut().expect("Cancel 非空");
        *cell = true;
    }

    /// 原版 task-handle.h:30-34 — CancelIfNonEmpty: 空则无操作
    pub fn cancel_if_non_empty(&mut self) {
        if let Some(cell) = self.is_cancelled.as_mut() {
            *cell = true;
        }
    }

    /// 原版 task-handle.h:36-39 — IsCanceled: DCHECK 非空后读
    pub fn is_canceled(&self) -> bool {
        *self.is_cancelled.as_ref().expect("IsCanceled 非空")
    }

    /// 原版 task-handle.h:42-44 — 活跃 = 非空 && 未取消
    pub fn is_active(&self) -> bool {
        self.is_cancelled.as_ref().map_or(false, |c| !**c)
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "task_handle.rs.skel", body);
console.log(`OK task-handle.h.js: 锚 ${n} 条 (结构1+标签1+构造2+方法4+活跃式1+DCHECK 计数1); 产物 ${out}`);
