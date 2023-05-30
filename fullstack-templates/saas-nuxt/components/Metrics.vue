<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { faker } from '@faker-js/faker';

const amount1 = ref('');
const amount2 = ref('');
const amount3 = ref('');
const amount4 = ref('');

onMounted(() => {
  amount1.value = faker.finance.amount();
  amount2.value = faker.finance.amount();
  amount3.value = faker.finance.amount();
  amount4.value = faker.finance.amount();
});
</script>
<template>
  <div class="flex flex-col gap-4 justify-between w-10">
    <div class="flex flex-col justify-center items-center shadow-md rounded-md transition-all px-10 py-4">
      <p class="text-xl font-semibold text-slate-900">{{amount1}}</p>
      <h2 class="text-sm text-slate-400">Leads</h2>
    </div>
    <div class="flex flex-col justify-center items-center shadow-md rounded-md transition-all px-10 py-4">
      <p class="text-xl font-semibold text-slate-900">{{amount2}}</p>
      <h2 class="text-sm text-slate-400">Open Sales Deals</h2>
    </div>
    <div class="flex flex-col justify-center items-center shadow-md rounded-md transition-all px-10 py-4 ">
      <p class="text-xl font-semibold text-slate-900">£{{amount3}}</p>
      <h2 class="text-sm text-slate-400">Closed this week:</h2>
    </div>
    <div class="flex flex-col justify-center items-center shadow-md rounded-md transition-all px-10 py-4">
      <p class="text-xl font-semibold text-slate-900">£{{amount4}}</p>
      <h2 class="text-sm text-slate-400">Closed this month:</h2>
    </div>
  </div>
</template>