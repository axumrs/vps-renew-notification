<script setup lang="ts" name="VpsListPage">
import { onMounted, ref, computed } from "vue";
import PageTitle from "@/components/PageTitle.vue";
import Button from "@/components/Button.vue";
import useFetch from "@/hooks/useFetch";
import dayjs from "dayjs";
import { useStatusStore } from "@/store/status";

const _vpsList = ref<VPSWithProvider[]>();
const keyword = ref("");
const vpsList = computed<VPSWithProvider[]>(() =>
  _vpsList.value
    ? _vpsList.value.filter(
        (v) => v.name.toLowerCase().indexOf(keyword.value.toLowerCase()) >= 0
      )
    : []
);

const { get, patch, del } = useFetch();
const { setOkMsg } = useStatusStore();

const loadVpsList = () => {
  get("/vps").then((resp: VPSWithProvider[]) => {
    _vpsList.value = resp;
  });
};

onMounted(() => {
  loadVpsList();
});

const renewHandler = (id: string) => {
  if (!window.confirm("确定续期？")) {
    return;
  }
  patch(`/vps/${id}`).then(() => {
    setOkMsg("续期成功");
    loadVpsList();
  });
};

const delhandler = (id: string) => {
  if (!window.confirm("确定删除？")) {
    return;
  }
  del(`/vps/${id}`).then(() => {
    setOkMsg("删除成功");
    loadVpsList();
  });
};
</script>

<template>
  <PageTitle>VPS列表</PageTitle>

  <div class="flex justify-start items-center gap-x-2 my-6">
    <!-- <div>{{ checkedIds }}</div> -->
    <div>
      <div>
        <input
          class="border focus:outline-none px-3 py-1"
          placeholder="输入关键字"
          v-model.trim="keyword"
        />
      </div>
    </div>
  </div>

  <section>
    <table>
      <thead>
        <tr class="">
          <th>#</th>
          <th>名称</th>
          <th>服务商</th>
          <th>到期时间</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        <tr v-if="vpsList" v-for="v in vpsList" :key="v.id">
          <td>{{ v.id }}</td>
          <td>{{ v.name }}</td>
          <td>{{ v.provider_name }}</td>
          <td>{{ dayjs(v.expire).format("YYYY/MM/DD") }}</td>
          <td>
            <div>
              <Button theme="primary" size="xs" @click="renewHandler(v.id)"
                >续期</Button
              >
              <Button theme="info" size="xs" :href="`/vps/edit/${v.id}`"
                >修改</Button
              >
              <Button theme="danger" size="xs" @click="delhandler(v.id)"
                >删除</Button
              >
            </div>
          </td>
        </tr>
        <tr v-else>
          <td colspan="5">暂无数据</td>
        </tr>
      </tbody>
    </table>
  </section>
</template>
