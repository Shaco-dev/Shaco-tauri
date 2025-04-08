<script setup lang="ts">
import Logo from '../assets/logo.png'

const upcomingFeatures = [
  {
    title: 'Bulk Key Management',
    description: 'Add new translation keys to multiple languages simultaneously'
  },
  {
    title: 'Advanced Title Editor',
    description: 'Visual title customization with color picker and styling options'
  },
  {
    title: 'Format Conversion',
    description: 'Seamless conversion between JSON and text formats'
  }
]
</script>

<template>
  <div class="bg-[#f6f6f6] dark:bg-[#2f2f2f] ">
    <!-- Main Content -->
    <main class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-12">
      <!-- Feature Grid -->
      <div class="grid grid-cols-2 md:grid-cols-2 gap-8 mb-8">
        <div class="bg-white dark:bg-[#2f2f2f] p-8 rounded-xl w-full">
          <h2 class="text-2xl font-semibold mb-6 text-[#0f0f0f] dark:text-[#f6f6f6] text-center">
            <span style='color: #b54925;'>Shaco TFM - </span>

            <span style='color: #e48a1f;'>All-In-One Tool</span>
          </h2>
          <img :src="Logo" alt="Logo" class="h-54 w-54 m-auto" />
          <p class="m-auto text-center mt-5">
            <span style='color: #b54925;'>Version : </span>
            <span style='color: #e48a1f;'>0.0.1-beta</span>
          </p>
          <p class="m-auto text-center">
            <span style='color: #b54925;'>Discord : </span>
            <span style='color: #e48a1f;'>yaso_yt</span>
          </p>
        </div>
        <!-- Upcoming Features -->
        <div class="bg-white dark:bg-[#2f2f2f] p-8 rounded-xl border border-gray-200 dark:border-gray-700">
          <h2 class="text-2xl font-semibold mb-6 text-[#0f0f0f] dark:text-[#f6f6f6]">
            What's Coming
          </h2>
          <div class="space-y-6">
            <div v-for="roadmap in upcomingFeatures" :key="roadmap.title"
              class="border-l-4 border-[#646cff] dark:border-[#ffbd2e] pl-4">
              <h3 class="font-medium text-[#0f0f0f] dark:text-[#f6f6f6] mb-2">
                {{ roadmap.title }}
              </h3>
              <p class="text-gray-600 dark:text-gray-300 text-sm">
                {{ roadmap.description }}
              </p>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>


<style scoped>
.flex-col.items-center {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
