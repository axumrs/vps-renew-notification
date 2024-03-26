<script setup lang="ts" name="Login">
import Input from "@/components/Input.vue";
import Form from "@/components/Form.vue";
import Button from "@/components/Button.vue";

import { useStatusStore } from "@/store/status";
import { useAuthStore } from "@/store/auth";

import useFetch from "@/hooks/useFetch";

import { reactive } from "vue";
import { useRoute, useRouter } from "vue-router";

const state = reactive({ username: "", password: "" });
const { setMsg, setOkMsg } = useStatusStore();
const { setLoginResp } = useAuthStore();
const { post } = useFetch();

const router = useRouter();
const route = useRoute();

const loginHandler = () => {
  if (!state.username) {
    setMsg("请输入账号");
    return;
  }
  if (!state.password) {
    setMsg("请输入密码");
    return;
  }

  post("/auth/login", state).then((resp: LoginResponse) => {
    Promise.all([
      (async () => {
        setLoginResp(resp);
      })(),
      (async () => {
        const to = route.query.to?.toString() || "/";
        router.replace(to);
      })(),
      (async () => {
        setOkMsg("你已成功登录");
      })(),
    ]).then();
  });
};
</script>

<template>
  <div class="my-3 text-xl text-purple-600 text-center">登录</div>
  <Form :no-reset-button="true" :no-submit-button="true">
    <Input label="账号" v-model="state.username" />
    <Input label="密码" type="password" v-model="state.password" />
    <template #buttons>
      <div>
        <Button theme="primary" @click="loginHandler">登录</Button>
      </div>
    </template>
  </Form>
</template>
