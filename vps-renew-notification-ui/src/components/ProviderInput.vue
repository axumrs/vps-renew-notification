<script setup lang="ts" name="ProviderInput">
import Input from "@/components/Input.vue";
import Form from "@/components/Form.vue";
import { reactive } from "vue";

const props = defineProps<{ item?: Provider }>();

const provider = reactive<Provider>(
  props.item ?? {
    id: "",
    name: "",
    renew_days: 0,
    notify_days: 0,
    dateline: "",
  }
);

const emit = defineEmits<{
  (e: "submit", provider: Provider): void;
}>();

const submitHandler = () => {
  emit("submit", provider);
};
</script>

<template>
  <Form @submit="submitHandler">
    <Input label="名称" v-model="provider.name" required />
    <Input label="续期天数" v-model.number="provider.renew_days" required />
    <Input label="通知天数" v-model.number="provider.notify_days" required />
  </Form>
</template>
