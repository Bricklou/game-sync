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
      <div class="mx-4 flex flex-row items-center gap-2">
        <GSInput
          id="search"
          label="Search a game"
          name="search"
          type="search"
          class="flex-1"
          :icon="Search"
          :disabled="loading || !!error || !data?.length"
          @input="onSearch($event.target.value)"
        />
        <button
          type="button"
          tabindex="0"
          class="mt-2 hover:bg-cyan-500 hover:bg-opacity-25 rounded-md p-2 group-focus:"
          @click="onSort()"
        >
          <ArrowDownAZ v-if="sortAsc" class="w-6 h-6 text-gray-700" />
          <ArrowUpAZ v-else class="w-6 h-6 text-gray-700" />
        </button>
      </div>

      <!-- Games list -->
      <ul
        v-if="!loading && data?.length"
        class="flex flex-col flex-auto overflow-y-auto h-10 scroll-smooth scroll-m-4 gap-y-1"
      >
        <li v-for="game in data" :key="game.id">
          <router-link :to="`/dashboard/games/${game.id}`">
            <div
              :class="[
                'flex flex-row items-center justify-between px-8 py-2 hover:bg-cyan-500 hover:bg-opacity-20 rounded-lg mx-4',
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

        <!-- Add a paginator if the current page is lower than the total number of pages -->
        <li
          v-if="meta && meta.current_page < meta.total_pages"
          class="flex flex-row justify-between mx-8 items-end flex-1 mb-1"
        >
          <GSButton
            type="button"
            tabindex="0"
            bg-color="bg-transparent hover:bg-gray-200"
            text-color="text-gray-700 hover:text-cyan-600"
            :class="[
              'flex flex-row items-center gap-2',
              { invisible: meta.current_page === 0 },
            ]"
            @click="fetchGames(Math.max(meta.current_page - 1, 0))"
          >
            <ArrowLeft class="w-6 h-6" />
            <span> Previous </span>
          </GSButton>
          <GSButton
            type="button"
            tabindex="0"
            bg-color="bg-transparent hover:bg-gray-200"
            text-color="text-gray-700 hover:text-cyan-600"
            :class="[
              'flex flex-row items-center gap-2',
              { invisible: meta.current_page + 1 >= meta.total_pages },
            ]"
            @click="
              fetchGames(Math.min(meta.current_page + 1, meta.total_pages))
            "
          >
            <span>Next</span>
            <ArrowRight class="w-6 h-6" />
          </GSButton>
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
import GSInput from "@/components/form/GSInput.vue";
import {
  PlusIcon,
  ServerCrash,
  Search,
  Unplug,
  ArrowUpAZ,
  ArrowDownAZ,
} from "lucide-vue-next";
import { useRouter } from "vue-router";
import { getGames } from "@/api/game";
import { onBeforeMount, ref, watch } from "vue";
import { Game } from "@/types/game";
import { HttpError } from "@/types/http_errors";
import { debounce } from "@/utils/debounce";
import { PaginationMeta } from "@/types/pagination";
import { ArrowLeft } from "lucide-vue-next";
import { ArrowRight } from "lucide-vue-next";

const router = useRouter();

const loading = ref(true);
const data = ref<Game[]>([]);
const meta = ref<PaginationMeta | null>(null);
const error = ref<string | null>(null);

const sortAsc = ref(true);
const searchInput = ref("");

async function fetchGames(page: number = 0) {
  let timeout = setTimeout(() => {
    loading.value = true;
  }, 500);

  try {
    const games = await getGames({
      sortOrder: sortAsc.value ? "asc" : "desc",
      search: searchInput.value,
      pagination: {
        page,
      },
    });
    data.value = games.data;
    meta.value = games.meta;
  } catch (e) {
    if (e instanceof HttpError) {
      error.value = e.message;
    } else {
      error.value = "An unknown error occurred";
    }
  } finally {
    clearTimeout(timeout);
    loading.value = false;
  }
}

const onSearch = debounce((value: string) => {
  searchInput.value = value;
}, 200);
const onSort = debounce(() => {
  sortAsc.value = !sortAsc.value;
}, 200);

onBeforeMount(() => {
  fetchGames();
});

watch([sortAsc, searchInput], () => {
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
