<script>
  let {
    platform = null,
    metric = "score", // "score" or "scorePerMb"
  } = $props();

  const SVG_WIDTH = 840;
  const BAR_START_X = 220;
  const MAX_BAR_WIDTH = 440;
  const ROW_HEIGHT = 44;
  const BAR_HEIGHT = 20;

  const engines = $derived(platform ? platform.engines : []);
  const max_val = $derived(
    Math.max(
      ...engines.map((e) => (metric === "score" ? e.score : e.scorePerMb)),
      1
    )
  );

  const sorted_engines = $derived(
    [...engines].sort((a, b) => {
      const valA = metric === "score" ? a.score : a.scorePerMb;
      const valB = metric === "score" ? b.score : b.scorePerMb;
      return valB - valA;
    })
  );

  const chart_height = $derived(sorted_engines.length * ROW_HEIGHT + 30);
</script>

<template lang="pug">
.svg-chart-wrapper
  svg.chart-svg(viewBox={"0 0 " + SVG_WIDTH + " " + chart_height} width="100%" height={chart_height})
    defs
      linearGradient#grad-v8rs(x1="0%" y1="0%" x2="100%" y2="0%")
        stop(offset="0%" stop-color="#0969da")
        stop(offset="100%" stop-color="#218bff")
      linearGradient#grad-v8(x1="0%" y1="0%" x2="100%" y2="0%")
        stop(offset="0%" stop-color="#8c959f")
        stop(offset="100%" stop-color="#afb8c1")
      linearGradient#grad-jsc(x1="0%" y1="0%" x2="100%" y2="0%")
        stop(offset="0%" stop-color="#0550ae")
        stop(offset="100%" stop-color="#0969da")
      linearGradient#grad-other(x1="0%" y1="0%" x2="100%" y2="0%")
        stop(offset="0%" stop-color="#afb8c1")
        stop(offset="100%" stop-color="#d0d7de")

    +each('sorted_engines as eng, idx')
      g.chart-row(class:is-current={eng.is_current})
        // 引擎标签
        text.label-engine(x="16" y={idx * ROW_HEIGHT + 24})
          | {eng.name}
          +if('eng.is_current')
            tspan.badge-star(dx="6") [Current]

        // 槽底
        rect.bar-track(
          x={BAR_START_X}
          y={idx * ROW_HEIGHT + 14}
          width={MAX_BAR_WIDTH}
          height={BAR_HEIGHT}
          rx="6"
        )

        // 数值条
        rect.bar-rect(
          x={BAR_START_X}
          y={idx * ROW_HEIGHT + 14}
          width={Math.max(12, ((metric === 'score' ? eng.score : eng.scorePerMb) / max_val) * MAX_BAR_WIDTH)}
          height={BAR_HEIGHT}
          rx="6"
          fill={eng.is_current ? 'url(#grad-v8rs)' : (eng.key === 'v8' ? 'url(#grad-v8)' : (eng.key === 'jsc' ? 'url(#grad-jsc)' : 'url(#grad-other)'))}
        )

        // 数值文字
        text.val-text(x={BAR_START_X + Math.max(12, ((metric === 'score' ? eng.score : eng.scorePerMb) / max_val) * MAX_BAR_WIDTH) + 12} y={idx * ROW_HEIGHT + 24})
          tspan.num-val {(metric === 'score' ? eng.score : eng.scorePerMb).toLocaleString()}
          tspan.unit-val(dx="4") {metric === 'score' ? 'pts' : 'pts/MB'}
</template>
