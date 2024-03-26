<script setup lang="ts" name="VpsAddPage">
import { useRouter } from "vue-router";
import PageTitle from "@/components/PageTitle.vue";
import useFetch from "@/hooks/useFetch";
import { useStatusStore } from "@/store/status";

import VpsInput from "@/components/VpsInput.vue";

const { setOkMsg } = useStatusStore();
const { post } = useFetch();

const router = useRouter();

const submitHanlder = (vps: VPS) => {
  post(`/vps`, vps).then(() => {
    setOkMsg("添加成功");
    router.push("/vps");
  });
};
</script>

<template>
  <PageTitle>添加VPS</PageTitle>

  <VpsInput @submit="submitHanlder" />
</template>
