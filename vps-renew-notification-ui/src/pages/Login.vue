<script setup lang="ts" name="LoginPage">
import Login from "@/components/Login.vue";
import Mask from "@/components/Mask.vue";
</script>

<template>
  <Mask transparent="full">
    <div
      class="absolute min-w-80 bg-white p-6 rounded shadow top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
    >
      <Login />
    </div>
  </Mask>
</template>
