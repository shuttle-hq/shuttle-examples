<template>
  <NuxtLayout name="authed">
  <form
    class="px-5 min-h-screen flex flex-col items-center justify-center bg-gray-100"
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
      <h1 class="lg:text-2xl text-xl text-center font-bold">Checkout</h1>

      <fieldset class="mt-10">
        <label for="name" class="text-xs tracking-wide text-gray-600">
          Name:
        </label>
        <input
          type="text"
          name="name"
          required
          class="text-sm
            placeholder-gray-500
            px-4
            rounded-md
            border border-gray-400
            w-full
            py-2
            focus:outline-none focus:border-black mb-4"
          v-model="name"
          placeholder="Enter your full name"
        />

        <!-- similar for email, card, year, month, cvc -->

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
            <span class="mr-2 uppercase">Submit &rarr;</span>
          </button>
        </div>
      </fieldset>
    </div>
  </form>
  </NuxtLayout>
</template>

<script>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

definePageMeta({
  layout: false,
});


export default {
  setup() {
    const router = useRouter()

    const name = ref('')
    const email = ref('')
    const card = ref('')
    const year = ref('')
    const month = ref('')
    const cvc = ref('')

    const handleSubmit = async () => {
      const url = `//${window.location.host}/api/payments/pay`

      try {
        const res = await fetch(url, {
          method: 'POST',
          mode: 'cors',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            name: name.value,
            email: email.value,
            card: card.value,
            expyear: parseInt(year.value),
            expmonth: parseInt(month.value),
            cvc: cvc.value,
          }),
        })

        if (res.ok) {
          router.push('/dashboard/upgrade/checkout/success')
        }
      } catch (e) {
        console.log(`Error: ${e}`)
      }
    }

    return {
      name,
      email,
      card,
      year,
      month,
      cvc,
      handleSubmit,
    }
  },
}
</script>
