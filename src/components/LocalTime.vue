<template>
  <el-card shadow="always" class="" style="width: 200px;">
    <template #header>
      <div>
        <span class="text font-bold">本机时钟</span>
      </div>
    </template>
    <div class="flex flex-row items-center justify-center h-full">
      <span class="block mt-.5 ml-1 text-xl font-600 = text-#D35CFF">{{ currentTime }}</span>
    </div>
  </el-card>
</template>

<script setup lang="ts">
import {onMounted, onUnmounted, ref} from "vue";

const currentTime = ref(new Date().toLocaleTimeString());

let interval: (number | undefined) = undefined

onMounted(() => {
  interval = window.setInterval(() => {
    currentTime.value = new Date().toLocaleTimeString();
  }, 200);
})

onUnmounted(() => {
  clearInterval(interval);
})

</script>

<style scoped>
::v-deep(.el-card__header) {
  --el-card-padding: 10px;
}

::v-deep(.el-card__body) {
  --el-card-padding: 10px;
}
</style>