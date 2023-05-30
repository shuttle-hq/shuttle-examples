<script setup>
import { ref, onMounted } from 'vue';
import { useAccountStore } from '@/stores/account';

definePageMeta({
  layout: false,
});


const { email } = useAccountStore();

const data = ref(null);
const router = useRouter();
const id = router.currentRoute.value.params.id;

const handleDelete = async () => {
  const url = `//${window.location.host}/api/customers/${id}`;

  try {
    const res = await fetch(url, {
      mode: 'cors',
      method: 'DELETE',
      headers: new Headers({
        'Content-Type': 'application/json',
      }),
      body: JSON.stringify({
        email: email,
      }),
    });

    if (res.ok) {
      await router.push('/dashboard/customers');
    }
  } catch (e) {
    console.log(`Error: ${e}`);
  }
};

onMounted(async () => {
  const url = `//${window.location.host}/api/customers/${id}`;

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

    const customerData = await res.json();
    data.value = customerData;
  } catch (e) {
    console.log(`Error: ${e}`);
  }
});
</script>
<template>
  <NuxtLayout name="authed">
    <div class="py-10 flex flex-col items-center gap-4">
      <div v-if="data" class="px-10 py-4 bg-stone-200 flex flex-col gap-2 w-full">
        <p class="text-xl">{{ data.firstname }} {{ data.lastname }}</p>
        <p>Email: {{ data.email }}</p>
        <p>Phone: {{ data.phone }}</p>
        <button @click="handleDelete" class="px-5 py-2">Delete Customer</button>
      </div>
      <p v-else>User does not exist :(</p>
    </div>
  </NuxtLayout>
</template>
