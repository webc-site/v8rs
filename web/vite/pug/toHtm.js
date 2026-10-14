import pug from "pug";
import pugHack from "./hack.js";

const MIXIN = "mixin ",
  BLOCK_NL = "\n%_block\n",
  GET_MIXINS = () =>
    [
      MIXIN + "if(condition)",
      "%_| {#if !{condition}}" + BLOCK_NL + "%_| {/if}",
      MIXIN + "else",
      "%_| {:else}" + BLOCK_NL,
      MIXIN + "elseif(condition)",
      "%_| {:else if !{condition}}" + BLOCK_NL,
      MIXIN + "key(expression)",
      "%_| {#key !{expression}}" + BLOCK_NL + "%_| {/key}",
      MIXIN + "each(loop)",
      "%_| {#each !{loop}}" + BLOCK_NL + "%_| {/each}",
      MIXIN + "await(promise)",
      "%_| {#await !{promise}}" + BLOCK_NL + "%_| {/await}",
      MIXIN + "then(answer)",
      "%_| {:then !{answer}}" + BLOCK_NL,
      MIXIN + "catch(error)",
      "%_| {:catch !{error}}" + BLOCK_NL,
      MIXIN + "html(expression)",
      "%_| {@html !{expression}}",
      MIXIN + "const(expression)",
      "%_| {@const !{expression}}",
      MIXIN + "debug(variables)",
      "%_| {@debug !{variables}}",
      MIXIN + "snippet(name)",
      "%_| {#snippet !{name}}" + BLOCK_NL + "%_| {/snippet}",
      MIXIN + "render(name)",
      "%_| {@render !{name}}",
    ]
      .join("\n\n")
      .replaceAll("%_", "  ") + "\n",
  transformer = (content, filename, options = {}) => {
    const pug_options = Object.assign(
        {
          doctype: "html",
          compileDebug: false,
          filename,
        },
        options,
      ),
      input_content = content.trimStart(),
      mixins = GET_MIXINS(),
      input = input_content.startsWith("extends ")
        ? input_content.replace(/^(extends\s+[^\n]+)/, "$1\n" + mixins)
        : mixins + "\n" + content,
      compiled = pug.compile(pugHack(input, filename), pug_options);
    return [compiled(options), compiled.dependencies || []];
  };

export default transformer;
