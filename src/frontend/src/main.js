//import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'
import RegisterApp from './RegisterApp.vue'
import router from './router'

import 'bootstrap/dist/css/bootstrap.css';
import 'bootstrap-vue/dist/bootstrap-vue.css';

const app = createApp(App)
const registerApp = createApp(RegisterApp)

app.use(router)
registerApp.use(router)

app.mount('#app')
registerApp.mount('#register')