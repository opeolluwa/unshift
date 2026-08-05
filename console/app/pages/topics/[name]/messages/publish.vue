<script setup lang="ts">
import type { PublishMessageRequest } from '../../../../bindings/PublishMessageRequest'

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

const payloadLanguage = ref<PayloadLanguage>('auto')
const published = ref(false)
const formatError = ref<string | null>(null)

function reset() {
  formState.key = ''
  formState.payload = ''
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
</script>

<template>
  <div class="flex flex-col flex-1 w-full gap-6 min-h-[calc(100dvh-4rem)]">
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
      class="flex flex-col gap-4 flex-1 min-h-0"
      @submit="submit"
    >
      <AppInput
        v-model="formState.key"
        label="Key"
        name="key"
        placeholder="Optional message key"
        hint="Leave empty for no key."
        class="max-w-xl"
      />

      <div class="flex flex-col gap-2 flex-1 min-h-0">
        <div class="flex items-center justify-between gap-4 w-full max-w-4xl mx-auto">
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

        <div class="flex-1 min-h-0 min-h-[20rem] w-full max-w-4xl mx-auto">
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
          class="text-sm text-red-500 w-full max-w-4xl mx-auto"
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

      <div class="flex justify-end gap-2 pt-2 w-full max-w-4xl mx-auto">
        <AppButton
          color="neutral"
          variant="soft"
          @click="reset"
        >
          Reset
        </AppButton>

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
