<script setup lang="ts">
import type { Message } from '~/bindings/Message'

const props = defineProps<{
  message: Message | null
}>()

const open = defineModel<boolean>('open')

// Pretty-print JSON payloads, but never lose the original: a payload that
// isn't valid JSON is shown verbatim.
const isJson = computed(() => {
  try {
    JSON.parse(props.message?.payload ?? '')
    return true
  } catch {
    return false
  }
})

const formattedPayload = computed(() => {
  const payload = props.message?.payload ?? ''

  return isJson.value ? JSON.stringify(JSON.parse(payload), null, 2) : payload
})

type Token = { text: string, class: string }

const TOKEN_PATTERN = /("(?:\\.|[^"\\])*")(\s*:)?|\b(true|false)\b|\bnull\b|(-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)|([{}[\],])/g

// Hand-rolled tokenizer instead of a highlighter dependency: the payload is
// always JSON here (non-JSON falls back to a single plain token), and this
// keeps the preview free of CodeMirror, which is editor-shaped rather than
// read-only-shaped.
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
      // A string followed by `:` is a property name, not a value.
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

const copied = ref(false)
let copyTimeout: ReturnType<typeof setTimeout> | undefined

async function copyPayload() {
  if (!props.message) {
    return
  }

  try {
    await navigator.clipboard.writeText(props.message.payload)
  } catch {
    return
  }

  copied.value = true
  clearTimeout(copyTimeout)
  copyTimeout = setTimeout(() => {
    copied.value = false
  }, 1500)
}

// Reset the transient copy feedback whenever a different message is shown.
watch(() => props.message, () => {
  clearTimeout(copyTimeout)
  copied.value = false
})

onBeforeUnmount(() => clearTimeout(copyTimeout))
</script>

<template>
  <USlideover
    v-model:open="open"
    side="right"
    title="Message"
    :description="message ? `Partition ${message.partition} · Offset ${message.offset}` : undefined"
  >
    <template #body>
      <div
        v-if="message"
        class="flex flex-col gap-6"
      >
        <dl class="grid grid-cols-2 gap-4">
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

          <div class="col-span-2 flex flex-col gap-1">
            <dt class="text-xs uppercase tracking-wide text-muted">
              Key
            </dt>
            <dd class="font-mono text-sm break-all">
              {{ message.key ?? '—' }}
            </dd>
          </div>
        </dl>

        <div class="flex flex-col gap-2">
          <div class="flex items-center justify-between gap-2">
            <span class="text-xs uppercase tracking-wide text-muted">
              Payload
            </span>

            <UButton
              :icon="copied ? 'i-lucide-check' : 'i-lucide-copy'"
              :label="copied ? 'Copied' : 'Copy'"
              size="xs"
              variant="ghost"
              color="neutral"
              @click="copyPayload"
            />
          </div>

          <pre class="message-payload max-h-[60vh] overflow-auto rounded-lg border border-default bg-elevated/50 p-3 font-mono text-xs whitespace-pre-wrap break-all"><span
            v-for="(token, index) in tokens"
            :key="index"
            :class="token.class"
          >{{ token.text }}</span></pre>
        </div>
      </div>
    </template>
  </USlideover>
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
