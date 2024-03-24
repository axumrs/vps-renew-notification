<script setup lang="ts" name="Login">
import Input from "@/components/Input.vue";
import Form from "@/components/Form.vue";
import Button from "@/components/Button.vue";

import { useStatusStore } from "@/store/status";
import { useAuthStore } from "@/store/auth";

import useFetch from "@/hooks/useFetch";

import { reactive } from "vue";

const state = reactive({ username: "", password: "" });
const { setMsg } = useStatusStore();
const { setLoginResp } = useAuthStore();
const { post } = useFetch();

const emit = defineEmits(["callback"]);

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
    setLoginResp(resp);
    emit("callback");
  });

  // fetch(`${import.meta.env.VITE_API_URL}/auth/login`, {
  //   method: "POST",
  //   body: JSON.stringify(state),
  //   headers: {
  //     "content-type": "application/json",
  //   },
  // })
  //   .then((resp) => resp.json())
  //   .then((resp: ApiResponse<LoginResponse>) => {
  //     if (resp.code !== 0) {
  //       setMsg(resp.msg);
  //       return;
  //     }
  //     setLoginResp(resp.data!);
  //     emit("callback");
  //   })
  //   .catch((e) => {
  //     setMsg("请检查网络");
  //     console.log(e);
  //   })
  //   .finally(() => {
  //     loading.value = false;
  //   });
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
