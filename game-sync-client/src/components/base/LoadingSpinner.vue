<template>
  <div class="text-center">
    <div class="mx-auto">
      <div class="spinner">
        <div class="spin-block-1"></div>
        <div class="spin-block-2"></div>
        <div class="spin-block-3"></div>
      </div>

      <p class="text-2xl animate-pulse">Loading...</p>
    </div>
  </div>
</template>

<script setup lang="ts"></script>

<style scoped>
.spinner {
  @apply relative w-14 h-14 mx-auto grid grid-cols-2 grid-rows-2 gap-1 mb-4;

  --animation-time: 5s;

  animation: pulse calc(var(--animation-time) / 2) ease-in-out infinite both;

  > * {
    @apply rounded-lg bg-gradient-to-br shadow-lg;
  }

  .spin-block-1 {
    @apply bg-cyan-600;
    @apply from-cyan-500 to-cyan-600;
    @apply origin-bottom-left;

    animation: assemble calc(var(--animation-time) / 2) linear infinite;
  }

  .spin-block-2 {
    @apply bg-cyan-600 col-start-2;
    @apply from-cyan-700 to-cyan-800;
  }

  .spin-block-3 {
    @apply row-start-2 col-span-2;
    @apply from-cyan-900 to-cyan-950;
  }
}

@keyframes assemble {
  0%,
  20% {
    transform: rotate(-35deg) translate3d(-5px, -15px, 0) scale(0.5);
    opacity: 0;
    transform-origin: bottom left;
  }

  27.5% {
    transform: rotate(0deg) scale(1);
    opacity: inherit;
    transform-origin: bottom left;
  }
}

@keyframes pulse {
  0%,
  30% {
    /* transform: scale(1) translateY(0); */
    opacity: 1;
  }
  40% {
    /* transform: scale(1.1) translateY(-5px); */
    opacity: 0.75;
  }
  50%,
  100% {
    /* transform: scale(1) translateY(0); */
    opacity: 1;
  }
}
</style>
