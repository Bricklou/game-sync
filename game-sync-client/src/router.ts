import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import { useAppStore } from "./store/modules/app";
import { useAuthStore } from "./store/modules/auth";

declare module "vue-router" {
  interface RouteMeta {
    fullScreen?: boolean;
    transition?: "fade" | "slide-left" | "slide-right";
    navigationIndex?: number;
  }
}

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    redirect: "/dashboard/home",
  },
  {
    path: "/setup",
    name: "Setup",
    component: () => import("./pages/SetupPage.vue"),
    meta: {
      fullScreen: true,
    },
  },
  {
    path: "/login",
    name: "Login",
    component: () => import("./pages/LoginPage.vue"),
    meta: {
      fullScreen: true,
    },
  },
  {
    path: "/dashboard",
    component: () => import("./components/partials/MainLayout.vue"),
    children: [
      {
        path: "/dashboard/home",
        name: "Home",
        component: () => import("./pages/dashboard/HomePage.vue"),
        meta: {
          navigationIndex: 0,
        },
      },
      {
        path: "/dashboard/games",
        meta: {
          navigationIndex: 1,
        },
        children: [
          {
            path: "/dashboard/games",
            name: "Games",
            component: () => import("./pages/dashboard/games/GamesPage.vue"),
          },
          {
            path: "/dashboard/games/new",
            name: "AddGame",
            component: () => import("./pages/dashboard/games/AddGamePage.vue"),
          },
          {
            path: "/dashboard/games/:id",
            name: "GameView",
            component: () => import("./pages/dashboard/games/GameViewPage.vue"),
          },
        ],
      },
      {
        path: "/dashboard/account",
        name: "Account",
        component: () => import("./pages/dashboard/AccountPage.vue"),
        meta: {
          navigationIndex: 2,
        },
      },
    ],
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
  } else {
    if (to.name === "Setup") {
      return next("/dashboard/home");
    }
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
  } else {
    if (to.name === "Login") {
      return next("/dashboard/home");
    }
  }

  next();
});

router.beforeResolve(async (_to, _from, next) => {
  const app = useAppStore();
  next();

  app.setLoading(false);
});

router.afterEach(async (to, from) => {
  if (["Login", "Setup"].includes(to.name as string)) {
    to.meta.transition = "fade";

    return;
  }

  const toDepth = to.path.split("/").length;
  const fromDepth = from.path.split("/").length;

  if (toDepth < fromDepth) {
    to.meta.transition = "fade";
  } else if (
    to.meta.navigationIndex !== undefined &&
    from.meta.navigationIndex !== undefined
  ) {
    const direction = to.meta.navigationIndex < from.meta.navigationIndex;
    to.meta.transition = direction ? "slide-right" : "slide-left";
  }
});

export default router;
