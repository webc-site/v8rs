#!/usr/bin/env bun
// tool/src/inspector/runtime.js — Runtime 域 + 注入脚本组
// 覆盖: src/inspector/v8-runtime-agent-impl.h/.cc + injected-script.h/.cc +
//       inspected-context.h/.cc + remote-object-id.h/.cc + value-mirror.h/.cc (5 对, 10 文件)
// 产出: v8-inspector/src/runtime.rs
// 复跑幂等: 纯文本扫描 + 确定性输出, 无时间戳。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/inspector/runtime.js";
const FILES = [
  "src/inspector/v8-runtime-agent-impl.h",
  "src/inspector/v8-runtime-agent-impl.cc",
  "src/inspector/injected-script.h",
  "src/inspector/injected-script.cc",
  "src/inspector/inspected-context.h",
  "src/inspector/inspected-context.cc",
  "src/inspector/remote-object-id.h",
  "src/inspector/remote-object-id.cc",
  "src/inspector/value-mirror.h",
  "src/inspector/value-mirror.cc",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[runtime.js] ${m}`); };
const need = (f, s) => { if (!txt[f].includes(s)) die(`${f} 缺针 ${s}`); };

// ① Runtime Agent 类面针
need("src/inspector/v8-runtime-agent-impl.h", "class V8RuntimeAgentImpl : public protocol::Runtime::Backend {");
// ② InjectedScript 类面 + DiscardReason 枚举针
need("src/inspector/injected-script.h", "class InjectedScript final {");
need("src/inspector/injected-script.h", "enum class DiscardReason {");
for (const v of ["kFulfilled,", "kPromiseCollected,", "kTearDown,"]) need("src/inspector/injected-script.h", v);
need("src/inspector/injected-script.h", "class EvaluateCallback {");
// ③ InspectedContext + V8InternalValueType 枚举针
need("src/inspector/inspected-context.h", "enum class V8InternalValueType {");
for (const v of ["kNone,", "kEntry,", "kScope,", "kScopeList,", "kPrivateMethodList,", "kPrivateMethod"]) {
  need("src/inspector/inspected-context.h", v);
}
need("src/inspector/inspected-context.h", "class InspectedContext {");
need("src/inspector/inspected-context.h", "static int contextId(v8::Local<v8::Context>);");
// ④ RemoteObjectId 三类针
need("src/inspector/remote-object-id.h", "class RemoteObjectIdBase {");
need("src/inspector/remote-object-id.h", "class RemoteObjectId final : public RemoteObjectIdBase {");
need("src/inspector/remote-object-id.h", "class RemoteCallFrameId final : public RemoteObjectIdBase {");
// ⑤ ValueMirror 针
need("src/inspector/value-mirror.h", "class ValueMirror {");

// ⑥ 机械提取: V8InternalValueType 变体
const vitBlock = txt["src/inspector/inspected-context.h"].match(/enum class V8InternalValueType \{([^}]*)\}/)[1];
const vitVars = [...vitBlock.matchAll(/k(\w+)/g)].map((m) => m[1]);
// ⑦ 机械提取: DiscardReason 变体
const drBlock = txt["src/inspector/injected-script.h"].match(/enum class DiscardReason \{([^}]*)\}/)[1];
const drVars = [...drBlock.matchAll(/k(\w+)/g)].map((m) => m[1]);

if (vitVars.length !== 6) die(`V8InternalValueType 变体数=${vitVars.length}, 期望 6`);
if (drVars.length !== 3) die(`DiscardReason 变体数=${drVars.length}, 期望 3`);

const out = [];
out.push(banner(GEN, FILES.join(", "), "Runtime 域: V8InternalValueType/DiscardReason 枚举与 InspectedContext/RemoteObject"));
out.push(`use serde::{Deserialize, Serialize};`);
out.push(`use strum::{Display, EnumString};`);
out.push(``);
out.push(`/// V8 内部值类型 (v8/src/inspector/inspected-context.h:31)`);
out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]`);
out.push(`pub enum V8InternalValueType {`);
for (const v of vitVars) {
  out.push(`    /// ${v}`);
  out.push(`    ${v},`);
}
out.push(`}`);
out.push(``);
out.push(`impl V8InternalValueType {`);
out.push(`    /// 是否为无内部值`);
out.push(`    #[inline]`);
out.push(`    pub const fn is_none(&self) -> bool {`);
out.push(`        matches!(self, Self::None)`);
out.push(`    }`);
out.push(``);
out.push(`    /// 是否为作用域类型`);
out.push(`    #[inline]`);
out.push(`    pub const fn is_scope(&self) -> bool {`);
out.push(`        matches!(self, Self::Scope | Self::ScopeList)`);
out.push(`    }`);
out.push(``);
out.push(`    /// 是否为私有方法`);
out.push(`    #[inline]`);
out.push(`    pub const fn is_private_method(&self) -> bool {`);
out.push(`        matches!(self, Self::PrivateMethod | Self::PrivateMethodList)`);
out.push(`    }`);
out.push(`}`);
out.push(``);
out.push(`/// 注入脚本丢弃原因 (v8/src/inspector/injected-script.h:299)`);
out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]`);
out.push(`pub enum DiscardReason {`);
for (const v of drVars) {
  out.push(`    /// ${v}`);
  out.push(`    ${v},`);
}
out.push(`}`);
out.push(``);
out.push(`impl DiscardReason {`);
out.push(`    /// 是否因已敲定而丢弃`);
out.push(`    #[inline]`);
out.push(`    pub const fn is_fulfilled(&self) -> bool {`);
out.push(`        matches!(self, Self::Fulfilled)`);
out.push(`    }`);
out.push(``);
out.push(`    /// 是否因销毁而丢弃`);
out.push(`    #[inline]`);
out.push(`    pub const fn is_tear_down(&self) -> bool {`);
out.push(`        matches!(self, Self::TearDown)`);
out.push(`    }`);
out.push(`}`);
out.push(``);
out.push(`/// 受检上下文元数据 (v8/src/inspector/inspected-context.h:38)`);
out.push(`#[derive(Debug, Clone, PartialEq, Eq)]`);
out.push(`pub struct InspectedContext {`);
out.push(`    context_id: i32,`);
out.push(`    context_group_id: i32,`);
out.push(`    origin: String,`);
out.push(`    human_readable_name: String,`);
out.push(`}`);
out.push(``);
out.push(`impl InspectedContext {`);
out.push(`    /// 构造受检上下文`);
out.push(`    pub fn new(`);
out.push(`        context_id: i32,`);
out.push(`        context_group_id: i32,`);
out.push(`        origin: impl Into<String>,`);
out.push(`        human_readable_name: impl Into<String>,`);
out.push(`    ) -> Self {`);
out.push(`        Self {`);
out.push(`            context_id,`);
out.push(`            context_group_id,`);
out.push(`            origin: origin.into(),`);
out.push(`            human_readable_name: human_readable_name.into(),`);
out.push(`        }`);
out.push(`    }`);
out.push(``);
out.push(`    /// 上下文 ID`);
out.push(`    #[inline]`);
out.push(`    pub const fn context_id(&self) -> i32 {`);
out.push(`        self.context_id`);
out.push(`    }`);
out.push(``);
out.push(`    /// 上下文组 ID`);
out.push(`    #[inline]`);
out.push(`    pub const fn context_group_id(&self) -> i32 {`);
out.push(`        self.context_group_id`);
out.push(`    }`);
out.push(``);
out.push(`    /// 来源源名`);
out.push(`    #[inline]`);
out.push(`    pub fn origin(&self) -> &str {`);
out.push(`        &self.origin`);
out.push(`    }`);
out.push(``);
out.push(`    /// 可读名称`);
out.push(`    #[inline]`);
out.push(`    pub fn human_readable_name(&self) -> &str {`);
out.push(`        &self.human_readable_name`);
out.push(`    }`);
out.push(`}`);
out.push(``);
out.push(`/// 注入脚本模型 (v8/src/inspector/injected-script.h:78)`);
out.push(`#[derive(Debug, Clone, PartialEq, Eq)]`);
out.push(`pub struct InjectedScript {`);
out.push(`    injected_script_id: i32,`);
out.push(`    context_id: i32,`);
out.push(`}`);
out.push(``);
out.push(`impl InjectedScript {`);
out.push(`    /// 构造注入脚本实例`);
out.push(`    pub const fn new(injected_script_id: i32, context_id: i32) -> Self {`);
out.push(`        Self { injected_script_id, context_id }`);
out.push(`    }`);
out.push(``);
out.push(`    /// 注入脚本标识`);
out.push(`    #[inline]`);
out.push(`    pub const fn injected_script_id(&self) -> i32 {`);
out.push(`        self.injected_script_id`);
out.push(`    }`);
out.push(``);
out.push(`    /// 上下文标识`);
out.push(`    #[inline]`);
out.push(`    pub const fn context_id(&self) -> i32 {`);
out.push(`        self.context_id`);
out.push(`    }`);
out.push(`}`);
out.push(``);
out.push(`/// CDP 远程对象表达 (Chrome DevTools Protocol RemoteObject)`);
out.push(`#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]`);
out.push(`pub struct RemoteObject {`);
out.push(`    #[serde(rename = "type")]`);
out.push(`    pub type_: String,`);
out.push(`    #[serde(skip_serializing_if = "Option::is_none")]`);
out.push(`    pub subtype: Option<String>,`);
out.push(`    #[serde(skip_serializing_if = "Option::is_none")]`);
out.push(`    pub value: Option<serde_json::Value>,`);
out.push(`    #[serde(skip_serializing_if = "Option::is_none")]`);
out.push(`    pub description: Option<String>,`);
out.push(`    #[serde(skip_serializing_if = "Option::is_none")]`);
out.push(`    pub object_id: Option<String>,`);
out.push(`}`);
out.push(``);
out.push(`impl RemoteObject {`);
out.push(`    /// 构造基础字符串类型远程对象`);
out.push(`    pub fn string(val: impl Into<String>) -> Self {`);
out.push(`        let s = val.into();`);
out.push(`        Self {`);
out.push(`            type_: "string".to_string(),`);
out.push(`            subtype: None,`);
out.push(`            description: Some(s.clone()),`);
out.push(`            value: Some(serde_json::Value::String(s)),`);
out.push(`            object_id: None,`);
out.push(`        }`);
out.push(`    }`);
out.push(``);
out.push(`    /// 构造基础数字类型远程对象`);
out.push(`    pub fn number(val: f64) -> Self {`);
out.push(`        Self {`);
out.push(`            type_: "number".to_string(),`);
out.push(`            subtype: None,`);
out.push(`            description: Some(val.to_string()),`);
out.push(`            value: serde_json::Number::from_f64(val).map(serde_json::Value::Number),`);
out.push(`            object_id: None,`);
out.push(`        }`);
out.push(`    }`);
out.push(``);
out.push(`    /// 构造布尔类型远程对象`);
out.push(`    pub fn boolean(val: bool) -> Self {`);
out.push(`        Self {`);
out.push(`            type_: "boolean".to_string(),`);
out.push(`            subtype: None,`);
out.push(`            description: Some(val.to_string()),`);
out.push(`            value: Some(serde_json::Value::Bool(val)),`);
out.push(`            object_id: None,`);
out.push(`        }`);
out.push(`    }`);
out.push(``);
out.push(`    /// 构造 undefined`);
out.push(`    pub fn undefined() -> Self {`);
out.push(`        Self {`);
out.push(`            type_: "undefined".to_string(),`);
out.push(`            subtype: None,`);
out.push(`            description: Some("undefined".to_string()),`);
out.push(`            value: None,`);
out.push(`            object_id: None,`);
out.push(`        }`);
out.push(`    }`);
out.push(``);
out.push(`    /// 构造 null`);
out.push(`    pub fn null() -> Self {`);
out.push(`        Self {`);
out.push(`            type_: "object".to_string(),`);
out.push(`            subtype: Some("null".to_string()),`);
out.push(`            description: Some("null".to_string()),`);
out.push(`            value: Some(serde_json::Value::Null),`);
out.push(`            object_id: None,`);
out.push(`        }`);
out.push(`    }`);
out.push(`}`);
out.push(``);
out.push(`/// 值镜像结构 (v8/src/inspector/value-mirror.h:50)`);
out.push(`#[derive(Debug, Clone, PartialEq, Eq)]`);
out.push(`pub struct ValueMirror {`);
out.push(`    type_: String,`);
out.push(`    subtype: Option<String>,`);
out.push(`    description: String,`);
out.push(`}`);
out.push(``);
out.push(`impl ValueMirror {`);
out.push(`    /// 构造镜像`);
out.push(`    pub fn new(type_: impl Into<String>, subtype: Option<String>, description: impl Into<String>) -> Self {`);
out.push(`        Self {`);
out.push(`            type_: type_.into(),`);
out.push(`            subtype,`);
out.push(`            description: description.into(),`);
out.push(`        }`);
out.push(`    }`);
out.push(``);
out.push(`    /// 主类型`);
out.push(`    #[inline]`);
out.push(`    pub fn type_name(&self) -> &str {`);
out.push(`        &self.type_`);
out.push(`    }`);
out.push(``);
out.push(`    /// 描述信息`);
out.push(`    #[inline]`);
out.push(`    pub fn description(&self) -> &str {`);
out.push(`        &self.description`);
out.push(`    }`);
out.push(``);
out.push(`    /// 子类型`);
out.push(`    #[inline]`);
out.push(`    pub fn subtype(&self) -> Option<&str> {`);
out.push(`        self.subtype.as_deref()`);
out.push(`    }`);
out.push(`}`);
out.push(``);

writeSrc("v8-inspector", "runtime.rs", out.join("\n") + "\n");
console.log(`OK runtime.js: V8InternalValueType/DiscardReason/InspectedContext/RemoteObject → v8-inspector/src/runtime.rs`);
