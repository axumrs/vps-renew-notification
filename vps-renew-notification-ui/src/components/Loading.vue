<script setup lang="ts" name="Loading">
import { LoaderCircle as LoadingIcon } from "lucide-vue-next";
import Mask from "./Mask.vue";
</script>

<template>
  <Mask transparent="none" z="z-[100]">
    <div
      class="p-3 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
    >
      <LoadingIcon class="animate-spin text-blue-900" :size="32" />
    </div>
  </Mask>
</template>
