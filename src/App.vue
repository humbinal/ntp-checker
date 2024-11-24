<template>
  <el-config-provider :locale="zhCn" :button="{autoInsertSpace:true}">
    <div>
      <Header/>
      <div class="p-2">
        <el-card shadow="always">
          <LocalTime></LocalTime>
        </el-card>
        <el-card shadow="always" class="mt-2">
          <NtpChecker></NtpChecker>
        </el-card>
        <el-card shadow="always" class="mt-2">
          <NtpServer></NtpServer>
        </el-card>
      </div>
    </div>
  </el-config-provider>
</template>

<script setup lang="ts">
import zhCn from 'element-plus/es/locale/lang/zh-cn';
import Header from "@/components/Header.vue";
import LocalTime from "./components/LocalTime.vue";
import NtpChecker from "@/components/NtpChecker.vue";
import NtpServer from "@/components/NtpServer.vue";
</script>

<style scoped>

</style>