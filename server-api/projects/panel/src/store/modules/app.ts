import { defineStore } from "pinia";
import { ref } from "vue";
import * as appApi from "@/api/app.api";

export const useAppStore = defineStore("app", () => {
  const configured = ref<boolean>(false);
  const loading = ref<boolean>(false);

  const fetchAppData = async () => {
    const status = await appApi.status();

    configured.value = status.configured;
  };

  const setLoading = (value: boolean) => {
    loading.value = value;
  };

  return {
    configured,
    fetchAppData,
    loading,
    setLoading,
  };
});
