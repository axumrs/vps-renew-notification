import { createRouter, createWebHistory } from "vue-router";
import provider from "./provider";
import vps from "./vps";

export default createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/vps/renew",
    },
    {
      name: "logout",
      path: "/logout",
      component: () => import("@/pages/Logout.vue"),
    },
    {
      name: "change-password",
      path: "/change-password",
      component: () => import("@/pages/ChangePassword.vue"),
    },
    {
      name: "notification",
      path: "/notification",
      component: () => import("@/pages/Notification.vue"),
    },
    provider,
    vps,
  ],
});
