<template>
  <div class="relative py-3 mt-2 mb-6">
    <textarea
      :id="$props.id"
      ref="textarea"
      v-model="value"
      :type="$props.type"
      :name="$props.name"
      :class="[
        'rounded-lg flex-1 appearance-none border w-full py-2 px-4 text-gray-700 shadow-sm text-base peer',
        'mt-1 outline-offset-2 invalid:outline invalid:outline-red-600 invalid:outline-2',
        'placeholder-transparent autofill:outline-yellow-500 resize-none',
        bgColor,
        borderColor,
        $props.icon ? 'pl-10' : '',
        errorMessage
          ? 'outline outline-red-600 outline-2'
          : 'focus:outline focus:outline-cyan-600',
      ]"
      :placeholder="$props.label"
      :autocomplete="$props.autocompletion"
    ></textarea>

    <component
      :is="$props.icon"
      v-if="$props.icon"
      aria-hidden="true"
      :class="[
        'absolute left-2 top-6 peer-focus:text-cyan-600 transition-colors w-6 h-6 pointer-events-none',
        errorMessage ? 'text-red-500' : 'text-gray-400',
      ]"
    />

    <label
      :for="$props.id"
      :class="[
        'text-cyan-600 absolute left-2 -top-4 transition-all duration-100 ease-in-out',
        'peer-focus:-top-4 peer-focus:left-2',
        'peer-placeholder-shown:top-6 peer-focus:text-cyan-600',
        'peer-placeholder-shown:text-gray-400 pointer-events-none',

        $props.icon
          ? 'peer-placeholder-shown:left-10'
          : 'peer-placeholder-shown:left-4',
      ]"
    >
      {{ $props.label }}
    </label>

    <p v-if="errorMessage" class="absolute text-sm text-red-500 ml-4 mt-1">
      {{ errorMessage }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { Icon } from "lucide-vue-next";
import { useField } from "vee-validate";
import { nextTick, ref, watch } from "vue";

interface TextareaProps {
  type?: "text" | "email" | "password" | "url";
  id: string;
  name: string;
  label: string;
  icon?: Icon;
  autocompletion?: string;
  borderColor?: string;
  bgColor?: string;
}
// Input type
const props = withDefaults(defineProps<TextareaProps>(), {
  type: "text",
  icon: undefined,
  autocompletion: undefined,
  borderColor: "border-gray-300",
  bgColor: "bg-white autofill:bg-white",
});

const textarea = ref<HTMLTextAreaElement | null>(null);
const { value, errorMessage } = useField<string>(() => props.name);

watch(value, () => {
  if (!textarea.value) return;
  textarea.value.style.height = "auto";

  nextTick(() => {
    if (!textarea.value) return;
    textarea.value.style.height = `${textarea.value?.scrollHeight + 2}px`;
  });
});
</script>
