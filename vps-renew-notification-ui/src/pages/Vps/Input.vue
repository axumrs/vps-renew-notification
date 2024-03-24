<script setup lang="ts" name="VpsInputPage">
import { useRoute, useRouter } from "vue-router";
import PageTitle from "@/components/PageTitle.vue";
import Input from "@/components/Input.vue";
import Select from "@/components/Select.vue";
import Form from "@/components/Form.vue";
import useFetch from "@/hooks/useFetch";
import { useStatusStore } from "@/store/status";
import { onMounted, reactive, ref } from "vue";
import dayjs from "dayjs";

const { setOkMsg, setMsg } = useStatusStore();
const { get, put, post } = useFetch();

const r = useRoute();
const { id } = r.params;
const router = useRouter();

const title = id ? "修改" : "添加";

const emptyVps: VPS = {
  id: "",
  provider_id: "",
  name: "",
  expire: dayjs().format("YYYY-MM-DDT00:00:00+08:00"),
  dateline: "",
};
const vps = reactive<VPS>(emptyVps);

const providerList = ref<Provider[]>();

const loadProviderList = async () => {
  get("/provider").then((resp: Provoider[]) => {
    providerList.value = resp;
  });
};
const loadVps = async (id: string) => {
  get(`/vps/${id}`).then((resp: VPS) => {
    Object.assign(vps, resp);
  });
};

onMounted(() => {
  Object.assign(vps, emptyVps);
  if (id) {
    Promise.all([loadProviderList(), loadVps(id as string)]).then();
  }
  loadProviderList().then();
});

const submitHanlder = () => {
  if (id) {
    put(`/vps`, vps).then(() => {
      setOkMsg("修改成功");
      router.push("/vps");
    });
    return;
  }
  post(`/vps`, vps).then(() => {
    setOkMsg("添加成功");
    router.push("/vps");
  });
};
</script>

<template>
  <PageTitle>{{ title }}VPS</PageTitle>
  {{ vps }}

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
