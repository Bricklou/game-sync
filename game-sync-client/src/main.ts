import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import router from "./router";
import store from "@/store/setup";

createApp(App).use(router).use(store).mount("#app");
