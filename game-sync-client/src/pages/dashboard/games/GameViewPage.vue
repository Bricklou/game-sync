<template>
  <article
    v-if="game"
    class="flex-1 pb-1 flex flex-col relative overflow-y-auto scroll-smooth h-56"
  >
    <!-- Header -->
    <header class="mb-2 h-48 z-10 sticky -top-32 flex flex-col items-center">
      <!-- Game cover -->
      <figure
        class="w-full h-48 overflow-hidden absolute top-0 left-0 select-none pointer-events-none"
      >
        <img
          v-if="game.cover"
          :src="game.cover"
          :alt="game.name"
          class="w-full h-full object-cover"
        />

        <div
          v-else
          class="w-full h-full flex flex-col justify-center items-center bg-gray-400"
        >
          <span class="text-4xl text-gray-700"> ? </span>
        </div>
      </figure>

      <div class="h-16 sticky top-0 w-full">
        <div
          class="flex flex-col justify-between z-10 items-center px-8 pt-4 h-full"
        >
          <div
            class="z-10 flex flex-row justify-between items-center w-full sticky top-0"
          >
            <span class="flex flex-row justify-start items-center gap-2">
              <router-link
                to="/dashboard/games"
                :class="[
                  'block p-1 rounded-md hover:bg-gray-200 hover:bg-opacity-40 hover:text-cyan-600 transition-all duration-100 ease-in-out',
                  'before:!left-full',
                ]"
                aria-label="Back to games"
                data-tip="Back to games"
              >
                <ArrowLeft class="w-6 h-6" />
              </router-link>
              <h1 class="text-2xl font-semibold">{{ game.name }}</h1>
            </span>

            <span>
              <router-link
                :to="`/dashboard/games/${route.params.id}/edit`"
                class="block p-1 rounded-md hover:bg-gray-200 hover:bg-opacity-40 hover:text-cyan-600 transition-all duration-100 ease-in-out text-gray-600"
                aria-label="Edit game"
                data-tip="Edit game"
              >
                <Pencil class="w-6 h-6" />
              </router-link>
            </span>
          </div>
        </div>
      </div>

      <!-- Play/Download button -->
      <section class="mx-auto z-10 sticky flex-1 flex flex-col justify-end">
        <GSButton
          v-if="installed"
          :icon="Play"
          class="shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-75 ease-out"
        >
          Play
        </GSButton>

        <GSButton
          v-else
          :icon="Download"
          class="shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-75 ease-out mb-2.5"
        >
          Download
        </GSButton>
      </section>
    </header>

    <main class="mx-8 h-0 flex-1 flex flex-col mt-8">
      <!-- Game info -->
      <div class="game-infos mb-8">
        <p class="text-gray-600">{{ game.description }}</p>
      </div>

      <!-- Versions -->
      <div class="versions flex-1 flex flex-col">
        <div class="flex flex-row justify-between items-center mb-2">
          <h2 class="uppercase font-semibold text-gray-700">Versions</h2>

          <router-link
            :to="`/dashboard/games/${game.id}/versions/new`"
            class="text-gray-500 block p-1 rounded-md hover:bg-gray-200 hover:text-cyan-600 transition-all duration-100 ease-in-out before:!right-full before:!left-0"
            aria-label="Add new version"
            data-tip="Add new version"
          >
            <Plus class="w-6 h-6" />
          </router-link>
        </div>

        <!-- Versions list -->
        <div class="flex-1 flex flex-col">
          <ul class="flex flex-col flex-auto gap-y-1">
            <li
              v-for="g in [
                {
                  id: 2,
                  name: 'lol1',
                  created_at: new Date().toLocaleDateString(),
                },
                {
                  id: 3,
                  name: 'lol2',
                  created_at: new Date().toLocaleDateString(),
                },
                {
                  id: 4,
                  name: 'lol3',
                  created_at: new Date().toLocaleDateString(),
                },
                {
                  id: 5,
                  name: 'lol4',
                  created_at: new Date().toLocaleDateString(),
                },
                {
                  id: 6,
                  name: 'lol5',
                  created_at: new Date().toLocaleDateString(),
                },
                {
                  id: 7,
                  name: 'lol6',
                  created_at: new Date().toLocaleDateString(),
                },
                {
                  id: 8,
                  name: 'lol7',
                  created_at: new Date().toLocaleDateString(),
                },
                {
                  id: 9,
                  name: 'lol8',
                  created_at: new Date().toLocaleDateString(),
                },
                {
                  id: 10,
                  name: 'lol9',
                  created_at: new Date().toLocaleDateString(),
                },
              ].reverse()"
              :key="g.id"
            >
              <div
                :class="[
                  'flex flex-row items-center justify-between px-4 py-2 hover:bg-cyan-500 hover:bg-opacity-20 rounded-lg',
                  'transition-colors duration-100 ease-in-out',
                ]"
              >
                <div class="flex flex-col">
                  <span class="text-lg font-semibold">{{ g.name }}</span>
                  <span class="text-sm text-gray-500 truncate">
                    {{ g.created_at }}
                  </span>
                </div>

                <div>
                  <GSButton
                    data-tip="Download the version"
                    aria-label="Download the version"
                    class="before:!-left-1 before:!right-0 py-2 px-2"
                    bg-color="bg-transparent hover:bg-white hover:bg-opacity-50"
                    text-color="text-gray-500 hover:text-cyan-600"
                  >
                    <Download class="w-6 h-6" />
                  </GSButton>
                </div>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </main>
  </article>

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
import { ArrowLeft, Pencil, Play, Plus, Download } from "lucide-vue-next";
import { Game } from "@/types/game";
import { onBeforeMount, ref } from "vue";
import { useRoute } from "vue-router";
import { HttpError } from "@/types/http_errors";
import { getGame } from "@/api/game";
import GSButton from "@/components/base/GSButton.vue";

const route = useRoute();

const game = ref<Game | null>(null);
const installed = ref<boolean>(false);
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
