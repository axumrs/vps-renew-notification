<script setup lang="ts" name="ProviderInputPage">
import PageTitle from "@/components/PageTitle.vue";
import Input from "@/components/Input.vue";
import Form from "@/components/Form.vue";
import { useRoute, useRouter } from "vue-router";
import { onMounted, reactive } from "vue";
import useFetch from "@/hooks/useFetch";
import { useStatusStore } from "@/store/status";

const { id } = useRoute().params;
const title = id ? "修改" : "添加";

const provider = reactive<Provider>({
  id: "",
  name: "",
  renew_days: 0,
  notify_days: 0,
  dateline: "",
});

const { get, put, post } = useFetch();
const { setOkMsg } = useStatusStore();

const router = useRouter();

onMounted(() => {
  if (id) {
    get(`/provider/${id}`).then((resp: Provider) => {
      Object.assign(provider, resp);
    });
  }
});

const submitHandler = () => {
  if (id) {
    put("/provider", provider).then(() => {
      setOkMsg("修改成功");
      router.push("/provider");
    });
    return;
  }
  post("/provider", provider).then(() => {
    setOkMsg("添加成功");
    router.push("/provider");
  });
};
</script>

<template>
  <PageTitle>{{ title }}服务商</PageTitle>

  <Form @submit="submitHandler">
    <Input label="名称" v-model="provider.name" required />
    <Input label="续期天数" v-model.number="provider.renew_days" required />
    <Input label="通知天数" v-model.number="provider.notify_days" required />
  </Form>
</template>
