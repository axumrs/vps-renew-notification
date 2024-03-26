<script setup lang="ts" name="Message">
import { onMounted, ref } from "vue";

const props = defineProps<{ callback?: () => void }>();
const show = ref(true);

onMounted(() => {
  (async () => {
    const t = setTimeout(() => {
      show.value = false;
      if (props.callback) {
        props.callback();
      }
      clearTimeout(t);
    }, 3000);
  })().then();
});
</script>

<template>
  <div
    v-if="show"
    class="fixed left-1/2 top-2 -translate-x-1/2 bg-gradient-to-b from-purple-500 to-purple-700 text-sm text-white px-2 py-1 shadow rounded z-[999]"
  >
    <slot>提示信息</slot>
  </div>
</template>
