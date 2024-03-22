import { createRouter, createWebHistory } from "vue-router";
import provider from "./provider";

export default createRouter({
  history: createWebHistory(),
  routes: [provider],
});
