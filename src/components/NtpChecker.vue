<template>
  <el-card shadow="always" class="mt-2">
    <template #header>
      <div>
        <span class="text font-bold">NTP地址测试</span>
      </div>
    </template>
    <div class="flex flex-col">
      <div class="flex flex-col">
        <div class="flex flex-row items-center">
          <span class="block text-sm font">快捷输入:</span>
          <el-tooltip v-for="address in commonAddresses" :content="address.comment" placement="top" effect="light">
            <el-tag class="ml-2 cursor-pointer"
                    type="warning"
                    @click="commonAddressesClick(address)">
              {{ address.ip }}
            </el-tag>
          </el-tooltip>
        </div>
        <div class="mt-2 flex flex-row items-center">
          <span class="block text-sm font">服务地址:</span>
          <el-input class="ml-2"
                    style="width: 160px;"
                    type="text"
                    placeholder="请输入NTP服务器地址"
                    v-model="ntpServerAddress.ip"/>
          <span class="block text-sm ml-1 line-height-24px">:</span>
          <el-input-number class="ml-1"
                           style="width: 84px;"
                           :min="1"
                           :max="65535"
                           :step="1"
                           controls-position="right"
                           v-model="ntpServerAddress.port"/>
          <el-button class="ml-2"
                     strong
                     secondary
                     type="primary"
                     :loading="checkBtnDisabled"
                     :disabled="checkBtnDisabled"
                     @click="checkBtnClick">
            测试
          </el-button>
        </div>
      </div>
      <el-alert style="margin-top: 6px;"
                v-if="checkResult.title"
                :type="checkResult.success?'success':'error'"
                :title="checkResult.title"
                :description="checkResult.description"
                show-icon
                :closable="false"/>
    </div>

  </el-card>
</template>

<script setup lang="ts">
import {reactive, ref, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";

type CommonAddressType = {
  ip: string,
  port: number,
  comment: string
}

const commonAddresses: CommonAddressType[] = [
  {
    ip: "ntp.aliyun.com",
    port: 123,
    comment: "阿里云NTP服务器"
  },
  {
    ip: "192.168.1.1",
    port: 123,
    comment: "经典内网服务器"
  },
  {
    ip: "127.0.0.1",
    port: 10123,
    comment: "本地服务器"
  }
]

const ntpServerAddress = reactive({
  ip: commonAddresses[0].ip,
  port: commonAddresses[0].port,
  getAddr(): string {
    return `${this.ip}:${this.port}`
  }
});

const checkResult = reactive({
  success: true,
  title: "",
  description: "",
  onSuccess(ip: string, port: number, date: string): void {
    this.success = true;
    this.title = port === 123 ? `NTP服务器 ${ip} 测试成功` : `NTP服务器 ${ip}:${port} 测试成功`;
    this.description = `NTP服务器返回时间: ${date}`;
  },
  onError(ip: string, port: number, msg: string): void {
    this.success = false;
    this.title = port === 123 ? `NTP服务器 ${ip} 测试失败` : `NTP服务器 ${ip}:${port} 测试失败`;
    this.description = msg;
  },
  clear(): void {
    this.title = ""
  }
});


const commonAddressesClick = (addr: CommonAddressType) => {
  ntpServerAddress.ip = addr.ip;
  ntpServerAddress.port = addr.port;
}

watch(() => ntpServerAddress, () => {
      console.log("ntpServerAddress changed: ", ntpServerAddress.getAddr())
      checkResult.clear()
    },
    {
      deep: true,
      immediate: true
    }
)


const checkBtnDisabled = ref(false);

const checkBtnClick = () => {
  console.log("ntpServerAddress: ", ntpServerAddress.getAddr());
  checkBtnDisabled.value = true;
  checkResult.clear();
  invoke("ntp_check", {
    ip: ntpServerAddress.ip, port: ntpServerAddress.port,
  }).then((res: any) => {
    console.log(res);
    checkResult.onSuccess(res.ip, res.port, res.date);
  }).catch((res: any) => {
    console.log(res);
    checkResult.onError(res.ip, res.port, res.msg);
  }).finally(() => {
    checkBtnDisabled.value = false;
  })
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