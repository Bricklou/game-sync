<template>
  <FullScreenPage>
    <div class="container h-full mx-auto py-20">
      <GSCard title="Register" class="mx-auto md:max-w-lg">
        <form @submit="onSubmit">
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

          <GSInput
            id="password_confirmation"
            name="password_confirmation"
            label="Confirm Password"
            type="password"
            :icon="Lock"
            :error="errors.password_confirmation"
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
              Register
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
import { Lock, LogIn, Mail } from "lucide-vue-next";
import { object, string, ref } from "yup";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/yup";
import { useAuthStore } from "@/store/modules/auth";
import router from "@/router";
import FullScreenPage from "@/components/partials/FullScreenPage.vue";

const schema = object({
  email: string().email().label("Email address").required(),
  password: string().min(8).label("Password").required(),
  password_confirmation: string()
    .label("Password confirmation")
    .required()
    .oneOf([ref("password")], "Passwords must match"),
});

const { errors, handleSubmit, isSubmitting, meta } = useForm({
  validationSchema: toTypedSchema(schema),
});

const authStore = useAuthStore();
const onSubmit = handleSubmit(async (values) => {
  try {
    await authStore.register(values.email, values.password);

    await router.push("/");
  } catch (error) {
    console.error(error);
  }
});
</script>
