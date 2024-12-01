<template>
  <div>
    <Header/>
    <div class="p-2">
      <div class="flex flex-row gap-col-2">
        <LocalTime/>
        <NtpServer/>
      </div>
      <div>
        <NtpChecker/>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">

import Header from "@/components/Header.vue";
import LocalTime from "@/components/LocalTime.vue";
import NtpChecker from "@/components/NtpChecker.vue";
import NtpServer from "@/components/NtpServer.vue";
</script>

<style scoped>

</style>