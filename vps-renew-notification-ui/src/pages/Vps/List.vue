<script setup lang="ts" name="VpsListPage">
import PageTitle from "@/components/PageTitle.vue";
import Button from "@/components/Button.vue";
</script>

<template>
  <PageTitle>VPS列表</PageTitle>

  <section>
    <table>
      <thead>
        <tr class="">
          <th>#</th>
          <th>名称</th>
          <th>服务商</th>
          <th>到期时间</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>1</td>
          <td>Hax-1</td>
          <td>Hax</td>
          <td>2024/03/24</td>
          <td>
            <div>
              <Button theme="primary" size="xs">续期</Button>
              <Button theme="info" size="xs" href="/vps/edit/1">修改</Button>
              <Button theme="danger" size="xs">删除</Button>
            </div>
          </td>
        </tr>
        <tr>
          <td>2</td>
          <td>VC-1</td>
          <td>VC</td>
          <td>2024/03/24</td>
          <td>
            <div>
              <Button theme="primary" size="xs">续期</Button>
              <Button theme="info" size="xs">修改</Button>
              <Button theme="danger" size="xs">删除</Button>
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </section>
</template>
