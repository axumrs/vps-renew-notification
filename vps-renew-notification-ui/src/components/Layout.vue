<script setup lang="ts" name="Layout">
import SideMenu from "./SideMenu.vue";
</script>
<template>
  <div class="grid grid-cols-12 gap-x-1">
    <aside class="col-span-2 min-h-screen px-6 py-6 bg-blue-50">
      <h2 class="text-xl font-normal text-center text-purple-600">
        VPS续费通知
      </h2>
      <SideMenu />
    </aside>
    <main class="col-span-10 p-6">
      <slot></slot>
    </main>
  </div>
</template>
