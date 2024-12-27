import {createApp} from 'vue'
import App from './App.vue'
import './registerServiceWorker'
import router from './router'
import './style.css'

import {createPinia} from 'pinia'
import i18n from "@/i18n";

const pinia = createPinia()
const app = createApp(App);

app.directive('click-outside', {
    bind() {
        this.event = event => this.vm.$emit(this.expression, event)
        this.el.addEventListener('click', this.stopProp)
        document.body.addEventListener('click', this.event)
    },
    unbind() {
        this.el.removeEventListener('click', this.stopProp)
        document.body.removeEventListener('click', this.event)
    },

    stopProp(event) {
        event.stopPropagation()
    }
})

app.use(router)
    .use(pinia)
    .use(i18n)
    .mount('#app')
