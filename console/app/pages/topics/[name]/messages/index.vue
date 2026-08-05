<script setup lang="ts">
import { h } from 'vue'
import type { TableColumn } from '@nuxt/ui'
import type { Message } from '~/bindings/Message'

const route = useRoute()

useHead({ title: 'Messages' })

const kafkaStore = useKafkaStore()
const { messages, loading, error } = storeToRefs(kafkaStore)

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

const refresh = () => kafkaStore.fetchMessages(topicName.value ?? '')

await refresh()
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
          :loading="loading"
          @click="refresh"
        />
      </div>

      <AppEmptyState
        v-if="!messages.length && error"
        icon="heroicons:exclamation-triangle"
        title="Failed to load messages"
        :description="error"
      />

      <AppEmptyState
        v-else-if="!messages.length && !loading"
        icon="heroicons:inbox"
        title="No messages"
        description="No messages were read from the beginning of this topic."
      />

      <UTable
        v-else
        :data="messages"
        :columns="columns"
      />
    </div>
  </div>
</template>
