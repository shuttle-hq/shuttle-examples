<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useAccountStore } from '@/stores/account';

definePageMeta({
  layout: false,
});


interface Customer {
  id: number;
  firstname: string;
  lastname: string;
  email: string;
  phone: string;
}

const data = ref<Customer[]>([]);
const accountStore = useAccountStore();
const router = useRouter();

onMounted(async () => {
  const url = `//${window.location.host}/api/customers`;

  try {
    const res = await fetch(url, {
      method: 'POST',
      mode: 'cors',
      headers: new Headers({
        'Content-Type': 'application/json',
      }),
      body: JSON.stringify({
        email: accountStore.email,
      }),
    });

    if (res.status == 403) {
      await router.push('/login');
    }

    const jsonData = await res.json();

    data.value = jsonData;
  } catch (e) {
    console.log(`Error: ${e}`);
  }
});

const handleDelete = async (customerId: number) => {
  const url = `//${window.location.host}/api/customers/${customerId}`;

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
    });

    if (res.ok) {
      await router.go();
    }
  } catch (e) {
    console.log(`Error: ${e}`);
  }
};
</script>
<template>
  <NuxtLayout name="authed">
    <div class="py-10 flex flex-col gap-4 w-full px-5 md:px-24 items-start">
      <div>
        <h2 class="text-2xl font-semibold leading-tight my-10">Customers</h2>
      </div>
      <table class="min-w-full leading-normal">
        <thead>
          <tr>
            <th class="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
              Customer
            </th>
            <th class="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
              Email
            </th>
            <th class="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
              Phone Number
            </th>
            <th class="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
              More Info
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="cust in data" :key="cust.id">
            <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">
              <div class="flex items-center">
                <div class="ml-3">
                  <p class="text-gray-900 whitespace-no-wrap">
                    {{ cust.firstname }} {{ cust.lastname }}
                  </p>
                </div>
              </div>
            </td>
            <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">
              <p class="text-gray-900 whitespace-no-wrap">{{ cust.email }}</p>
            </td>
            <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">
              <p class="text-gray-900 whitespace-no-wrap">{{ cust.phone }}</p>
            </td>
            <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">
              <button
                :data-id="cust.id"
                @click="handleDelete(cust.id)"
                class="px-5 py-2 hover:bg-red-700 transition-all mt-4 rounded text-white bg-red-500"
              >
                <font-awesome-icon icon="trash" color="white" /> Delete Customer
              </button>
            </td>
          </tr>
        </tbody>
      </table>
      <nuxt-link
        to="/dashboard/customers/create"
        class="transition-all mt-4 bg-black hover:bg-slate-950 text-white font-bold py-2 px-4 rounded"
      >
        <font-awesome-icon icon="plus" color="white" /> Add Customer
      </nuxt-link>
    </div>
  </NuxtLayout>
</template>