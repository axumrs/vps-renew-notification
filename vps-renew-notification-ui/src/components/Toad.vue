<script setup lang="ts" name="Toad">
import { onMounted, ref } from "vue";
import Mask from "./Mask.vue";

const isShow = ref(true);

const props = withDefaults(
  defineProps<{
    duration?: number;
    callback?: () => void;
  }>(),
  { duration: 3000 }
);

onMounted(() => {
  const t = setTimeout(() => {
    isShow.value = false;
    if (props.callback) {
      props.callback();
    }
    clearTimeout(t);
  }, props.duration);
});
</script>

<template>
  <Mask transparent="none" v-if="isShow" z="z-[100]">
    <div
      class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 p-6 bg-black/80 rounded text-white shadow"
    >
      <slot>提示信息</slot>
    </div>
  </Mask>
</template>
