<script setup lang="ts" name="Input">
import { type InputTypeHTMLAttribute } from "vue";
const modelValue = defineModel<any>();
defineProps<{
  required?: boolean;
  placeholder?: string;
  type?: InputTypeHTMLAttribute;
  label?: string;
}>();
</script>
<template>
  <label class="block">
    <div v-if="label">{{ label }}</div>
    <input
      v-model="modelValue"
      class="border block w-full focus:outline-none px-2 py-1"
      :required="required"
      :placeholder="placeholder"
      :type="type"
    />
  </label>
</template>
