import { createRouter, createWebHistory } from "vue-router";
import provider from "./provider";
import vps from "./vps";
import { useAuthStore } from "@/store/auth";

const router = createRouter({
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
    {
      name: "login",
      path: "/login",
      component: () => import("@/pages/Login.vue"),
    },
    provider,
    vps,
  ],
});

router.beforeEach(async (to) => {
  const { isLogined } = useAuthStore();
  if (to.name !== "login" && !isLogined()) {
    return { name: "login", query: { to: to.fullPath } };
  }
  return;
});

export default router;
