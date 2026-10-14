#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/trace-event.h.js — v8/src/heap/cppgc-internal/trace-event.h → v8-heap-cppgc/src/trace_event.rs.skel
// 职责: cppgc 内嵌 trace 宏层 — CategoryGroupEnabledFlags 位标志 + AddTraceEvent 0/1/2 参重载 + SetTraceValue 三型分派
// 规则:
//   ① 位标志 2 个: kEnabledForRecording = 1<<0, kEnabledForEventCallback = 1<<2 (须与 chromium trace_log.h 同步)
//   ② AddTraceEvent 3 重载 (0/1/2 参数) 全部转发 AddTraceEventImpl → TracingController::AddTraceEvent; num_args<=2 DCHECK
//   ③ SetTraceValue: 整型/枚举基模板三选 (bool/signed/unsigned), X-macro 实例化 double 与 const char* (memcpy 存值)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/trace-event.h";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need("enum CategoryGroupEnabledFlags {", "位标志枚举");
need("kEnabledForRecording_CategoryGroupEnabledFlags = 1 << 0,", "recording 位");
need("kEnabledForEventCallback_CategoryGroupEnabledFlags = 1 << 2,", "event callback 位");
need("#define INTERNAL_TRACE_EVENT_CATEGORY_GROUP_ENABLED_FOR_RECORDING_MODE()", "录制判定宏");
need("class TraceEventHelper {", "helper 类");
need("static TracingController* GetTracingController();", "controller 取用");
need("static V8_INLINE uint64_t AddTraceEventImpl(", "实现函数");
need("TRACE_VALUE_TYPE_CONVERTABLE", "可转换类型判定");
need("DCHECK_LE(num_args, 2);", "参数上限断言");
need("controller->AddTraceEvent(phase, category_group_enabled, name, scope,", "controller 转发");
need("std::is_integral_v<T> || std::is_enum_v<T>", "基模板 SFINAE");
need("TRACE_VALUE_TYPE_BOOL", "bool 型选");
need("TRACE_VALUE_TYPE_INT", "int 型选");
need("TRACE_VALUE_TYPE_UINT", "uint 型选");
need("INTERNAL_DECLARE_SET_TRACE_VALUE(double, TRACE_VALUE_TYPE_DOUBLE)", "X-macro double");
need("INTERNAL_DECLARE_SET_TRACE_VALUE(const char*, TRACE_VALUE_TYPE_STRING)", "X-macro string");
need("0 /* num_args */", "0 参重载");

