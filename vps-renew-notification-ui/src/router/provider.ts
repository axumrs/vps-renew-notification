export default {
  name: "provider",
  path: "/provider",
  children: [
    {
      name: "provider-list",
      path: "",
      component: () => import("@/pages/Provider/List.vue"),
    },
    {
      name: "provider-add",
      path: "add",
      component: () => import("@/pages/Provider/Input.vue"),
    },
  ],
};
