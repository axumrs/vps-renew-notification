<script setup lang="ts" name="MenuItem">
import { type Menu } from "@/types/menu";
defineProps<{
  item: Menu;
}>();
</script>

<template>
  <li class="w-full">
    <RouterLink :to="item.href!">{{ item.label }}</RouterLink>
  </li>
</template>
