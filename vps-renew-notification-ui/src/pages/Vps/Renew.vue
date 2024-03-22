<script setup lang="ts" name="VpsRenewPage">
import { CalendarDays } from "lucide-vue-next";
import PageTitle from "@/components/PageTitle.vue";
import Button from "@/components/Button.vue";
</script>

<template>
  <PageTitle>VPS续期</PageTitle>

  <ul class="grid grid-cols-5 gap-2">
    <li :class="`border rounded shadow p-3 flex flex-col gap-y-2`">
      <h2 class="text-lg">Hax-1</h2>
      <div class="flex justify-start items-center gap-x-1 text-sm">
        <div><CalendarDays :size="14" /></div>
        <div>2024/03/24</div>
      </div>
      <div>
        <Button theme="primary">续期</Button>
      </div>
    </li>
    <li
      :class="`border rounded shadow p-3 flex flex-col gap-y-2 border-orange-500`"
    >
      <h2 class="text-lg">Hax-1</h2>
      <div class="flex justify-start items-center gap-x-1 text-sm">
        <div><CalendarDays :size="14" /></div>
        <div>2024/03/24</div>
      </div>
      <div>
        <Button theme="primary">续期</Button>
      </div>
    </li>
  </ul>
</template>
