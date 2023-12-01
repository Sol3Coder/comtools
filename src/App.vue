<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
</script>

<template>
  <div class="container">
    <a-layout>
      <div>
        串口号
        <a-dropdown @select="handleSelect">
          <a-button>Click Me</a-button>
          <template #content>
            <a-doption>Option 1</a-doption>
            <a-doption disabled>Option 2</a-doption>
            <a-doption :value="{ value: 'Option3' }">Option 3</a-doption>
          </template>
        </a-dropdown>
      </div>
      <div>
        波特率
        <a-dropdown @select="handleSelect">
          <a-button>Click Me</a-button>
          <template #content>
            <a-doption>Option 1</a-doption>
            <a-doption disabled>Option 2</a-doption>
            <a-doption :value="{ value: 'Option3' }">Option 3</a-doption>
          </template>
        </a-dropdown>
      </div>
    </a-layout>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
