<template>
  <div class="relative py-2 mt-3">
    <input
      :id="$props.id"
      :type="$props.type"
      :name="$props.name"
      :class="[
        'rounded-lg border-transparent flex-1 appearance-none border border-gray-300 w-full py-2 px-4 bg-white text-gray-700',
        'placeholder-gray-400 shadow-sm text-base focus:border-transparent peer',
        'focus:outline-offset-2 focus:outline focus:outline-cyan-600 mt-1 invalid:outline invalid:outline-red-600 invalid:outline-2',
        'placeholder-transparent autofill:bg-white autofill:outline-yellow-500',
        $props.icon ? 'pl-10' : '',
      ]"
      :placeholder="$props.label"
    />

    <component
      :is="$props.icon"
      v-if="$props.icon"
      class="absolute left-2 top-5 text-gray-400 peer-focus:text-cyan-600 transition-colors w-6 h-6 pointer-events-none"
    />

    <label
      :for="$props.id"
      :class="[
        'text-cyan-600 absolute left-2 -top-4 transition-all duration-100 ease-in-out',
        'peer-focus:-top-4 peer-focus:left-2',
        'peer-placeholder-shown:top-5 peer-focus:text-cyan-600',
        'peer-placeholder-shown:text-gray-400 pointer-events-none',

        $props.icon
          ? 'peer-placeholder-shown:left-10'
          : 'peer-placeholder-shown:left-4',
      ]"
    >
      {{ $props.label }}
    </label>
  </div>
</template>

<script setup lang="ts">
import { Icon } from "lucide-vue-next";

interface InputProps {
  type: "text" | "email" | "password";
  id: string;
  name: string;
  label: string;
  icon?: Icon;
}
// Input type
withDefaults(defineProps<InputProps>(), {
  type: "text",
  icon: undefined,
});
</script>
