<template>
  <div
    v-if="progressStore.isVisible"
    class="z-50 fixed top-0 left-0 w-full h-1 bg-gray-900"
  >
    <div
      class="h-full bg-cyan-500 transition-all opacity-0"
      :style="{
        width: `${progressStore.progress}%`,
        opacity: progressStore.isVisible ? 1 : 0,
      }"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { useProgressStore } from "@/store/modules/progress";

const progressStore = useProgressStore();
</script>
