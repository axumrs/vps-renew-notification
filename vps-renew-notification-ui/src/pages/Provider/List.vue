<script setup lang="ts" name="ProviderListPage">
import PageTitle from "@/components/PageTitle.vue";
import Button from "@/components/Button.vue";
</script>

<template>
  <PageTitle>服务商列表</PageTitle>

  <section>
    <table>
      <thead>
        <tr class="">
          <th>#</th>
          <th>名称</th>
          <th>续期天数</th>
          <th>通知天数</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>1</td>
          <td>Hax</td>
          <td>5</td>
          <td>2</td>
          <td>
            <div>
              <Button theme="info" size="xs">修改</Button>
              <Button theme="danger" size="xs">删除</Button>
            </div>
          </td>
        </tr>
        <tr>
          <td>2</td>
          <td>VC</td>
          <td>10</td>
          <td>1</td>
          <td>
            <div>
              <Button theme="info" size="xs">修改</Button>
              <Button theme="danger" size="xs">删除</Button>
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </section>
</template>
