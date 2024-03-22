<script setup lang="ts" name="NotificationPage">
import PageTitle from "@/components/PageTitle.vue";
import Input from "@/components/Input.vue";
import Form from "@/components/Form.vue";
import Button from "@/components/Button.vue";

const testHandler = () => {
  window.confirm("我们将发送一条测试信息到TG机器人，是否继续？");
};
</script>

<template>
  <PageTitle>通知设置</PageTitle>

  <Form>
    <div class="flex justify-start items-end gap-x-2">
      <div class="grow">
        <Input label="TG机器人Token" />
      </div>
      <div class="shrink-0 relative">
        <Button theme="info" @click="testHandler">测试</Button>
      </div>
    </div>
  </Form>
</template>
