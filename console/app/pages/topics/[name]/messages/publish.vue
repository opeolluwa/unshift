<script setup lang="ts">
import type { PublishMessageRequest } from '../../../../bindings/PublishMessageRequest'

const route = useRoute()

useHead({ title: 'Publish message' })

const kafkaStore = useKafkaStore()

const topicName = computed(() => {
  const name = route.params.name
  return Array.isArray(name) ? name[0] : name
})

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

const payloadLanguage = ref<PayloadLanguage>('auto')
const published = ref(false)
const formatError = ref<string | null>(null)
const saved = ref(false)
const showSaved = ref(false)

const topicSavedMessages = computed(() =>
  kafkaStore.savedMessages.filter(m => m.topic === topicName.value)
)

onMounted(() => {
  kafkaStore.fetchSavedMessages()
})

function reset() {
  formState.key = ''
  formState.payload = ''
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
    formatError.value = 'Invalid JSON. Fix the payload before formatting.'
  }
}

async function submit() {
  const payload: PublishMessageRequest = {
    key: formState.key,
    payload: formState.payload
  }

  await kafkaStore.publishMessage(topicName.value ?? '', payload)

  if (kafkaStore.error) {
    return
  }

  published.value = true
}

async function saveCurrentMessage() {
  saved.value = false
  await kafkaStore.saveMessage({
    label: null,
    topic: topicName.value ?? '',
    key: formState.key,
    payload: formState.payload
  })

  if (!kafkaStore.error) {
    saved.value = true
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
      <div>
        <h1 class="text-2xl font-bold">
          {{ topicName }}
        </h1>
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Compose and publish a message
        </p>
      </div>
      <AppTopicTabs :topic-name="topicName ?? ''" />
    </div>

    <AppCard v-if="topicSavedMessages.length > 0">
      <template #header>
        <button
          class="flex items-center gap-2 text-sm font-semibold"
          @click="showSaved = !showSaved"
        >
          <UIcon
            :name="showSaved ? 'i-lucide-chevron-down' : 'i-lucide-chevron-right'"
            class="size-4"
          />
          Saved messages
          <UBadge
            variant="subtle"
            color="neutral"
            size="xs"
          >
            {{ topicSavedMessages.length }}
          </UBadge>
        </button>
      </template>

      <div
        v-if="showSaved"
        class="flex flex-col gap-1 max-h-48 overflow-y-auto"
      >
        <div
          v-for="msg in topicSavedMessages"
          :key="msg.id"
          class="flex items-center justify-between gap-2 px-3 py-2 rounded-lg hover:bg-gray-50 dark:hover:bg-white/5 cursor-pointer group transition-colors"
          @click="loadSavedMessage(msg)"
        >
          <div class="flex flex-col gap-0.5 min-w-0">
            <span class="text-sm font-medium truncate">
              {{ msg.key || '(no key)' }}
            </span>
            <span class="text-xs text-gray-500 dark:text-gray-400 truncate font-mono">
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
    </AppCard>

    <AppCard title="Message">
      <UForm
        :state="formState"
        class="flex flex-col gap-4"
        @submit="submit"
      >
        <AppInput
          v-model="formState.key"
          label="Key"
          name="key"
          placeholder="Optional message key"
          hint="Leave empty for no key."
        />

        <div class="flex flex-col gap-2">
          <div class="flex items-center justify-between gap-4 w-full">
            <span class="text-xs font-medium text-gray-600 dark:text-gray-400">
              Payload
            </span>

            <div class="flex items-center gap-2">
              <div class="flex items-center gap-0.5 rounded-lg bg-gray-100 dark:bg-white/5 p-0.5">
                <UButton
                  v-for="option in languageOptions"
                  :key="option.value"
                  size="xs"
                  :color="payloadLanguage === option.value ? 'primary' : 'neutral'"
                  :variant="payloadLanguage === option.value ? 'solid' : 'ghost'"
                  @click="payloadLanguage = option.value"
                >
                  {{ option.label }}
                </UButton>
              </div>

              <UButton
                icon="i-lucide-wand-2"
                label="Format"
                size="xs"
                variant="outline"
                color="neutral"
                @click="format"
              />
            </div>
          </div>

          <div class="h-[45vh] w-full">
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
            class="text-sm text-red-500"
          >
            {{ formatError }}
          </p>
        </div>

        <p
          v-if="kafkaStore.error"
          class="text-sm text-red-500"
        >
          {{ kafkaStore.error }}
        </p>

        <p
          v-else-if="published"
          class="text-sm text-green-600 dark:text-green-400"
        >
          Message published successfully.
        </p>

        <p
          v-else-if="saved"
          class="text-sm text-green-600 dark:text-green-400"
        >
          Message saved.
        </p>

        <div class="flex justify-end gap-2 pt-2 w-full">
          <UButton
            color="neutral"
            variant="soft"
            @click="reset"
          >
            Reset
          </UButton>

          <UButton
            icon="i-lucide-bookmark"
            color="neutral"
            variant="outline"
            :disabled="!formState.payload.trim()"
            @click="saveCurrentMessage"
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
      </UForm>
    </AppCard>
  </div>
</template>
