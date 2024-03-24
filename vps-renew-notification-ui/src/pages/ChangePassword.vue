<script setup lang="ts" name="ChangePasswordPage">
import PageTitle from "@/components/PageTitle.vue";
import Input from "@/components/Input.vue";
import Form from "@/components/Form.vue";
import useFetch from "@/hooks/useFetch";
import { useStatusStore } from "@/store/status";
import { reactive } from "vue";
import { useAuthStore } from "@/store/auth";

const { put } = useFetch();
const { setOkMsg } = useStatusStore();
const { getUser } = useAuthStore();
const state = reactive({
  password: "",
  new_password: "",
  re_password: "",
  id: getUser().id,
});

const submitHandler = () => {
  put(`/user/change-password`, state).then(() => {
    setOkMsg("修改成功");
  });
};
</script>

<template>
  <PageTitle>修改密码</PageTitle>

  <Form @submit="submitHandler">
    <Input label="新密码" type="password" v-model="state.password" required />
    <Input
      label="重复密码"
      type="password"
      v-model="state.new_password"
      required
    />
    <Input
      label="当前密码"
      type="password"
      v-model="state.re_password"
      required
    />
  </Form>
</template>
