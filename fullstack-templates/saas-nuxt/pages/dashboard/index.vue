<template>
    <NuxtLayout name="authed">
      
    <div class="flex flex-col gap-8 pt-10 justify-center bg-slate-50 items-center pb-10 overflow-y-scroll">
      <div class="w-full px-5 sm:px-24 py-10">
        <SalesChart v-if="data" />

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <RecentSales />
          <TopClients />
        </div>
      </div>
    </div>
    <p v-if="!data">It looks like something went wrong when retrieving data :(</p>
    </NuxtLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useAccountStore } from '@/stores/account';
import SalesChart from '@/components/SalesChart.vue';
import RecentSales from '@/components/RecentSales.vue';
import TopClients from '@/components/TopClients.vue';

definePageMeta({
  layout: false,
});

interface DashboardData {
  sales_deals_info: SalesDealsInfo;
  sales_per_day_info: SalesPerDayInfo;
}

interface SalesDealsInfo {
  open: number;
  ready: number;
  awaitingresponse: number;
  closed: number;
  total_amt_closed: number;
}

interface SalesPerDayInfo {
  date: string;
  sales_total: number;
}

const data = ref<DashboardData | null>(null);
const accountStore = useAccountStore();
const router = useRouter();

onMounted(async () => {
  const url = `//${window.location.host}/api/dashboard`;

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
</script>
