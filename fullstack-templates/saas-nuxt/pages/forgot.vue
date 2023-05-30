<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'

let router = useRouter()

let firstName = ref("")
let lastName = ref("")
let email = ref("")
let phone = ref("")
let priority = ref("")

const handleSubmit = async () => {
  const url = `//${window.location.host}/api/auth/register`

  try {
    let res = await fetch(url, {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        firstName: firstName.value,
        lastName: lastName.value,
        email: email.value,
        phone: phone.value,
        priority: Number(priority.value)
      }),
    })

    if (res.ok) {
      router.push("/dashboard/customers")
    }

  } catch (e: any) {
    console.log(`Error: ${e}`)
  }
}
</script>
<template>
    <form @submit.prevent="handleSubmit" class="py-10 flex flex-col gap-4 justify-center items-center">
      <h1 class="lg:text-2xl text-xl text-center">Create Customer</h1>
      <label for="firstname">
        <span>First name: </span>
        <input type="text" name="firstname" class="px-5 py-2" v-model="firstName">
      </label>
      <label for="lastname">
        <span>Last name: </span>
        <input type="text" name="lastname" class="px-5 py-2" v-model="lastName">
      </label>
      <label for="email">
        <span>Email address: </span>
        <input type="email" name="email" class="px-5 py-2" v-model="email">
      </label>
      <label for="phone">
        <span>Mobile number: </span>
        <input type="text" name="phone" class="px-5 py-2" v-model="phone">
      </label>
      <label for="priority">
        <span>Priority: </span>
        <select name="priority" v-model="priority">
          <option value="1">Very Low</option>
          <option value="2">Low</option>
          <option value="3">Medium</option>
          <option value="4">High</option>
          <option value="5">Very High</option>
        </select>
      </label>
      <button type="submit">Submit</button>
    </form>
</template>