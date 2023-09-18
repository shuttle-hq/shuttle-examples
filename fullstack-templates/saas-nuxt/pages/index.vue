<script setup lang="ts">
const cards = [
  {
    title: 'Deployed from Shuttle',
    description: 'Deployed via Shuttle, a Rust-native cloud dev platform.',
    href: 'https://www.shuttle.rs',
  },
  {
    title: 'Learn About Rust',
    description: 'Rust is a brilliant language for writing memory-safe, efficient software.',
    href: 'https://doc.rust-lang.org/book/',
  },
  {
    title: 'Learn About Nuxt.js',
    description: 'Nuxt.js is a Vue.js meta-framework at the forefront of JavaScript.',
    href: 'https://nuxt.com/docs/getting-started/introduction',
  },
  {
    title: 'Join the community',
    description: 'Join a thriving community with like-minded Rustaceans and web developers.',
    href: 'https://discord.com/invite/shuttle',
  },
];
</script>
<template>
    <section class="min-h-full py-40 w-full flex flex-col justify-center items-center gap-10">
      <h1 class="text-5xl font-bold text-center px-5">
        Welcome to <span class="text-[#EF924C]">Nuxt.js + Rust SaaS Template</span>
      </h1>

      <div class="px-5 lg:w-1/2 grid grid-cols-1 sm:grid-cols-2 justify-center gap-6">
        <div
          v-for="card in cards"
          :key="card.title"
          class="px-10 py-5 rounded-lg border h-full flex flex-col justify-between space-y-2"
        >
          <h2 class="font-bold text-xl">{{ card.title }} &rarr;</h2>
          <p>{{ card.description }}</p>
          <a :href="card.href" target="_blank" class="text-[#EF924C] transition-all hover:underline">
            Learn more here
          </a>
        </div>
      </div>

      <p class="text-xs">
        Powered by{' '}
        <a class="font-semibold" href="https://shuttle.rs" target="_blank">
          🚀 Shuttle
        </a>
      </p>
    </section>
</template>
