<template>
  <div class="flex-1 py-8 flex flex-col">
    <header class="flex flex-row justify-between items-center mx-8 mb-2">
      <h1 class="text-2xl font-semibold">Add a game</h1>
    </header>

    <div class="mx-8">
      <form class="container">
        <GSInput id="name" name="name" label="Name" :icon="Gamepad2" />

        <!-- Game Logo -->

        <!-- Game author -->

        <!-- Game description -->
        <GSTextarea
          id="description"
          name="description"
          label="Description"
          :icon="FileText"
        />

        <!-- Game previews -->

        <!-- Game tags -->

        <!-- Social links -->

        <div class="flex flex-row justify-end items-center gap-4">
          <GSButton
            bg-color="bg-transparent hover:bg-gray-200 active:bg-gray-300"
            outline-color="outline-red-600"
            text-color="text-red-600"
            @click.prevent="router.push('/dashboard/games')"
          >
            Cancel
          </GSButton>

          <!-- Redirect on the game page after submit -->
          <GSButton type="submit" :icon="PlusIcon">Add</GSButton>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import GSButton from "@/components/base/GSButton.vue";
import GSInput from "@/components/form/GSInput.vue";
import GSTextarea from "@/components/form/GSTextarea.vue";
import { PlusIcon } from "lucide-vue-next";
import { FileText } from "lucide-vue-next";
import { Gamepad2 } from "lucide-vue-next";
import { useRouter } from "vue-router";

const router = useRouter();
</script>
