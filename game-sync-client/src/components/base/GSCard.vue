<template>
  <section class="rounded-lg bg-gray-800 py-8 px-6 shadow-md">
    <h1 class="font-bold text-3xl">
      {{ $props.title }}
    </h1>

    <div class="mt-8">
      <slot></slot>
    </div>
  </section>
</template>

<script setup lang="ts">
interface CardProps {
  title: string;
}

defineProps<CardProps>();
</script>
