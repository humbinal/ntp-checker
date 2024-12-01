<template>
  <el-card shadow="always" class="flex-1">
    <template #header>
      <div>
        <span class="text font-bold">本地NTP服务器</span>
      </div>
    </template>
    <div class="flex flex-row items-center">

      <div class="mt-2 flex flex-row items-center">
        <span class="block text-sm font">绑定端口:</span>
        <el-input-number class="ml-2"
                         style="width: 84px;"
                         size="small"
                         :min="1"
                         :max="65535"
                         :step="1"
                         controls-position="right"
                         v-model="bindPort"/>

        <el-tooltip :content="serverRunning?'停止':'启动'" placement="bottom" effect="light">
          <el-button class="ml-4"
                     size="small"
                     style="padding: 2px 2px !important;"
                     :disabled="serverBtnDisabled">
            <svg-icon v-if="!serverRunning"
                      name="play"
                      color="#ca3ef6"
                      class="h-24px w-24px"
                      @click="startBtnClick"/>
            <svg-icon v-else
                      name="stop"
                      color="#ca3ef6"
                      class="h-24px w-24px"
                      @click="stopBtnClick"/>
          </el-button>
        </el-tooltip>
      </div>
    </div>
  </el-card>
</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {ElMessage, ElNotification} from "element-plus";
import {onBeforeMount, ref} from "vue";
import SvgIcon from "./SvgIcon.vue";

const bindPort = ref<number>(10123);
const serverRunning = ref<boolean>(false);

const serverBtnDisabled = ref(false);

onBeforeMount(async () => {
  try {
    let running: boolean = await invoke('get_ntp_server_state');
    serverRunning.value = running;
  } catch (error) {
    console.error('获取失败:', error);
    serverRunning.value = false;
  }
})

const startBtnClick = async () => {
  serverBtnDisabled.value = true;
  try {
    await invoke('start_ntp_server', {port: bindPort.value});
    ElMessage.success("启动成功");
    serverRunning.value = true;
  } catch (error) {
    console.error('启动失败:', error);
    ElMessage.error("启动失败");
    serverRunning.value = false;
  } finally {
    serverBtnDisabled.value = false;
  }
}

const stopBtnClick = async () => {
  serverBtnDisabled.value = true;
  try {
    await invoke('stop_ntp_server');
    ElMessage.success("NTP服务器已停止");
    serverRunning.value = false;
  } catch (error) {
    console.error('停止失败:', error);
    ElMessage.error("NTP服务器停止失败");
    serverRunning.value = true;
  } finally {
    serverBtnDisabled.value = false;
  }
}

</script>

<style scoped>
::v-deep(.el-card__header) {
  --el-card-padding: 10px;
}

::v-deep(.el-card__body) {
  --el-card-padding: 10px;
}

::v-deep(.el-input-number).is-controls-right .el-input__wrapper {
  padding-left: 0;
  padding-right: 36px;
}
</style>