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
                class="px-3 py-2 text-sm font-medium text-white rounded-md hover:bg-gray-800"
              >
                Dashboard
              </router-link>
            </div>
          </div>
        </div>

        <!-- Desktop menu section (right) -->
        <div class="items-center hidden md:flex">
          <div class="flex items-baseline ml-10 space-x-4">
            <div v-if="auth.user" class="text-white relative group">
              <label
                tabindex="0"
                class="px-3 py-4 text-sm font-medium hover:bg-gray-800 rounded-md select-none group-focus-within:bg-gray-800 group-focus-within:rounded-b-none"
              >
                <User
                  class="inline-block mr-1 w-8 h-8 rounded-full bg-gray-600 p-1"
                />
                {{ auth.user.email }}
              </label>
              <ul
                :class="[
                  'absolute invisible flex flex-col flex-wrap p-2 rounded-b-md bg-gray-800 z-[1] w-full top-9 gap-y-1 transform-gpu transition-transform duration-200',
                  'origin-top scale-y-0 group-focus-within:visible group-focus-within:scale-y-100',
                ]"
              >
                <li class="hover:bg-gray-700 rounded-md px-2 py-1">Item</li>
                <li class="hover:bg-gray-700 rounded-md px-2 py-1">Item</li>
                <li class="hover:bg-gray-700 rounded-md px-3 py-2">
                  <button
                    class="appearance-none grid grid-flow-col content-start items-center gap-2 select-none text-sm"
                    @click="auth.logout()"
                  >
                    <LogOut class="inline-block w-4 h-4" />
                    Logout
                  </button>
                </li>
              </ul>
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
import { useAuthStore } from "@/store/modules/auth";
import { RouterLink } from "vue-router";
import { User } from "lucide-vue-next";
import { LogOut } from "lucide-vue-next";

const auth = useAuthStore();
</script>
