<script setup lang="ts">
import type { TableColumn } from '@nuxt/ui'
import type { MessageHeader } from '~/bindings/MessageHeader'

const route = useRoute()
const kafkaStore = useKafkaStore()

useHead({ title: 'Message' })

const topicName = computed(() => {
  const name = route.params.name
  return Array.isArray(name) ? name[0] : name
})

const encodedTopic = computed(() => encodeURIComponent(topicName.value ?? ''))

const message = computed(() => kafkaStore.selectedMessage)

onMounted(() => {
  if (!message.value) {
    navigateTo(`/topics/${encodedTopic.value}/messages`)
  }
})

onBeforeUnmount(() => {
  kafkaStore.clearSelectedMessage()
})

const isJson = computed(() => {
  try {
    JSON.parse(message.value?.payload ?? '')
    return true
  } catch {
    return false
  }
})

const formattedPayload = computed(() => {
  const payload = message.value?.payload ?? ''
  return isJson.value ? JSON.stringify(JSON.parse(payload), null, 2) : payload
})

type Token = { text: string, class: string }

const TOKEN_PATTERN = /("(?:\\.|[^"\\])*")(\s*:)?|\b(true|false)\b|\bnull\b|(-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)|([{}[\],])/g

const tokens = computed<Token[]>(() => {
  const doc = formattedPayload.value

  if (!isJson.value) {
    return [{ text: doc, class: 'tok-plain' }]
  }

  const result: Token[] = []
  let lastIndex = 0

  for (const match of doc.matchAll(TOKEN_PATTERN)) {
    const [raw, string, colon, bool, number, punctuation] = match
    const index = match.index ?? 0

    if (index > lastIndex) {
      result.push({ text: doc.slice(lastIndex, index), class: 'tok-plain' })
    }

    if (string) {
      result.push({ text: string, class: colon ? 'tok-property' : 'tok-string' })
      if (colon) {
        result.push({ text: colon, class: 'tok-operator' })
      }
    } else if (bool) {
      result.push({ text: raw, class: 'tok-bool' })
    } else if (number) {
      result.push({ text: raw, class: 'tok-number' })
    } else if (punctuation) {
      result.push({ text: raw, class: 'tok-operator' })
    } else {
      result.push({ text: raw, class: 'tok-atom' })
    }

    lastIndex = index + raw.length
  }

  if (lastIndex < doc.length) {
    result.push({ text: doc.slice(lastIndex), class: 'tok-plain' })
  }

  return result
})

const headerColumns: TableColumn<MessageHeader>[] = [
  {
    accessorKey: 'key',
    header: 'Key',
    meta: { class: { td: 'font-mono text-sm' } }
  },
  {
    accessorKey: 'value',
    header: 'Value',
    cell: ({ row }) => row.getValue('value') ?? '—',
    meta: { class: { td: 'font-mono text-sm break-all' } }
  }
]

type CopyTarget = 'payload' | 'headers' | 'all'

const copiedTarget = ref<CopyTarget | null>(null)
let copyTimeout: ReturnType<typeof setTimeout> | undefined

async function copyToClipboard(target: CopyTarget) {
  if (!message.value) return

  let text: string

  switch (target) {
    case 'payload':
      text = message.value.payload
      break
    case 'headers':
      text = JSON.stringify(
        Object.fromEntries(message.value.headers.map(h => [h.key, h.value])),
        null,
        2
      )
      break
    case 'all':
      text = JSON.stringify(
        {
          partition: message.value.partition,
          offset: message.value.offset,
          key: message.value.key,
          timestamp: message.value.timestamp,
          headers: Object.fromEntries(message.value.headers.map(h => [h.key, h.value])),
          payload: message.value.payload
        },
        null,
        2
      )
      break
  }

  try {
    await navigator.clipboard.writeText(text)
  } catch {
    return
  }

  copiedTarget.value = target
  clearTimeout(copyTimeout)
  copyTimeout = setTimeout(() => {
    copiedTarget.value = null
  }, 1500)
}

onBeforeUnmount(() => clearTimeout(copyTimeout))
</script>

