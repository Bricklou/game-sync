import { User } from "@/types/user";
import { defineStore } from "pinia";
import { computed, ref } from "vue";
import * as authApi from "@/api/auth";
import router from "@/router";

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
        await logout();
      }
    }
  };

  const login = async (email: string, password: string) => {
    [token.value, user.value] = await authApi.login(email, password);
  };

  const refresh = async () => {
    if (!token.value) {
      throw new Error("Cannot refresh if the token is not set");
    }

    try {
      const data = await authApi.refresh(token.value);
      user.value = data;
    } catch (e) {
      await logout();
      throw e;
    }
  };

  const logout = async () => {
    if (!token.value) {
      throw new Error("Cannot logout if the token is not set");
    }

    user.value = null;
    await authApi.logout(token.value);
    token.value = null;

    await router.push({ name: "Login" });
  };

  return {
    currentUser,
    isAuthenticated,
    login,
    logout,
    loadLocalInfos,
  };
});
