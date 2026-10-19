<script setup lang="ts">
import type { PublishMessageRequest } from '../../../../bindings/PublishMessageRequest'
import type { MessageHeader } from '../../../../bindings/MessageHeader'

const route = useRoute()

useHead({ title: 'Publish message' })

const kafkaStore = useKafkaStore()

const topicName = computed(() => {
  const name = route.params.name
  return Array.isArray(name) ? name[0] : name
})

const encodedTopic = computed(() => encodeURIComponent(topicName.value ?? ''))

type PayloadLanguage = 'auto' | 'json' | 'yaml'

const languageOptions: { value: PayloadLanguage, label: string }[] = [
  { value: 'auto', label: 'Auto' },
  { value: 'json', label: 'JSON' },
  { value: 'yaml', label: 'YAML' }
]

const formState = reactive({
  key: '',
  payload: ''
})

const headers = ref<{ key: string, value: string }[]>([])
const showHeaders = ref(false)

function addHeader() {
  headers.value.push({ key: '', value: '' })
  showHeaders.value = true
}

function removeHeader(index: number) {
  headers.value.splice(index, 1)
}

const payloadLanguage = ref<PayloadLanguage>('auto')
const published = ref(false)
const formatError = ref<string | null>(null)
const saved = ref(false)
const showSaved = ref(false)
const showSaveModal = ref(false)
const saveKey = ref('')

const topicSavedMessages = computed(() =>
  kafkaStore.savedMessages.filter(m => m.topic === topicName.value)
)

onMounted(() => {
  kafkaStore.fetchSavedMessages()
})

function reset() {
  formState.key = ''
  formState.payload = ''
  headers.value = []
  showHeaders.value = false
  published.value = false
  saved.value = false
  formatError.value = null
}

function format() {
  const trimmed = formState.payload.trim()

  if (!trimmed) {
    return
  }

  try {
    formState.payload = JSON.stringify(JSON.parse(trimmed), null, 2)
    formatError.value = null
  } catch {
    formatError.value = 'Invalid JSON — fix the payload before formatting.'
  }
}

async function submit() {
  const nonEmptyHeaders: MessageHeader[] = headers.value
    .filter(h => h.key.trim())
    .map(h => ({ key: h.key, value: h.value || null }))

  const payload: PublishMessageRequest = {
    key: formState.key,
    payload: formState.payload,
    headers: nonEmptyHeaders.length > 0 ? nonEmptyHeaders : null
  }

  await kafkaStore.publishMessage(topicName.value ?? '', payload)

  if (kafkaStore.error) {
    return
  }

  published.value = true
}

function openSaveModal() {
  saveKey.value = formState.key
  showSaveModal.value = true
}

async function confirmSave() {
  saved.value = false
  await kafkaStore.saveMessage({
    label: null,
    topic: topicName.value ?? '',
    key: saveKey.value,
    payload: formState.payload
  })

  if (!kafkaStore.error) {
    saved.value = true
    showSaveModal.value = false
  }
}

function loadSavedMessage(msg: { key: string, payload: string }) {
  formState.key = msg.key
  formState.payload = msg.payload
  published.value = false
  saved.value = false
  showSaved.value = false
}

async function deleteSaved(id: number) {
  await kafkaStore.deleteSavedMessage(id)
}
</script>

