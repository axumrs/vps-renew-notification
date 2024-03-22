export default {
  name: "provider",
  path: "/provider",
  children: [
    {
      name: "provider-add",
      path: "add",
      component: () => import("@/pages/provider/Provider/Input.vue"),
    },
  ],
};
