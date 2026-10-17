#include <stdio.h>
#include <assert.h>
#include <math.h>
#include "../../include/rs8-capi.h"

int main() {
    printf("[C-API-TEST] Initializing rs8 platform...\n");
    assert(v8_initialize());
    assert(v8_is_initialized());

    const char* version = v8_get_version();
    printf("[C-API-TEST] Version: %s\n", version);
    assert(version != NULL);

    V8Isolate* isolate = v8_isolate_new(NULL);
    assert(isolate != NULL);

    v8_isolate_enter(isolate);

    V8Local context = v8_context_new(isolate);
    assert(!v8_local_is_empty(context));
    v8_context_enter(context);

    // 1. 运行第一个脚本：定义全局变量与函数
    const char* script1_src = 
        "var factor = 3;\n"
        "function compute(x, y) {\n"
        "    return (x * y + 10) * factor;\n"
        "}\n";
    V8Local s1 = v8_string_new_from_utf8(isolate, script1_src, -1);
    assert(!v8_local_is_empty(s1));

    V8Script* script1 = v8_script_compile(context, s1);
    assert(script1 != NULL);
    V8Local r1 = v8_script_run(script1, context);
    (void)r1;

    // 2. 运行第二个脚本：调用之前上下文中定义的函数
    // compute(4, 5) = (4 * 5 + 10) * 3 = 30 * 3 = 90.0
    const char* script2_src = "compute(4, 5);";
    V8Local s2 = v8_string_new_from_utf8(isolate, script2_src, -1);
    assert(!v8_local_is_empty(s2));

    V8Script* script2 = v8_script_compile(context, s2);
    assert(script2 != NULL);

    V8Local result = v8_script_run(script2, context);
    assert(!v8_local_is_empty(result));

    double final_num = v8_value_number_value(result, context);
    printf("[C-API-TEST] Result of compute(4, 5) = %f\n", final_num);
    assert(fabs(final_num - 90.0) < 1e-6);

    // 3. 释放资源
    v8_value_dispose(result);
    v8_value_dispose(s2);
    v8_script_dispose(script2);
    v8_value_dispose(s1);
    v8_script_dispose(script1);

    v8_context_exit(context);
    v8_context_dispose(context);

    v8_isolate_exit(isolate);
    v8_isolate_dispose(isolate);

    assert(v8_dispose());
    printf("[C-API-TEST] ALL ASSERTIONS PASSED SUCCESSFULLY!\n");
    return 0;
}
