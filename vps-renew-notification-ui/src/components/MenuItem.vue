<script setup lang="ts" name="MenuItem">
defineProps<{
  item: Menu;
}>();
</script>

<template>
  <li class="w-full">
    <RouterLink :to="item.href">{{ item.label }}</RouterLink>
  </li>
</template>
