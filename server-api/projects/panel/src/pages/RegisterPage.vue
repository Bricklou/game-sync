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
            <GSButton type="submit" class="mt-4" :icon="LogIn"
              >Register</GSButton
            >
          </div>
        </form>
      </GSCard>
    </div>
  </FullScreenPage>
</template>

<script setup lang="ts">
import GSCard from "@/components/GSCard.vue";
import GSInput from "@/components/form/GSInput.vue";
import FullScreenPage from "@/components/partials/FullScreenPage.vue";
import GSButton from "@/components/base/GSButton.vue";
import { Lock, LogIn } from "lucide-vue-next";
import { Mail } from "lucide-vue-next";
import { object, string, ref } from "yup";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/yup";
import { useAuthStore } from "@/store/modules/auth";

const schema = object({
  email: string().email().label("Email address").required(),
  password: string().min(8).label("Password").required(),
  password_confirmation: string()
    .label("Password confirmation")
    .required()
    .oneOf([ref("password")], "Passwords must match"),
});

const { errors, handleSubmit } = useForm({
  validationSchema: toTypedSchema(schema),
});

const authStore = useAuthStore();
const onSubmit = handleSubmit(async (values) => {
  try {
    await authStore.register(values.email, values.password);
  } catch (error) {
    console.error(error);
  }
});
</script>
