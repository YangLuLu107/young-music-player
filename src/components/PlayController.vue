<template>
  <div id="play-controller">
    <n-flex vertical>
      <n-slider :value="sliderValue"
                :max="maxDuration"
                :min="minDuration"
                :default-value="0"
                :step="1"
                :keyboard="false"
                :tooltip="false" />
      <n-flex class="controller" justify="center">
        <n-flex class="cover">
          <n-image
              width="50" height="40"
              src="images/cover.png"
          />
        </n-flex>
        <n-flex>
          <n-image
              width="40" height="30"
              src="images/prev-song.png"
          />
          <n-image
              width="50" height="40"
              src="images/play.png"
          />
          <n-image
              width="40" height="30"
              src="images/next-song.png"
          />
        </n-flex>
        <n-flex class="tools">
          <n-image
              width="60" height="40"
              src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
          />
          <n-image
              width="60" height="40"
              src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
          />
          <n-image
              width="60" height="40"
              src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
          />
        </n-flex>
      </n-flex>
    </n-flex>
  </div>
</template>
<script setup lang="ts">
  import { ref } from 'vue'
  import { NSlider  } from 'naive-ui'


  const sliderValue = ref(0);
  const minDuration = ref(0);
  const maxDuration = ref(400);

</script>
<style scoped>
  .controller {
    position: relative;
  }
  .cover {
    position: absolute;
    left: 32px;
  }
  .tools {
    position: absolute;
    right: 32px;
  }
</style>