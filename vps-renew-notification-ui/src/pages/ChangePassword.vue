<script setup lang="ts" name="ChangePasswordPage">
import PageTitle from "@/components/PageTitle.vue";
import Input from "@/components/Input.vue";
import Form from "@/components/Form.vue";
</script>

<template>
  <PageTitle>修改密码</PageTitle>

  <Form>
    <Input label="新密码" type="password" />
    <Input label="重复密码" type="password" />
    <Input label="当前密码" type="password" />
  </Form>
</template>
