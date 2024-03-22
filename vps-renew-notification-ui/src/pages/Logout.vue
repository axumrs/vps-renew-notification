<script setup lang="ts" name="LogoutPage">
import Button from "@/components/Button.vue";

const cancelHandler = () => {
  window.history.back();
};
</script>

<template>
  <div class="fixed inset-0 bg-black/70">
    <div
      class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white p-6 rounded shadow min-w-64"
    >
      <div class="text-center my-6">确定退出？</div>
      <div class="flex justify-between w-1/2 mx-auto">
        <Button theme="primary">确定</Button>
        <Button @click="cancelHandler">取消</Button>
      </div>
    </div>
  </div>
</template>
