<template>
  <article v-if="!loading || !isEditing" class="flex-1 py-8 flex flex-col">
    <header class="flex flex-row justify-between items-center mx-8 mb-2">
      <h1 v-if="isEditing" class="text-2xl font-semibold">Edit game</h1>
      <h1 v-else class="text-2xl font-semibold">Add a game</h1>
    </header>

    <div class="mx-8">
      <form class="container" @submit="onSubmit">
        <GSInput
          id="name"
          name="name"
          label="Name"
          :icon="Gamepad2"
          :error="errors.name"
        />

        <!-- Game Logo -->

        <!-- Game author -->

        <!-- Game description -->
        <GSTextarea
          id="description"
          name="description"
          label="Description"
          :icon="FileText"
          :error="errors.description"
        />

        <!-- Game previews -->

        <!-- Game tags -->

        <!-- Social links -->

        <div class="flex flex-row-reverse justify-start items-center gap-4">
          <!-- Redirect on the game page after submit -->
          <GSButton
            v-if="isEditing"
            type="submit"
            :icon="PencilIcon"
            :loading="isSubmitting"
            :disabled="!meta.valid"
          >
            Update
          </GSButton>

          <GSButton
            v-else
            type="submit"
            :icon="PlusIcon"
            :loading="isSubmitting"
            :disabled="!meta.valid"
          >
            Add
          </GSButton>
          <GSButton
            bg-color="bg-transparent hover:bg-gray-200 active:bg-gray-300"
            outline-color="outline-red-600"
            text-color="text-red-600"
            :disabled="isSubmitting"
            @click.prevent="
              router.push(
                isEditing
                  ? `/dashboard/games/${route.params.id}`
                  : '/dashboard/games',
              )
            "
          >
            Cancel
          </GSButton>
        </div>
      </form>
    </div>
  </article>

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
import GSButton from "@/components/base/GSButton.vue";
import GSInput from "@/components/form/GSInput.vue";
import GSTextarea from "@/components/form/GSTextarea.vue";
import LoadingSpinner from "@/components/base/LoadingSpinner.vue";
import { toTypedSchema } from "@vee-validate/yup";
import {
  PlusIcon,
  FileText,
  Gamepad2,
  PencilIcon,
  ArrowLeft,
  ServerCrash,
} from "lucide-vue-next";
import { useForm } from "vee-validate";
import { useRouter } from "vue-router";
import { object, string } from "yup";
import * as gamesApi from "@/api/game";
import { HttpError, HttpValidationError } from "@/types/http_errors";
import { GameInput } from "@/types/game";
import { objectEntries } from "@/types/helpers";
import { useRoute } from "vue-router";
import { onBeforeMount, ref } from "vue";
import { getGame } from "@/api/game";

const router = useRouter();
const route = useRoute();

const loading = ref(route.params.id !== undefined);
const isEditing = ref(route.params.id !== undefined);
const error = ref<string | null>(null);

async function fetchGame() {
  try {
    const gameId = Number.parseInt(route.params.id as string);
    const game = await getGame(gameId);

    setValues({
      name: game.name,
      description: game.description,
    });
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
  if (isEditing.value) {
    fetchGame();
  }
});

const schema = object({
  name: string().required(),
  description: string(),
});

const { errors, handleSubmit, isSubmitting, meta, setFieldError, setValues } =
  useForm({
    validationSchema: toTypedSchema(schema),
  });

const onSubmit = handleSubmit(async (values) => {
  try {
    if (isEditing.value) {
      await gamesApi.updateGame(route.params.id as string, values);
      await router.push(`/dashboard/games/${route.params.id}`);
    } else {
      await gamesApi.createGame(values);
      await router.push("/dashboard/games");
    }
  } catch (error) {
    if (HttpValidationError.assert<keyof GameInput>(error)) {
      for (const [field, messages] of objectEntries(error.fields)) {
        setFieldError(field, messages[0]);
      }
    } else if (error instanceof HttpError) {
      setFieldError("name", error.message);
    }
  }
});
</script>
