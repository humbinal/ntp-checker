<template>
  <button @click="startNtpServer">启动 NTP 服务器</button>
  <button @click="stopNtpServer">停止 NTP 服务器</button>
</template>

<script setup lang="ts">

import {invoke} from "@tauri-apps/api/core";

const startNtpServer = async () => {
  try {
    await invoke('start_ntp_server');
    alert('NTP 服务器已启动');
  } catch (error) {
    console.error('启动失败:', error);
  }
}

const stopNtpServer = async () => {
  try {
    await invoke('stop_ntp_server');
    alert('NTP 服务器已停止');
  } catch (error) {
    console.error('停止失败:', error);
  }
}

</script>

<style scoped>

</style>