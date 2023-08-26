<template>
  <div v-if="!appStore.loading" class="flex flex-col">
    <NavBar v-if="!route.meta.hideNavbar" />

    <main class="flex-1">
      <router-view />
    </main>
  </div>
  <LoadingIndicator v-else />
</template>

<script setup lang="ts">
import { onBeforeMount } from "vue";
import { RouterView, useRoute } from "vue-router";
import NavBar from "./components/partials/NavBar.vue";
import { useAppStore } from "./store/modules/app";
import LoadingIndicator from "./components/partials/LoadingIndicator.vue";
import router from "./router";

const appStore = useAppStore();
const route = useRoute();

onBeforeMount(async () => {
  appStore.setLoading(true);

  await appStore.fetchAppData();

  if (appStore.configured) {
    router.push("/register");
  }

  appStore.setLoading(false);
});
</script>
