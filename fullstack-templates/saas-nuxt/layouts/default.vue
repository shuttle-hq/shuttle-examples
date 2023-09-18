<script setup lang="ts">
import { useAccountStore } from "@/stores/account";
import Navbar from "@/components/navbar.vue";

const accountStore = useAccountStore();
const email = accountStore.email;
</script>
<template>
  <div :class="email === '' ? 'min-h-screen flex-col items-center' : 'min-h-screen w-screen flex flex-col lg:flex-row items-top'">
    <Navbar />
    <div :class="`relative bg-slate-50 h-screen overflow-y-scroll ${email ? 'w-full' : 'w-screen'}`">
      <slot />
    </div>
  </div>
</template>

