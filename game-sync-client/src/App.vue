<template>
  <div v-if="!appStore.loading" class="min-h-screen flex flex-col">
    <router-view v-slot="{ Component }">
      <transition name="fade" mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
  </div>

  <LoadingIndicator v-else class="min-h-screen flex flex-col" />
</template>

<script setup lang="ts">
import LoadingIndicator from "./components/partials/LoadingIndicator.vue";
import { useAppStore } from "@/store/modules/app";

const appStore = useAppStore();
</script>
