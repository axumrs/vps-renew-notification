<script setup lang="ts" name="Button">
import { type RouteLocationRaw } from "vue-router";
import { Ellipsis as LoadingIcon } from "lucide-vue-next";
const props = withDefaults(
  defineProps<{
    type?: "button" | "submit" | "reset";
    class?: string;
    size?: "xs" | "lg";
    theme?: "primary" | "warning" | "danger" | "info";
    href?: RouteLocationRaw;
    disabled?: boolean;
    loading?: boolean;
  }>(),
  { class: "" }
);

const emit = defineEmits(["click"]);

let sizeCss = "px-2 py-1";
let themeCss = "hover:bg-gray-50";
let loadingIconSize: number;

switch (props.size) {
  case "xs":
    sizeCss = "px-1 py-0.5 text-xs";
    loadingIconSize = 16;
    break;
  case "lg":
    sizeCss = "px-4 py-2 text-lg";
    loadingIconSize = 16;
    break;
  default:
    break;
}

switch (props.theme) {
  case "primary":
    themeCss =
      "text-white bg-blue-600 border-blue-700 hover:bg-blue-700 disabled:bg-blue-500 disabled:border-blue-600";
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
    :disabled="loading || disabled"
    v-else
  >
    <LoadingIcon v-if="loading" class="animate-ping" :size="loadingIconSize" />
    <slot v-else></slot>
  </button>
</template>
