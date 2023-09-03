import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import { useAppStore } from "./store/modules/app";
import { useAuthStore } from "./store/modules/auth";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "Home",
    component: () => import("./pages/HomePage.vue"),
  },
  {
    path: "/setup",
    name: "Setup",
    component: () => import("./pages/SetupPage.vue"),
  },
  {
    path: "/login",
    name: "Login",
    component: () => import("./pages/LoginPage.vue"),
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach(async (to, _from, next) => {
  // Check if the application is loaded
  const app = useAppStore();
  const authStore = useAuthStore();

  // Call this part only when the application is booting
  if (!app.loaded) {
    console.log("Application not loaded");
    app.setLoading(true);

    await app.loadLocalInfos();
    await authStore.loadLocalInfos();
  }

  // If the application is not configured
  if (!app.configured) {
    console.log("Application not configured");
    // If the user is not on the setup page, redirect to the setup page
    if (to.name !== "Setup") {
      return next("/setup");
    }

    // If the user is on the setup page, let him pass
    return next();
  }

  // Here the application is configured

  // Check if the user is logged in
  if (!authStore.isAuthenticated) {
    console.log("User not logged in");
    // If the user is not on the login page, redirect to the login page
    if (to.name !== "Login") {
      return next("/login");
    }

    // If the user is on the login page, let him pass
    return next();
  }

  next();
});

router.beforeResolve(async (_to, _from, next) => {
  const app = useAppStore();
  next();

  app.setLoading(false);
});

export default router;
