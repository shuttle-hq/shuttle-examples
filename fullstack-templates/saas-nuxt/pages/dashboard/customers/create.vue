<script setup>
import { ref } from 'vue';
import { useAccountStore } from '@/stores/account';

definePageMeta({
  layout: false,
});

const firstName = ref('');
const lastName = ref('');
const custEmail = ref('');
const phone = ref('');
const priority = ref('1');
const accountStore = useAccountStore();
const router = useRouter();

const handleSubmit = async () => {
  const url = `//${window.location.host}/api/customers/create`;

  try {
    const res = await fetch(url, {
      method: 'POST',
      mode: 'cors',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        firstName: firstName.value,
        lastName: lastName.value,
        email: custEmail.value,
        phone: phone.value,
        priority: Number(priority.value),
        userEmail: accountStore.email,
      }),
    });

    if (res.ok) {
      await router.push('/dashboard/customers');
    }
  } catch (e) {
    console.log(`Error: ${e}`);
  }
};
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
        <h1 class="lg:text-2xl text-xl text-center">New Customer</h1>
        <fieldset class="mt-10">
          <label for="first_name" class="text-xs tracking-wide text-gray-600">
            First name:
            <input
              type="text"
              name="first_name"
              class="text-sm mb-4
                placeholder-gray-500
                px-4
                rounded-md
                border border-gray-400
                w-full
                py-2
                focus:outline-none focus:border-black"
              v-model="firstName"
              placeholder="Enter first name"
            />
          </label>

          <label for="last_name" class="text-xs tracking-wide text-gray-600">
            Last name:
            <input
              type="text"
              name="last_name"
              class="text-sm mb-4
                placeholder-gray-500
                px-4
                rounded-md
                border border-gray-400
                w-full
                py-2
                focus:outline-none focus:border-black"
              v-model="lastName"
              placeholder="Enter last name"
            />
          </label>
          <label for="email" class="text-xs tracking-wide text-gray-600">
            Email address:
            <input
              type="email"
              name="email"
              class="text-sm mb-4
                placeholder-gray-500
                px-4
                rounded-md
                border border-gray-400
                w-full
                py-2
                focus:outline-none focus:border-black"
              v-model="custEmail"
              placeholder="Enter email address"
            />
          </label>
          <label for="phone" class="text-xs tracking-wide text-gray-600">
            <span>Mobile number: </span>
            <input
              type="text"
              name="phone"
              class="text-sm mb-4
                placeholder-gray-500
                px-4
                rounded-md
                border border-gray-400
                w-full
                py-2
                focus:outline-none focus:border-black"
              v-model="phone"
              placeholder="Enter mobile number"
            />
          </label>
          <label for="priority" class="text-xs tracking-wide text-gray-600">
            <span>Priority: </span>
            <select
              name="priority"
              class="text-sm mb-4
                placeholder-gray-500
                px-4
                rounded-md
                border border-gray-400
                w-full
                py-2
                focus:outline-none focus:border-black bg-white"
              v-model="priority"
            >
              <option value="1">Very Low</option>
              <option value="2">Low</option>
              <option value="3">Medium</option>
              <option value="4">High</option>
              <option value="5">Very High</option>
            </select>
          </label>
          <div class="flex w-full">
            <button
              type="submit"
              class="
                flex
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
          </div>
        </fieldset>
      </div>
    </form>
  </NuxtLayout>
</template>
