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
  }
]
</script>

<template>
  <div class="bg-[#f6f6f6] dark:bg-[#2f2f2f]">
    <main class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-12">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mb-2">
        <!-- Tool Info Card -->
        <div class="bg-white dark:bg-[#2f2f2f] p-8 rounded-xl w-full text-center">
          <h2 class="text-xl font-semibold mb-4 text-[#0f0f0f] dark:text-[#f6f6f6]">
            <span style="color: #b54925;">Shaco TFM - </span>
            <span style="color: #e48a1f;">All-In-One Tool</span>
          </h2>
          <img :src="Logo" alt="Logo" class="h-40 w-40 mx-auto mb-2" />
          <p class="flex justify-center items-center gap-2 mt-1 text-[#0f0f0f] dark:text-[#f6f6f6]">
            <!-- <DiscordIcon class="w-5 h-5 text-[#5865F2]" /> -->
            <img :src="DiscordLogo" alt="Discord Logo" class="w-8 h-8" />
            <a href="https://discord.gg/U3TTvaSaPS" target="_blank"
              class="decoration-none text-[#e48a1f] hover:underline">
              Join our Discord
            </a>
          </p>
        </div>

        <!-- Release Notes Card -->
        <div class="bg-white dark:bg-[#2f2f2f] p-8 rounded-xl border border-[#e48a1f] dark:border-[#e48b1f2f]">
          <h2 class="text-2xl font-semibold mb-6 text-[#0f0f0f] dark:text-[#f6f6f6]">
            <span style="color: #b54925;">Release Notes</span>
            <span style="color: #e48a1f;"> & Features</span>
            <span class="text-[#0f0f0f] dark:text-[#f6f6f6]"> (v0.2.0)</span>
          </h2>
          <div class="space-y-3">
            <ul class="list-disc pl-5 text-[15px] text-[#e0e0e0] dark:text-[#dcdcdc] leading-relaxed">
              <li v-for="feature in versionFeatures" :key="feature.description">
                {{ feature.description }}
              </li>
            </ul>
          </div>
        </div>
      </div>

      <!-- Credits -->
      <div
        class="bg-white dark:bg-[#2f2f2f] p-6 rounded-xl text-center border-b border-[#e48a1f] dark:border-[#e48a1f]">
        <h2 class="text-xl font-semibold text-[#0f0f0f] dark:text-[#f6f6f6] mb-2">
          Credits & Contributors
        </h2>
        <p class="text-sm text-gray-700 dark:text-gray-300">
          Made with <span class="text-[#e48a1f] text-md">♥</span> by <a href="https://github.com/Shaco-dev"
            target="_blank" class="text-[#e48a1f] hover:underline">Shaco-dev</a> —
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
