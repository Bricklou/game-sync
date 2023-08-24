import { RouteRecordRaw, createRouter, createWebHistory } from "vue-router";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "Home",
    component: () => "<h1>Home</h1>",
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
