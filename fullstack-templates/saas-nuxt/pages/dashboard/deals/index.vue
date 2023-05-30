<script setup>
import { ref, onMounted } from 'vue';
import { useAccountStore } from '@/stores/account';

definePageMeta({
  layout: false,
});


const { email } = useAccountStore();
const deals = ref([]);
const router = useRouter();

const handleStatus = async (e) => {
  const element = e.target;
  const id = element.getAttribute('data-id');
  const status = element.value;

  const url = `//${window.location.host}/api/deals/${id}`;

  try {
    const res = await fetch(url, {
      method: 'PUT',
      mode: 'cors',
      headers: new Headers({
        'Content-Type': 'application/json',
      }),
      body: JSON.stringify({
        new_value: status,
        email: email,
      }),
    });

    if (res.ok) {
      element.value = status;
    }
  } catch (e) {
    console.log(e.message);
  }
};

onMounted(async () => {
  const url = `//${window.location.host}/api/deals`;

  try {
    const res = await fetch(url, {
      method: 'POST',
      mode: 'cors',
      headers: new Headers({
        'Content-Type': 'application/json',
      }),
      body: JSON.stringify({
        email: email,
      }),
    });

    if (res.status === 403) {
      await router.push('/login');
      return;
    }

    const dealData = await res.json();
    deals.value = dealData;
  } catch (e) {
    console.log(`Error: ${e}`);
  }
});
</script>
<template>
  <NuxtLayout name="authed">
  <div class="px-5 py-10 flex flex-col gap-4 w-full md:px-24 items-start">
    <div>
      <h2 class="text-2xl font-semibold leading-tight my-10">Deals</h2>
    </div>

    <table class="min-w-full leading-normal">
      <thead>
        <tr>
          <th
            class="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider"
          >
            Customer
          </th>
          <th
            class="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider"
          >
            Invoice Amount
          </th>
          <th
            class="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider"
          >
            Status
          </th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="deal in deals" :key="deal.id">
          <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">
            <div class="flex items-center">
              <div class="ml-3">
                <p class="text-gray-900 whitespace-no-wrap">{{ deal.customer_name }}</p>
              </div>
            </div>
          </td>

          <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">
            <p class="text-gray-900 whitespace-no-wrap">£{{ deal.estimate_worth }}.00</p>
          </td>
          <td class="px-5 py-5 border-b border-gray-200 bg-white text-sm">
            <select
              class="text-gray-900 whitespace-no-wrap bg-white"
              :data-id="deal.id"
              v-model="deal.status"
              @change="handleStatus"
            >
              <option value="open">Open</option>
              <option value="awaitingresponse">Awaiting Response</option>
              <option value="ready">Ready to close</option>
              <option value="closed">Closed</option>
            </select>
          </td>
        </tr>
      </tbody>
    </table>
    <NuxtLink
      to="/dashboard/deals/create"
      class="transition-all mt-4 bg-black hover:bg-slate-950 text-white font-bold py-2 px-4 rounded"
    >
      <font-awesome-icon icon="faPlus" color="white" /> Create Deal
    </NuxtLink>
  </div>
</NuxtLayout>
</template>

