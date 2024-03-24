<script setup lang="ts" name="ProviderListPage">
import PageTitle from "@/components/PageTitle.vue";
import Button from "@/components/Button.vue";
import { useStatusStore } from "@/store/status";
import useFetch from "@/hooks/useFetch";
import { onMounted, ref } from "vue";

const providerList = ref<Provider[]>();

const { get, del } = useFetch();
const { setOkMsg } = useStatusStore();

const loadProviderList = () => {
  get("/provider").then((resp: Provider[]) => {
    providerList.value = resp;
  });
};

onMounted(() => {
  loadProviderList();
});

const delHandler = (id: string) => {
  if (!window.confirm("确定删除？")) {
    return;
  }
  del(`/provider/${id}`).then(() => {
    setOkMsg("删除成功");
    loadProviderList();
  });
};
</script>

<template>
  <PageTitle>服务商列表</PageTitle>

  <section>
    <table>
      <thead>
        <tr class="">
          <th>#</th>
          <th>名称</th>
          <th>续期天数</th>
          <th>通知天数</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        <tr v-if="providerList" v-for="p in providerList" :key="p.id">
          <td>{{ p.id }}</td>
          <td>{{ p.name }}</td>
          <td>{{ p.renew_days }}</td>
          <td>{{ p.notify_days }}</td>
          <td>
            <div>
              <Button theme="info" size="xs" :href="`/provider/edit/${p.id}`"
                >修改</Button
              >
              <Button theme="danger" size="xs" @click="delHandler(p.id)"
                >删除</Button
              >
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </section>
</template>
