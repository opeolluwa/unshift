<script setup lang="ts">
import { h, resolveComponent } from 'vue'
import type { TableColumn } from '@nuxt/ui'
import type { TopicSummary } from '../bindings/TopicSummary'

const UBadge = resolveComponent('UBadge')
const NuxtLink = resolveComponent('NuxtLink')

const kafkaStore = useKafkaStore()
const { overview } = storeToRefs(kafkaStore)

await kafkaStore.fetchClusterOverview()

type OverviewRow = {
  metric: string
  value: string
}

const overviewRows = computed<OverviewRow[]>(() => {
  const o = overview.value
  if (!o) {
    return []
  }

  return [
    { metric: 'Bootstrap servers', value: o.bootstrapServers },
    { metric: 'Total topics', value: String(o.totalTopics) },
    { metric: 'Total partitions', value: String(o.totalPartitions) },
    {
      metric: 'Total preferred partition leader',
      value: `${o.preferredPartitionLeaderPercentage}%`
    },
    { metric: 'Total under-replicated partitions', value: String(o.totalUnderReplicatedPartitions) }
  ]
})

const overviewColumns: TableColumn<OverviewRow>[] = [
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

await kafkaStore.fetchTopics()

const data = computed(() => kafkaStore.topics)

const preferredLeaderColor = (percent: number) =>
  percent === 100 ? 'success' : percent > 0 ? 'warning' : 'error'

const columns: TableColumn<TopicSummary>[] = [
  {
    accessorKey: 'topic',
    header: 'Topic',
    cell: ({ row }) => {
      const topic = row.getValue<string>('topic')
      return h(
        NuxtLink,
        {
          to: `/topics/${encodeURIComponent(topic)}`,
          class: 'text-primary cursor-pointer hover:underline'
        },
        () => topic
      )
    }
  },
  {
    accessorKey: 'partitions',
    header: 'Partitions',
    meta: {
      class: {
        th: 'text-right',
        td: 'text-right font-medium'
      }
    }
  },
  {
    accessorKey: 'preferredLeaderPercent',
    header: '% Preferred',
    cell: ({ row }) => {
      const percent = row.getValue<number>('preferredLeaderPercent')
      return h(
        UBadge,
        { variant: 'subtle', color: preferredLeaderColor(percent) },
        () => `${percent}%`
      )
    }
  },
  {
    accessorKey: 'underReplicated',
    header: '# Under-replicated',
    cell: ({ row }) => {
      const count = row.getValue<number>('underReplicated')
      return h(
        UBadge,
        { variant: 'subtle', color: count > 0 ? 'error' : 'success' },
        () => String(count)
      )
    }
  },
  {
    accessorKey: 'customConfigs',
    header: 'Custom Config',
    cell: ({ row }) => {
      const count = row.getValue<number>('customConfigs')
      return count > 0 ? `${count} config${count > 1 ? 's' : ''}` : '—'
    }
  }
]

const globalFilter = ref('')
const showCreateTopic = ref(false)
</script>

<template>
  <div class="flex flex-col flex-1 w-full gap-8">
    <div class="flex flex-col gap-2">
      <AppLeadingText>Kafka cluster overview</AppLeadingText>

      <UTable
        :data="overviewRows"
        :columns="overviewColumns"
      />
    </div>

    <div class="flex flex-col gap-2">
      <AppLeadingText>Topics</AppLeadingText>
      <div class="flex items-center justify-between gap-4 px-4 py-3.5 border-b border-accented">
        <UInput
          v-model="globalFilter"
          class="max-w-sm"
          placeholder="Filter..."
        />

        <AppButton
          icon="i-lucide-plus"
          size="lg"
          @click="showCreateTopic = true"
        >
          Create topic
        </AppButton>
      </div>
      <UTable
        v-model:global-filter="globalFilter"
        :data="data"
        :columns="columns"
      />
    </div>

    <AppCreateTopicModal v-model:open="showCreateTopic" />
  </div>
</template>
