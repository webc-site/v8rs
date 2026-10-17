/**
 * rs8-capi.h
 *
 * C ABI declarations for the v8rs JavaScript Engine (rs8-capi).
 * Compatible with C99, C11, C++11 and higher.
 */

#ifndef RS8_CAPI_H
#define RS8_CAPI_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ========================================================================= */
/* 类型与句柄定义                                                            */
/* ========================================================================= */

/** 不透明隔离区句柄 (Isolate) */
typedef struct V8Isolate V8Isolate;

/** 不透明上下文句柄 (Context) */
typedef struct V8Context V8Context;

/** 不透明脚本编译实例 (Script) */
typedef struct V8Script V8Script;

/** 不透明平台实例 (Platform) */
typedef struct V8Platform V8Platform;

/** 内部 Local 句柄包装 (对齐 C++ v8::Local<T>) */
typedef struct {
    void* ptr;
} V8Local;

/** 内部 Global 持久句柄包装 (对齐 C++ v8::Global<T>) */
typedef struct {
    void* ptr;
} V8Global;

/** 启动快照数据描述体 */
typedef struct {
    const char* data;
    int raw_size;
} V8StartupData;

/* ========================================================================= */
/* 全局生命周期与配置                                                        */
/* ========================================================================= */

/** 初始化 V8 运行时平台与基础设施 */
bool v8_initialize(void);

/** 销毁 V8 全局状态与资源 */
bool v8_dispose(void);

/** 查询 V8 是否已初始化 */
bool v8_is_initialized(void);

/** 获取引擎版本号字符串 (静态字符串，无需释放) */
const char* v8_get_version(void);

/** 通过字符串设置命令行标志参数 (例如 "--harmony --max-opt=2") */
bool v8_set_flags_from_string(const char* flags);

/* ========================================================================= */
/* 平台 (Platform) 接口                                                      */
/* ========================================================================= */

/** 创建默认并发线程平台实例 */
V8Platform* v8_platform_new_default(int thread_pool_size, bool idle_tasks);

/** 推进消息队列事件泵 */
bool v8_platform_pump_message_loop(V8Platform* platform, V8Isolate* isolate);

/** 销毁平台实例 */
void v8_platform_dispose(V8Platform* platform);

/* ========================================================================= */
/* 隔离区 (Isolate) 接口                                                     */
/* ========================================================================= */

/** 创建新的隔离区实例 */
V8Isolate* v8_isolate_new(const void* create_params);

/** 进入隔离区 (设为当前线程活跃 Isolate) */
void v8_isolate_enter(V8Isolate* isolate);

/** 退出隔离区 */
void v8_isolate_exit(V8Isolate* isolate);

/** 获取当前线程活跃的 Isolate 实例 */
V8Isolate* v8_isolate_get_current(void);

/** 销毁隔离区实例并回收相关堆内存 */
void v8_isolate_dispose(V8Isolate* isolate);

/* ========================================================================= */
/* 上下文 (Context) 接口                                                     */
/* ========================================================================= */

/** 在隔离区中创建全局执行上下文 */
V8Local v8_context_new(V8Isolate* isolate);

/** 进入上下文环境 */
void v8_context_enter(V8Local context);

/** 退出上下文环境 */
void v8_context_exit(V8Local context);

/** 销毁上下文实例并回收环境作用域 */
void v8_context_dispose(V8Local context);

/* ========================================================================= */
/* 句柄工具函数                                                              */
/* ========================================================================= */

/** 检查 Local 句柄是否为空 */
bool v8_local_is_empty(V8Local local);

/** 检查 Global 句柄是否为空 */
bool v8_global_is_empty(const V8Global* global);

/** 重置 Global 句柄为 null */
void v8_global_reset(V8Global* global);

/* ========================================================================= */
/* 基础字面量与数值对象                                                      */
/* ========================================================================= */

/** 获取 undefined 静态单例 */
V8Local v8_undefined(V8Isolate* isolate);

/** 获取 null 静态单例 */
V8Local v8_null(V8Isolate* isolate);

/** 获取 true 布尔单例 */
V8Local v8_true(V8Isolate* isolate);

/** 获取 false 布尔单例 */
V8Local v8_false(V8Isolate* isolate);

/** 从 C 布尔值创建 V8 布尔句柄 */
V8Local v8_boolean_new(V8Isolate* isolate, bool value);

/** 检查值是否为 undefined */
bool v8_value_is_undefined(V8Local val);

/** 检查值是否为 null */
bool v8_value_is_null(V8Local val);

/** 检查值是否为布尔类型 */
bool v8_value_is_boolean(V8Local val);

/** 检查值是否为 true */
bool v8_value_is_true(V8Local val);

/** 检查值是否为 false */
bool v8_value_is_false(V8Local val);

/* ========================================================================= */
/* 字符串、数值与脚本编译执行                                                */
/* ========================================================================= */

/**
 * 从 UTF-8 字节串创建 V8 字符串句柄
 * @param length 若为负数，则自动计算 strlen(data)
 */
V8Local v8_string_new_from_utf8(V8Isolate* isolate, const char* data, int length);

/** 创建 64 位双精度浮点数值句柄 */
V8Local v8_number_new(V8Isolate* isolate, double value);

/**
 * 编译 JavaScript 源代码生成 AST 执行单元
 * @return 成功返回 V8Script* 指针；语法错误时返回 NULL
 */
V8Script* v8_script_compile(V8Local context, V8Local source);

/**
 * 在指定上下文环境中执行已编译的脚本
 * @return 成功返回最终求值句柄；执行抛错时返回 empty 句柄
 */
V8Local v8_script_run(V8Script* script, V8Local context);

/** 释放已编译的脚本实例 */
void v8_script_dispose(V8Script* script);

/** 从 V8Local 句柄中解构 f64 数值 */
double v8_value_number_value(V8Local val, V8Local context);

/** 释放由 v8_string_new / v8_number_new / v8_script_run 分配的堆值句柄 */
void v8_value_dispose(V8Local val);

/* ========================================================================= */
/* 快照辅助函数                                                              */
/* ========================================================================= */

/** 检查启动快照数据是否为空 */
bool v8_startup_data_is_empty(const V8StartupData* data);

#ifdef __cplusplus
}
#endif

#endif /* RS8_CAPI_H */
