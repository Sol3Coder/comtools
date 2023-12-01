<script setup lang="ts">
const serialNameData = [
  {
    key: "node1",
    title: "Leaf",
  },
];
const baudRateData = [
  {
    key: "node1",
    title: "9600",
  },
  {
    key: "node2",
    title: "38400",
  },
  {
    key: "node3",
    title: "115200",
  },
];
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
</script>

<template>
  <a-grid :cols="2" :colGap="0" :rowGap="16">
    <a-grid-item>
      <div class="container">
        <a-grid :cols="2" :colGap="0" :rowGap="16">
          <a-grid-item>串口号</a-grid-item>
          <a-grid-item>
            <a-tree-select
              :data="baudRateData"
              :label-in-value="true"
              :default-value="{ value: 'node1', label: '9600' }"
              placeholder="Please select ..."
              style="width: 300px"
              @change="onChange"
            ></a-tree-select>
          </a-grid-item>
          <a-grid-item>波特率</a-grid-item>
          <a-grid-item>
            <a-tree-select
              :data="baudRateData"
              :label-in-value="true"
              :default-value="{ value: 'node1', label: '9600' }"
              placeholder="Please select ..."
              style="width: 300px"
              @change="onChange"
            ></a-tree-select
          ></a-grid-item>
          <a-grid-item>数据位</a-grid-item>
          <a-grid-item>
            <a-tree-select
              :data="dataBitsData"
              :label-in-value="true"
              :default-value="{ value: 'node2', label: '8' }"
              placeholder="Please select ..."
              style="width: 300px"
              @change="onChange"
            ></a-tree-select
          ></a-grid-item>
          <a-grid-item>校验位</a-grid-item>
          <a-grid-item>
            <a-tree-select
              :data="parityData"
              :label-in-value="true"
              :default-value="{ value: 'node1', label: 'None' }"
              placeholder="Please select ..."
              style="width: 300px"
              @change="onChange"
            ></a-tree-select
          ></a-grid-item>
          <a-grid-item>停止位</a-grid-item>
          <a-grid-item>
            <a-tree-select
              :data="stopBitsData"
              :label-in-value="true"
              :default-value="{ value: 'node1', label: '1' }"
              placeholder="Please select ..."
              style="width: 300px"
              @change="onChange"
            ></a-tree-select
          ></a-grid-item>
          <a-grid-item :span="2">
            <a-button type="outline" long>连接</a-button>
          </a-grid-item>
        </a-grid>
      </div>
    </a-grid-item>
    <a-grid-item>
      <div>
        <a-textarea placeholder="Please enter something" allow-clear /></div
    ></a-grid-item>
    <a-grid-item>
      <div class="container">
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
      </div>
    </a-grid-item>
    <a-grid-item>
      <div>
        <a-textarea
          placeholder="Please enter something"
          allow-clear
        /></div></a-grid-item
  ></a-grid>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
