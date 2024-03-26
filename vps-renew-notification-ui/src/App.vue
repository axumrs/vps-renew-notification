<script setup lang="ts">
import Layout from "@/components/Layout.vue";
import Loading from "@/components/Loading.vue";
import Toad from "@/components/Toad.vue";
import Message from "@/components/Message.vue";
import { useStatusStore } from "@/store/status";

const { isLoading, msg, setMsg, okMsg, setOkMsg } = useStatusStore();
</script>

<template>
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
  <Layout v-else>
    <RouterView />
  </Layout>

  <Message
    v-if="okMsg"
    :callback="
      () => {
        setOkMsg('');
      }
    "
    >{{ okMsg }}</Message
  >
</template>
