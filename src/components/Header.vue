<template>
  <div data-tauri-drag-region class="h-40px flex flex-row items-center justify-between select-none" style="box-shadow: rgba(0, 0, 0, 0.08) 0 2px 3px 0;">
    <div class="flex items-center">
      <img :src="logoIcon" alt="" class="ml-2 h-18px">
      <span class="ml-2 block text-gray-9 dark:text-white text-sm font-600" style="font-family: var(--el-font-family);">NTP工具箱</span>
    </div>
    <div class="flex items-center">
      <el-button class="mr-6"
                 link
                 size="large"
                 :icon="isDark?Sunny:Moon"
                 @click="toggleDark()"/>
      <div class="mr-4 flex items-center justify-center h-23px w-23px hover:bg-gray-1 dark:hover:bg-gray-7 rounded"
           @click="minimizeBtnClick">
        <svg-icon name="minimize" :color="windowIconColor" class="h-11px w-11px"></svg-icon>
      </div>
      <div class="mr-4 flex items-center justify-center h-23px w-23px hover:bg-gray-1 dark:hover:bg-gray-7 rounded"
           @click="maximumAndRestoreBtnClick">
        <svg-icon v-if="!isMaximized" name="maximize" :color="windowIconColor" class="h-11px w-11px mr-1px"></svg-icon>
        <svg-icon v-else name="restore" :color="windowIconColor" class="h-11px w-11px"></svg-icon>
      </div>
      <div class="mr-2 flex items-center justify-center h-23px w-23px hover:bg-red-5 dark:hover:bg-gray-7 rounded"
           @click="closeBtnClick">
        <svg-icon name="close" :color="windowIconColor" class="h-15px w-15px mr-2px"></svg-icon>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref} from "vue";
import {Moon, Sunny} from "@element-plus/icons-vue";
import {useDark, useToggle} from "@vueuse/core";
import {getCurrentWindow} from "@tauri-apps/api/window";
import SvgIcon from "./SvgIcon.vue";

import logoIcon from "@/assets/icons/logo.png";

const isDark = useDark()
const toggleDark = useToggle(isDark)
const isMaximized = ref(false);

const windowIconColor = computed(() => isDark.value ? "#ffffff" : "#222222");

const minimizeBtnClick = async () => {
  await getCurrentWindow().minimize();
}

const maximumAndRestoreBtnClick = async () => {
  isMaximized.value = await getCurrentWindow().isMaximized();
  if (isMaximized.value) {
    await getCurrentWindow().unmaximize();
    isMaximized.value = false;
  } else {
    await getCurrentWindow().maximize();
    isMaximized.value = true;
  }
}

const closeBtnClick = async () => {
  await getCurrentWindow().close();
}

</script>

<style scoped>

</style>