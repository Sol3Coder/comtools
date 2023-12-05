<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";

const dataBitsData = [
  {
    key: "node1",
    title: "7",
  },
  {
    key: "node2",
    title: "8",
  },
];
const parityData = [
  {
    key: "node1",
    title: "None",
  },
  {
    key: "node2",
    title: "Odd",
  },
  {
    key: "node3",
    title: "Even",
  },
  {
    key: "node4",
    title: "Mark",
  },
  {
    key: "node5",
    title: "Space",
  },
];
const stopBitsData = [
  {
    key: "node1",
    title: "1",
  },
  {
    key: "node2",
    title: "1.5",
  },
  {
    key: "node3",
    title: "2",
  },
];

const serialNameData = getJsonObject();

function getJsonObject(): any {
  var ports_json = [];

  invoke("get_ports").then((ports) => {
    var nIndex = 1;
    ports.forEach((port) => {
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
              <a-button type="outline" long>连接</a-button>
            </a-grid-item>
          </a-grid>
        </div>
      </a-layout-sider>
      <a-layout-sider style="width: 70%">
        <div class="container" style="padding-top: 5%">
          <a-textarea
            style="height: 20rem"
            placeholder="Please enter something"
            allow-clear
          />
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
