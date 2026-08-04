import type { AddTopicRequest } from '../bindings/AddTopicRequest'
import type { CreateTopicResponse } from '../bindings/CreateTopicResponse'
import type { PublishMessageRequest } from '../bindings/PublishMessageRequest'
import type { PublishResponse } from '../bindings/PublishResponse'
import type { TopicResponse } from '../bindings/TopicResponse'
import type { TopicsResponse } from '../bindings/TopicsResponse'

export const useKafkaStore = defineStore('kafka', {
  state: () => ({
    topics: [] as string[],
    topic: null as TopicResponse | null,
    loading: false,
    error: null as string | null
  }),

  actions: {
    async fetchTopics() {
      this.loading = true
      this.error = null

      try {
        const { $api } = useNuxtApp()
        const { data } = await $api.get<TopicsResponse>('/topics')
        this.topics = data.topics
      } catch (err) {
        this.error = (err as Error).message
      } finally {
        this.loading = false
      }
    },

    async createTopic(payload: AddTopicRequest) {
      this.loading = true
      this.error = null

      try {
        const { $api } = useNuxtApp()
        await $api.post<CreateTopicResponse>('/topics', payload)
        await this.fetchTopics()
      } catch (err) {
        this.error = (err as Error).message
      } finally {
        this.loading = false
      }
    },

    async getTopic(name: string) {
      this.loading = true
      this.error = null

      try {
        const { $api } = useNuxtApp()
        const { data } = await $api.get<TopicResponse>(`/topics/${name}`)
        this.topic = data
      } catch (err) {
        this.error = (err as Error).message
      } finally {
        this.loading = false
      }
    },

    async publishMessage(topicName: string, payload: PublishMessageRequest) {
      this.loading = true
      this.error = null

      try {
        const { $api } = useNuxtApp()
        await $api.post<PublishResponse>(`/topics/${topicName}/messages`, payload)
      } catch (err) {
        this.error = (err as Error).message
      } finally {
        this.loading = false
      }
    }
  }
})
