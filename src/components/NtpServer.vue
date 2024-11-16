<template>
  <el-button @click="startNtpServer">启动 NTP 服务器</el-button>
  <el-button @click="stopNtpServer">停止 NTP 服务器</el-button>
</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {ElMessage} from "element-plus";

const startNtpServer = async () => {
  try {
    await invoke('start_ntp_server');
    ElMessage.success("启动成功");
  } catch (error) {
    console.error('启动失败:', error);
    ElMessage.error("启动失败");
  }
}

const stopNtpServer = async () => {
  try {
    await invoke('stop_ntp_server');
    ElMessage.success("NTP服务器已停止");
  } catch (error) {
    console.error('停止失败:', error);
    ElMessage.error("NTP服务器停止失败");
  }
}

</script>

<style scoped>

</style>