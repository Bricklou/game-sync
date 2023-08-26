<template>
  <FullScreenPage>
    <div class="container h-full mx-auto py-20">
      <GSCard title="Register" class="mx-auto md:max-w-lg">
        <GSInput id="email" name="email" label="Email Address" :icon="Mail" />

        <GSInput
          id="password"
          name="password"
          label="Password"
          type="password"
          :icon="Lock"
        />

        <GSInput
          id="password_confirmation"
          name="password_confirmation"
          label="Confirm Password"
          type="password"
          :icon="Lock"
        />

        <div class="text-center">
          <GSButton type="submit" class="mt-4" :icon="LogIn">Register</GSButton>
        </div>
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
</script>
