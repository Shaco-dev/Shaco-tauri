<script setup lang="ts">
import Logo from '../assets/logo.png'
import DiscordLogo from '../assets/discord.png'


const currentVersion = '0.4.1'

const versionsHistory = {
  '0.4.1': {
    date: '2025-04-20',
    features: [
      'fix window layout & disable maximization',
    ]
  },
  '0.4.0': {
    date: '2025-04-20',
    features: [
      'New feature to download known swf libraries (.swf)',
      'New feature to download known music and sound (.mp3)',
      'New feature to download images from selected database(.png/.jpg)',
      // 'New popup for accepting / refusing auto-updates',
      'Dark mode is now enforced due to issues with light mode',
      'UI improvements'
    ]
  },
  '0.3.0': {
    date: '2025-04-17',
    features: [
      'Fixed light/dark mode UI inconsistencies',
      'Improved overall code quality and performance',
      'Added feature to add translations for specific languages',
      'Add support for editing translation key'
    ]
  },
  '0.2.1': {
    date: '',
    features: [
      'Add support for auto-update',
      'Add new feature to download the latest translations',
      'Exporting translations to Text/Gz file format',
    ]
  },
  '0.1.0': {
    date: '',
    features: [
      'Initial release of Shaco TFM',
      'Basic translation file management'
    ]
  }
}
</script>

<template>
  <div class="bg-[#f6f6f6] dark:bg-[#2f2f2f]">
    <main class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-6">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mb-2">
        <!-- Tool Info Card -->
        <div class="bg-white dark:bg-[#0f0f0f98] p-8 rounded-xl w-full text-center shadow">
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
        <div class="bg-white dark:bg-[#0f0f0f98] rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm">
          <h2 class="text-xl font-semibold text-[#1f1f1f] dark:text-[#f6f6f6] p-4">
            <span class="text-[#b54925]">Release History</span>
            <span class="text-[#e48a1f]"> & Changelog</span>
          </h2>
          <div class="px-6">
            <div class="overflow-y-auto h-58 pr-4 scrollbar">
              <div v-for="[version, details] in Object.entries(versionsHistory)" :key="version"
                class="mb-4 pb-6 border-b border-gray-200 dark:border-gray-700 last:border-0 last:pb-0">
                <div class="flex justify-between items-start mb-3">
                  <div>
                    <h3 class="text-lg font-semibold text-[#1f1f1f] dark:text-[#f6f6f6]">
                      v{{ version }}
                      <span v-if="version === currentVersion"
                        class="ml-2 px-2 py-1 bg-[#e48a1f] text-white text-xs rounded-full">
                        Latest
                      </span>
                    </h3>
                    <p v-if="details.date" class="text-sm text-gray-500 dark:text-gray-400">
                      Released on {{ details.date }}
                    </p>
                  </div>
                </div>
                <ul class="space-y-1">
                  <li v-for="(feature, index) in details.features" :key="index"
                    class="flex items-start text-[#1f1f1f] dark:text-[#f6f6f6]">
                    <span class="text-[#e48a1f] mr-2">•</span>
                    <span class="text-sm">{{ feature }}</span>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Credits -->
      <div
        class="mt-6 bg-white dark:bg-[#0f0f0f98] p-4 rounded-xl text-center border-t border-[#e48a1f] dark:border-[#e48a1f] shadow">
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
