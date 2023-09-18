<script setup lang="ts">
import { onMounted, ref, watchEffect } from 'vue'
import { useAccountStore } from '@/stores/account'
import Navbar from '@/components/navbar.vue'

const router = useRouter()
const route = useRoute()
const accountStore = useAccountStore()

const email = accountStore.email

const checkUserStatus = () => {
  if (route.fullPath.includes("/dashboard") && !accountStore.email) {
    router.push('/login')
  }
}

watchEffect(checkUserStatus)

</script>
<template>
  <div class="min-h-screen flex-col items-center w-screen flex lg:flex-row items-top">
    <Navbar />
      <div :class="`relative bg-slate-50 h-screen overflow-y-scroll ${email ? 'w-full' : 'w-screen'}`">
      <slot />
    </div>
  </div>
</template>