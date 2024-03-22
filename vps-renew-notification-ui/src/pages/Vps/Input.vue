<script setup lang="ts" name="VpsInputPage">
import { useRoute } from "vue-router";
import PageTitle from "@/components/PageTitle.vue";
import Input from "@/components/Input.vue";
import Form from "@/components/Form.vue";

const r = useRoute();
const { id } = r.params;

const title = id ? "修改" : "添加";
</script>

<template>
  <PageTitle>{{ title }}VPS</PageTitle>

  <Form>
    <Input label="名称" />
    <Input label="服务商" />
    <Input label="到期时间" />
  </Form>
</template>
