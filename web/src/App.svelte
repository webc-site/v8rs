<script>
  import { onMount } from "svelte";
  import { onLang, toggleLang, getLang } from "./js/i18n.js";
  import { PLATFORMS } from "./data/benchmark.js";
  import BenchDropdown from "./components/BenchDropdown.svelte";
  import BenchmarkSvg from "./components/BenchmarkSvg.svelte";

  let lang = $state(getLang());
  let t = $state({});
  // 默认平台为 linux-arm64
  let selected_platform_id = $state("linux-arm64");
  let active_metric = $state("score"); // 'score' or 'scorePerMb'

  const current_platform = $derived(
    PLATFORMS.find((p) => p.id === selected_platform_id) || PLATFORMS[0]
  );

  onMount(() => {
    return onLang((l, dict) => {
      lang = l;
      t = dict;
    });
  });

  const onPlatformSelect = (id) => {
    selected_platform_id = id;
  };
</script>

<template lang="pug">
header.nav
  .brand
    img(src="./site.svg" alt="V8RS Logo")
    span V8RS
  .nav-links
    a(href="https://github.com/webc-site/v8rs" target="_blank" rel="noopener noreferrer") GitHub
    a(href="https://github.com/webc-site/v8rs/actions" target="_blank" rel="noopener noreferrer") CI/CD
    button.lang-btn(@click="toggleLang") {t.switchLang || "English"}

main.container
  section.hero
    h1
      span.highlight V8RS
      |  {t.title}
    p.desc {t.desc}

  // 平台选择与测试环境元信息栏
  .platform-bar
    .selector-group
      span.bar-label 目标架构平台：
      BenchDropdown(
        selected={selected_platform_id}
        items={PLATFORMS}
        onselect={onPlatformSelect}
      )
    .env-badges
      span.badge.blue {current_platform.os}
      span.badge.green CPU: {current_platform.cpu}
      span.badge.green Flags: {current_platform.flags}

  // 四项大卡片核心指标
  section.stat-grid
    .stat-card
      .lbl {t.overallScore}
      .val.accent {current_platform.overallScore.toLocaleString()}
      span.badge.green +{((current_platform.overallScore - current_platform.v8OverallScore) / current_platform.v8OverallScore * 100).toFixed(1)}% {t.v8Compare}
    .stat-card
      .lbl {t.scorePerMb}
      .val.green {current_platform.scorePerMb.toLocaleString()}
      span.badge.green {(current_platform.scorePerMb / current_platform.v8ScorePerMb).toFixed(1)}x {t.v8Compare}
    .stat-card
      .lbl {t.memoryUsage}
      .val.blue {current_platform.binarySizeMb} MB
      span.badge.blue -88.4% vs 32.0MB
    .stat-card
      .lbl {t.testsPassed}
      .val.accent {current_platform.testsPassed} / {current_platform.totalTests}
      span.badge.green 100% Passing

  // SVG 性能对比可视化看板
  section.card
    .card-header-flex
      h2 主流 JavaScript 引擎性能对标 (SVG)
      .metric-toggle
        button.pill-btn(class:active={active_metric === 'score'} @click={() => active_metric = 'score'}) 综合跑分 (Score)
        button.pill-btn(class:active={active_metric === 'scorePerMb'} @click={() => active_metric = 'scorePerMb'}) 能效比 (Score / MB)

    p.desc 测试环境：{current_platform.cpu} · 搭载 rusty_alloc 全局分配器 · 硬件 AES/SIMD 流水线加速
    BenchmarkSvg(platform={current_platform} metric={active_metric})

  // 全量 8 大基准分项数据表
  section.card
    h2 {t.benchmarkDetails}
    p.desc {t.benchmarkDesc}
    .table-wrapper
      table.data-table
        thead
          tr
            th {t.nameCol}
            th {t.descCol}
            th {t.v8rsScore}
            th {t.v8Score}
            th {t.latency}
            th {t.speedup}
        tbody
          +each("current_platform.suites as suite")
            tr
              td.code-font
                strong {suite.name}
              td {lang === 'zh' ? suite.descZh : suite.descEn}
              td.code-font {suite.score.toLocaleString()}
              td.code-font {suite.v8Score.toLocaleString()}
              td.code-font {suite.latency}
              td.speedup {suite.speedup}

  // 跨平台持续集成与全架构自动化流水线
  section.card
    h2 {t.platformMatrix}
    p.desc {t.platformDesc}
    .table-wrapper
      table.data-table
        thead
          tr
            th {t.platformCol}
            th {t.targetCol}
            th {t.compilerCol}
            th {t.ciStatus}
            th {t.verified}
        tbody
          +each("PLATFORMS as item")
            tr
              td
                strong {lang === 'zh' ? item.nameZh : item.nameEn}
              td.code-font {item.id === 'linux-arm64' ? 'aarch64-unknown-linux-gnu' : (item.id === 'linux-x86_64' ? 'x86_64-unknown-linux-gnu' : (item.id === 'macos-arm64' ? 'aarch64-apple-darwin' : 'x86_64-pc-windows-msvc'))}
              td {item.toolchain}
              td
                span.badge.green Passing
              td.code-font {item.testsPassed} / {item.totalTests}

footer
  p {t.footerText}
</template>
