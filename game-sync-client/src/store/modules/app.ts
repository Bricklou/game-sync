import { defineStore } from "pinia";
import { computed, ref } from "vue";
import * as appApi from "@/api/app";
import { fetch as tauriFetch } from "@tauri-apps/plugin-http";
import { useAuthStore } from "./auth";
import router from "@/router";

export const useAppStore = defineStore("app", () => {
  const server_url = ref<string | null>(null);
  const loading = ref<boolean>(false);
  const loaded = ref<boolean>(false);

  const configured = computed(() => server_url.value !== null);

  const loadLocalInfos = async () => {
    server_url.value = await appApi.getConfiguredServer();
    if (server_url.value) {
      console.debug("Server url is configured");
    }
    loaded.value = true;
  };

  const setLoading = (value: boolean) => {
    loading.value = value;
  };

  const setServerUrl = async (value: string | null) => {
    await appApi.validateServerUrl(value);
    await appApi.setConfiguredServer(value);
    server_url.value = value;
  };

  // Work like the "fetch" method from tauri http plugin, except that it
  // automatically prepend the server url if it is configured
  const fetch = async (
    input: RequestInfo | URL,
    init?: RequestInit | undefined,
  ) => {
    if (!server_url.value) {
      throw new Error("Cannot fetch if the server url is not configured");
    }

    if (typeof input === "string") {
      input = new URL(input, server_url.value);
    } else {
      input = new URL(input.toString(), server_url.value);
    }

    const auth = useAuthStore();
    if (!init) {
      init = {
        headers: {},
      };
    }
    if (auth.token) {
      init.headers = Object.assign(init?.headers || {}, {
        Authorization: `Bearer ${auth.token}`,
      });
    }

    return await tauriFetch(input, init);
  };

  const forgotServer = async () => {
    const auth = useAuthStore();
    await auth.logout().catch(console.error);

    await appApi.setConfiguredServer(null);
    server_url.value = null;

    console.debug("Server url is not configured anymore");

    await router.push("/setup");
  };

  return {
    server_url,
    configured,
    loadLocalInfos,
    loading,
    setLoading,
    loaded,
    setServerUrl,
    fetch,
    forgotServer,
  };
});
