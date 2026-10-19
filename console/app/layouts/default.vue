<template>
  <div class="h-dvh flex flex-col bg-gray-50/50 dark:bg-gray-950">
    <header class="shrink-0 border-b border-gray-200 dark:border-white/10 bg-white dark:bg-gray-900">
      <div class="flex items-center justify-between px-4 h-14 sm:px-6 lg:px-48">
        <NuxtLink
          to="/"
          class="flex items-center gap-2.5"
        >
          <div class="flex size-8 items-center justify-center rounded-lg bg-primary">
            <UIcon
              name="i-lucide-arrow-up-right"
              class="size-4 text-white"
            />
          </div>
          <span class="text-lg font-bold tracking-tight">
            <span class="text-primary">Un</span>SHIFT
          </span>
        </NuxtLink>
        <div class="flex items-center gap-1">
          <UColorModeButton size="sm" />
          <UButton
            to="https://github.com/opeolluwa/unshift"
            target="_blank"
            icon="i-simple-icons-github"
            aria-label="GitHub"
            color="neutral"
            variant="ghost"
            size="sm"
          />
        </div>
      </div>
    </header>

    <main class="flex-1 overflow-y-auto">
      <div class="px-4 py-6 sm:px-6 lg:px-48">
        <slot />
      </div>
    </main>
  </div>
</template>
