<script setup lang="ts">
import Logo from '../assets/logo.png'
import DiscordLogo from '../assets/discord.png'

const versionFeatures = [
  {
    description: 'Added feature to download languages with a beautiful and simple interface.'
  },
  {
    description: 'Introduced a live language editor for easy translation management.'
  },
  {
    description: 'Implemented an auto-update feature to keep the app up-to-date.'
  },
  {
    description: 'Fixed an issue where languages did not load correctly after download.'
  },
]
</script>

<template>
  <div class="bg-[#f6f6f6] dark:bg-[#2f2f2f] min-h-screen">
    <main class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-12">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mb-2">
        <!-- Tool Info Card -->
        <div class="bg-white dark:bg-[#2f2f2f] p-8 rounded-xl w-full text-center shadow">
          <h2 class="text-xl font-semibold mb-4 text-[#1f1f1f] dark:text-[#f6f6f6]">
            <span class="text-[#b54925]">Shaco TFM - </span>
            <span class="text-[#e48a1f]">All-In-One Tool</span>
          </h2>
          <img :src="Logo" alt="Logo" class="h-40 w-40 mx-auto mb-4" />
          <p class="flex justify-center items-center gap-2 text-[#1f1f1f] dark:text-[#f6f6f6]">
            <img :src="DiscordLogo" alt="Discord Logo" class="w-6 h-6" />
            <a href="https://discord.gg/U3TTvaSaPS" target="_blank" class="text-[#e48a1f] hover:underline">
              Join our Discord
            </a>
          </p>
        </div>

        <!-- Release Notes Card -->
        <div class="bg-white dark:bg-[#2f2f2f] p-8 rounded-xl border border-[#e48a1f] dark:border-[#e48b1f2f] shadow">
          <h2 class="text-2xl font-semibold mb-6 text-[#1f1f1f] dark:text-[#f6f6f6]">
            <span class="text-[#b54925]">Release Notes</span>
            <span class="text-[#e48a1f]"> & Features</span>
            <span class="text-[#1f1f1f] dark:text-[#f6f6f6]"> (v0.2.1)</span>
          </h2>
          <ul class="list-disc pl-5 text-[15px] text-[#333] dark:text-[#dcdcdc] leading-relaxed space-y-2">
            <li v-for="feature in versionFeatures" :key="feature.description">
              {{ feature.description }}
            </li>
          </ul>
        </div>
      </div>

      <!-- Credits -->
      <div class="bg-white dark:bg-[#2f2f2f] p-6 rounded-xl text-center border-t border-[#e48a1f] dark:border-[#e48a1f] shadow">
        <h2 class="text-xl font-semibold text-[#1f1f1f] dark:text-[#f6f6f6] mb-2">
          Credits & Contributors
        </h2>
        <p class="text-sm text-gray-700 dark:text-gray-300">
          Made with <span class="text-[#e48a1f] text-md">♥</span> by
          <a href="https://github.com/Shaco-dev" target="_blank" class="text-[#e48a1f] hover:underline">Shaco-dev</a> —
          idea support by <span class="text-[#b54925]">Amr Dev</span>.
        </p>
        <p class="text-sm text-gray-700 dark:text-gray-300 mt-1">
          You're welcome to contribute! Check out the repository on
          <a href="https://github.com/Shaco-dev/Shaco-tauri" target="_blank" class="text-[#646cff] hover:underline">
            github.com/Shaco-dev/Shaco-tauri
          </a>
        </p>
      </div>
    </main>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
