<script setup>
const sidebarOpen = ref(false)

const route = useRoute()
watch(() => route.path, () => {
  sidebarOpen.value = false
})
</script>

<template>
  <div class="flex h-dvh">
    <aside class="hidden lg:flex w-60 shrink-0 flex-col border-r border-gray-200 dark:border-white/10 bg-white dark:bg-gray-900">
      <AppSidebar />
    </aside>

    <div class="flex flex-1 flex-col min-w-0">
      <header class="flex items-center gap-3 border-b border-gray-200 dark:border-white/10 bg-white dark:bg-gray-900 px-4 h-14 shrink-0 lg:hidden">
        <button
          class="flex size-9 items-center justify-center rounded-lg hover:bg-gray-100 dark:hover:bg-white/5 transition-colors"
          @click="sidebarOpen = true"
        >
          <UIcon
            name="i-lucide-menu"
            class="size-5 text-gray-600 dark:text-gray-400"
          />
        </button>
        <NuxtLink
          to="/"
          class="flex items-center gap-2"
        >
          <div class="flex size-7 items-center justify-center rounded-lg bg-primary">
            <UIcon
              name="i-lucide-arrow-up-right"
              class="size-3.5 text-white"
            />
          </div>
          <span class="font-bold tracking-tight">
            <span class="text-primary">Un</span>SHIFT
          </span>
        </NuxtLink>
      </header>

      <main class="flex-1 overflow-y-auto bg-gray-50/50 dark:bg-gray-950">
        <div class="mx-auto max-w-6xl px-4 py-6 sm:px-6 lg:px-8">
          <slot />
        </div>
      </main>
    </div>

    <ClientOnly>
      <Teleport to="body">
        <Transition name="overlay">
          <div
            v-if="sidebarOpen"
            class="fixed inset-0 z-40 bg-black/40 backdrop-blur-sm lg:hidden"
            @click="sidebarOpen = false"
          />
        </Transition>

        <Transition name="sidebar">
          <aside
            v-if="sidebarOpen"
            class="fixed inset-y-0 left-0 z-50 w-60 bg-white dark:bg-gray-900 shadow-xl lg:hidden"
          >
            <AppSidebar @navigate="sidebarOpen = false" />
          </aside>
        </Transition>
      </Teleport>
    </ClientOnly>
  </div>
</template>

<style scoped>
.overlay-enter-active,
.overlay-leave-active {
  transition: opacity 0.2s ease;
}
.overlay-enter-from,
.overlay-leave-to {
  opacity: 0;
}
.sidebar-enter-active,
.sidebar-leave-active {
  transition: transform 0.2s ease;
}
.sidebar-enter-from,
.sidebar-leave-to {
  transform: translateX(-100%);
}
</style>
