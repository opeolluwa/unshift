---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: Unshift
  text: Kafka admin UI
  tagline: Manage and inspect your Kafka cluster from the browser — and publish messages straight from the console.
  actions:
    - theme: brand
      text: Getting started
      link: /guide/getting-started
    - theme: alt
      text: API reference
      link: /api/

features:
  - icon: 📊
    title: Cluster overview
    details: Bootstrap servers, topic and partition counts, preferred leader percentage and under-replicated partitions at a glance.
  - icon: 🗂️
    title: Browse & create topics
    details: List every topic with partitions, replication health and custom configs; create a topic with a chosen partition count and replication factor.
  - icon: 📨
    title: Read messages
    details: Read messages from a topic across all partitions from the earliest offset, with a bounded, adjustable limit.
  - icon: 🚀
    title: Publish messages
    details: Produce a message with a key and payload to any topic directly from the UI.
  - icon: 💾
    title: Saved messages
    details: Save reusable message templates to a local store and republish them with one click.
  - icon: 📦
    title: Single image
    details: The Rust API and the built frontend are served together on one port — no extra proxy required.
---
