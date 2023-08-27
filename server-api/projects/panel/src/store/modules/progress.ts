import { defineStore } from "pinia";
import { ref } from "vue";

export const useProgressStore = defineStore("progress", () => {
  const progress = ref(50);
  const isVisible = ref(true);

  const setProgress = (value: number) => {
    progress.value = value;
  };

  const show = () => {
    progress.value = 0;
    isVisible.value = true;
  };

  const hide = () => {
    isVisible.value = false;
  };

  return {
    progress,
    setProgress,
    isVisible,
    show,
    hide,
  };
});