<template>
  <div class="flex flex-col gap-6">
    <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
      <div class="flex items-center gap-3">
        <UButton
          icon="i-lucide-arrow-left"
          size="sm"
          variant="ghost"
          color="neutral"
          :to="`/topics/${encodedTopic}`"
        />
        <div>
          <h1 class="text-2xl font-bold">
            {{ topicName }}
          </h1>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Compose and publish a message
          </p>
        </div>
      </div>
      <AppTopicTabs :topic-name="topicName ?? ''" />
    </div>

    <!-- Saved messages -->
    <div
      v-if="topicSavedMessages.length > 0"
      class="flex flex-col gap-2"
    >
      <button
        class="flex items-center gap-2 text-sm font-medium text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 transition-colors w-fit"
        @click="showSaved = !showSaved"
      >
        <UIcon
          :name="showSaved ? 'i-lucide-chevron-down' : 'i-lucide-chevron-right'"
          class="size-3.5"
        />
        Saved messages
        <span class="inline-flex items-center justify-center size-5 rounded-full bg-gray-200 dark:bg-white/10 text-[10px] font-semibold">
          {{ topicSavedMessages.length }}
        </span>
      </button>

      <div
        v-if="showSaved"
        class="grid gap-1.5"
      >
        <div
          v-for="msg in topicSavedMessages"
          :key="msg.id"
          class="flex items-center justify-between gap-3 px-3.5 py-2.5 rounded-lg border border-gray-200 dark:border-white/10 bg-white dark:bg-gray-900/50 hover:border-gray-300 dark:hover:border-white/20 cursor-pointer group transition-colors"
          @click="loadSavedMessage(msg)"
        >
          <div class="flex flex-col gap-0.5 min-w-0">
            <span class="text-sm font-medium truncate">
              {{ msg.key || '(no key)' }}
            </span>
            <span class="text-xs text-gray-400 dark:text-gray-500 truncate font-mono">
              {{ msg.payload.slice(0, 80) }}{{ msg.payload.length > 80 ? '...' : '' }}
            </span>
          </div>
          <UButton
            icon="i-lucide-trash-2"
            size="xs"
            variant="ghost"
            color="error"
            class="opacity-0 group-hover:opacity-100 shrink-0"
            @click.stop="deleteSaved(msg.id)"
          />
        </div>
      </div>
    </div>

    <!-- Compose form -->
    <UForm
      :state="formState"
      class="flex flex-col gap-5"
      @submit="submit"
    >
      <!-- Headers -->
      <div class="flex flex-col gap-3">
        <div class="flex items-center justify-between px-1">
          <span
            v-if="headers.length > 0"
            class="text-xs font-medium text-gray-600 dark:text-gray-400"
          >
            Headers
            <span class="inline-flex items-center justify-center size-4 rounded-full bg-gray-200 dark:bg-white/10 text-[10px] font-semibold ml-1">
              {{ headers.length }}
            </span>
          </span>
          <span v-else />

          <UButton
            type="button"
            icon="i-lucide-plus"
            label="Add header"
            size="sm"
            variant="outline"
            color="neutral"
            @click="addHeader"
          />
        </div>

        <div
          v-if="headers.length > 0"
          class="flex flex-col gap-2"
        >
          <div
            v-for="(header, index) in headers"
            :key="index"
            class="flex items-center gap-2"
          >
            <input
              v-model="header.key"
              type="text"
              placeholder="Header key"
              class="flex-1 rounded-lg border border-gray-200 dark:border-white/10 bg-white dark:bg-gray-900 px-3 py-1.5 text-sm placeholder-gray-400 dark:placeholder-gray-500 focus:outline-none focus:border-gray-900 dark:focus:border-white/30 transition-colors"
            >
            <input
              v-model="header.value"
              type="text"
              placeholder="Header value"
              class="flex-1 rounded-lg border border-gray-200 dark:border-white/10 bg-white dark:bg-gray-900 px-3 py-1.5 text-sm placeholder-gray-400 dark:placeholder-gray-500 focus:outline-none focus:border-gray-900 dark:focus:border-white/30 transition-colors"
            >
            <button
              type="button"
              class="flex items-center justify-center size-7 rounded-md text-gray-400 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-500/10 transition-colors shrink-0"
              @click="removeHeader(index)"
            >
              <UIcon
                name="i-lucide-x"
                class="size-3.5"
              />
            </button>
          </div>
        </div>
      </div>

      <!-- Editor block -->
      <div class="flex flex-col gap-0">
        <div class="flex items-center justify-between gap-4 px-1 pb-2">
          <span class="text-xs font-medium text-gray-600 dark:text-gray-400">
            Payload
          </span>

          <div class="flex items-center gap-2">
            <div class="flex items-center gap-0.5 rounded-md bg-gray-100 dark:bg-white/5 p-0.5">
              <button
                v-for="option in languageOptions"
                :key="option.value"
                type="button"
                class="rounded px-2 py-0.5 text-[11px] font-medium transition-colors"
                :class="payloadLanguage === option.value
                  ? 'bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm'
                  : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200'"
                @click="payloadLanguage = option.value"
              >
                {{ option.label }}
              </button>
            </div>

            <button
              type="button"
              class="flex items-center gap-1.5 rounded-md px-2 py-1 text-[11px] font-medium text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-white/5 transition-colors"
              @click="format"
            >
              <UIcon
                name="i-lucide-wand-2"
                class="size-3"
              />
              Format
            </button>
          </div>
        </div>

        <div class="h-[55vh] w-full rounded-xl overflow-hidden ring-1 ring-gray-200 dark:ring-white/10 focus-within:ring-gray-900 dark:focus-within:ring-white/30 transition-shadow">
          <ClientOnly>
            <AppCodeEditor
              v-model="formState.payload"
              :language="payloadLanguage"
              placeholder="Enter a JSON or YAML payload..."
            />
          </ClientOnly>
        </div>

        <p
          v-if="formatError"
          class="mt-2 flex items-center gap-1.5 text-xs text-red-500"
        >
          <UIcon
            name="i-lucide-circle-alert"
            class="size-3.5 shrink-0"
          />
          {{ formatError }}
        </p>
      </div>

      <!-- Status + actions -->
      <div class="flex items-center justify-between gap-4 pt-1">
        <div class="min-w-0">
          <p
            v-if="kafkaStore.error"
            class="flex items-center gap-1.5 text-sm text-red-500"
          >
            <UIcon
              name="i-lucide-circle-x"
              class="size-4 shrink-0"
            />
            {{ kafkaStore.error }}
          </p>

          <p
            v-else-if="published"
            class="flex items-center gap-1.5 text-sm text-green-600 dark:text-green-400"
          >
            <UIcon
              name="i-lucide-circle-check"
              class="size-4 shrink-0"
            />
            Message published.
          </p>

          <p
            v-else-if="saved"
            class="flex items-center gap-1.5 text-sm text-green-600 dark:text-green-400"
          >
            <UIcon
              name="i-lucide-circle-check"
              class="size-4 shrink-0"
            />
            Message saved.
          </p>
        </div>

        <div class="flex items-center gap-2 shrink-0">
          <UButton
            color="neutral"
            variant="ghost"
            size="sm"
            @click="reset"
          >
            Reset
          </UButton>

          <UButton
            icon="i-lucide-bookmark"
            color="neutral"
            variant="outline"
            size="sm"
            :disabled="!formState.payload.trim()"
            @click="openSaveModal"
          >
            Save
          </UButton>

          <AppButton
            type="submit"
            color="primary"
            icon="i-lucide-send"
            :loading="kafkaStore.loading"
          >
            Publish
          </AppButton>
        </div>
      </div>
    </UForm>

    <!-- Save modal -->
    <UModal v-model:open="showSaveModal">
      <template #content>
        <div class="p-6 flex flex-col gap-4">
          <div>
            <h3 class="text-lg font-semibold">
              Save message
            </h3>
            <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
              Save this message for quick reuse later.
            </p>
          </div>

          <div class="flex flex-col gap-1.5">
            <label
              for="save-key"
              class="text-xs font-medium text-gray-600 dark:text-gray-400"
            >
              Message key
            </label>
            <input
              id="save-key"
              v-model="saveKey"
              type="text"
              placeholder="Optional message key"
              class="w-full rounded-lg border border-gray-200 dark:border-white/10 bg-white dark:bg-gray-900 px-3 py-2 text-sm placeholder-gray-400 dark:placeholder-gray-500 focus:outline-none focus:border-gray-900 dark:focus:border-white/30 transition-colors"
            >
            <p class="text-xs text-gray-400 dark:text-gray-500">
              Leave empty for no key.
            </p>
          </div>

          <div class="flex justify-end gap-2 pt-2">
            <UButton
              color="neutral"
              variant="ghost"
              @click="showSaveModal = false"
            >
              Cancel
            </UButton>
            <UButton
              icon="i-lucide-bookmark"
              color="primary"
              @click="confirmSave"
            >
              Save
            </UButton>
          </div>
        </div>
      </template>
    </UModal>
  </div>
</template>
