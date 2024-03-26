<script setup lang="ts" name="VpsInputPage">
import { useRoute, useRouter } from "vue-router";
import PageTitle from "@/components/PageTitle.vue";

import useFetch from "@/hooks/useFetch";
import { useStatusStore } from "@/store/status";
import { onMounted, reactive } from "vue";

import VpsInput from "@/components/VpsInput.vue";

const { setOkMsg } = useStatusStore();
const { get, put } = useFetch();

const r = useRoute();
const { id } = r.params;
const router = useRouter();

const vps = reactive<VPS>({
  id: "",
  provider_id: "",
  name: "",
  expire: "",
  dateline: "",
});

const loadVps = async (id: string) => {
  get(`/vps/${id}`).then((resp: VPS) => {
    Object.assign(vps, resp);
  });
};

onMounted(() => {
  loadVps(id as string).then();
});

const submitHanlder = (vps: VPS) => {
  put(`/vps`, vps).then(() => {
    setOkMsg("修改成功");
    router.push("/vps");
  });
  return;
};
</script>

<template>
  <PageTitle>修改VPS</PageTitle>

  <VpsInput :item="vps" @submit="submitHanlder" />
</template>
