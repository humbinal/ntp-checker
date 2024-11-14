<template>
  <n-config-provider :locale="zhCN" :date-locale="dateZhCN">
    <n-message-provider>
      <div class="p-2">
        <n-card size="small">
          <NtpServer></NtpServer>
        </n-card>
        <n-card size="small" class="mt-2">
          <LocalTime></LocalTime>
        </n-card>
        <n-card size="small" class="mt-2">
          <NtpChecker></NtpChecker>
        </n-card>
      </div>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import {NConfigProvider} from 'naive-ui';
import {zhCN, dateZhCN} from 'naive-ui';
import LocalTime from "./components/LocalTime.vue";
import NtpChecker from "@/components/NtpChecker.vue";
import NtpServer from "@/components/NtpServer.vue";
</script>

<style scoped>

</style>