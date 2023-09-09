<template>
  <div class="flex-1 py-8 flex flex-col">
    <header class="flex flex-row justify-between items-center mx-8 mb-2">
      <h1 class="text-2xl font-semibold">Add a game</h1>
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
            type="submit"
            :icon="PlusIcon"
            :loading="isSubmitting"
            :disabled="!meta.valid"
            >Add</GSButton
          >
          <GSButton
            bg-color="bg-transparent hover:bg-gray-200 active:bg-gray-300"
            outline-color="outline-red-600"
            text-color="text-red-600"
            :disabled="isSubmitting"
            @click.prevent="router.push('/dashboard/games')"
          >
            Cancel
          </GSButton>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import GSButton from "@/components/base/GSButton.vue";
import GSInput from "@/components/form/GSInput.vue";
import GSTextarea from "@/components/form/GSTextarea.vue";
import { toTypedSchema } from "@vee-validate/yup";
import { PlusIcon } from "lucide-vue-next";
import { FileText } from "lucide-vue-next";
import { Gamepad2 } from "lucide-vue-next";
import { useForm } from "vee-validate";
import { useRouter } from "vue-router";
import { object, string } from "yup";
import * as gamesApi from "@/api/game";
import { HttpError, HttpValidationError } from "@/types/http_errors";
import { GameInput } from "@/types/game";
import { objectEntries } from "@/types/helpers";

const router = useRouter();

const schema = object({
  name: string().required(),
  description: string(),
});

const { errors, handleSubmit, isSubmitting, meta, setFieldError } = useForm({
  validationSchema: toTypedSchema(schema),
});

const onSubmit = handleSubmit(async (values) => {
  try {
    await gamesApi.createGame(values);

    await router.push("/dashboard/games");
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
