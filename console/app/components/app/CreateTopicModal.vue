<script setup lang="ts">
import type { AddTopicRequest } from '../../bindings/AddTopicRequest'

const kafkaStore = useKafkaStore()

const open = defineModel<boolean>('open')

const formState = reactive({
  name: '',
  partitions: 1,
  replicationFactor: 1
})

type FieldErrors = { name: string, message: string }[]

function validate(state: typeof formState): FieldErrors {
  const errors: FieldErrors = []

  if (!state.name.trim()) {
    errors.push({ name: 'topic-name', message: 'Topic name is required.' })
  }

  if (!Number.isInteger(state.partitions) || state.partitions < 1) {
    errors.push({ name: 'partitions', message: 'Partitions must be at least 1.' })
  }

  if (!Number.isInteger(state.replicationFactor) || state.replicationFactor < 1) {
    errors.push({ name: 'replication-factor', message: 'Replication factor must be at least 1.' })
  }

  return errors
}

function reset() {
  formState.name = ''
  formState.partitions = 1
  formState.replicationFactor = 1
}

function cancel() {
  open.value = false
  reset()
}

async function submit() {
  const payload: AddTopicRequest = {
    name: formState.name.trim(),
    numPartitions: formState.partitions,
    replicationFactor: formState.replicationFactor
  }

  await kafkaStore.createTopic(payload)

  if (kafkaStore.error) {
    return
  }

  open.value = false
  reset()
}
</script>

<template>
  <UModal v-model:open="open">
    <template #header>
      <h3 class="text-lg font-semibold">
        Create topic
      </h3>
    </template>

    <template #body>
      <UForm
        :state="formState"
        :validate="validate"
        class="flex flex-col gap-4 py-4"
        @submit="submit"
      >
        <AppInput
          v-model="formState.name"
          label="Topic name"
          name="topic-name"
          placeholder="e.g. orders"
        />

        <div class="flex gap-4">
          <AppNumberInput
            v-model="formState.partitions"
            label="Partitions"
            name="partitions"
            :min="1"
            class="flex-1"
          />
          <AppNumberInput
            v-model="formState.replicationFactor"
            label="Replication factor"
            name="replication-factor"
            :min="1"
            class="flex-1"
          />
        </div>

        <p
          v-if="kafkaStore.error"
          class="text-sm text-red-500"
        >
          {{ kafkaStore.error }}
        </p>

        <div class="flex justify-end gap-2 pt-2">
          <UButton
            color="neutral"
            variant="soft"
            @click="cancel"
          >
            Cancel
          </UButton>

          <AppButton
            type="submit"
            color="primary"
            :loading="kafkaStore.loading"
          >
            Create
          </AppButton>
        </div>
      </UForm>
    </template>
  </UModal>
</template>
