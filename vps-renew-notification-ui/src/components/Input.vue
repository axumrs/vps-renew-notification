<script setup lang="ts" name="Input">
import { computed, type InputTypeHTMLAttribute } from "vue";
const modelValue = defineModel<any>();
const props = defineProps<{
  required?: boolean;
  placeholder?: string;
  type?: InputTypeHTMLAttribute;
  label?: string;
  errMsg?: string;
}>();
const borderColor = computed(() => (props.errMsg ? "border-red-600" : ""));
</script>
<template>
  <label class="block">
    <div v-if="label">{{ label }}</div>
    <input
      v-model="modelValue"
      :class="`border block w-full focus:outline-none px-2 py-1 ${borderColor}`"
      :required="required"
      :placeholder="placeholder"
      :type="type"
    />
    <div class="text-sm text-red-600" v-if="errMsg">{{ errMsg }}</div>
  </label>
</template>
