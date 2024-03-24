<script setup lang="ts" name="VpsRenewPage">
import { onMounted, ref } from "vue";
import { CalendarDays } from "lucide-vue-next";
import PageTitle from "@/components/PageTitle.vue";
import Button from "@/components/Button.vue";
import useFetch from "@/hooks/useFetch";
import { useStatusStore } from "@/store/status";

import dayjs from "dayjs";
import duration from "dayjs/plugin/duration";
dayjs.extend(duration);

const vpsList = ref<VPS[]>();

const { get, patch } = useFetch();
const { setMsg } = useStatusStore();

const loadVpsList = () => {
  get("/vps").then((data: VPS[]) => {
    vpsList.value = data;
  });
};
onMounted(() => {
  loadVpsList();
});

const renewHandler = (id: string) => {
  patch(`/vps/${id}`).then(() => {
    setMsg("续期成功");
    const t = setTimeout(() => {
      loadVpsList();
      clearTimeout(t);
    }, 3000);
  });
};
</script>

<template>
  <PageTitle>VPS续期</PageTitle>

  <ul class="grid grid-cols-5 gap-2" v-if="vpsList">
    <li
      :class="`border rounded shadow p-3 flex flex-col gap-y-2 ${
        dayjs.duration(dayjs(v.expire).diff(dayjs())).days() + 1 <= 2
          ? 'border-orange-600'
          : ''
      }`"
      v-for="v in vpsList!"
      :key="v.id"
    >
      <h2 class="text-lg">{{ v.name }}</h2>
      <div class="flex justify-start items-center gap-x-1 text-sm">
        <div><CalendarDays :size="14" /></div>
        <div>
          {{ dayjs(v.expire).format("YYYY/MM/DD") }}
        </div>
      </div>
      <div>
        <Button theme="primary" @click="renewHandler(v.id)">续期</Button>
      </div>
    </li>
  </ul>
</template>
