<template>
  <nav class="bg-gray-900">
    <div class="px-4 mx-auto max-w-7xl">
      <div class="flex items-center justify-between h-16">
        <!-- Desktop menu section (left) -->
        <div class="flex items-center">
          <router-link to="/dashboard/home">
            <span class="text-white">
              <span class="font-extrabold">Game-Sync </span>
              <span class="text-cyan-600">Admin</span>
            </span>
          </router-link>

          <div class="hidden md:block">
            <div class="flex items-baseline ml-10 space-x-4">
              <router-link
                to="/dashboard/home"
                class="px-3 py-2 text-sm font-medium text-white rounded-md hover:bg-gray-800"
              >
                Dashboard
              </router-link>
            </div>
          </div>
        </div>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { RouterLink } from "vue-router";
</script>
