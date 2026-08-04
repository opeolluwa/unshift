<script setup lang="ts">
import { h, resolveComponent } from 'vue'
import type { TableColumn } from '@nuxt/ui'
import type { TopicConfig } from '../../bindings/TopicConfig'

const UBadge = resolveComponent('UBadge')

const route = useRoute()

const kafkaStore = useKafkaStore()
const { topic, loading, error } = storeToRefs(kafkaStore)

const topicName = computed(() => {
  const name = route.params.name
  return Array.isArray(name) ? name[0] : name
})

await kafkaStore.getTopic(topicName.value ?? '')

const summaryRows = computed(() => {
  const t = topic.value
  if (!t) {
    return []
  }

  return [
    { metric: 'Partitions', value: String(t.partitions) },
    { metric: '% Preferred', value: `${t.preferredLeaderPercent}%` },
    { metric: '# Under-replicated', value: String(t.underReplicated) },
    { metric: 'Custom Config', value: String(t.customConfigs) }
  ]
})

const summaryColumns: TableColumn<{ metric: string, value: string }>[] = [
  {
    accessorKey: 'metric',
    header: 'Metric'
  },
  {
    accessorKey: 'value',
    header: 'Value',
    meta: {
      class: {
        th: 'text-right',
        td: 'text-right font-medium'
      }
    }
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
  <div class="flex flex-col flex-1 w-full gap-8">
    <UButton
      to="/"
      icon="i-lucide-arrow-left"
      label="Back to topics"
      variant="subtle"
      color="neutral"
      class="self-start"
    />

    <div class="flex flex-col gap-2">
      <AppPageHeader
        :title="topicName ?? ''"
        subtitle="Topic details"
      />

      <AppEmptyState
        v-if="!topic && error"
        icon="heroicons:exclamation-triangle"
        title="Topic not found"
        :description="error"
      />

      <template v-else-if="topic">
        <UTable
          :data="summaryRows"
          :columns="summaryColumns"
        />

        <div class="flex flex-col gap-2">
          <AppLeadingText>Custom configs</AppLeadingText>

          <UTable
            v-if="topic.configs.length"
            :data="topic.configs"
            :columns="configColumns"
          />
          <p
            v-else
            class="text-sm text-muted"
          >
            No custom configs set for this topic.
          </p>
        </div>
      </template>

      <div
        v-else-if="loading"
        class="text-sm text-muted"
      >
        Loading topic...
      </div>
    </div>
  </div>
</template>
