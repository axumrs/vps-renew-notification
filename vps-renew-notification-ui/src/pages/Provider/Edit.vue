<script setup lang="ts" name="ProviderEditPage">
import PageTitle from "@/components/PageTitle.vue";
import { useRoute, useRouter } from "vue-router";
import { onMounted, reactive } from "vue";
import useFetch from "@/hooks/useFetch";
import { useStatusStore } from "@/store/status";
import ProviderInput from "@/components/ProviderInput.vue";

const { id } = useRoute().params;

const provider = reactive<Provider>({
  id: "",
  name: "",
  renew_days: 0,
  notify_days: 0,
  dateline: "",
});

const { get, put } = useFetch();
const { setOkMsg } = useStatusStore();

const router = useRouter();

onMounted(() => {
  if (id) {
    get(`/provider/${id}`).then((resp: Provider) => {
      Object.assign(provider, resp);
    });
  }
});

const submitHandler = (provider: Provider) => {
  put("/provider", provider).then(() => {
    setOkMsg("修改成功");
    router.push("/provider");
  });
  return;
};
</script>

<template>
  <PageTitle>修改服务商</PageTitle>

  <ProviderInput :item="provider" @submit="submitHandler" />
</template>
