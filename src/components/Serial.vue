<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";
import { h } from "vue";
import { Modal } from "@arco-design/web-vue";
const inputText = ref(" ");
const scrollbarRef = ref();
const readData = ref("");
var nHeight = 0;
async function listen_to_event() {
  await listen("readMsgEvent", (event: any) => {
    readData.value += event.payload + " ";
    nHeight = nHeight + 20;
    scrollbarRef.value.scrollTop(nHeight);
  });
}
onMounted(() => {
  listen_to_event();
});
const ModalContent = {
  setup() {
    return () =>
      h("div", { class: "warning-modal-content" }, [
        h(
          "span",
          { style: "margin-bottom: 10px;" },
          "打开串口失败！请检查是否占用！"
        ),
      ]);
  },
};
const serialName = ref({});
const serialNameData = getJsonObject();
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
      serialName.value = jsonObject;
      ports_json.push(jsonObject);
      nIndex++;
    });
  });
  return ports_json;
}
var isOpen = false;
const buttonText = ref("连接");
function serialSwitch() {
  if (isOpen) {
    buttonText.value = "连接";
    invoke("close");
    isOpen = false;
  } else {
    invoke("open", { name: serialName.value["label"] }).then((bSuc) => {
      if (bSuc) {
        buttonText.value = "断开";
        isOpen = true;
      } else {
        Modal.warning({
          title: "错误",
          content: () => h(ModalContent),
        });
      }
    });
  }
}
function sendMsg() {
  invoke("send", { msg: inputText.value });
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
              <a-select :default-value="' '" v-model="serialName">
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
          <a-scrollbar
            ref="scrollbarRef"
            style="height: 300px; overflow: auto; text-align: left"
          >
            <p v-text="readData"></p>
          </a-scrollbar>
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
          <a-textarea v-model="inputText" style="height: 10rem" allow-clear />
          <a-button @click="sendMsg" type="outline">发送</a-button>
        </div></a-layout-sider
      >
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
