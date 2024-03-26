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
      component: () => import("@/pages/Provider/Add.vue"),
    },
    {
      name: "provider-edit",
      path: "edit/:id",
      component: () => import("@/pages/Provider/Edit.vue"),
    },
  ],
};
