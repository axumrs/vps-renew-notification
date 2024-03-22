import { createRouter, createWebHistory } from "vue-router";
import provider from "./provider";
import vps from "./vps";

export default createRouter({
  history: createWebHistory(),
  routes: [provider, vps],
});
