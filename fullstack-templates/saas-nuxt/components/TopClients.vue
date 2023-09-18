<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { faker } from '@faker-js/faker';

const clients = ref([{firstName: '', lastName: '', amount: '', avatar: '', email: ''}]);

onMounted( () => {
  clients.value = new Array(7).fill(0).map((_, i) => ({
    firstName: faker.name.firstName(),
    lastName: faker.name.lastName(),
    amount: faker.finance.amount(),
    avatar: faker.image.people(40, 40, true),
    email: faker.internet.email()
  }));
});

</script>
<template>
  <div className="rounded-md shadow-md p-6 flex flex-col gap-4 w-full items-start bg-white">
      <h2 className="text-2xl font-bold leading-tight my-2">Top Clients</h2>
      <div className="justify-center items-center w-full">
        <div v-for="(client, index) in clients" :key="index" class="px-2 py-4 w-full flex justify-between items-center">
          <div class="flex gap-4 items-center">
            <Image
              :src="client.avatar"
              width={40}
              height={40}
              alt="Avatar"
              class="rounded-full"
            />

            <p class="flex flex-col gap-1">
              <span class="text-slate-900 text-xs">
                {{ client.firstName }} {{ client.lastName }}
              </span>
              <span class="text-slate-500 text-xs">{{ client.email }}</span>
            </p>
          </div>
          <p class="text-slate-900 text-md font-semibold">£{{ client.amount }}</p>
        </div>
      </div>
    </div>
</template>