import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import { useAppStore } from "./store/modules/app";

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
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach(async (to, _from, next) => {
  // Check if the application is loaded
  const app = useAppStore();

  // Call this part only when the application is booting
  if (!app.loaded) {
    console.log("Application not loaded");
    app.setLoading(true);

    await app.loadLocalInfos();
  }

  // If the application is not configured, redirect to the "first run" page
  if (!app.configured && to.name !== "Setup") {
    console.log("Application not configured");
    return next("/setup");
  }

  next();
});

router.beforeResolve(async (_to, _from, next) => {
  const app = useAppStore();
  next();

  app.setLoading(false);
});

export default router;
