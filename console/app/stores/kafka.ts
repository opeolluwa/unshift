import type { AddTopicRequest } from '../bindings/AddTopicRequest'
import type { ClusterOverviewResponse } from '../bindings/ClusterOverviewResponse'
import type { CreateTopicResponse } from '../bindings/CreateTopicResponse'
import type { Message } from '../bindings/Message'
import type { MessagesResponse } from '../bindings/MessagesResponse'
import type { PublishMessageRequest } from '../bindings/PublishMessageRequest'
import type { PublishResponse } from '../bindings/PublishResponse'
import type { TopicSummary } from '../bindings/TopicSummary'
import type { TopicsResponse } from '../bindings/TopicsResponse'

export const useKafkaStore = defineStore('kafka', {
  state: () => ({
    overview: null as ClusterOverviewResponse | null,
    topics: [] as TopicSummary[],
    topic: null as TopicSummary | null,
    messages: [] as Message[],
    loading: false,
    error: null as string | null
  }),

  actions: {
    async fetchClusterOverview() {
      this.loading = true
      this.error = null

      try {
        const { $api } = useNuxtApp()
        const { data } = await $api.get<ClusterOverviewResponse>('/cluster/overview')
        this.overview = data
      } catch (err) {
        this.error = (err as Error).message
      } finally {
        this.loading = false
      }
    },

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
        const { data } = await $api.get<TopicSummary>(`/topics/${name}`)
        this.topic = data
      } catch (err) {
        this.error = (err as Error).message
      } finally {
        this.loading = false
      }
    },

    async fetchMessages(topicName: string, limit = 10) {
      this.loading = true
      this.error = null

      try {
        const { $api } = useNuxtApp()
        const { data } = await $api.get<MessagesResponse>(`/topics/${topicName}/messages`, {
          params: { limit }
        })
        this.messages = data.messages
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
