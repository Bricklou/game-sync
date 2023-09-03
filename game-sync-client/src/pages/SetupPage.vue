<template>
  <section class="min-h-screen py-12 px-8 text-white flex flex-col">
    <header class="mb-8">
      <h1 class="text-4xl font-bold mb-8">Welcome to GameSync</h1>

      <p class="text-gray-100">
        GameSync is a tool allowing you to synchronise you games and saves files
        across multiple computers.
      </p>
    </header>

    <main class="flex flex-col flex-1">
      <p class="mb-4">
        To get started, you'll need to add a remote server to store to.
      </p>

      <form class="flex-1 flex flex-col justify-between" @submit="onSubmit">
        <GSInput
          id="serverUrl"
          name="serverUrl"
          label="Server URL"
          type="url"
          placeholder="https://example.com"
          :icon="Link"
          :error="errors.serverUrl"
        />

        <div class="text-right">
          <GSButton
            type="submit"
            class="mt-4"
            :icon="ArrowRight"
            :disabled="!meta.valid"
            :loading="isSubmitting"
            direction="right"
          >
            Connect
          </GSButton>
        </div>
      </form>
    </main>
  </section>
</template>

<script setup lang="ts">
import GSInput from "@/components/form/GSInput.vue";
import GSButton from "@/components/base/GSButton.vue";
import { toTypedSchema } from "@vee-validate/yup";
import { Link } from "lucide-vue-next";
import { useForm } from "vee-validate";
import { object, string as string } from "yup";
import { ArrowRight } from "lucide-vue-next";
import { useAppStore } from "@/store/modules/app";
import router from "@/router";

const schema = object({
  serverUrl: string().required().label("Server URL"),
});

const { meta, errors, isSubmitting, handleSubmit, setFieldError } = useForm({
  validationSchema: toTypedSchema(schema),
});

const appStore = useAppStore();
const onSubmit = handleSubmit(async (values) => {
  try {
    await appStore.setServerUrl(values.serverUrl ?? null);

    await router.push({ name: "Login" });
  } catch (error) {
    if (error instanceof Error) {
      setFieldError("serverUrl", error.message);
    } else if (typeof error === "string") {
      setFieldError("serverUrl", error);
    } else {
      setFieldError("serverUrl", "An unknown error occured");
    }
  }
});
</script>

<style scoped>
section {
  animation: fadeIn 0.25s ease-in-out;

  background-image: repeating-linear-gradient(
      theme("colors.cyan.600" / 20%) 0 1px,
      transparent 1px 100%
    ),
    repeating-linear-gradient(
      90deg,
      theme("colors.cyan.600" / 20%) 0 1px,
      transparent 1px 100%
    );
  background-position:
    -1rem -1rem,
    -1rem -1rem;
  background-size: 2rem 2rem;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: scale(0.75);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
