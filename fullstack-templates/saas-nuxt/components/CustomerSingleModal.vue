<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import { useAccountStore } from '@/stores/account'
import { useRoute, useRouter } from 'vue-router'


interface Customer {
  id: number;
  firstname: string;
  lastname: string;
  email: string;
  phone: string;
}

const route = useRoute()
const router = useRouter()
const accountStore = useAccountStore()
const id = ref(route.params.id as string)
const vis = ref(false)

const setVis = (value: boolean) => {
  vis.value = value
}

const customer = ref<Customer | null>(null)


const handleDelete = async () => {
  const url = `//${window.location.host}/api/customers/${id.value}`

  try {
    const res = await fetch(url, {
      mode: 'cors',
      method: 'DELETE',
      headers: new Headers({
        'Content-Type': 'application/json',
      }),
      body: JSON.stringify({
        email: accountStore.email,
      }),
    })

    if (res.ok) {
      //router.go()
      console.log('Customer deleted')
    }
  } catch (e: any) {
    console.log(`Error: ${e}`)
  }
}
</script>
<template>
  <div v-if="vis" class="w-full h-screen backdrop-blur z-50 absolute">
    <div class="relative py-8 px-5 md:px-10 bg-white shadow-md rounded border border-gray-400 flex items-center justify-center">
      <div>
        <h1 class="text-gray-800 font-lg font-bold tracking-normal leading-tight mb-4">
          Customer details
        </h1>

        <button @click="setVis(false)" class="text-right">
          <font-awesome-icon
            icon={faMultiply}
            className="text-2xl hover:text-red-500 transition-all"
            color="rgb(59 130 246)"
          />
        </button>
      </div>
      <div v-if="customer" class="space-y-2">
        <p class="text-xl">
          Name: {{customer.firstname}} {{customer.lastname}}
        </p>
        <p> Email: {{customer.email}} </p>
        <p> Phone: {{customer.phone}} </p>
      </div>
      <button @click="handleDelete" class="bg-[#EF924C] px-5 py-2 text-white">
        Delete Customer
      </button>
      <p v-if="!accountStore.email">Customer does not exist :(</p>
    </div>
  </div>
</template>


