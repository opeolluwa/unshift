// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ["@nuxt/eslint", "@nuxt/ui", "@pinia/nuxt"],

  devtools: {
    enabled: true,
  },

  css: ["~/assets/css/main.css"],

  routeRules: {
    "/": { prerender: true },
  },

  compatibilityDate: "2026-06-30",

  app: {
    pageTransition: { name: "slide-left", mode: "out-in" },
    head: {
      titleTemplate: "%s | Unshift",
      meta: [
        { name: "robots", content: "noindex, nofollow" },
        { name: "description", content: "Kafka admin UI" },
        { name: "theme-color", content: "#ffffff" },
      ],
    },
  },

  eslint: {
    config: {
      stylistic: {
        commaDangle: "never",
        braceStyle: "1tbs",
      },
    },
  },
});
