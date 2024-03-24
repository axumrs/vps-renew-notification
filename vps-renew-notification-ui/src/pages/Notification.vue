<script setup lang="ts" name="NotificationPage">
import PageTitle from "@/components/PageTitle.vue";
import Input from "@/components/Input.vue";
import Form from "@/components/Form.vue";
import { reactive } from "vue";
import useFetch from "@/hooks/useFetch";
import { useStatusStore } from "@/store/status";

const msg = reactive({ text: "" });

const { post } = useFetch();
const { setOkMsg } = useStatusStore();

const submitHanlder = () => {
  post("/bot/send-message", msg).then(() => {
    setOkMsg("消息发送成功");
  });
};
</script>

<template>
  <PageTitle>通知设置</PageTitle>

  <Form @submit="submitHanlder">
    <Input label="消息内容" v-model="msg.text" required />
  </Form>
</template>
