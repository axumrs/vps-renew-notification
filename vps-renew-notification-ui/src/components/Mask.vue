<script setup lang="ts" name="Mask">
const props = defineProps<{
  transparent?: "half" | "full" | "none";
  z?: string;
}>();
let css = "bg-black/70";
if (props.transparent === "full") {
  css = "bg-gray-200";
}
if (props.transparent === "none") {
  css = "bg-transparent";
}

const zCss = props.z ?? "z-50";
</script>

<template>
  <div :class="`fixed inset-0 ${zCss}  ${css}`">
    <slot></slot>
  </div>
</template>
