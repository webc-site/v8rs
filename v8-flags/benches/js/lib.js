#!/usr/bin/env bun
// benches/js/lib.js — json 解析/转换公共件: criterion 输出目录采集、汇总结构、格式化
// 引用: 路径常量取用 ./conf.js, 本文件只放纯数据函数

import { readdir, readFile, writeFile } from "node:fs/promises";

export const readJson = async path => JSON.parse(await readFile(path, "utf8"));

export const writeText = async (path, text) => void await writeFile(path, text, "utf8");

const listNames = async dir =>
  (await readdir(dir, { withFileTypes: true }))
    .filter(entry => entry.isDirectory())
    .map(entry => entry.name)
    .sort();

// 遍历 criterion/<group>/<impl>/new/{estimates,benchmark}.json 汇总全部测量
// (criterion 0.8 布局; 无 new 子目录的条目如 report/base 跳过)
export const collectMeasures = async criterion_dir => {
  const measures = [];
  for (const group of await listNames(criterion_dir)) {
    const group_dir = `${criterion_dir}/${group}`;
    for (const impl of await listNames(group_dir)) {
      if (!(await listNames(`${group_dir}/${impl}`)).includes("new")) continue;
      const { mean, median, std_dev } = await readJson(`${group_dir}/${impl}/new/estimates.json`);
      const { throughput } = await readJson(`${group_dir}/${impl}/new/benchmark.json`);
      measures.push({
        group,
        impl,
        mean_ns: mean.point_estimate,
        median_ns: median.point_estimate,
        std_dev_ns: std_dev.point_estimate,
        ci_lower_ns: mean.confidence_interval.lower_bound,
        ci_upper_ns: mean.confidence_interval.upper_bound,
        throughput: throughput?.point_estimate ?? null,
      });
    }
  }
  measures.sort((a, b) => a.group.localeCompare(b.group) || a.impl.localeCompare(b.impl));
  const groups_ordered = [...new Set(measures.map(measure => measure.group))];
  const impls_ordered = [...new Set(measures.map(measure => measure.impl))];
  return {
    crate: "v8-flags",
    generated_at: new Date().toISOString(),
    groups: groups_ordered,
    impls: impls_ordered,
    measures,
  };
};

export const fmtNs = value =>
  value >= 1e6
    ? `${(value / 1e6).toFixed(2)} ms`
    : value >= 1e3
      ? `${(value / 1e3).toFixed(2)} µs`
      : `${value.toFixed(1)} ns`;

export const fmtRatio = value => `${value.toFixed(2)}×`;

export const measureOf = (result, group, impl) =>
  result.measures.find(measure => measure.group === group && measure.impl === impl);
