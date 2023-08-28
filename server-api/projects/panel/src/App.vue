<template>
  <ProgressBar />

  <template v-if="!appStore.loading">
    <NavBar v-if="!route.meta.hideNavbar" />

    <main class="flex flex-col flex-1">
      <router-view />
    </main>
  </template>

  <LoadingIndicator v-else />
</template>

<script setup lang="ts">
import { RouterView, useRoute } from "vue-router";
import NavBar from "./components/partials/NavBar.vue";
import { useAppStore } from "./store/modules/app";
import LoadingIndicator from "./components/partials/LoadingIndicator.vue";
import ProgressBar from "./components/partials/ProgressBar.vue";

const appStore = useAppStore();
const route = useRoute();
</script>
