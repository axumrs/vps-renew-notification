<script setup lang="ts" name="VpsRenewPage">
import { onMounted, ref, computed } from "vue";
import {
  CalendarDays,
  DatabaseZap as ProviderIcon,
  Check as CheckIcon,
} from "lucide-vue-next";
import PageTitle from "@/components/PageTitle.vue";
import Button from "@/components/Button.vue";
import useFetch from "@/hooks/useFetch";
import { useStatusStore } from "@/store/status";

import dayjs from "dayjs";
import duration from "dayjs/plugin/duration";
dayjs.extend(duration);

const _vpsList = ref<VPSWithProvider[]>();
const checkedIds = ref<string[]>([]);
const keyword = ref("");

const vpsList = computed<VPSWithProvider[]>(() =>
  _vpsList.value
    ? _vpsList.value.filter(
        (v) => v.name.toLowerCase().indexOf(keyword.value.toLowerCase()) >= 0
      )
    : []
);

const { get, patch, post } = useFetch();
const { setOkMsg } = useStatusStore();

const loadVpsList = () => {
  get("/vps").then((data: VPSWithProvider[]) => {
    _vpsList.value = data;
  });
};
onMounted(() => {
  loadVpsList();
});

const renewHandler = (id: string) => {
  if (!window.confirm("确认续期？")) {
    return;
  }
  patch(`/vps/${id}`).then(() => {
    setOkMsg("续期成功");
    loadVpsList();
  });
};
const selectedVpsIdx = (id: string) => {
  return checkedIds.value.findIndex((_id) => _id === id);
};
const isSelectedVps = (id: string) => {
  const idx = selectedVpsIdx(id);
  return idx >= 0;
};
const selectRenewVps = (id: string) => {
  const idx = selectedVpsIdx(id);
  if (idx >= 0) {
    checkedIds.value.splice(idx, 1);
    return;
  }
  checkedIds.value.push(id);
};
const batchRenew = () => {
  if (!window.confirm("确定续期？")) {
    return;
  }
  post("/vps/batch-renew", checkedIds.value).then(() => {
    setOkMsg("续期成功");
    checkedIds.value = [];
    loadVpsList();
  });
};
</script>

<template>
  <PageTitle>VPS续期</PageTitle>

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
    <div>
      <Button theme="info" v-if="checkedIds.length > 0" @click="batchRenew"
        >一键续期</Button
      >
    </div>
  </div>
  <ul class="grid grid-cols-5 gap-2" v-if="vpsList">
    <li
      :class="`relative border rounded shadow p-3 flex flex-col gap-y-2 ${
        dayjs.duration(dayjs(v.expire).diff(dayjs())).days() + 1 <=
        v.notify_days
          ? 'border-orange-600'
          : ''
      }
      ${isSelectedVps(v.id) ? 'bg-blue-50' : ''}
      `"
      v-for="v in vpsList"
      :key="v.id"
      @click="selectRenewVps(v.id)"
    >
      <div
        class="absolute z-10 right-1 text-blue-600"
        v-if="isSelectedVps(v.id)"
      >
        <CheckIcon />
      </div>
      <h2 class="text-lg">{{ v.name }}</h2>
      <div class="flex justify-start items-center gap-x-1">
        <div><ProviderIcon :size="14" /></div>
        <div>{{ v.provider_name }}</div>
      </div>
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
