<template>
  <nav class="bg-gray-900">
    <div class="px-8 mx-auto max-w-7xl">
      <div class="flex items-center justify-between h-16">
        <!-- Desktop menu section (left) -->
        <div class="flex items-center">
          <router-link to="/">
            <span class="text-white">
              <span class="font-extrabold">Game-Sync </span>
              <span class="text-cyan-600">Admin</span>
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

        <!-- Desktop menu section (right) -->
        <div class="items-center hidden md:flex">
          <div class="flex items-baseline ml-10 space-x-4">
            <span
              v-if="auth.user"
              class="px-3 py-2 text-sm font-medium text-white"
            >
              <User
                class="inline-block mr-1 w-8 h-8 rounded-full bg-gray-600 p-1"
              />
              {{ auth.user.email }}
            </span>
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
import { useAuthStore } from "@/store/modules/auth";
import { RouterLink } from "vue-router";
import { User } from "lucide-vue-next";

const auth = useAuthStore();
</script>
