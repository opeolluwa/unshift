// Vue's compile-time feature flags are normally substituted by the bundler, but
// `pinia` ships only a bundler build and stays external in the Nitro server
// bundle, so its bare `__VUE_PROD_DEVTOOLS__` references survive to runtime.
// With NODE_ENV=production the guard no longer short-circuits, and the first
// `createPinia()` throws a ReferenceError — which breaks prerendering and SSR.
export default defineNitroPlugin(() => {
  const flags = globalThis as unknown as Record<string, unknown>

  flags.__VUE_OPTIONS_API__ ??= true
  flags.__VUE_PROD_DEVTOOLS__ ??= false
  flags.__VUE_PROD_HYDRATION_MISMATCH_DETAILS__ ??= false
})