<template>
  <div
    v-if="message"
    class="flex flex-col gap-6"
  >
    <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
      <div class="flex items-center gap-3">
        <UButton
          icon="i-lucide-arrow-left"
          size="sm"
          variant="ghost"
          color="neutral"
          :to="`/topics/${encodedTopic}/messages`"
        />
        <div>
          <h1 class="text-2xl font-bold">
            Message
          </h1>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Partition {{ message.partition }} · Offset {{ message.offset }}
            <template v-if="message.timestamp != null"> · {{ new Date(message.timestamp).toLocaleString() }}</template>
          </p>
        </div>
      </div>
      <AppTopicTabs :topic-name="topicName ?? ''" />
    </div>

    <AppCard title="Details">
      <template #trailing>
        <UButton
          :icon="copiedTarget === 'all' ? 'i-lucide-check' : 'i-lucide-copy'"
          :label="copiedTarget === 'all' ? 'Copied' : 'Copy message'"
          size="xs"
          variant="ghost"
          color="neutral"
          @click="copyToClipboard('all')"
        />
      </template>

      <dl class="grid grid-cols-2 gap-4 sm:grid-cols-4">
        <div class="flex flex-col gap-1">
          <dt class="text-xs uppercase tracking-wide text-muted">
            Partition
          </dt>
          <dd class="font-mono text-sm">
            {{ message.partition }}
          </dd>
        </div>

        <div class="flex flex-col gap-1">
          <dt class="text-xs uppercase tracking-wide text-muted">
            Offset
          </dt>
          <dd class="font-mono text-sm">
            {{ message.offset }}
          </dd>
        </div>

        <div class="flex flex-col gap-1">
          <dt class="text-xs uppercase tracking-wide text-muted">
            Key
          </dt>
          <dd class="font-mono text-sm break-all">
            {{ message.key ?? '—' }}
          </dd>
        </div>

        <div class="col-span-2 sm:col-span-1 flex flex-col gap-1">
          <dt class="text-xs uppercase tracking-wide text-muted">
            Timestamp
          </dt>
          <dd class="font-mono text-sm whitespace-nowrap">
            {{ message.timestamp != null ? new Date(message.timestamp).toLocaleString() : '—' }}
          </dd>
        </div>
      </dl>
    </AppCard>

    <AppCard title="Headers">
      <template #trailing>
        <UButton
          v-if="message.headers.length"
          :icon="copiedTarget === 'headers' ? 'i-lucide-check' : 'i-lucide-copy'"
          :label="copiedTarget === 'headers' ? 'Copied' : 'Copy headers'"
          size="xs"
          variant="ghost"
          color="neutral"
          @click="copyToClipboard('headers')"
        />
      </template>

      <p
        v-if="!message.headers.length"
        class="text-sm text-muted"
      >
        No headers
      </p>

      <UTable
        v-else
        :data="message.headers"
        :columns="headerColumns"
      />
    </AppCard>

    <AppCard title="Payload">
      <template #trailing>
        <UButton
          :icon="copiedTarget === 'payload' ? 'i-lucide-check' : 'i-lucide-copy'"
          :label="copiedTarget === 'payload' ? 'Copied' : 'Copy payload'"
          size="xs"
          variant="ghost"
          color="neutral"
          @click="copyToClipboard('payload')"
        />
      </template>

      <pre class="message-payload overflow-auto rounded-lg border border-default bg-elevated/50 p-3 font-mono text-xs whitespace-pre-wrap break-all"><span
        v-for="(token, index) in tokens"
        :key="index"
        :class="token.class"
      >{{ token.text }}</span></pre>
    </AppCard>
  </div>
</template>

<style>
.message-payload {
  --msg-plain: #24292f;
  --msg-string: #0a3069;
  --msg-number: #0550ae;
  --msg-bool: #cf222e;
  --msg-atom: #0550ae;
  --msg-property: #953800;
  --msg-operator: #6e7781;
}

.dark .message-payload {
  --msg-plain: #e6edf3;
  --msg-string: #a5d6ff;
  --msg-number: #79c0ff;
  --msg-bool: #ff7b72;
  --msg-atom: #79c0ff;
  --msg-property: #d2a8ff;
  --msg-operator: #8b949e;
}

.message-payload .tok-plain { color: var(--msg-plain); }
.message-payload .tok-string { color: var(--msg-string); }
.message-payload .tok-number { color: var(--msg-number); }
.message-payload .tok-bool { color: var(--msg-bool); }
.message-payload .tok-atom { color: var(--msg-atom); }
.message-payload .tok-property { color: var(--msg-property); }
.message-payload .tok-operator { color: var(--msg-operator); }
</style>
