import { useProgressStore } from "./store/modules/progress";
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

router.beforeEach(async (to, _from, next) => {
  const progress = useProgressStore();
  if (to.name) {
    progress.show();
  }
  next();
});

router.beforeResolve(async (_to, _from, next) => {
  const progress = useProgressStore();
  const app = useAppStore();
  progress.hide();
  next();

  app.setLoading(false);
});

router.beforeEach(async (to, _from, next) => {
  // Check if the application is loaded
  const app = useAppStore();
  const auth = useAuthStore();

  // Call this part only when the application is booting
  if (!app.loaded) {
    console.debug("Application not loaded");
    app.setLoading(true);
    await app.fetchAppData();

    // If the app is configured try to fetch the user
    if (app.configured) {
      console.debug("Fetching user");
      await auth.fetchUser();
    }
  }

  // If the application is not configured, redirect to the register page
  if (!app.configured && to.name !== "Register") {
    console.log("Application not configured");
    return next("/register");
  }

  // Then check if the user is logged in
  const authRequired = !to.meta.guest;

  // If the route require the user to be logged in and
  // the user is not logged in, redirect to the login page
  if (authRequired && !auth.isLoggedIn && to.name !== "Login") {
    auth.setReturnUrl(to.fullPath);
    return next("/login");
  }

  // If the route doesn't require the user to be logged in and
  // the user is logged in, redirect to the home page
  if (!authRequired && auth.isLoggedIn) {
    return next("/");
  }

  next();
});

export default router;
