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

    <main class="flex-1 flex flex-col">
      <!-- Games list -->
      <ul
        v-if="!loading && data?.length"
        class="flex-auto overflow-y-auto h-96 scroll-smooth scroll-m-4"
      >
        <li v-for="game in data" :key="game.id">
          <router-link :to="`/dashboard/games/${game.id}`">
            <div
              :class="[
                'flex flex-row items-center justify-between px-8 py-4 hover:bg-cyan-500 hover:bg-opacity-20 rounded-lg mx-4 my-2',
                'transition-colors duration-100 ease-in-out',
              ]"
            >
              <div class="flex flex-col">
                <span class="text-lg font-semibold">{{ game.name }}</span>
                <span class="text-sm text-gray-700 truncate">{{
                  game.description
                }}</span>
              </div>
            </div>
          </router-link>
        </li>
      </ul>

      <!-- Error state -->
      <div
        v-else-if="error"
        class="flex flex-1 flex-col items-center justify-center"
      >
        <div class="mb-4 glitched">
          <ServerCrash class="text-gray-700" />
          <ServerCrash class="text-gray-700" />
          <ServerCrash class="text-gray-700" />
        </div>
        <span class="">{{ error }}</span>
      </div>

      <!-- Empty state -->
      <div
        v-else-if="!loading && !data?.length"
        class="flex flex-1 flex-col items-center justify-center"
      >
        <div class="mb-4 glitched">
          <Unplug class="text-gray-700" />
          <Unplug class="text-gray-700" />
          <Unplug class="text-gray-700" />
        </div>
        <span class="text-gray-700">No games found</span>
      </div>

      <!-- Loading indicator -->
      <div
        v-else-if="loading"
        class="flex flex-1 flex-col items-center justify-center"
      >
        <LoadingSpinner />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import GSButton from "@/components/base/GSButton.vue";
import LoadingSpinner from "@/components/base/LoadingSpinner.vue";
import { PlusIcon, ServerCrash } from "lucide-vue-next";
import { useRouter } from "vue-router";
import { getGames } from "@/api/game";
import { onBeforeMount, ref } from "vue";
import { Game } from "@/types/game";
import { HttpError } from "@/types/http_errors";
import { Unplug } from "lucide-vue-next";

const router = useRouter();

const loading = ref(true);
const data = ref<Game[]>([]);
const meta = ref({} as any);
const error = ref<string | null>(null);

async function fetchGames() {
  loading.value = true;
  try {
    const games = await getGames();
    data.value = games.data;
    meta.value = games.meta;

    console.log(games);
  } catch (e) {
    if (e instanceof HttpError) {
      error.value = e.message;
    } else {
      error.value = "An unknown error occurred";
    }
  } finally {
    loading.value = false;
  }
}

onBeforeMount(() => {
  fetchGames();
});
</script>

<style scoped>
.glitched {
  @apply relative w-20 h-20;

  > svg {
    @apply absolute top-0 left-0 w-full h-full antialiased;

    &:nth-child(2),
    &:nth-child(3) {
      clip: rect(0 0 0 0);
    }

    &:nth-child(2) {
      @apply -left-[2px] bg-red-50 stroke-red-950;

      animation: glitch 8s infinite linear alternate-reverse;
    }
    &:nth-child(3) {
      @apply -left-[1px] bg-gray-100;

      animation: glitch 10s infinite linear alternate-reverse;
    }
  }
}

@keyframes glitch {
  0% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  2.5% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  5% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  7.5% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  10% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  12.5% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  15% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  17.5% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  20% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  22.5% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  25% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  27.5% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  30% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  32.5% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  35% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  37.5% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  40% {
    clip: rect(calc(random(0, 5) * 1rem), 10rem, calc(random(0, 5) * 1rem), 0);
  }

  42.5%,
  100% {
    clip: rect(0 0 0 0);
  }
}
</style>
