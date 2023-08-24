<template>
  <div class="flex flex-col">
    <NavBar />

    <main class="flex-1">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { RouterView } from "vue-router";
import NavBar from "./components/partials/NavBar.vue";
</script>
