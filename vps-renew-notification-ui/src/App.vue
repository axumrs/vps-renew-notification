<script setup lang="ts">
import Layout from "@/components/Layout.vue";
import Login from "@/components/Login.vue";
import Mask from "@/components/Mask.vue";
import Loading from "@/components/Loading.vue";
import Toad from "@/components/Toad.vue";
import { useStatusStore } from "@/store/status";
import { useAuthStore } from "./store/auth";

const { isLoading, msg, setMsg } = useStatusStore();
const { isLogined, loginResp } = useAuthStore();
const checkIsLogined = () => {
  return isLogined();
};
</script>

<template>
  <div class="fixed z-[9999] hidden">{{ loginResp }}</div>
  <Loading v-if="isLoading"></Loading>
  <Toad
    v-if="msg"
    :callback="
      () => {
        setMsg('');
      }
    "
    >{{ msg }}</Toad
  >
  <Mask transparent="full" v-if="!checkIsLogined()">
    <div
      class="absolute bg-white p-6 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 min-w-80 rounded shadow"
    >
      <Login @callback="checkIsLogined" />
    </div>
  </Mask>

  <Layout v-else>
    <RouterView />
  </Layout>
</template>
