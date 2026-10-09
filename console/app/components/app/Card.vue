<script setup lang="ts">
defineProps<{
  title?: string
}>()
</script>

<template>
  <section
    class="rounded-xl border border-gray-200 bg-white p-4 dark:border-gray-400/20 dark:bg-gray-900/50"
  >
    <header
      v-if="title || $slots.header || $slots.trailing"
      class="flex items-center justify-between gap-3 mb-3"
    >
      <slot name="header">
        <div class="flex min-w-0 items-center">
          <h3
            v-if="title"
            class="truncate text-sm font-semibold text-gray-900 dark:text-white"
          >
            {{ title }}
          </h3>
        </div>
      </slot>

      <slot name="trailing" />
    </header>

    <slot />
  </section>
</template>
