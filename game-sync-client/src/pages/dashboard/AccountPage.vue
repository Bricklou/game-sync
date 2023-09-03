<template>
  <div
    v-if="auth.currentUser && app.server_url"
    class="flex-1 p-8 flex flex-col"
  >
    <h1 class="text-2xl font-semibold mb-2">My Account</h1>

    <div class="flex-1">
      <table
        class="table-fixed border-separate border-spacing-x-4 border-spacing-y-2 text-left"
      >
        <tbody>
          <tr>
            <th>Current instance</th>
            <td>
              <a
                class="text-cyan-600 hover:text-cyan-500"
                target="_blank"
                :href="app.server_url"
                >{{ app.server_url }}</a
              >
            </td>
          </tr>
          <tr>
            <th>Email</th>
            <td>{{ auth.currentUser.email }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="flex flex-row items-center justify-center gap-4">
      <GSButton
        type="button"
        bg-color="bg-transparent-600 hover:enabled:bg-red-600"
        border-color="border border-red-600"
        text-color="text-red-600 hover:enabled:text-white"
        direction="right"
        :icon="LogOut"
        @click="auth.logout()"
      >
        Logout
      </GSButton>

      <GSButton
        type="button"
        bg-color="bg-transparent-600 hover:enabled:bg-red-600"
        border-color="border border-red-600"
        text-color="text-red-600 hover:enabled:text-white"
        :icon="Trash2"
        @click="app.forgotServer()"
      >
        Reset
      </GSButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import GSButton from "@/components/base/GSButton.vue";
import { useAppStore } from "@/store/modules/app";
import { useAuthStore } from "@/store/modules/auth";
import { LogOut, Trash2 } from "lucide-vue-next";

const app = useAppStore();
const auth = useAuthStore();
</script>
