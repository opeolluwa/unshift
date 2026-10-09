<script setup lang="ts">
const route = useRoute()

const props = defineProps<{
  topicName: string
}>()

const encodedTopic = computed(() => encodeURIComponent(props.topicName))

const tabs = computed(() => [
  { label: 'Overview', icon: 'i-lucide-info', to: `/topics/${encodedTopic.value}` },
  { label: 'Messages', icon: 'i-lucide-list', to: `/topics/${encodedTopic.value}/messages` },
  { label: 'Publish', icon: 'i-lucide-send', to: `/topics/${encodedTopic.value}/messages/publish` }
])

const activeIndex = computed(() => {
  const path = route.path
  if (path.endsWith('/publish')) return 2
  if (path.endsWith('/messages')) return 1
  return 0
})
</script>

<template>
  <div class="flex items-center gap-1 rounded-lg bg-gray-100 dark:bg-white/5 p-1">
    <NuxtLink
      v-for="(tab, index) in tabs"
      :key="tab.label"
      :to="tab.to"
      class="flex items-center gap-1.5 rounded-md px-3 py-1.5 text-sm transition-all"
      :class="activeIndex === index
        ? 'bg-white dark:bg-gray-800 text-gray-900 dark:text-white font-medium shadow-sm'
        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'"
    >
      <UIcon
        :name="tab.icon"
        class="size-3.5"
      />
      {{ tab.label }}
    </NuxtLink>
  </div>
</template>
