<template>
  <el-config-provider :locale="zhCn" :button="{autoInsertSpace:true}">
    <LayoutIndex/>
  </el-config-provider>
</template>

<script setup lang="ts">
import zhCn from 'element-plus/es/locale/lang/zh-cn';
import LayoutIndex from "@/layout/LayoutIndex.vue";

</script>

<style scoped>

</style>