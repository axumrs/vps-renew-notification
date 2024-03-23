<script setup lang="ts" name="Mask">
const props = defineProps<{
  transparent?: "half" | "full" | "none";
}>();
let css = "bg-black/70";
if (props.transparent === "full") {
  css = "bg-gray-200";
}
if (props.transparent === "none") {
  css = "bg-transparent";
}
</script>

<template>
  <div :class="`fixed inset-0 z-50  ${css}`">
    <slot></slot>
  </div>
</template>
