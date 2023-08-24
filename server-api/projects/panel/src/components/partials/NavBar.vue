<template>
  <nav class="bg-gray-900">
    <div class="px-8 mx-auto max-w-7xl">
      <div class="flex items-center justify-between h-16">
        <div class="flex items-center">
          <router-link to="/">
            <span class="text-white">
              Game-Sync
              <span class="font-extrabold text-cyan-600">Admin</span>
            </span>
          </router-link>

          <div class="hidden md:block">
            <div class="flex items-baseline ml-10 space-x-4">
              <router-link
                to="/"
                class="px-3 py-2 text-sm font-medium text-white rounded-md hover:bg-gray-700"
              >
                Dashboard
              </router-link>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Mobile menu section -->
    <div class="md:hidden">
      <div class="px-2 pt-2 pb-3 space-y-1 sm:px-3">
        <router-link
          to="/"
          class="text-gray-100 hover:text-cyan-600 block px-3 py-2 rounded-md text-base font-medium"
        >
          Dashboard
        </router-link>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { RouterLink } from "vue-router";
</script>
