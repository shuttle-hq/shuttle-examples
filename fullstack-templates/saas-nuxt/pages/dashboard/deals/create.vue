<script setup>
import { ref, onMounted } from 'vue';
import { useAccountStore } from '@/stores/account';

definePageMeta({
  layout: false,
});


const { email } = useAccountStore();
const estimate = ref('');
const custId = ref('');
const custnames = ref([]);

const router = useRouter();

const handleSubmit = async () => {
  const url = `//${window.location.host}/api/deals/create`;

  try {
    const res = await fetch(url, {
      method: 'POST',
      mode: 'cors',
      headers: new Headers({
        'Content-Type': 'application/json',
      }),

      body: JSON.stringify({
        estimatedworth: parseInt(estimate.value),
        cust_id: parseInt(custId.value),
        useremail: email,
      }),
    });

    if (res.status == 403) {
      return router.push('/login');
    }

    router.push('/dashboard/deals');
  } catch (e) {
    console.log(`Error: ${e}`);
  }
};

onMounted(async () => {
  const url = `//${window.location.host}/api/customers/names`;

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

    if (res.status == 403) {
      return router.push('/login');
    }

    const data = await res.json();

    custnames.value = data;
  } catch (e) {
    console.log(`Error: ${e}`);
  }
});
</script>
<template>
  <NuxtLayout name="authed">
    <form
      class="min-h-screen flex flex-col items-center justify-center bg-gray-100"
      @submit.prevent="handleSubmit"
    >
      <div
        class="
        flex flex-col
        bg-white
        shadow-md
        px-4
        sm:px-6
        md:px-8
        lg:px-10
        py-8
        rounded-md
        w-50
        max-w-md"
      >
        <h1 class="lg:text-2xl text-xl text-center">New Deal</h1>
        <fieldset class="mt-10">
          <label for="estimated_worth">
            Total to be invoiced:
            <input
              type="text"
              name="estimated_worth"
              class="text-sm mb-4
                placeholder-gray-500
                px-4
                rounded-md
                border border-gray-400
                w-full
                py-2
                focus:outline-none focus:border-black"
              v-model="estimate"
              placeholder="Enter amount"
            />
          </label>
          <label for="custId">
            Customer:
            <select
              v-model="custId"
              class="text-sm mb-4
                placeholder-gray-500
                px-4
                rounded-md
                border border-gray-400
                w-full
                py-2
                focus:outline-none focus:border-black bg-white"
            >
              <option value=""></option>
              <option v-for="cust in custnames" :key="cust.id" :value="cust.id">{{ cust.customer_name }}</option>
            </select>
          </label>
          <button
            type="submit"
            class="flex
              mt-2
              items-center
              justify-center
              focus:outline-none
              text-white text-sm
              sm:text-base
              bg-black
              hover:bg-slate-950
              rounded-md
              py-2
              w-full
              transition
              duration-150
              ease-in"
          >
            <span class="mr-2 uppercase">Create</span>
          </button>
        </fieldset>
      </div>
    </form>
  </NuxtLayout>
</template>