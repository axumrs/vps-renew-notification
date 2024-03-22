export default {
  name: "vps",
  path: "/vps",
  children: [
    {
      name: "vps-list",
      path: "",
      component: () => import("@/pages/Vps/List.vue"),
    },
    {
      name: "vps-add",
      path: "add",
      component: () => import("@/pages/Vps/Input.vue"),
    },
  ],
};