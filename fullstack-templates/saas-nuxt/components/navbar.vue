<script setup lang="ts">
import { ref, toRefs } from "vue";
import { useAccountStore } from "@/stores/account";

const accountStore = useAccountStore();
const router = useRouter();
const open = ref(false);
const { email } = toRefs(accountStore);

const toggleOpen = () => {
  open.value = !open.value;
};

const handleLogout = async () => {
  const url = `//${window.location.host}/api/auth/logout`;

  const res = await fetch(url);

  if (res.ok) {
    accountStore.changeEmail('');
    router.push('/');
  }
};
</script>
<template>
  <div>
    <Navigation v-if="email === ''" />

    <aside
      v-else
      class="flex flex-col w-full relative z-10 lg:w-64 lg:h-screen px-5 py-8 overflow-y-auto bg-white border-r rtl:border-r-0 rtl:border-l dark:bg-gray-900 dark:border-gray-700"
      :class="{'h-screen': open}"
    >
      <button class="absolute right-6 dark:text-[#D8D8D8] lg:hidden" @click="toggleOpen">
        <font-awesome-icon icon="faBars" color="black" size="2x" fixedWidth />
      </button>
      <NuxtLink to="/">
        <font-awesome-icon icon="faRocket" size="2x" color="black" />
      </NuxtLink>
      <div class="lg:flex flex-col justify-between flex-1 mt-6" :class="{'hidden': !open}">
        <nav class="-mx-3 space-y-3 lg:space-y-6 flex flex-col lg:justify-between lg:h-full">
          <div class="space-y-3">
            <NuxtLink
              class="flex items-center px-3 py-2 transition-colors duration-300 transform rounded-lg dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 dark:hover:text-gray-200 hover:text-gray-700"
              to="/dashboard"
            >
              <font-awesome-icon icon="faGauge" color="black" fixedWidth />
              <span class="mx-2 text-sm font-medium">Dashboard</span>
            </NuxtLink>

            <NuxtLink
              class="flex items-center px-3 py-2 transition-colors duration-300 transform rounded-lg dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 dark:hover:text-gray-200 hover:text-gray-700"
              to="/dashboard/customers"
            >
              <font-awesome-icon icon="faUsers" color="black" fixedWidth />
              <span class="mx-2 text-sm font-medium">Customers</span>
            </NuxtLink>

            <NuxtLink
              class="flex items-center px-3 py-2 transition-colors duration-300 transform rounded-lg dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 dark:hover:text-gray-200 hover:text-gray-700"
              to="/dashboard/deals"
            >
              <font-awesome-icon icon="faHandshake" color="black" fixedWidth />
              <span class="mx-2 text-sm font-medium">Deals</span>
            </NuxtLink>

            <NuxtLink
              class="flex items-center px-3 py-2 transition-colors duration-300 transform rounded-lg dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 dark:hover:text-gray-200 hover:text-gray-700"
              to="/dashboard/upgrade"
            >
              <font-awesome-icon icon="faCoins" color="black" fixedWidth />
              <span class="mx-2 text-sm font-medium">Upgrade</span>
            </NuxtLink>
          </div>

          <div class="space-y-3 flex flex-col dark:text-gray-200 items-start">
            <div class="px-3">
              <font-awesome-icon icon="faUserSecret" color="black" fixedWidth />
              <span class="text-sm mx-2 font-medium">{{ email }}</span>
            </div>
            <div class="px-3">
              <font-awesome-icon icon="faRightFromBracket" color="black" fixedWidth />
              <button class="text-sm mx-2 font-medium" @click="handleLogout">Log Out</button>
            </div>
          </div>
        </nav>
      </div>
    </aside>
  </div>
</template>