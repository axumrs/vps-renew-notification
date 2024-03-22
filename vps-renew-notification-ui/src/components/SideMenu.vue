<script setup lang="ts" name="SideMenu">
import { Server, DatabaseZap, Bell, LockKeyhole } from "lucide-vue-next";
import MenuGroup from "./Menu.vue";

const menu: Menu[] = [
  {
    id: "vps",
    label: "VPS管理",
    icon: Server,
    expand: true,
    children: [
      {
        id: "vps-list",
        label: "VPS列表",
        href: "/vps",
      },
      {
        id: "vps-add",
        label: "添加VPS",
        href: "/vps/add",
      },
    ],
  },
  {
    id: "provider",
    label: "服务商",
    icon: DatabaseZap,
    children: [
      {
        id: "provider-list",
        label: "服务商列表",
        href: "/provider",
      },
      {
        id: "provider-add",
        label: "添加服务商",
        href: "/provider/add",
      },
    ],
  },
  {
    id: "notification",
    label: "通知",
    icon: Bell,
    children: [
      {
        id: "notification-setting",
        label: "通知设置",
        href: "/notification",
      },
    ],
  },
  {
    id: "account",
    label: "账号",
    icon: LockKeyhole,
    children: [
      {
        id: "change-password",
        label: "修改密码",
        href: "/change-password",
      },
      {
        id: "logout",
        label: "安全退出",
        href: "/logout",
      },
    ],
  },
];
</script>
<template>
  <ul class="my-6">
    <MenuGroup v-for="m in menu" :key="m.id" :menu="m" :expand="m.expand" />
  </ul>
</template>
