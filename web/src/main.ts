import { createApp } from "vue"
import { createPinia } from "pinia"
import { addCollection } from "@iconify/vue"
import { icons as icIcons } from "@iconify-json/ic"
import App from "./App.vue"
import router from "./router"
import "./style.css"

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.mount("#app")
