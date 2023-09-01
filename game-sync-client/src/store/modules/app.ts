import { defineStore } from "pinia";
import { computed, ref } from "vue";
import * as appApi from "@/api/app";

export const useAppStore = defineStore("app", () => {
  const server_url = ref<string | null>(null);
  const loading = ref<boolean>(false);
  const loaded = ref<boolean>(false);

  const configured = computed(() => server_url.value !== null);

  const loadLocalInfos = async () => {
    server_url.value = await appApi.get_configured_server();
    loaded.value = true;
  };

  const setLoading = (value: boolean) => {
    loading.value = value;
  };

  return {
    server_url,
    configured,
    loadLocalInfos,
    loading,
    setLoading,
    loaded,
  };
});
