<script setup lang="ts" name="VpsInputPage">
import PageTitle from "@/components/PageTitle.vue";
import Input from "@/components/Input.vue";
import Form from "@/components/Form.vue";
</script>

<template>
  <PageTitle>添加VPS</PageTitle>

  <Form>
    <Input label="名称" />
    <Input label="服务商" />
    <Input label="到期时间" />
  </Form>
</template>
