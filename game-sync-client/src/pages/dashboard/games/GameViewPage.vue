<template>
  <div v-if="game" class="flex-1 py-8 flex flex-col">
    <header class="flex flex-row justify-start items-center mx-8 mb-2">
      <h1 class="text-2xl font-semibold">{{ game.name }}</h1>
    </header>
  </div>

  <div v-else class="flex-1 flex flex-col py-8 px-4">
    <header>
      <router-link
        to="/dashboard/games"
        :class="[
          'inline-flex flex-row items-center justify-start gap-2 mb-2',
          'rounded-md px-4 py-2 text-gray-800 hover:text-cyan-600 hover:bg-gray-200',
          'transition-all duration-100 ease-in-out',
        ]"
      >
        <ArrowLeft class="w-6 h-6" />
        <span>Back</span>
      </router-link>
    </header>
    <div class="flex-1 flex flex-col items-center justify-center">
      <LoadingSpinner />
    </div>
  </div>
</template>

<script setup lang="ts">
import LoadingSpinner from "@/components/base/LoadingSpinner.vue";
import { ArrowLeft } from "lucide-vue-next";
import { Game } from "@/types/game";
import { onBeforeMount, ref } from "vue";
import { useRoute } from "vue-router";
import { HttpError } from "@/types/http_errors";
import { getGame } from "@/api/game";

const route = useRoute();

const game = ref<Game | null>(null);
const error = ref<string | null>(null);

async function fetchGame() {
  try {
    const gameId = Number.parseInt(route.params.id as string);
    game.value = await getGame(gameId);
    console.log(game.value);
  } catch (e) {
    if (e instanceof HttpError) {
      error.value = e.message;
    } else {
      error.value = "An unknown error occurred";
    }
  }
}

onBeforeMount(() => {
  fetchGame();
});
</script>