// AddTraceEvent 重载计数: Impl 1 + 0/1/2 参 3 = 4 个定义
const addsImpl = (src.match(/AddTraceEventImpl\(/g) || []).length;
if (addsImpl !== 1) throw new Error(`Impl 计数: got=${addsImpl} want=1`);
const addsOver = (src.match(/uint64_t AddTraceEvent\(/g) || []).length;
if (addsOver !== 3) throw new Error(`重载计数: got=${addsOver} want=3`);
n += 2;

const body = `${banner("src/heap/cppgc-internal/trace-event.h.js", REL, "cppgc trace 宏层 — 位标志判定 + AddTraceEvent 0/1/2 参重载 + SetTraceValue 型分派 (非 standalone 配置)")}
// ══ CategoryGroupEnabledFlags (原版 trace-event.h:29-34) ══
// 与 chromium trace_log.h 宏值同步 — 仅作 bool 用。

/// bitfield: recording = 1<<0; event-callback = 1<<2
pub const K_ENABLED_FOR_RECORDING_CATEGORY_GROUP_ENABLED_FLAGS: u8 = 1 << 0;
pub const K_ENABLED_FOR_EVENT_CALLBACK_CATEGORY_GROUP_ENABLED_FLAGS: u8 = 1 << 2;

/// 原版 :36-38 — 录制模式判定 = load & (recording | event_callback)
#[inline]
pub fn category_group_enabled_for_recording_mode(loaded: u8) -> bool {
    loaded & (K_ENABLED_FOR_RECORDING_CATEGORY_GROUP_ENABLED_FLAGS
        | K_ENABLED_FOR_EVENT_CALLBACK_CATEGORY_GROUP_ENABLED_FLAGS) != 0
}

// ══ TraceEventHelper (原版 trace-event.h:138-141) ══
pub struct TraceEventHelper;
impl TraceEventHelper {
    pub fn get_tracing_controller() -> *mut TracingController { unimplemented!() }
}

// ══ SetTraceValue 型分派 (原版 trace-event.h:167-187) ══

/// 基实现: 整型/枚举 → bool/signed/unsigned 三选 (原版 :167-175)
#[inline]
pub fn set_trace_value_integral<T: IntegralOrEnum>(arg: T, type_: &mut u8, value: &mut u64) {
    *type_ = if T::IS_BOOL { TRACE_VALUE_TYPE_BOOL } else if T::IS_SIGNED { TRACE_VALUE_TYPE_INT } else { TRACE_VALUE_TYPE_UINT };
    *value = arg.as_u64();
}

/// X-macro 实例 1: double — 原版 :180-186 (type 直落 + memcpy 存值)
#[inline]
pub fn set_trace_value_double(arg: f64, type_: &mut u8, value: &mut u64) {
    *type_ = TRACE_VALUE_TYPE_DOUBLE;
    *value = 0;
    debug_assert!(std::mem::size_of::<f64>() <= std::mem::size_of::<u64>());
    unsafe { std::ptr::copy_nonoverlapping(&arg as *const f64 as *const u8, value as *mut u64 as *mut u8, 8) };
}

/// X-macro 实例 2: const char* — 原版 :180-186 (指针位存值)
#[inline]
pub fn set_trace_value_string(arg: *const u8, type_: &mut u8, value: &mut u64) {
    *type_ = TRACE_VALUE_TYPE_STRING;
    *value = 0;
    debug_assert!(std::mem::size_of::<*const u8>() <= std::mem::size_of::<u64>());
    unsafe { std::ptr::copy_nonoverlapping(&arg as *const *const u8 as *const u8, value as *mut u64 as *mut u8, std::mem::size_of::<*const u8>()) };
}

// ══ AddTraceEvent 4 定义 (原版 trace-event.h:146-165, 194-243) ══

/// Impl — 原版 :146-165: 可转换参数 (前 2 个) 转 ConvertableToTraceFormat 归属权, DCHECK<=2, 转发 controller
pub fn add_trace_event_impl(
    phase: u8, category_group_enabled: *const u8, name: *const u8, scope: *const u8,
    id: u64, bind_id: u64, num_args: i32,
    arg_names: *const *const u8, arg_types: *const u8, arg_values: *const u64,
    flags: u32, platform: *mut Platform,
) -> u64 {
    let mut arg_convertables: [Option<Box<dyn ConvertableToTraceFormat>>; 2] = [None, None];
    // v8: 原版 num_args>0/1 且 arg_types CONVERTABLE 时, 从 arg_values 位型恢复指针归属权
    debug_assert!(num_args <= 2);
    let controller = unsafe { &*platform }.get_tracing_controller();
    controller.add_trace_event(phase, category_group_enabled, name, scope, id, bind_id, num_args, arg_names, arg_types, arg_values, arg_convertables, flags)
}

/// 0 参重载 — 原版 :194-203: num_args 固定 0, 其余空
pub fn add_trace_event_0(phase: u8, category_group_enabled: *const u8, name: *const u8, scope: *const u8, id: u64, bind_id: u64, flags: u32, platform: *mut Platform) -> u64 {
    add_trace_event_impl(phase, category_group_enabled, name, scope, id, bind_id, 0, std::ptr::null(), std::ptr::null(), std::ptr::null(), flags, platform)
}

/// 1 参重载 — 原版 :207-218: SetTraceValue 后转发
pub fn add_trace_event_1(phase: u8, category_group_enabled: *const u8, name: *const u8, scope: *const u8, id: u64, bind_id: u64, flags: u32, platform: *mut Platform, arg1_name: *const u8, arg1_type: u8, arg1_value: u64) -> u64 {
    add_trace_event_impl(phase, category_group_enabled, name, scope, id, bind_id, 1, &arg1_name, &arg1_type, &arg1_value, flags, platform)
}

/// 2 参重载 — 原版 :221-243: 双 SetTraceValue 后转发
pub fn add_trace_event_2(phase: u8, category_group_enabled: *const u8, name: *const u8, scope: *const u8, id: u64, bind_id: u64, flags: u32, platform: *mut Platform, arg1_name: *const u8, arg1_type: u8, arg1_value: u64, arg2_name: *const u8, arg2_type: u8, arg2_value: u64) -> u64 {
    let arg_names = [arg1_name, arg2_name];
    let arg_types = [arg1_type, arg2_type];
    let arg_values = [arg1_value, arg2_value];
    add_trace_event_impl(phase, category_group_enabled, name, scope, id, bind_id, 2, arg_names.as_ptr(), arg_types.as_ptr(), arg_values.as_ptr(), flags, platform)
}
`;
const out = writeSrc("v8-heap-cppgc", "trace_event.rs.skel", body);
console.log(`OK trace-event.h.js: 锚 ${n} 条 (位标志4+宏1+类2+Impl4+SFINAE4+X-macro2+重载计数1); 4 定义 3 重载; 产物 ${out}`);
