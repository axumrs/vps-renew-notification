<script setup lang="ts" name="Button">
import { type RouteLocationRaw } from "vue-router";
const props = withDefaults(
  defineProps<{
    type?: "button" | "submit" | "reset";
    class?: string;
    size?: "xs" | "lg";
    theme?: "primary" | "warning" | "danger" | "info";
    href?: RouteLocationRaw;
  }>(),
  { class: "" }
);

const emit = defineEmits(["click"]);

let sizeCss = "px-2 py-1";
let themeCss = "hover:bg-gray-50";

switch (props.size) {
  case "xs":
    sizeCss = "px-1 py-0.5 text-xs";
    break;
  case "lg":
    sizeCss = "px-4 py-2 text-lg";
    break;
  default:
    break;
}

switch (props.theme) {
  case "primary":
    themeCss = "text-white bg-blue-600 border-blue-700 hover:bg-blue-700";
    break;
  case "warning":
    themeCss = "text-white bg-orange-600 border-orange-700 hover:bg-orange-700";
    break;
  case "danger":
    themeCss = "text-white bg-red-600 border-red-700 hover:bg-red-700";
    break;
  case "info":
    themeCss = "text-white bg-cyan-600 border-cyan-700 hover:bg-cyan-700";
    break;
}
</script>

<template>
  <RouterLink
    :to="href"
    v-if="href"
    :class="`border ${sizeCss} ${themeCss} ${props.class}`"
    @click.prevent="emit('click')"
    ><slot></slot
  ></RouterLink>
  <button
    :class="`border ${sizeCss} ${themeCss} ${props.class}`"
    :type="type"
    @click="emit('click')"
    v-else
  >
    <slot></slot>
  </button>
</template>
