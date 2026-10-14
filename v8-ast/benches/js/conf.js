// 报告脚本公共配置: 输出目录、默认语言、i18n 加载 (md.js 等按此加载语言文件)

const REPORT_DIR = `${import.meta.dirname}/report`;
const DEFAULT_LANG = "zh";

// 加载 i18n/语言/文件名.js 语言模块 (export const 大写变量名, 不用字典)
const i18n_read = async (lang = DEFAULT_LANG) => import(`./i18n/${lang}.js`);

export { DEFAULT_LANG, REPORT_DIR, i18n_read };
export default { DEFAULT_LANG, REPORT_DIR, i18n_read };
