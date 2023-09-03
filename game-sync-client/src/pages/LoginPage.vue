<template>
  <FullScreenPage>
    <div class="container h-full mx-auto">
      <GSCard title="Login" class="mx-auto max-w-sm">
        <form @submit="onSubmit">
          <div v-if="errors" class=""></div>
          <GSInput
            id="email"
            name="email"
            label="Email Address"
            type="email"
            :icon="Mail"
            :error="errors.email"
            autocompletion="email"
          />

          <GSInput
            id="password"
            name="password"
            label="Password"
            type="password"
            :icon="Lock"
            :error="errors.password"
            autocompletion="new-password"
          />

          <div class="text-center">
            <GSButton
              type="submit"
              class="mt-4"
              :icon="LogIn"
              :disabled="!meta.valid"
              :loading="isSubmitting"
            >
              Login
            </GSButton>
          </div>
        </form>
      </GSCard>
    </div>
  </FullScreenPage>
</template>

<script setup lang="ts">
import GSButton from "@/components/base/GSButton.vue";
import GSCard from "@/components/base/GSCard.vue";
import GSInput from "@/components/form/GSInput.vue";
import FullScreenPage from "@/components/partials/FullScreenPage.vue";
import router from "@/router";
import { useAuthStore } from "@/store/modules/auth";
import { toTypedSchema } from "@vee-validate/yup";
import { LogIn } from "lucide-vue-next";
import { Mail } from "lucide-vue-next";
import { Lock } from "lucide-vue-next";
import { useForm } from "vee-validate";
import { object, string } from "yup";

const schema = object({
  email: string().email().label("Email address").required(),
  password: string().min(8).label("Password").required(),
});

const { errors, handleSubmit, isSubmitting, meta, setFieldError } = useForm({
  validationSchema: toTypedSchema(schema),
});

const authStore = useAuthStore();
const onSubmit = handleSubmit(async (values) => {
  try {
    await authStore.login(values.email, values.password);

    await router.push({ name: "Home" });
  } catch (error) {
    if (error instanceof Error) {
      setFieldError("email", error.message);
    } else {
      setFieldError("email", "An error occurred, please try again later.");
    }
    console.error(error);
  }
});
</script>
