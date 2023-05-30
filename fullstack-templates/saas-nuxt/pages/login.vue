<script setup lang="ts">
import { ref } from 'vue';
import { useAccountStore } from '@/stores/account';
import { faAt, faLock, faEye, faEyeSlash } from '@fortawesome/free-solid-svg-icons';

const loginEmail = ref('');
const pw = ref('');
const pwVis = ref(false);

const { changeEmail } = useAccountStore();

const router = useRouter();
const route = useRoute();

const handleSubmit = async (e: Event) => {
  e.preventDefault();

  const url = "/api/auth/login";

  try {
    let res = await fetch(url, {
      method: 'POST',
      //mode: 'cors',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        email: loginEmail.value,
        password: pw.value,
      }),
    })
    // if response type is 200 then redirect to dashboard
    if (res.ok) {
      changeEmail(loginEmail.value);
      router.push('/dashboard');
    } else {
      console.log('Incorrect login details.');
    }
  } catch (e) {
    console.log(`Error: ${e}`);
  }
};

const togglePassword = (e: Event) => {
  e.preventDefault();

  pwVis.value = !pwVis.value;
};
</script>
<template>
  <form
    class="px-5 min-h-screen flex flex-col items-center justify-center bg-gray-100"
    @submit="handleSubmit"
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
      <h1 class="lg:text-2xl text-xl text-center font-bold">Welcome Back</h1>
      <fieldset class="mt-10">
        <label for="email" class="text-xs tracking-wide text-gray-600">
          E-Mail Address:
        </label>
        <div class="relative mb-4">
          <font-awesome-icon
            class="inline-flex
              items-center
              justify-center
              absolute
              left-3
              top-[30%]
              h-full"
            :icon="faAt"
            color="black"
          />
          <input
            type="email"
            name="email"
            class="text-sm
              placeholder-gray-500
              pl-10
              pr-4
              rounded-md
              border border-gray-400
              w-full
              py-2
              focus:outline-none focus:border-black"
            v-model="loginEmail"
            placeholder="Enter your email"
          />
        </div>
        <label for="password" class="text-xs tracking-wide text-gray-600">
          Password:
        </label>

        <div class="relative mb-4">
          <font-awesome-icon
            class="inline-flex
              items-center
              justify-center
              absolute
              left-3
              top-[30%]
              h-full"
            :icon="faLock"
            color="black"
          />

          <input
            :type="pwVis ? 'text' : 'password'"
            name="password"
            class="text-sm
              placeholder-gray-500
              pl-10
              pr-4
              rounded-md
              border border-gray-400
              w-full
              py-2
              focus:outline-none focus:border-black"
            v-model="pw"
            placeholder="Enter your password"
          />
          <font-awesome-icon
            class="inline-flex
              items-center
              justify-center
              absolute
              right-3
              top-[30%]
              h-full"
            :icon="pwVis ? faEyeSlash : faEye"
            @click="togglePassword"
            color="black"
          />
        </div>

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
              ease-in
            "
          >
            <span class="mr-2 uppercase">Sign In &rarr;</span>
          </button>
        </div>
      </fieldset>
    </div>
    <div class="flex justify-center items-center mt-6">
      <span
        class=" inline-flex
        items-center
        text-gray-700
        font-medium
        text-xs text-center"
      >
        Don&apos;t have an account?
        <nuxt-link to="/register" class="text-xs ml-2 text-black font-semibold">
          Register now
        </nuxt-link>
      </span>
    </div>
  </form>
</template>

