<script setup lang="ts" name="Form">
import Button from "@/components/Button.vue";
defineProps<{
  noSubmitButton?: boolean;
  noResetButton?: boolean;
}>();

const emit = defineEmits<{
  (e: "submit"): void;
}>();
</script>

<template>
  <form class="flex flex-col gap-y-2" @submit.prevent="emit('submit')">
    <slot></slot>

    <slot name="buttons">
      <div class="flex justify-start items-center gap-x-2">
        <Button theme="primary" type="submit" v-if="!noSubmitButton"
          >提交</Button
        >
        <Button type="reset" v-if="!noResetButton">重置</Button>
      </div>
    </slot>
  </form>
</template>
