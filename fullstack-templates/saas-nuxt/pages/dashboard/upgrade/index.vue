<script setup lang="ts">
definePageMeta({
  layout: false,
});

</script>
  <template>
  <NuxtLayout name="authed">
    <section class="py-10 flex flex-col justify-center items-center w-full">
      <h1 class="text-xl lg:text-3xl text-center my-10">Pricing</h1>

      <div class="">
        <div class="container px-6 py-8 mx-auto">
          <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
            <!-- Pricing card 1 -->
            <div class="flex flex-col w-full max-w-sm p-8 space-y-8 text-center bg-white border border-gray-200 rounded-md ">
              <div class="flex-shrink-0">
                <h2 class="inline-flex items-center justify-center px-2 font-semibold tracking-tight text-black uppercase rounded bg-gray-50">
                  Casual
                </h2>
              </div>
              <div class="flex-shrink-0">
                <span class="pt-2 text-4xl font-bold text-gray-800 uppercase">Free</span>
              </div>
              <ul class="flex-1 space-y-4">
                <li class="text-gray-500">Up to 5 projects</li>
                <li class="text-gray-500">Up to 10 collaborators</li>
                <li class="text-gray-500">2Gb of storage</li>
              </ul>

              <span class="h-10 flex justify-center items-center">Your plan</span>
            </div>

            <!-- Pricing card 2 -->
            <div class="flex flex-col w-full max-w-sm p-8 space-y-8 text-center bg-white border-2 border-gray-200 rounded-md  ">
              <div class="flex-shrink-0">
                <h2 class="inline-flex items-center justify-center px-2 font-semibold tracking-tight text-black uppercase rounded bg-gray-50">
                  Professional
                </h2>
              </div>
              <div class="flex-shrink-0">
                <span class="pt-2 text-4xl font-bold text-gray-800 uppercase">$24.90</span>
                <span class="text-gray-500">/month</span>
              </div>
              <ul class="flex-1 space-y-4">
                <li class="text-gray-500">Up to 10 projects</li>
                <li class="text-gray-500">Up to 20 collaborators</li>
                <li class="text-gray-500">10Gb of storage</li>
                <li class="text-gray-500">Real-time collaborations</li>
              </ul>

              <NuxtLink
                to="/dashboard/upgrade/checkout"
                class="inline-flex items-center justify-center px-4 py-2 font-semibold text-white uppercase transition-colors bg-black rounded hover:bg-slate-950 focus:outline-none"
              >
                Upgrade
              </NuxtLink>
            </div>

            <!-- Pricing card 3 -->
            <div class="flex flex-col w-full max-w-sm p-8 space-y-8 text-center bg-white border-2 border-gray-200 rounded-md  ">
              <div class="flex-shrink-0">
                <h2 class="inline-flex items-center justify-center px-2 font-semibold tracking-tight text-black uppercase rounded bg-gray-50 ">
                  Expert
                </h2>
              </div>
              <div class="flex-shrink-0">
                <span class="pt-2 text-4xl font-bold text-gray-800 uppercase ">$49.90</span>
                <span class="text-gray-500 ">/month</span>
              </div>
              <ul class="flex-1 space-y-4">
                <li class="text-gray-500">Unlimited projects</li>
                <li class="text-gray-500">Unlimited collaborators</li>
                <li class="text-gray-500">Unlimited storage</li>
                <li class="text-gray-500">Real-time collaborations</li>
                <li class="text-gray-500">24x7 Support</li>
              </ul>

              <NuxtLink
                to="/dashboard/upgrade/checkout"
                class="inline-flex items-center justify-center px-4 py-2 font-semibold text-white uppercase transition-colors bg-black rounded hover:bg-slate-950 focus:outline-none"
              >
                Upgrade
              </NuxtLink>
            </div>
          </div>
        </div>
      </div>
    </section>
  </NuxtLayout>
</template>
