import { createRouter, createWebHistory } from "vue-router";
import provider from "./provider";
import vps from "./vps";

export default createRouter({
  history: createWebHistory(),
  routes: [
    {
      name: "logout",
      path: "/logout",
      component: () => import("@/pages/Logout.vue"),
    },
    provider,
    vps,
  ],
});
