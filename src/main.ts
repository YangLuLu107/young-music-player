import { createApp } from "vue";
import App from "./App.vue";
import {NButton, NFlex, NImage, create} from 'naive-ui'
// 通用字体
import 'vfonts/Lato.css'
// 等宽字体
import 'vfonts/FiraCode.css'

const naive = create({
    components: [NButton, NFlex, NImage]
})

const app = createApp(App);
app.use(naive);
app.mount("#app");
