import { User } from "@/types/user";
import { defineStore } from "pinia";
import { computed, ref } from "vue";
import * as authApi from "@/api/auth";

export const useAuthStore = defineStore("auth", () => {
  const user = ref<User | null>(null);
  const token = ref<string | null>(null);

  const isAuthenticated = computed(() => user.value !== null);
  const currentUser = computed(() => user.value);

  const loadLocalInfos = async () => {
    console.log("Trying to load user infos from store");
    token.value = await authApi.getStoredToken();
    if (token.value) {
      console.log("Token found, trying to refresh");
      try {
        await refresh();
      } catch (e) {
        token.value = null;
      }
    }
  };

  const login = async (email: string, password: string) => {
    const data = await authApi.login(email, password);
    user.value = data;
  };

  const refresh = async () => {
    if (!token.value) {
      throw new Error("Cannot refresh if the token is not set");
    }

    const data = await authApi.refresh(token.value);
    user.value = data;
  };

  const logout = async () => {
    if (!token.value) {
      throw new Error("Cannot logout if the token is not set");
    }

    user.value = null;
    await authApi.logout(token.value);
  };

  return {
    currentUser,
    isAuthenticated,
    login,
    logout,
    loadLocalInfos,
  };
});
