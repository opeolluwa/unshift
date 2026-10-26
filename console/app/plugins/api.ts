import axios from 'axios'

const NETWORK_REQUEST_TIMEOUT = 7500

export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig()

  const api = axios.create({
    baseURL: config.public.apiBase,
    headers: {
      Accept: 'application/json'
    },
    timeout: NETWORK_REQUEST_TIMEOUT
  })

  api.interceptors.response.use(
    response => response,
    (error) => {
      const message
        = error.response?.data?.message || error.message || 'Unknown error'

      return Promise.reject(new Error(message))
    }
  )

  return {
    provide: {
      api
    }
  }
})
