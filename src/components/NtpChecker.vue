<template>
  <div class="flex flex-col">
    <div class="flex flex-row items-center">
      <span class="block text-lg font-600">NTP服务器地址:</span>
      <n-input class="ml-1"
               style="width: 200px;"
               type="text"
               placeholder="请输入NTP服务器地址"
               v-model:value="ntpServerAddress"
      />
      <n-button class="ml-1"
                strong
                secondary
                type="primary"
                :disabled="checkBtnDisabled"
                @click="checkBtnClick">测试
      </n-button>
    </div>
    <p class="mt-1"
       :style="{color: checkResult.success ? '#0098ff':'red' }">
      {{ checkResult.value }}
    </p>
  </div>
</template>

<script setup lang="ts">
import {reactive, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
// import {useMessage} from 'naive-ui';

// const message = useMessage();

const ntpServerAddress = ref("ntp.aliyun.com");
const checkResult = reactive({
  success: true,
  value: "",
});

const checkBtnDisabled = ref(false);

const checkBtnClick = () => {
  console.log("ntpServerAddress: ", ntpServerAddress.value);
  checkBtnDisabled.value = true;

  invoke("ntp_check",
      {address: ntpServerAddress.value}
  ).then((res) => {
    let result = res as string;
    console.log(result);
    checkResult.success = true;
    checkResult.value = result;
  }).catch((err) => {
    console.log(err);
    checkResult.success = false;
    checkResult.value = err;
  }).finally(() => {
    checkBtnDisabled.value = false;
  })
}

</script>

<style scoped>

</style>