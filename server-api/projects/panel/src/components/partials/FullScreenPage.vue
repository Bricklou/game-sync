<template>
  <div
    class="relative min-h-screen flex flex-col content-center justify-center bg-gray-900 text-white"
  >
    <div class="z-10 px-2">
      <slot />
    </div>

    <ul class="bg-animation" aria-hidden="true">
      <li v-for="n in 20" :key="n"></li>
    </ul>
  </div>
</template>

<style scoped>
.bg-animation {
  @apply absolute top-0 left-0 w-full h-full overflow-hidden;

  li {
    @apply absolute block w-5 h-5 bg-cyan-600 bg-opacity-50 -top-[150px];
    animation: fall-down 25s linear infinite;

    &:nth-child(1) {
      @apply left-1/4 w-20 h-20;
      animation-delay: 20s;
    }

    &:nth-child(2) {
      @apply left-[10%] w-5 h-5;
      animation-delay: 2s;
      animation-duration: 12s;
    }

    &:nth-child(3) {
      @apply left-[70%] w-5 h-5;
      animation-delay: 4s;
    }

    &:nth-child(4) {
      @apply left-[40%] w-[60px] h-[60px];
      animation-duration: 18s;
    }

    &:nth-child(5) {
      @apply left-[65%] w-5 h-5;
    }

    &:nth-child(6) {
      @apply left-3/4 w-[110px] h-[110px];
      animation-delay: 3s;
    }

    &:nth-child(7) {
      @apply left-[35%] w-[150px] h-[150px];
      animation-delay: 7s;
    }

    &:nth-child(8) {
      @apply left-1/2 w-[25px] h-[25px];
      animation-delay: 15s;
      animation-duration: 45s;
    }

    &:nth-child(9) {
      @apply left-[20%] w-[15px] h-[15px];
      animation-delay: 2s;
      animation-duration: 35s;
    }

    &:nth-child(10) {
      @apply left-[70%] w-[150px] h-[150px];
      animation-duration: 11s;
    }

    &:nth-child(11) {
      @apply left-[25%] w-[25px] h-[25px];
      animation-delay: 45s;
      animation-duration: 11s;
    }

    &:nth-child(12) {
      @apply left-[75%] w-[110px] h-[110px];
      animation-delay: 3s;
    }

    &:nth-child(13) {
      @apply left-[90%] w-[60px] h-[60px];
      animation-delay: 5s;
    }

    &:nth-child(14) {
      @apply left-[45%] w-[25px] h-[25px];
      animation-delay: 15s;
      animation-duration: 11s;
    }

    &:nth-child(15) {
      @apply left-[20%] w-[15px] h-[15px];
      animation-delay: 2s;
    }

    &:nth-child(16) {
      @apply left-[5%] w-[150px] h-[150px];
      animation-duration: 9s;
    }

    &:nth-child(17) {
      @apply left-[25%] w-[25px] h-[25px];
      animation-delay: 5s;
    }

    &:nth-child(18) {
      @apply left-[15%] w-[110px] h-[110px];
      animation-delay: 3s;
      animation-duration: 11s;
    }

    &:nth-child(19) {
      @apply left-[90%] w-[60px] h-[60px];
      animation-delay: 5s;
      animation-duration: 11s;
    }

    &:nth-child(20) {
      @apply left-[50%] w-[25px] h-[25px];
      animation-delay: 15s;
    }
  }
}

@keyframes fall-down {
  0% {
    @apply opacity-100 rounded-[10%];
    transform: translateY(0) rotate(0deg) scale(1);
  }

  100% {
    @apply opacity-0 rounded-[50%];
    transform: translateY(90vh) rotate(720deg) scale(0.6);
  }
}
</style>
