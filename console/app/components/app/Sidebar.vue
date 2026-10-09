<script setup lang="ts">
const emit = defineEmits<{
  navigate: []
}>()

const route = useRoute()
const kafkaStore = useKafkaStore()

const isHome = computed(() => route.path === '/')

const activeTopicName = computed(() => {
  const name = route.params.name
  if (!name) return null
  return Array.isArray(name) ? name[0] : name
})

onMounted(() => {
  if (!kafkaStore.topics.length) {
    kafkaStore.fetchTopics()
  }
})
</script>

<template>
  <div class="flex h-full flex-col">
    <div class="flex items-center gap-2.5 px-4 h-14 shrink-0 border-b border-gray-200 dark:border-white/10">
      <NuxtLink
        to="/"
        class="flex items-center gap-2.5"
        @click="emit('navigate')"
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
    </div>

    <nav class="flex-1 overflow-y-auto px-3 py-3">
      <NuxtLink
        to="/"
        class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors"
        :class="isHome
          ? 'bg-primary/10 text-primary'
          : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-white/5 hover:text-gray-900 dark:hover:text-white'"
        @click="emit('navigate')"
      >
        <UIcon
          name="i-lucide-layout-dashboard"
          class="size-4 shrink-0"
        />
        Dashboard
      </NuxtLink>

      <p class="px-3 pt-5 pb-2 text-[10px] font-semibold uppercase tracking-widest text-gray-400 dark:text-white/30">
        Topics
      </p>

      <div class="space-y-0.5">
        <NuxtLink
          v-for="topic in kafkaStore.topics"
          :key="topic.topic"
          :to="`/topics/${encodeURIComponent(topic.topic)}`"
          class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors"
          :class="activeTopicName === topic.topic
            ? 'bg-primary/10 text-primary font-medium'
            : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-white/5 hover:text-gray-900 dark:hover:text-white'"
          @click="emit('navigate')"
        >
          <UIcon
            name="i-lucide-hash"
            class="size-4 shrink-0"
          />
          <span class="truncate">{{ topic.topic }}</span>
          <span class="ml-auto shrink-0 text-[10px] tabular-nums text-gray-400 dark:text-white/25">
            {{ topic.partitions }}
          </span>
        </NuxtLink>
      </div>

      <p
        v-if="!kafkaStore.topics.length && !kafkaStore.loading"
        class="px-3 py-2 text-xs text-gray-400 dark:text-white/25"
      >
        No topics yet
      </p>
    </nav>

    <div class="shrink-0 border-t border-gray-200 dark:border-white/10 px-4 py-3 flex items-center justify-between">
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
</template>
