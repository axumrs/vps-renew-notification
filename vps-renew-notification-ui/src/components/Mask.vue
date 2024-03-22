<script setup lang="ts" name="Mask">
defineProps<{
  noTransparent?: boolean;
}>();
</script>

<template>
  <div
    :class="`fixed inset-0 z-50 ${
      noTransparent ? 'bg-gray-200' : 'bg-black/70'
    }`"
  >
    <slot></slot>
  </div>
</template>
