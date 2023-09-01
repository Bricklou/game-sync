<template>
  <template v-if="!appStore.loading">
    <main class="flex flex-col flex-1">
      <router-view />
    </main>
  </template>

  <LoadingIndicator v-else />
</template>

<script setup lang="ts">
import LoadingIndicator from "./components/partials/LoadingIndicator.vue";
import { useAppStore } from "@/store/modules/app";

const appStore = useAppStore();
</script>
