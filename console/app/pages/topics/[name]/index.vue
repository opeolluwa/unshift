<script setup lang="ts">
import { h, resolveComponent } from 'vue'
import type { TableColumn, TableRow } from '@nuxt/ui'
import type { TopicConfig } from '~/bindings/TopicConfig.ts'
import type { Message } from '~/bindings/Message'

const UBadge = resolveComponent('UBadge')

const route = useRoute()

const kafkaStore = useKafkaStore()
const { topic, messages, loading, error } = storeToRefs(kafkaStore)

const topicName = computed(() => {
  const name = route.params.name
  return Array.isArray(name) ? name[0] : name
})

const encodedTopic = computed(() => encodeURIComponent(topicName.value ?? ''))

await kafkaStore.getTopic(topicName.value ?? '')

const messagesPending = ref(true)

async function refreshMessages() {
  messagesPending.value = true
  try {
    await kafkaStore.fetchMessages(topicName.value ?? '')
  } finally {
    messagesPending.value = false
  }
}

onMounted(refreshMessages)

const selectedMessage = ref<Message | null>(null)
const previewOpen = ref(false)

function previewMessage(_event: Event, row: TableRow<Message>) {
  selectedMessage.value = row.original
  previewOpen.value = true
}

const messageColumns: TableColumn<Message>[] = [
  {
    accessorKey: 'partition',
    header: 'Partition',
    meta: { class: { td: 'font-mono tabular-nums' } }
  },
  {
    accessorKey: 'offset',
    header: 'Offset',
    meta: { class: { td: 'font-mono tabular-nums' } }
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

const configColumns: TableColumn<TopicConfig>[] = [
  {
    accessorKey: 'name',
    header: 'Name'
  },
  {
    accessorKey: 'value',
    header: 'Value',
    cell: ({ row }) => row.getValue('value') ?? '—'
  },
  {
    accessorKey: 'readOnly',
    header: 'Read only',
    cell: ({ row }) => {
      const readOnly = row.getValue<boolean>('readOnly')
      return h(
        UBadge,
        { variant: 'subtle', color: readOnly ? 'warning' : 'success' },
        () => (readOnly ? 'Yes' : 'No')
      )
    }
  }
]
</script>

<template>
  <div class="flex flex-col gap-6">
    <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <h1 class="text-2xl font-bold">
          {{ topicName }}
        </h1>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Topic overview and configuration
        </p>
      </div>
      <AppTopicTabs :topic-name="topicName ?? ''" />
    </div>

    <AppEmptyState
      v-if="!topic && error"
      icon="heroicons:exclamation-triangle"
      title="Topic not found"
      :description="error"
    />

    <template v-else-if="topic">
      <div class="grid grid-cols-2 gap-3 sm:grid-cols-4">
        <AppCard>
          <div class="flex items-center gap-3">
            <div class="flex size-9 items-center justify-center rounded-lg bg-violet-50 dark:bg-violet-500/10">
              <UIcon
                name="i-lucide-layers"
                class="size-4 text-violet-600 dark:text-violet-400"
              />
            </div>
            <div>
              <p class="text-[11px] font-medium uppercase tracking-wide text-gray-500 dark:text-gray-400">
                Partitions
              </p>
              <p class="text-xl font-bold">
                {{ topic.partitions }}
              </p>
            </div>
          </div>
        </AppCard>

        <AppCard>
          <div class="flex items-center gap-3">
            <div class="flex size-9 items-center justify-center rounded-lg bg-emerald-50 dark:bg-emerald-500/10">
              <UIcon
                name="i-lucide-crown"
                class="size-4 text-emerald-600 dark:text-emerald-400"
              />
            </div>
            <div>
              <p class="text-[11px] font-medium uppercase tracking-wide text-gray-500 dark:text-gray-400">
                Preferred
              </p>
              <p class="text-xl font-bold">
                {{ topic.preferredLeaderPercent }}%
              </p>
            </div>
          </div>
        </AppCard>

        <AppCard>
          <div class="flex items-center gap-3">
            <div
              class="flex size-9 items-center justify-center rounded-lg"
              :class="topic.underReplicated > 0
                ? 'bg-red-50 dark:bg-red-500/10'
                : 'bg-emerald-50 dark:bg-emerald-500/10'"
            >
              <UIcon
                name="i-lucide-shield-check"
                class="size-4"
                :class="topic.underReplicated > 0
                  ? 'text-red-600 dark:text-red-400'
                  : 'text-emerald-600 dark:text-emerald-400'"
              />
            </div>
            <div>
              <p class="text-[11px] font-medium uppercase tracking-wide text-gray-500 dark:text-gray-400">
                Under-repl.
              </p>
              <p class="text-xl font-bold">
                {{ topic.underReplicated }}
              </p>
            </div>
          </div>
        </AppCard>

        <AppCard>
          <div class="flex items-center gap-3">
            <div class="flex size-9 items-center justify-center rounded-lg bg-amber-50 dark:bg-amber-500/10">
              <UIcon
                name="i-lucide-settings"
                class="size-4 text-amber-600 dark:text-amber-400"
              />
            </div>
            <div>
              <p class="text-[11px] font-medium uppercase tracking-wide text-gray-500 dark:text-gray-400">
                Configs
              </p>
              <p class="text-xl font-bold">
                {{ topic.customConfigs }}
              </p>
            </div>
          </div>
        </AppCard>
      </div>

      <AppCard title="Messages">
        <template #trailing>
          <div class="flex items-center gap-2">
            <UButton
              icon="i-lucide-refresh-cw"
              label="Refresh"
              size="sm"
              variant="outline"
              color="neutral"
              :loading="messagesPending"
              @click="refreshMessages"
            />
            <UButton
              :to="`/topics/${encodedTopic}/messages/publish`"
              icon="i-lucide-send"
              label="Publish"
              size="sm"
              color="primary"
            />
          </div>
        </template>

        <div
          v-if="messagesPending"
          class="flex flex-col gap-2"
        >
          <USkeleton
            v-for="row in 5"
            :key="row"
            class="h-10 w-full"
          />
        </div>

        <AppEmptyState
          v-else-if="!messages.length"
          icon="heroicons:inbox"
          title="No messages"
          description="No messages were read from the beginning of this topic."
        />

        <UTable
          v-else
          :data="messages"
          :columns="messageColumns"
          class="cursor-pointer"
          @select="previewMessage"
        />
      </AppCard>

      <AppCard
        v-if="topic.configs.length"
        title="Custom configuration"
      >
        <UTable
          :data="topic.configs"
          :columns="configColumns"
        />
      </AppCard>
    </template>

    <div
      v-else-if="loading"
      class="text-sm text-muted"
    >
      Loading topic...
    </div>

    <AppMessagePreview
      v-model:open="previewOpen"
      :message="selectedMessage"
    />
  </div>
</template>
