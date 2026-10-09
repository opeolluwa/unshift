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

const columns: TableColumn<Message>[] = [
  {
    accessorKey: 'partition',
    header: 'Partition',
    meta: {
      class: {
        td: 'font-mono tabular-nums'
      }
    }
  },
  {
    accessorKey: 'offset',
    header: 'Offset',
    meta: {
      class: {
        td: 'font-mono tabular-nums'
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
    cell: ({ row }) => h('div', { class: 'truncate max-w-md font-mono text-xs' }, row.getValue('payload'))
  }
]

const selectedMessage = ref<Message | null>(null)
const previewOpen = ref(false)

function previewMessage(_event: Event, row: TableRow<Message>) {
  selectedMessage.value = row.original
  previewOpen.value = true
}

const pending = ref(true)

async function refresh() {
  pending.value = true

  try {
    await kafkaStore.fetchMessages(topicName.value ?? '')
  } finally {
    pending.value = false
  }
}

onMounted(refresh)
</script>

<template>
  <div class="flex flex-col gap-6">
    <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <h1 class="text-2xl font-bold">
          {{ topicName }}
        </h1>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Browse messages from this topic
        </p>
      </div>
      <AppTopicTabs :topic-name="topicName ?? ''" />
    </div>

    <AppCard title="Messages">
      <template #trailing>
        <UButton
          icon="i-lucide-refresh-cw"
          label="Refresh"
          size="sm"
          variant="outline"
          color="neutral"
          :loading="pending"
          @click="refresh"
        />
      </template>

      <div
        v-if="pending"
        class="flex flex-col gap-2"
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
    </AppCard>

    <AppMessagePreview
      v-model:open="previewOpen"
      :message="selectedMessage"
    />
  </div>
</template>
