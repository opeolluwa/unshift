<template>
  <div>
    {{ topics }}
  </div>
</template>

<script lang="ts" setup>
const kafkaStore = useKafkaStore();
const topics = await kafkaStore.fetchTopics();
</script>
