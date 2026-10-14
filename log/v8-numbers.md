# v8-numbers 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 遗留 (阻塞项, 需上游接力)

- ~~v8-base dtoa 转写缺陷 (cached_powers.rs ceil 顺序)~~ 已修复:
  `GetCachedPowerForBinaryExponent` 的 k 计算归位 (ceil 作用于乘积),
  SHORTEST/PRECISION 通道与 oracle toPrecision/toExponential 差分
  已全量恢复 (10 万条 ×3 通道 + 1.6 万条, 0 差异)。
- ~~to_string.rs radix 入口 CHECK 降级~~ round2 已修复:
  `double_to_radix_string_view` 的 radix 域检查由 debug_assert 复位为
  assert (原版 conversions.cc:1444 CHECK, 非法 radix 会致 CHARS 越界/死循环)。
- ~~parse_int.rs detect_radix 丢 allow_trailing_junk 门控~~ round2 已修复:
  补回 conversions.cc:659 的 `leading_zero && allow_trailing_junk && !is_digit`
  条件 (前导零尾垃圾判 Zero 受其门控); parseInt 路径恒传 true 行为零变化,
  结构对齐 StringToIntHelper 基类, 防 v8-bigint 子类接入时漂移。
  round2 另以 node (V8 引擎) 909 条真值差分 (toString(radix) 684 +
  parseInt 矩阵 225) 全 0 差异佐证。
- Math.pow 非特判路径: legacy::pow 委托平台 libm (v8-base 推迟项),
  与 JSC 有 ≤8 ulp 差 (ECMAScript implementation-approximated 允许域)。
- JSC 差分局限: parseInt 大数 (JSC double 累乘近似)、toString(radix≠10)
  (implementation-defined) 不可作位级 oracle — 前者以 1 ulp 容差 +
  构造向量, 后者以 mjsunit V8 向量护航。

(en.md「Resolved upstream blocker」节与本节 dtoa 修复条目同源, 无独有信息, 未搬英文原文。)
