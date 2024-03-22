import { type RouteLocationRaw } from "vue-router";
import { type Component } from "vue";

type Menu = {
  id: string;
  label: string;
  href?: RouteLocationRaw;
  icon?: Component;
  children?: Menu[];
  expand?: boolean;
};
