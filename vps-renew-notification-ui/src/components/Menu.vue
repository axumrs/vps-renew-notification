<script setup lang="ts" name="Menu">
import { ref } from "vue";
import { ChevronDown as ArrowIcon } from "lucide-vue-next";
import MenuItem from "./MenuItem.vue";

const props = defineProps<{ menu: Menu; expand?: boolean }>();

const isExpand = ref(props.expand);
</script>
<template>
  <li
    class="my-3 p-3 bg-white rounded shadow-lg shadow-blue-100 flex flex-col gap-y-2"
  >
    <div
      class="flex items-center justify-between w-full cursor-pointer"
      @click="isExpand = !isExpand"
    >
      <div class="flex items-center justify-start gap-x-1">
        <div class="text-gray-600">
          <component :is="menu.icon" :size="20" />
        </div>
        <div>
          {{ menu.label }}
        </div>
      </div>
      <div>
        <ArrowIcon :class="isExpand ? 'rotate-180' : ''" />
      </div>
    </div>
    <ul class="px-6 w-full flex flex-col gap-y-1" v-show="isExpand">
      <MenuItem v-for="m in menu.children" :key="m.id" :item="m" />
    </ul>
  </li>
</template>
