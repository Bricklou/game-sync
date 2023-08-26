import { defineStore } from "pinia";
import { computed, ref } from "vue";
import router from "@/router.ts";

import * as authApi from "@/api/auth.api";
import { IUser } from "@/types/user.interface";

export const useAuthStore = defineStore("authUser", () => {
  const user = ref<IUser | null>(null);
  const returnUrl = ref<string | null>(null);

  const setUser = (newUser: IUser) => {
    user.value = newUser;
  };

  const isLoggedIn = computed(() => {
    return user.value !== null;
  });

  const login = async (email: string, password: string) => {
    // update pinia state
    user.value = await authApi.login(email, password);

    await router.push(returnUrl.value || "/");
  };

  const logout = async () => {
    await authApi.logout();
    // update pinia state
    user.value = null;

    await router.push("/login");
  };

  const fetchUser = async () => {
    // update pinia state
    user.value = await authApi.me();
  };

  const getUser = async () => {
    if (user.value === null) {
      await fetchUser();
    }
    return user.value;
  };

  const setReturnUrl = (url: string) => {
    returnUrl.value = url;
  };

  return {
    user,
    isLoggedIn,
    setUser,
    login,
    setReturnUrl,
    logout,
    fetchUser,
    getUser,
  };
});
