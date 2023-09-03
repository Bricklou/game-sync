<template>
  <div class="min-h-screen flex flex-col">
    <NavBar v-if="!currentRoute.meta.fullScreen" />
    <main
      class="flex flex-col flex-1 bg-gray-50 text-gray-900 overflow-hidden relative"
    >
      <router-view v-slot="{ Component, route }">
        <transition :name="route.meta.transition ?? 'fade'">
          <component :is="Component" :key="route.path" />
        </transition>
      </router-view>
    </main>
    <BottomFooter v-if="!currentRoute.meta.fullScreen" />
  </div>
</template>

<script setup lang="ts">
import { useRoute } from "vue-router";
import NavBar from "./NavBar.vue";
import BottomFooter from "./BottomFooter.vue";

const currentRoute = useRoute();
</script>
