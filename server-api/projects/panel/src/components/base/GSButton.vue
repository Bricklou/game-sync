<template>
  <button
    :class="[
      'px-4 py-2 bg-cyan-600 hover:bg-cyan-700 rounded-md motion-safe:transition-all duration-100 text-white',
      'outline-cyan-600 outline-offset-2 outline-2 focus:outline',
    ]"
    :type="$props.type"
  >
    <Loader2 v-if="$props.loading" class="mr-2 inline-block animate-spin" />

    <component
      :is="$props.icon"
      v-else-if="$props.icon"
      class="mr-2 inline-block"
    />
    <slot />
  </button>
</template>

<script setup lang="ts">
import { Icon } from "lucide-vue-next";
import { Loader2 } from "lucide-vue-next";

interface ButtonProps {
  type: "button" | "submit" | "reset";
  icon?: Icon;
  loading?: boolean;
}

// Input type
withDefaults(defineProps<ButtonProps>(), {
  type: "button",
  loading: false,
  icon: undefined,
});
</script>

<style scoped>
button {
  &:active:hover,
  &:active:focus {
    animation: button-pop 0s ease-out;
    transform: scale(0.97);
  }
}

@keyframes button-pop {
  0% {
    transform: scale(0.98);
  }
  40% {
    transform: scale(1.02);
  }
  100% {
    transform: scale(1);
  }
}
</style>
