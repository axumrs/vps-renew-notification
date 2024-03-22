<script setup lang="ts" name="Login">
import Input from "@/components/Input.vue";
import Form from "@/components/Form.vue";
import Button from "@/components/Button.vue";
</script>

<template>
  <div class="my-3 text-xl text-purple-600 text-center">登录</div>
  <Form :no-reset-button="true" :no-submit-button="true">
    <Input label="账号" required />
    <Input label="密码" type="password" required />
    <template #buttons>
      <div>
        <Button theme="primary">登录</Button>
      </div>
    </template>
  </Form>
</template>
