import { init, locale, addMessages, getLocaleFromNavigator } from "svelte-i18n";

import en from "./en.json";
import fr from "./fr.json";
import de from "./de.json";
import es from "./es.json";
import vi from "./vi.json";
import it from "./it.json";
import cn from "./cn.json";
import jp from "./jp.json";
import ru from "./ru.json";

export const supportedLanguages = [
    {
        name: "English",
        code: "en",
        json: en,
    },
    {
        name: "Français",
        code: "fr",
        json: fr,
    },
    {
        name: "Deutsch",
        code: "de",
        json: de,
    },
    {
        name: "Italiano",
        code: "it",
        json: it,
    },
    {
        name: "Español",
        code: "es",
        json: es,
    },
    {
        name: "Tiếng Việt",
        code: "vi",
        json: vi,
    },
    {
        name: "中文",
        code: "cn",
        json: cn,
    },
    {
        name: "日本",
        code: "jp",
        json: jp,
    },
    {
        name: "русский",
        code: "ru",
        json: ru,
    },
];

supportedLanguages.forEach(({ code, json }) => {
    addMessages(code, json);
});

init({
    fallbackLocale: "en",
    initialLocale: getStoredLocale(),
});

export function getStoredLocale(): string {
    return localStorage.getItem("openchat_locale") ?? getLocaleFromNavigator();
}

export function setLocale(code: string): void {
    locale.set(code);
    localStorage.setItem("openchat_locale", code);
}