<script>
  let {
    selected = "linux-arm64",
    items = [],
    onselect = () => {},
  } = $props();

  let is_open = $state(false),
    container_el = $state(null);

  const toggleDropdown = (e) => {
    e.stopPropagation();
    is_open = !is_open;
  };

  const selectItem = (id) => {
    is_open = false;
    onselect(id);
  };

  const onDocClick = (e) => {
    if (container_el && !container_el.contains(e.target)) {
      is_open = false;
    }
  };

  $effect(() => {
    if (is_open) {
      document.addEventListener("click", onDocClick);
      return () => document.removeEventListener("click", onDocClick);
    }
  });

  const selectedItem = $derived(items.find((i) => i.id === selected) || items[0]);
</script>

<template lang="pug">
.bench-dropdown(bind:this={container_el})
  button.dropdown-trigger(
    type="button"
    class:open={is_open}
    onclick={toggleDropdown}
    aria-haspopup="listbox"
    aria-expanded={is_open}
  )
    span.trigger-title {selectedItem ? (selectedItem.nameZh || selectedItem.name) : ""}
    svg.arrow-icon(
      viewBox="0 0 20 20"
      width="14"
      height="14"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    )
      path(d="M6 8l4 4 4-4")

  +if('is_open')
    .dropdown-menu(role="listbox")
      +each('items as item')
        button.dropdown-item(
          type="button"
          role="option"
          aria-selected={selected === item.id}
          class:selected={selected === item.id}
          onclick={() => selectItem(item.id)}
        )
          span.item-label {item.nameZh || item.name}
          +if('selected === item.id')
            span.item-check •
</template>
