

import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHashHistory } from "vue-router";

import Home from "./views/Home.vue";
import Languages from "./views/languages/index.vue";
import VueVirtualScroller from 'vue-virtual-scroller'
import "./main.css";
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import { checkForUpdates } from "./updater";
import Test from "./views/Test.vue";


checkForUpdates();

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", name: "Home", component: Home },
    { path: "/languages", name: "Languages", component: Languages },
    { path: "/test", name: "test", component: Test }
  ],
});

createApp(App).use(router).use(VueVirtualScroller).mount("#app");
