<script setup lang="ts" name="Select">
import { computed, type InputTypeHTMLAttribute } from "vue";
const modelValue = defineModel<any>();
const props = defineProps<{
  required?: boolean;
  placeholder?: string;
  type?: InputTypeHTMLAttribute;
  label?: string;
  errMsg?: string;
  options?: { value: string; label: string }[];
}>();
const borderColor = computed(() => (props.errMsg ? "border-red-600" : ""));
</script>
<template>
  <label class="block">
    <div v-if="label">{{ label }}</div>
    <select
      v-model="modelValue"
      :class="`border block w-full focus:outline-none px-2 py-1 ${borderColor}`"
      :required="required"
      :placeholder="placeholder"
      :type="type"
    >
      <option
        v-if="options"
        v-for="o in options"
        :key="o.value"
        :value="o.value"
      >
        {{ o.label }}
      </option>
    </select>
    <div class="text-sm text-red-600" v-if="errMsg">{{ errMsg }}</div>
  </label>
</template>
