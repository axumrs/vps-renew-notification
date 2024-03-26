<script setup lang="ts" name="ProviderAddPage">
import PageTitle from "@/components/PageTitle.vue";
import { useRouter } from "vue-router";
import useFetch from "@/hooks/useFetch";
import { useStatusStore } from "@/store/status";
import ProviderInput from "@/components/ProviderInput.vue";

const { post } = useFetch();
const { setOkMsg } = useStatusStore();

const router = useRouter();

const submitHandler = (provider: Provider) => {
  post("/provider", provider).then(() => {
    setOkMsg("添加成功");
    router.push("/provider");
  });
};
</script>

<template>
  <PageTitle>添加服务商</PageTitle>

  <ProviderInput @submit="submitHandler" />
</template>
