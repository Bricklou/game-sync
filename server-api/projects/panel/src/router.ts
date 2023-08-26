import { RouteRecordRaw, createRouter, createWebHistory } from "vue-router";
import { useAuthStore } from "@/store/modules/auth.ts";
import { useAppStore } from "./store/modules/app";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "Home",
    component: () => import("./pages/HomePage.vue"),
  },
  {
    path: "/login",
    name: "Login",
    component: () => import("./pages/LoginPage.vue"),
    meta: {
      guest: true,
      hideNavbar: true,
    },
  },
  {
    path: "/register",
    name: "Register",
    component: () => import("./pages/RegisterPage.vue"),
    meta: {
      guest: true,
      hideNavbar: true,
    },
  },
  {
    path: "/setup",
    name: "setup",
    component: () => import("./pages/SetupPage.vue"),
    meta: {
      guest: true,
      hideNavbar: true,
    },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach(async (to) => {
  const app = useAppStore();

  if (!app.configured && to.name !== "Register") {
    console.log("Application not configured");
    return "/register";
  }

  const authRequired = !to.meta.guest;
  const auth = useAuthStore();

  if (authRequired && !auth.isLoggedIn) {
    auth.setReturnUrl(to.fullPath);
    return "/login";
  }
});

export default router;
