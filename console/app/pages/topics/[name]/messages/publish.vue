<script setup lang="ts">
import type { PublishMessageRequest } from '../../../../bindings/PublishMessageRequest'

const route = useRoute()

const kafkaStore = useKafkaStore()

const topicName = computed(() => {
  const name = route.params.name
  return Array.isArray(name) ? name[0] : name
})

const encodedTopic = computed(() => encodeURIComponent(topicName.value ?? ''))

const formState = reactive({
  key: '',
  payload: ''
})

const published = ref(false)

function reset() {
  formState.key = ''
  formState.payload = ''
  published.value = false
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
  <div class="flex flex-col flex-1 w-full gap-8">
    <UButton
      :to="`/topics/${encodedTopic}`"
      icon="i-lucide-arrow-left"
      label="Back to topic"
      variant="subtle"
      color="neutral"
      class="self-start"
    />

    <div class="flex flex-col gap-2">
      <AppPageHeader
        title="Publish message"
        :subtitle="topicName ?? ''"
      />

      <UForm
        :state="formState"
        class="flex flex-col gap-4 max-w-xl"
        @submit="submit"
      >
        <AppInput
          v-model="formState.key"
          label="Key"
          name="key"
          placeholder="Optional message key"
          hint="Leave empty for no key."
        />

        <UFormField
          name="payload"
          label="Payload"
        >
          <UTextarea
            v-model="formState.payload"
            name="payload"
            :rows="8"
            placeholder="Message payload"
            class="w-full"
          />
        </UFormField>

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

        <div class="flex justify-end gap-2 pt-2">
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
  </div>
</template>
