export default defineNuxtConfig({
  ssr: false,
  nitro: {
    prerender: {
      crawlLinks: true
    }
  },
  css: [
    '@fortawesome/fontawesome-svg-core/styles.css'
  ],
  modules: ['@nuxtjs/tailwindcss', '@pinia/nuxt', '@pinia-plugin-persistedstate/nuxt'],
  compilerOptions: {
    types: ['@nuxt/types', '@nuxtjs/tailwindcss'],
  },
})