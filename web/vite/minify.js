import minifyHtml from "@minify-html/node";
import { transform } from "lightningcss";

const { minify } = minifyHtml;

export const htmlMinifyPlugin = () => ({
  name: "vite-plugin-minify-html",
  enforce: "post",
  transformIndexHtml: {
    order: "post",
    handler(html) {
      // 1. lightningcss 优化 inline styles
      let out_html = html.replace(
        /(<style[^>]*>)([\s\S]*?)(<\/style>)/gi,
        (match, open_tag, css_content, close_tag) => {
          try {
            const { code } = transform({
              filename: "inline.css",
              code: Buffer.from(css_content),
              minify: true,
              sourceMap: false,
            });
            return open_tag + code.toString() + close_tag;
          } catch {
            return match;
          }
        },
      );

      // 2. @minify-html/node 高性能二进制压缩
      try {
        return minify(Buffer.from(out_html), {
          minify_css: true,
          minify_js: true,
          keep_spaces_between_attributes: true,
        }).toString();
      } catch {
        return out_html;
      }
    },
  },
});
