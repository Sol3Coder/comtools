<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

const serialNameData = getJsonObject();
var isOpen = false;
var buttonText = ref("连接");
function serialSwitch() {
  if (isOpen) {
    buttonText.value = "连接";
    invoke("close");

    isOpen = false;
  } else {
    buttonText.value = "断开";
    invoke("open", { name: "COM1" });

    isOpen = true;
  }
}

function getJsonObject(): any {
  var ports_json: { value: string; label: any; other: string }[] = [];

  invoke("get_ports").then((ports) => {
    var nIndex = 1;
    ports.forEach((port: any) => {
      let jsonObject = {
        value: "node" + nIndex,

        label: port,
        other: "extra",
      };

      ports_json.push(jsonObject);
      nIndex++;
    });
  });

  return ports_json;
}
</script>

<template>
  <a-layout>
    <a-layout tyle="height: 60%">
      <a-layout-sider style="width: 30%">
        <div class="container" style="padding-top: 5%">
          <a-grid :cols="2" :colGap="0" :rowGap="16">
            <a-grid-item>串口号</a-grid-item>
            <a-grid-item>
              <a-select :default-value="' '">
                <a-option
                  v-for="item of serialNameData"
                  :value="item"
                  :label="item.label"
                />
              </a-select>
            </a-grid-item>
            <a-grid-item>波特率</a-grid-item>
            <a-grid-item>
              <a-select :default-value="'9600'">
                <a-option>9600</a-option>
                <a-option>38400</a-option>
                <a-option>115200</a-option>
              </a-select>
            </a-grid-item>
            <a-grid-item>数据位</a-grid-item>
            <a-grid-item>
              <a-select :default-value="'8'">
                <a-option>7</a-option>
                <a-option>8</a-option>
              </a-select>
            </a-grid-item>
            <a-grid-item>校验位</a-grid-item>
            <a-grid-item>
              <a-select :default-value="'None'">
                <a-option>None</a-option>
                <a-option>Even</a-option>
                <a-option>Odd</a-option>
                <a-option>Mark</a-option>
                <a-option>Space</a-option>
              </a-select>
            </a-grid-item>
            <a-grid-item>停止位</a-grid-item>
            <a-grid-item>
              <a-select :default-value="'1'">
                <a-option>1</a-option>
                <a-option>1.5</a-option>
                <a-option>2</a-option>
              </a-select>
            </a-grid-item>
            <a-grid-item :span="2">
              <a-button
                @click="serialSwitch"
                type="outline"
                id="open-btn"
                long
                >{{ buttonText }}</a-button
              >
            </a-grid-item>
          </a-grid>
        </div>
      </a-layout-sider>
      <a-layout-sider style="width: 70%">
        <div class="container" style="padding-top: 5%">
          <a-space>
            <a-descriptions :data="serialNameData" />
          </a-space>
        </div>
      </a-layout-sider>
    </a-layout>

    <a-layout tyle="height: 40%">
      <a-layout-sider style="width: 30%">
        <div class="container" style="padding-top: 5%">
          <a-grid :cols="2" :colGap="0" :rowGap="16">
            <a-grid-item>接收字节:</a-grid-item>
            <a-grid-item>0B</a-grid-item>
            <a-grid-item>发送字节:</a-grid-item>
            <a-grid-item>0B</a-grid-item>
            <a-grid-item>接收速度:</a-grid-item>
            <a-grid-item>0.00B/S</a-grid-item>
            <a-grid-item>发送速度:</a-grid-item>
            <a-grid-item>0.00B/S</a-grid-item>
          </a-grid>
        </div></a-layout-sider
      >
      <a-layout-sider style="width: 70%">
        <div class="container" style="padding-top: 5%">
          <a-textarea
            style="height: 10rem"
            placeholder="Please enter something"
            allow-clear
          /></div
      ></a-layout-sider>
    </a-layout>
  </a-layout>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
