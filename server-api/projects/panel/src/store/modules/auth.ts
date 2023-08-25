import { IUser } from "../interfaces/user.interface";
import { defineStore } from "pinia";

export const useAuthUserStore = defineStore("authUser", () => {
  const user = ref<IUser | null>(null);

  const setUser = (newUser: IUser) => {
    user.value = newUser;
  };

  const isLoggedIn = computed(() => {
    return user.value !== null;
  });

  return { user, isLoggedIn, setUser };
});
