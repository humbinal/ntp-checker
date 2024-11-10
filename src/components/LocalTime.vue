<template>
  <div class="flex flex-row items-center">
    <span class="block text-lg font-600">本地时间:</span>
    <span class="block mt-.5 ml-1 text-lg font-600 = text-#D35CFF">{{ currentTime }}</span>
  </div>
</template>

<script setup lang="ts">
import {onMounted, onUnmounted, ref} from "vue";

const currentTime = ref(new Date().toLocaleTimeString())

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

</style>