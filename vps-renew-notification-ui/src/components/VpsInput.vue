<script setup lang="ts" name="VpsInput">
import dayjs from "dayjs";
import { defineEmits, onMounted, reactive, ref } from "vue";
import useFetch from "@/hooks/useFetch";
import Input from "@/components/Input.vue";
import Select from "@/components/Select.vue";
import Form from "@/components/Form.vue";

const { get } = useFetch();

const emptyVps: VPS = {
  id: "",
  provider_id: "",
  name: "",
  expire: dayjs().format("YYYY-MM-DDT00:00:00+08:00"),
  dateline: "",
};

const emit = defineEmits<{
  (e: "submit", vps: VPS): void;
}>();

const props = defineProps<{
  item?: VPS;
}>();

const vps = reactive<VPS>(props.item ?? emptyVps);

const providerList = ref<Provider[]>();

const loadProviderList = async () => {
  get("/provider").then((resp: Provider[]) => {
    providerList.value = resp;
  });
};

const submitHanlder = () => {
  emit("submit", vps);
};

onMounted(() => {
  loadProviderList().then();
});
</script>

<template>
  <Form @submit="submitHanlder">
    <Input label="名称" v-model="vps.name" required />
    <Select
      label="服务商"
      :options="providerList?.map((p) => ({ value: p.id, label: p.name }))"
      v-model="vps.provider_id"
      required
    />
    <Input label="到期时间" v-model="vps.expire" required />
  </Form>
</template>
