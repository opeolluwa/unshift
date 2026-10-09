<script setup lang="ts">
import { h, resolveComponent } from 'vue'
import type { TableColumn } from '@nuxt/ui'
import type { TopicSummary } from '../bindings/TopicSummary'

const UBadge = resolveComponent('UBadge')
const NuxtLink = resolveComponent('NuxtLink')

const kafkaStore = useKafkaStore()
const { overview } = storeToRefs(kafkaStore)

await kafkaStore.fetchClusterOverview()
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
          class: 'text-primary cursor-pointer hover:underline font-medium'
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
        td: 'text-right tabular-nums'
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
    header: 'Under-replicated',
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
    header: 'Configs',
    cell: ({ row }) => {
      const count = row.getValue<number>('customConfigs')
      return count > 0 ? `${count}` : '—'
    }
  }
]

const globalFilter = ref('')
const showCreateTopic = ref(false)
</script>

<template>
  <div class="flex flex-col gap-6">
    <div>
      <h1 class="text-2xl font-bold">
        Dashboard
      </h1>
      <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
        Kafka cluster overview and management
      </p>
    </div>

    <div
      v-if="overview"
      class="grid grid-cols-2 gap-3 lg:grid-cols-4"
    >
      <AppCard>
        <div class="flex items-center gap-3">
          <div class="flex size-10 items-center justify-center rounded-xl bg-blue-50 dark:bg-blue-500/10">
            <UIcon
              name="i-lucide-server"
              class="size-5 text-blue-600 dark:text-blue-400"
            />
          </div>
          <div class="min-w-0">
            <p class="text-[11px] font-medium uppercase tracking-wide text-gray-500 dark:text-gray-400">
              Brokers
            </p>
            <p class="text-sm font-semibold truncate">
              {{ overview.bootstrapServers }}
            </p>
          </div>
        </div>
      </AppCard>

      <AppCard>
        <div class="flex items-center gap-3">
          <div class="flex size-10 items-center justify-center rounded-xl bg-violet-50 dark:bg-violet-500/10">
            <UIcon
              name="i-lucide-layers"
              class="size-5 text-violet-600 dark:text-violet-400"
            />
          </div>
          <div>
            <p class="text-[11px] font-medium uppercase tracking-wide text-gray-500 dark:text-gray-400">
              Topics
            </p>
            <p class="text-2xl font-bold">
              {{ overview.totalTopics }}
            </p>
          </div>
        </div>
      </AppCard>

      <AppCard>
        <div class="flex items-center gap-3">
          <div class="flex size-10 items-center justify-center rounded-xl bg-emerald-50 dark:bg-emerald-500/10">
            <UIcon
              name="i-lucide-git-branch"
              class="size-5 text-emerald-600 dark:text-emerald-400"
            />
          </div>
          <div>
            <p class="text-[11px] font-medium uppercase tracking-wide text-gray-500 dark:text-gray-400">
              Partitions
            </p>
            <p class="text-2xl font-bold">
              {{ overview.totalPartitions }}
            </p>
          </div>
        </div>
      </AppCard>

      <AppCard>
        <div class="flex items-center gap-3">
          <div
            class="flex size-10 items-center justify-center rounded-xl"
            :class="overview.totalUnderReplicatedPartitions > 0
              ? 'bg-red-50 dark:bg-red-500/10'
              : 'bg-emerald-50 dark:bg-emerald-500/10'"
          >
            <UIcon
              name="i-lucide-heart-pulse"
              class="size-5"
              :class="overview.totalUnderReplicatedPartitions > 0
                ? 'text-red-600 dark:text-red-400'
                : 'text-emerald-600 dark:text-emerald-400'"
            />
          </div>
          <div>
            <p class="text-[11px] font-medium uppercase tracking-wide text-gray-500 dark:text-gray-400">
              Health
            </p>
            <p class="text-2xl font-bold">
              {{ overview.preferredPartitionLeaderPercentage }}%
            </p>
          </div>
        </div>
      </AppCard>
    </div>

    <AppCard title="Topics">
      <template #trailing>
        <div class="flex items-center gap-2">
          <UInput
            v-model="globalFilter"
            icon="i-lucide-search"
            placeholder="Filter topics..."
            size="sm"
            class="w-48"
          />
          <UButton
            icon="i-lucide-plus"
            label="Create"
            size="sm"
            color="primary"
            @click="showCreateTopic = true"
          />
        </div>
      </template>

      <UTable
        v-model:global-filter="globalFilter"
        :data="data"
        :columns="columns"
      />
    </AppCard>

    <AppCreateTopicModal v-model:open="showCreateTopic" />
  </div>
</template>
