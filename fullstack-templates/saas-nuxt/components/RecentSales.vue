<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { faker } from '@faker-js/faker';

const salesData = ref([{firstName: '', lastName: '', amount: ''}]);

onMounted(async () => {
  salesData.value = new Array(7).fill(0).map((_, i) => ({
    firstName: faker.name.firstName(),
    lastName: faker.name.lastName(),
    amount: faker.finance.amount()
  }));
});
    

</script>
<template>
  <div class="rounded-md shadow-md p-6 flex flex-col gap-4 w-full items-start bg-white">
    <h2 class="text-2xl font-bold leading-tight my-2">Recent Sales</h2>

    <table class="w-full">
      <thead>
        <tr>
          <th class="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
            Customer
          </th>
          <th class="px-5 py-3 border-b-2 border-gray-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
            Amount Invoiced
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(sales, index) in salesData" :key="index">
          <td class="px-5 py-5 border-b border-gray-200 text-sm">
            <div class="flex items-center">
              <div class="ml-3">
                <p class="text-gray-900 whitespace-no-wrap">
                  {{sales.firstName}} {{sales.lastName}}
                </p>
              </div>
            </div>
          </td>
          <td class="px-5 py-5 border-b border-gray-200 text-sm">
            <p class="text-gray-900 whitespace-no-wrap">£{{sales.amount}}</p>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
