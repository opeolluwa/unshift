<script setup lang="ts">
import { h } from 'vue'
import type { TableColumn, TableRow } from '@nuxt/ui'
import type { Message } from '~/bindings/Message'

const route = useRoute()

useHead({ title: 'Messages' })

const kafkaStore = useKafkaStore()
const { messages, error } = storeToRefs(kafkaStore)

const topicName = computed(() => {
  const name = route.params.name
  return Array.isArray(name) ? name[0] : name
})

const encodedTopic = computed(() => encodeURIComponent(topicName.value ?? ''))

const columns: TableColumn<Message>[] = [
  {
    accessorKey: 'partition',
    header: 'Partition',
    meta: {
      class: {
        td: 'font-mono'
      }
    }
  },
  {
    accessorKey: 'offset',
    header: 'Offset',
    meta: {
      class: {
        td: 'font-mono'
      }
    }
  },
  {
    accessorKey: 'key',
    header: 'Key',
    cell: ({ row }) => row.getValue('key') ?? '—'
  },
  {
    accessorKey: 'payload',
    header: 'Payload',
    cell: ({ row }) => h('div', { class: 'truncate max-w-md font-mono' }, row.getValue('payload'))
  }
]

const selectedMessage = ref<Message | null>(null)
const previewOpen = ref(false)

function previewMessage(_event: Event, row: TableRow<Message>) {
  selectedMessage.value = row.original
  previewOpen.value = true
}

// Starts true so the first paint is the skeleton rather than a flash of the
// "No messages" empty state. Page-local, because the store's `loading` flag is
// shared by every action.
const pending = ref(true)

async function refresh() {
  pending.value = true

  try {
    await kafkaStore.fetchMessages(topicName.value ?? '')
  } finally {
    pending.value = false
  }
}

// Deliberately not a top-level `await`: that makes setup async, so Nuxt holds
// the route transition until the read resolves and the user sees no loader.
onMounted(refresh)
</script>

<template>
  <div class="flex flex-col flex-1 w-full gap-8">
    <UButton
      :to="`/topics/${encodedTopic}`"
      icon="i-lucide-arrow-left"
      label="Back to topic"
      variant="subtle"
      color="neutral"
      class="self-start"
    />

    <div class="flex flex-col gap-2">
      <div class="flex items-center justify-between gap-4">
        <AppPageHeader
          title="Messages"
          :subtitle="topicName ?? ''"
        />

        <UButton
          icon="i-lucide-refresh-cw"
          label="Refresh"
          variant="outline"
          color="neutral"
          :loading="pending"
          @click="refresh"
        />
      </div>

      <div
        v-if="pending"
        class="flex flex-col gap-2 pt-2"
      >
        <USkeleton
          v-for="row in 5"
          :key="row"
          class="h-10 w-full"
        />
      </div>

      <AppEmptyState
        v-else-if="error"
        icon="heroicons:exclamation-triangle"
        title="Failed to load messages"
        :description="error"
      />

      <AppEmptyState
        v-else-if="!messages.length"
        icon="heroicons:inbox"
        title="No messages"
        description="No messages were read from the beginning of this topic."
      />

      <UTable
        v-else
        :data="messages"
        :columns="columns"
        class="cursor-pointer"
        @select="previewMessage"
      />
    </div>

    <AppMessagePreview
      v-model:open="previewOpen"
      :message="selectedMessage"
    />
  </div>
</template>
