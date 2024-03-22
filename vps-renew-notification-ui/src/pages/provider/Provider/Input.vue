<script setup lang="ts" name="ProviderInputPage">
import PageTitle from "@/components/PageTitle.vue";
import Input from "@/components/Input.vue";

import Form from "@/components/Form.vue";
</script>

<template>
  <PageTitle>添加服务商</PageTitle>

  <Form>
    <Input label="名称" />
    <Input label="续期天数" />
    <Input label="通知天数" />
  </Form>
</template>
