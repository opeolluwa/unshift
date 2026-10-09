import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  // Served under the GitHub Pages project-site subpath (opeolluwa.github.io/unshift)
  base: "/unshift/",
  title: "Unshift",
  description:
    "A browser-based Kafka admin UI for managing and inspecting your cluster, with the ability to publish messages directly from the console.",
  cleanUrls: true,

  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      {
        text: "Guide",
        link: "/guide/getting-started",
        activeMatch: "/guide/",
      },
      { text: "API", link: "/api/", activeMatch: "/api/" },
      { text: "GitHub", link: "https://github.com/opeolluwa/unshift" },
    ],

    sidebar: [
      {
        text: "Guide",
        items: [
          { text: "Getting started", link: "/guide/getting-started" },
          { text: "Development", link: "/guide/development" },
          { text: "Configuration", link: "/guide/configuration" },
          { text: "Architecture", link: "/guide/architecture" },
          { text: "Using the console", link: "/guide/ui" },
        ],
      },
      {
        text: "API",
        items: [{ text: "REST reference", link: "/api/" }],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/opeolluwa/unshift" },
    ],

    editLink: {
      pattern:
        "https://github.com/opeolluwa/unshift/edit/main/docs/:path",
      text: "Edit this page on GitHub",
    },

    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © Unshift contributors",
    },
  },
});
