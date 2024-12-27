import {createI18n} from "vue-i18n"
import en from "./locales/en.json"
import fr from "./locales/fr.json"

export default createI18n({
    locale: 'fr',
    fallbackLocale: 'en',
    legacy: false,
    globalInjection: true,
    messages: {fr},
})

export const SUPPORTED_LOCALES = ['fr', 'en']
