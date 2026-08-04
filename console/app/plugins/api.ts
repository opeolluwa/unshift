import axios from 'axios'

const NETWORK_REQUEST_TIMEOUT = 7500 // 7.5 seconds

const api = axios.create({
  baseURL: 'http://localhost:8000/api',
  headers: {
    Accept: 'application/json'
  },
  timeout: NETWORK_REQUEST_TIMEOUT
})

// Response interceptor (handle errors globally)
api.interceptors.response.use(
  response => response,
  (error) => {
    const message
      = error.response?.data?.message || error.message || 'Unknown error'

    return Promise.reject(new Error(message))
  }
)

export default defineNuxtPlugin(() => {
  return {
    provide: {
      api
    }
  }
})
