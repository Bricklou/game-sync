<template>
  <div class="flex-1 py-8 flex flex-col">
    <header class="flex flex-row justify-between items-center mx-8 mb-2">
      <h1 class="text-2xl font-semibold">Games</h1>

      <GSButton
        :icon="PlusIcon"
        @click.prevent="router.push('/dashboard/games/new')"
      >
        Add a game
      </GSButton>
    </header>
  </div>
</template>

<script setup lang="ts">
import GSButton from "@/components/base/GSButton.vue";
import { PlusIcon } from "lucide-vue-next";
import { useRouter } from "vue-router";

const router = useRouter();
</script>
