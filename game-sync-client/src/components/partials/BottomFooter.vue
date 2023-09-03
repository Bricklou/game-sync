<template>
  <footer class="bg-gray-900 text-white px-4">
    <ul class="flex flex-row items-center justify-center">
      <li class="flex-1">
        <router-link
          to="/dashboard/home"
          title="Home"
          active-class="text-cyan-600"
        >
          <HomeIcon class="w-6 h-6 mx-auto" />
        </router-link>
      </li>
      <li class="flex-1">
        <router-link
          to="/dashboard/games"
          title="Games"
          active-class="text-cyan-600"
        >
          <Gamepad2 class="w-6 h-6 mx-auto" />
        </router-link>
      </li>
      <li class="flex-1">
        <router-link
          to="/dashboard/account"
          title="Account"
          active-class="text-cyan-600"
        >
          <UserIcon class="w-6 h-6 mx-auto" />
        </router-link>
      </li>
    </ul>
  </footer>
</template>

<script setup lang="ts">
import { Gamepad2 } from "lucide-vue-next";
import { HomeIcon, UserIcon } from "lucide-vue-next";
</script>

<style scoped>
li {
  @apply hover:text-cyan-500 block w-full h-full transition-colors duration-100 ease-in-out;

  a {
    @apply py-4 block;
  }
}
</style>
