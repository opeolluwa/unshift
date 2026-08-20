<script setup lang="ts">
import type { PublishMessageRequest, PublishMessageHeader } from '../../../../bindings/PublishMessageRequest'

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

const headers = ref<PublishMessageHeader[]>([])
const showHeaders = ref(false)

const payloadLanguage = ref<PayloadLanguage>('auto')
const published = ref(false)
const formatError = ref<string | null>(null)

function addHeader() {
  headers.value.push({ key: '', value: null })
}

function removeHeader(index: number) {
  headers.value.splice(index, 1)
}

function reset() {
  formState.key = ''
  formState.payload = ''
  headers.value = []
  showHeaders.value = false
  published.value = false
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
  const nonEmptyHeaders = headers.value.filter(h => h.key.trim() !== '')

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
</script>

<template>
  <div class="flex flex-col w-full gap-6">
    <UButton
      :to="`/topics/${encodedTopic}`"
      icon="i-lucide-arrow-left"
      label="Back to topic"
      variant="subtle"
      color="neutral"
      class="self-start"
    />

    <AppPageHeader
      title="Publish message"
      :subtitle="topicName ?? ''"
    />

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
        <div class="flex items-center gap-2">
          <UButton
            icon="i-lucide-list"
            label="Headers"
            size="xs"
            variant="outline"
            color="neutral"
            :trailing-icon="showHeaders ? 'i-lucide-chevron-up' : 'i-lucide-chevron-down'"
            @click="showHeaders = !showHeaders"
          />

          <span
            v-if="headers.length > 0"
            class="text-xs text-muted"
          >
            {{ headers.length }}
          </span>
        </div>

        <div
          v-if="showHeaders"
          class="flex flex-col gap-2 pl-2 border-l-2 border-gray-200 dark:border-gray-700"
        >
          <div
            v-for="(_, index) in headers"
            :key="index"
            class="flex items-center gap-2"
          >
            <UInput
              v-model="headers[index].key"
              placeholder="Key"
              class="flex-1"
              :ui="{ base: 'py-2 pl-3 bg-transparent text-sm' }"
            />

            <UInput
              v-model="headers[index].value"
              placeholder="Value (optional)"
              class="flex-1"
              :ui="{ base: 'py-2 pl-3 bg-transparent text-sm' }"
            />

            <UButton
              icon="i-lucide-x"
              color="neutral"
              variant="ghost"
              size="xs"
              @click="removeHeader(index)"
            />
          </div>

          <UButton
            icon="i-lucide-plus"
            label="Add header"
            size="xs"
            variant="ghost"
            color="neutral"
            class="self-start"
            @click="addHeader"
          />
        </div>
      </div>

      <div class="flex flex-col gap-2">
        <div class="flex items-center justify-between gap-4 w-full">
          <span class="text-xs font-medium text-gray-600 dark:text-gray-400">
            Payload
          </span>

          <div class="flex items-center gap-2">
            <div class="flex items-center gap-1 rounded-lg border border-gray-300 dark:border-gray-600 p-1">
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
              placeholder="Enter a JSON or YAML payload…"
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

      <div class="flex justify-end gap-2 pt-2 w-full">
        <UButton
          color="neutral"
          variant="soft"
          @click="reset"
        >
          Reset
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
  </div>
</template>
