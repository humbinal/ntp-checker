<template>
  <el-config-provider :locale="zhCn" :button="{autoInsertSpace:true}">
    <div class="p-2">
      <el-card shadow="always">
        <NtpServer></NtpServer>
      </el-card>
      <el-card shadow="always" class="mt-2">
        <LocalTime></LocalTime>
      </el-card>
      <el-card shadow="always" class="mt-2">
        <NtpChecker></NtpChecker>
      </el-card>
    </div>
  </el-config-provider>
</template>

<script setup lang="ts">
import {ElConfigProvider} from 'element-plus';
import zhCn from 'element-plus/es/locale/lang/zh-cn';
import LocalTime from "./components/LocalTime.vue";
import NtpChecker from "@/components/NtpChecker.vue";
import NtpServer from "@/components/NtpServer.vue";
</script>

<style scoped>

</style>