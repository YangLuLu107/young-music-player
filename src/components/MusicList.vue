<template>
  <n-split direction="horizontal" style="height: 100%" :max="0.75" :min="0.25">
    <template #1>
      <PlayList />
    </template>
    <template #2>
      Pane 2
    </template>
  </n-split>

</template>
<script setup lang="ts">
  import PlayList from "./PlayList.vue";
  import { NSplit } from 'naive-ui'

</script>
<style scoped>
  .main {
    width: 100%;
    height: 100%;
    position: absolute;
    left: 0;
    top: 0;
  }
  .play-list {
    width: 100px;
  }
  :deep(.n-split-pane-1) {
    position: relative;
  }
  :deep(.n-split-pane-2)  {
    position: relative;
  }
</style>