import {createApp} from "vue";
import ElementPlus from 'element-plus'
import 'virtual:uno.css';
import "virtual:svg-icons-register";
import "@/style/index.css";
import App from "./App.vue";

const app = createApp(App);
app.use(ElementPlus);
app.mount("#app");
