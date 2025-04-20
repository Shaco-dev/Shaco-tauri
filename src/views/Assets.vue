<template>
  <div class="bg-[#f6f6f6] dark:bg-[#2f2f2f]">
    <!-- Top Navigation -->
    <nav
      class="sticky top-0 z-50 border-b border-gray-300 dark:border-[#ffbd2e] backdrop-blur bg-white/70 dark:bg-[#2f2f2f]/80">
      <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div class="flex h-16 items-center justify-between text-sm">
          <span
            class="flex items-center bg-white shadow-md w-36 rounded-xl px-3 py-1 transition-colors duration-200 border-b border-[#ffbd2e] hover:bg-[#f9f9f9] dark:hover:bg-[#0f0f0f98] dark:bg-[#0f0f0f98]">
            <img :src="AssetsLogo" alt="Logo" class="mr-2" />
            <span class="text-md font-bold text-[#ffbd2e]">Assets</span>
          </span>

          <!-- toolbar actions here -->
        </div>
      </div>
    </nav>

    <!-- animation navigation  -->
    <div class="pt-4 px-4">
      <div class="flex items-center justify-center mb-3">
        <div class="bg-white dark:bg-[#0f0f0f98] flex gap-2 rounded-xl p-1 transition-colors duration-200 relative">
          <!-- Navigation Items -->
          <div v-for="tab in tabs" :key="tab.id"
            class="text-sm text-gray-700 dark:text-gray-300 px-3 py-1 rounded-xl cursor-pointer transition-colors duration-200"
            :class="{ 'text-[#ffbd2e] font-medium': currentAssetType === tab.id }" @click="setAssetType(tab.id)"
            ref="tabRefs">
            {{ tab.label }}
          </div>

          <!-- Animated Underline -->
          <div class="absolute bottom-0 h-0.5 bg-[#ffbd2e] rounded-full transition-all duration-300 w-10"
            :style="{ left: `${underlineLeft}px` }"></div>
        </div>
      </div>
    </div>

    <!-- Main Content with transition -->
    <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 overflow-hidden">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-6">
        <!-- Asset Selection Panels in a vertical layout -->
        <div class="lg:col-span-9 relative">
          <transition name="fade" mode="out-in">
            <div v-if="currentAssetType === 'images'" key="images" class="grid grid-cols-1 lg:grid-cols-3 gap-4">
              <!-- Images Panel -->
              <div
                class="rounded-xl bg-white dark:bg-[#0f0f0f98] p-5 shadow-sm border border-gray-200 dark:border-gray-700">
                <div class="flex items-center justify-between mb-3">
                  <h3 class="text-md font-medium text-gray-700 dark:text-gray-300 flex items-center">
                    <span class="text-xl mr-2">
                      <img :src="AssetImages" alt="Images" class="w-10" />
                    </span> Images
                  </h3>
                  <div class="flex gap-2">
                    <BaseButton v-if="!isDownloading" :loading="loadingDatabase" :icon="CloudArrowDownIcon"
                      :disabled="!isFormValid || loadingDatabase" @click="downloadImages" variant="primary" size="sm">
                      Download
                    </BaseButton>
                    <BaseButton v-else @click="cancelDownload" :icon="XMarkIcon" variant="danger" size="sm">
                      Cancel
                    </BaseButton>
                    <BaseButton v-if="isDownloading && isPaused" @click="resumeDownload" :icon="PlayIcon"
                      variant="primary" size="sm">
                      Resume
                    </BaseButton>
                    <BaseButton v-else-if="isDownloading" @click="pauseDownload" :icon="PauseIcon" variant="primary"
                      size="sm">
                      Pause
                    </BaseButton>
                  </div>
                </div>
                <div class="space-y-4">
                  <div>
                    <label class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">Database URL</label>
                    <ShacoInput :disabled="isDownloading" v-model="databaseUrl" placeholder="Enter database URL"
                      variant="sm" icon="database" />
                    <p v-if="dbError == ''" class="mt-1 text-xs text-gray-500">Database for images dictionary (json)</p>
                    <p v-else class="mt-1 text-xs text-red-500">{{ dbError }}</p>
                  </div>

                  <div class="grid grid-cols-12 gap-4">
                    <div class="col-span-10">
                      <label class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">Download Server
                        (source)</label>
                      <ShacoInput :disabled="isDownloading" v-model="imageDownloadServer" placeholder="Enter server URL"
                        variant="sm" icon="server" />
                      <p class="mt-1 text-xs text-gray-500">Source server for downloads</p>
                    </div>
                    <div class="col-span-2">
                      <label class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">Interval
                        (ms)</label>
                      <ShacoInput :disabled="isDownloading" type="number" v-model="interval"
                        placeholder="Enter interval" variant="sm" icon="clock" />
                      <p class="mt-1 text-xs text-gray-500">Interval for downloads</p>
                    </div>
                  </div>

                  <div>
                    <label class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">Download Path</label>
                    <PathSelector :disabled="isDownloading" :lengthToTruncate="10" v-model="downloadPath"
                      placeholder="Select download directory..." size="sm" />
                    <p class="mt-1 text-xs text-gray-500">Local destination for assets</p>
                  </div>
                </div>

                <!-- Download Progress -->
                <div v-if="isDownloading">
                  <div class="flex items-center justify-between mb-1 mt-2">
                    <span class="text-sm text-gray-500">{{ Math.round(downloadProgress) }}%</span>
                  </div>
                  <ProgressBar :progress="downloadProgress" color="bg-[#ffbd2e] dark:bg-[#ffbd2e]" height="h-1" />
                  <div class="mt-3 flex justify-between text-sm">
                    <span v-if="downloadError != ''" class="text-red-500 dark:text-red-500">
                      {{ downloadError }}
                    </span>
                    <span v-else class="text-gray-700 dark:text-gray-300">
                      Downloading : {{ currentFile }}
                    </span>
                    <span class="font-medium text-gray-600 dark:text-gray-400">
                      {{ downloadedCount }}/{{ totalFiles }}
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <div v-else-if="currentAssetType === 'music'" key="music" class="grid grid-cols-1 lg:grid-cols-3 gap-4">
              <!-- Music Panel -->
              <div
                class="rounded-xl bg-white dark:bg-[#0f0f0f98] p-5 shadow-sm border border-gray-200 dark:border-gray-700">
                <div class="flex items-center justify-between mb-3">
                  <h3 class="text-md font-medium text-gray-700 dark:text-gray-300 flex items-center">
                    <span class="text-xl mr-2">
                      <img :src="AssetMusic" alt="Music" class="w-10" />
                    </span> Musique & Son <span class="text-sm text-gray-500 ml-2">{{ musicFiles.length}} Musics & {{ soundFiles.length}} Sounds Known</span>
                  </h3>
                  <div class="flex gap-2">
                    <BaseButton v-if="!isDownloading" :loading="loadingDatabase" :icon="CloudArrowDownIcon"
                      :disabled="!isFormValid || loadingDatabase" @click="downloadAssets('music')" variant="primary"
                      size="sm">
                      Download
                    </BaseButton>
                    <BaseButton v-else @click="cancelDownload" :icon="XMarkIcon" variant="danger" size="sm">
                      Cancel
                    </BaseButton>
                    <BaseButton v-if="isDownloading && isPaused" @click="resumeDownload" :icon="PlayIcon"
                      variant="primary" size="sm">
                      Resume
                    </BaseButton>
                    <BaseButton v-else-if="isDownloading" @click="pauseDownload" :icon="PauseIcon" variant="primary"
                      size="sm">
                      Pause
                    </BaseButton>
                  </div>
                </div>
                <div class="space-y-4">
                  <div class="grid grid-cols-12 gap-4">
                    <div class="col-span-10">
                      <label class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">Download Server
                        (source)</label>
                      <ShacoInput :disabled="isDownloading" v-model="musicDownloadServer" placeholder="Enter server URL"
                        variant="sm" icon="server" />
                      <p class="mt-1 text-xs text-gray-500">Source server for downloads</p>
                    </div>
                    <div class="col-span-2">
                      <label class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">Interval
                        (ms)</label>
                      <ShacoInput :disabled="isDownloading" type="number" v-model="interval"
                        placeholder="Enter interval" variant="sm" icon="clock" />
                      <p class="mt-1 text-xs text-gray-500">Interval for downloads</p>
                    </div>
                  </div>
                  <div>
                    <label class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">Download Path</label>
                    <PathSelector :disabled="isDownloading" :lengthToTruncate="10" v-model="downloadPath"
                      placeholder="Select download directory..." size="sm" />
                    <p class="mt-1 text-xs text-gray-500">Local destination for assets</p>
                  </div>
                </div>
                <!-- Download Progress -->
                <div v-if="isDownloading">
                  <div class="flex items-center justify-between mb-1 mt-2">
                    <span class="text-sm text-gray-500">{{ Math.round(downloadProgress) }}%</span>
                  </div>
                  <ProgressBar :progress="downloadProgress" color="bg-[#ffbd2e] dark:bg-[#ffbd2e]" height="h-1" />
                  <div class="mt-3 flex justify-between text-sm">
                    <span v-if="downloadError != ''" class="text-red-500 dark:text-red-500">
                      {{ downloadError }}
                    </span>
                    <span v-else class="text-gray-700 dark:text-gray-300">
                      Downloading : {{ currentFile }}
                    </span>
                    <span class="font-medium text-gray-600 dark:text-gray-400">
                      {{ downloadedCount }}/{{ totalFiles }}
                    </span>
                  </div>
                </div>
                <!-- <p class="text-xs text-gray-600 dark:text-gray-400 mb-2 mt-3">
                  {{musicFiles.filter(f => f.selected).length}} of {{ musicFiles.length }} files selected
                </p>
                <div class="max-h-36 overflow-y-auto scrollbar pr-1">
                  <div v-for="(item, index) in musicFiles" :key="index"
                    class="flex items-center bg-gray-50 dark:bg-[#0f0f0f69] rounded-lg p-1.5 mb-1.5 cursor-pointer hover:bg-gray-100 dark:hover:bg-[#1f1f1f]"
                    :class="{ 'border border-[#396cd8] dark:border-[#24c8db]': item.selected }"
                    @click="toggleSelection(item)">
                    <input type="checkbox" :checked="item.selected" class="mr-2" />
                    <span class="text-xs truncate">{{ item.name }}</span>
                  </div>
                </div> -->
              </div>
            </div>

            <div v-else-if="currentAssetType === 'libraries'" key="libraries"
              class="grid grid-cols-1 lg:grid-cols-3 gap-4">
              <!-- Libraries Panel -->
              <div
                class="rounded-xl bg-white dark:bg-[#0f0f0f98] p-5 shadow-sm border border-gray-200 dark:border-gray-700">
                <div class="flex items-center justify-between mb-3">
                  <h3 class="text-md font-medium text-gray-700 dark:text-gray-300 flex items-center">
                    <span class="text-xl mr-2">
                      <img :src="AssetLibraries" alt="Libraries" class="w-10" />
                    </span> Bibliothèques <span class="text-sm text-gray-500 ml-2">({{ librarySwfArray.length}} Known SWFs) </span>
                  </h3>
                  <div class="flex gap-2">
                    <BaseButton v-if="!isDownloading" :loading="loadingDatabase" :icon="CloudArrowDownIcon"
                      :disabled="!isFormValid || loadingDatabase" @click="downloadAssets('libraries')" variant="primary"
                      size="sm">
                      Download
                    </BaseButton>
                    <BaseButton v-else @click="cancelDownload" :icon="XMarkIcon" variant="danger" size="sm">
                      Cancel
                    </BaseButton>
                    <BaseButton v-if="isDownloading && isPaused" @click="resumeDownload" :icon="PlayIcon"
                      variant="primary" size="sm">
                      Resume
                    </BaseButton>
                    <BaseButton v-else-if="isDownloading" @click="pauseDownload" :icon="PauseIcon" variant="primary"
                      size="sm">
                      Pause
                    </BaseButton>
                  </div>
                </div>
                <div class="space-y-4">
                  <div class="grid grid-cols-12 gap-4">
                    <div class="col-span-10">
                      <label class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">Download Server
                        (source)</label>
                      <ShacoInput :disabled="isDownloading" v-model="libraryDownloadServer"
                        placeholder="Enter server URL" variant="sm" icon="server" />
                      <p class="mt-1 text-xs text-gray-500">Source server for downloads</p>
                    </div>
                    <div class="col-span-2">
                      <label class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">Interval
                        (ms)</label>
                      <ShacoInput :disabled="isDownloading" type="number" v-model="interval"
                        placeholder="Enter interval" variant="sm" icon="clock" />
                      <p class="mt-1 text-xs text-gray-500">Interval for downloads</p>
                    </div>
                  </div>
                  <div>
                    <label class="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">Download Path</label>
                    <PathSelector :disabled="isDownloading" :lengthToTruncate="10" v-model="downloadPath"
                      placeholder="Select download directory..." size="sm" />
                    <p class="mt-1 text-xs text-gray-500">Local destination for assets</p>
                  </div>
                </div>
                <p class="text-xs text-gray-600 dark:text-gray-400 mb-2">
                  {{libraryFiles.filter(f => f.selected).length}} of {{ libraryFiles.length }} files selected
                </p>
                <!-- <div class="max-h-36 overflow-y-auto scrollbar pr-1 grid grid-cols-6 gap-4">
                  <div v-for="(item, index) in libraryFiles" :key="index"
                    class="flex items-center bg-gray-50 dark:bg-[#0f0f0f69] rounded-lg p-1.5 mb-1.5 cursor-pointer hover:bg-gray-100 dark:hover:bg-[#1f1f1f]"
                    :class="{ 'border border-[#396cd8] dark:border-[#e0983a]': item.selected }"
                    @click="toggleSelection(item)">
                    <input type="checkbox" :checked="item.selected" class="mr-2" />
                    <span class="text-xs truncate">{{ item.name }}</span>
                  </div>
                </div> -->
                <!-- Download Progress -->
                <div v-if="isDownloading">
                  <div class="flex items-center justify-between mb-1 mt-2">
                    <span class="text-sm text-gray-500">{{ Math.round(downloadProgress) }}%</span>
                  </div>
                  <ProgressBar :progress="downloadProgress" color="bg-[#ffbd2e] dark:bg-[#ffbd2e]" height="h-1" />
                  <div class="mt-3 flex justify-between text-sm">
                    <span v-if="downloadError != ''" class="text-red-500 dark:text-red-500">
                      {{ downloadError }}
                    </span>
                    <span v-else class="text-gray-700 dark:text-gray-300">
                      Downloading : {{ currentFile }}
                    </span>
                    <span class="font-medium text-gray-600 dark:text-gray-400">
                      {{ downloadedCount }}/{{ totalFiles }}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </transition>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue';
import PathSelector from '../components/PathSelector.vue';
import BaseButton from '../components/BaseButton.vue';
import ProgressBar from '../components/ProgressBar.vue';
import ShacoInput from '../components/ShacoInput.vue';
import { invoke } from '@tauri-apps/api/core';
import AssetImages from '@/assets/assets_images.png'
import AssetMusic from '@/assets/assets_music.png'
import AssetLibraries from '@/assets/assets_furs.png'

import AssetsLogo from '@/assets/assets.png'
import { CloudArrowDownIcon, PauseIcon, PlayIcon, XMarkIcon } from '@heroicons/vue/24/outline';

// Configuration
const downloadPath = ref('');
const databaseUrl = ref('https://derpolino.alwaysdata.net/imagetfm/getFiles.php?n=images&mode=tfm');
const imageDownloadServer = ref('http://www.transformice.com');

const musicDownloadServer = ref('https://audio.atelier801.com/transformice/');
const libraryDownloadServer = ref(' http://www.transformice.com/images/x_bibliotheques/');

// Download status
const isDownloading = ref(false);
const isPaused = ref(false);
const downloadProgress = ref(0);
const currentAssetType = ref('images');
const currentFile = ref('');
const downloadedCount = ref(0);
const totalFiles = ref(0);
const underlineLeft = ref(0);
const interval = ref(100);

const tabs = ref([
  { id: 'images', label: 'Images' },
  { id: 'music', label: 'Musique' },
  { id: 'libraries', label: 'Bibliothèques' },
]);

// Sample data for music files
const musicFiles = ref([
  { name: 'Intro', path: 'musique/intro.mp3', selected: false },
  { name: 'M1', path: 'musique/m1.mp3', selected: false },
  { name: 'M2', path: 'musique/m2.mp3', selected: false },
  { name: 'M3', path: 'musique/m3.mp3', selected: false },
  { name: 'M4', path: 'musique/m4.mp3', selected: false },
  { name: 'Magasin', path: 'musique/magasin.mp3', selected: false },
  { name: 'TFM1', path: 'musique/tfm1.mp3', selected: false },
  { name: 'TFM2', path: 'musique/tfm2.mp3', selected: false },
  { name: 'TFM3', path: 'musique/tfm3.mp3', selected: false },
]);

const soundFiles = ref([
  { name: 'Ballon', path: 'son/ballon.mp3', selected: false },
  { name: 'Boucle invoc', path: 'son/boucle-invoc.mp3', selected: false },
  { name: 'Bouton1', path: 'son/bouton1.mp3', selected: false },
  { name: 'Bouton2', path: 'son/bouton2.mp3', selected: false },
  { name: 'Bulle0', path: 'son/bulle0.mp3', selected: false },
  { name: 'Bulle1', path: 'son/bulle1.mp3', selected: false },
  { name: 'Bulle2', path: 'son/bulle2.mp3', selected: false },
  { name: 'Bulle3', path: 'son/bulle3.mp3', selected: false },
  { name: 'Chamane', path: 'son/chamane.mp3', selected: false },
  { name: 'Chamane2', path: 'son/chamane2.mp3', selected: false },
  { name: 'Clou', path: 'son/clou.mp3', selected: false },
  { name: 'Dash', path: 'son/dash.mp3', selected: false },
  { name: 'Depart', path: 'son/depart.mp3', selected: false },
  { name: 'Ecriture', path: 'son/ecriture.mp3', selected: false },
  { name: 'Emote', path: 'son/emote.mp3', selected: false },
  { name: 'Fleche', path: 'son/fleche.mp3', selected: false },
  { name: 'Fromage', path: 'son/fromage.mp3', selected: false },
  { name: 'Gel', path: 'son/gel.mp3', selected: false },
  { name: 'Grosse', path: 'son/grosse.mp3', selected: false },
  { name: 'Invoc', path: 'son/invoc.mp3', selected: false },
  { name: 'Invoc2', path: 'son/invoc2.mp3', selected: false },
  { name: 'Message', path: 'son/message.mp3', selected: false },
  { name: 'Mort', path: 'son/mort.mp3', selected: false },
  { name: 'Np', path: 'son/np.mp3', selected: false },
  { name: 'Patte', path: 'son/patte.mp3', selected: false },
  { name: 'Petard', path: 'son/petard.mp3', selected: false },
  { name: 'Piano', path: 'son/piano.mp3', selected: false },
  { name: 'Rebours', path: 'son/rebours.mp3', selected: false },
  { name: 'Resu', path: 'son/resu.mp3', selected: false },
  { name: 'Rire', path: 'son/rire.mp3', selected: false },
  { name: 'Saut', path: 'son/saut.mp3', selected: false },
  { name: 'Smiley', path: 'son/smiley.mp3', selected: false },
  { name: 'Tp', path: 'son/tp.mp3', selected: false },
  { name: 'Victoire', path: 'son/victoire.mp3', selected: false },
  { name: 'Victoire2', path: 'son/victoire2.mp3', selected: false },
  { name: 'Victoire first', path: 'son/victoire-first.mp3', selected: false }
]);

// Sample data for library files
const librarySwfArray = ['chamanes/o10,47.swf', 'chamanes/o17,59.swf', 'chamanes/o6,62.swf', 'costume1.swf', 'costume2.swf', 'fourrures/f221.swf', 'fourrures/f222.swf', 'fourrures/f223.swf', 'fourrures/f225.swf', 'fourrures/f226.swf', 'fourrures/f227.swf', 'fourrures/f228.swf', 'fourrures/f229.swf', 'fourrures/f233.swf', 'fourrures/f234.swf', 'fourrures/f235.swf', 'fourrures/f236.swf', 'fourrures/f237.swf', 'fourrures/f238.swf', 'fourrures/f239.swf', 'fourrures/f240.swf', 'fourrures/f241.swf', 'fourrures/f243.swf', 'fourrures/f244.swf', 'fourrures/f245.swf', 'fourrures/f246.swf', 'fourrures/f247.swf', 'fourrures/f248.swf', 'fourrures/f249.swf', 'fourrures/f250.swf', 'fourrures/f251.swf', 'fourrures/f252.swf', 'fourrures/f253.swf', 'fourrures/f254.swf', 'fourrures/f255.swf', 'fourrures/f256.swf', 'fourrures/f258.swf', 'fourrures/f259.swf', 'fourrures/f260.swf', 'fourrures/f261.swf', 'fourrures/f262.swf', 'fourrures/f264.swf', 'fourrures/f265.swf', 'fourrures/f266.swf', 'fourrures/f267.swf', 'fourrures/f268.swf', 'fourrures/f269.swf', 'fourrures/f270.swf', 'fourrures/f271.swf', 'fourrures/f272.swf', 'fourrures/f273.swf', 'fourrures/f274.swf', 'fourrures/f275.swf', 'fourrures/f276.swf', 'fourrures/f277.swf', 'fourrures/f278.swf', 'fourrures/f279.swf', 'fourrures/f280.swf', 'fourrures/f281.swf', 'fourrures/f282.swf', 'fourrures/f283.swf', 'fourrures/f285.swf', 'fourrures/f286.swf', 'fourrures/f289.swf', 'fourrures/f290.swf', 'fourrures/f291.swf', 'fourrures/f292.swf', 'fourrures/f293.swf', 'fourrures/f295.swf', 'fourrures/f296.swf', 'fourrures/f297.swf', 'fourrures/f298.swf', 'fourrures/f299.swf', 'fourrures/f300.swf', 'fourrures/f301.swf', 'fourrures/f302.swf', 'fourrures/f303.swf', 'fourrures/f304.swf', 'fourrures/f305.swf', 'fourrures/f307.swf', 'fourrures/f308.swf', 'fourrures/f309.swf', 'fourrures/f310.swf', 'fourrures/f311.swf', 'fourrures/f312.swf', 'fourrures/f314.swf', 'fourrures/f315.swf', 'fourrures/f316.swf', 'fourrures/f317.swf', 'fourrures/f318.swf', 'fourrures/f319.swf', 'fourrures/f320.swf', 'fourrures/f321.swf', 'fourrures/f322.swf', 'fourrures/f323.swf', 'fourrures/f324.swf', 'fourrures/f325.swf', 'fourrures/f326.swf', 'fourrures/f328.swf', 'fourrures/f329.swf', 'fourrures/f330.swf', 'fourrures/f331.swf', 'fourrures/f332.swf', 'fourrures/f333.swf', 'fourrures/f334.swf', 'fourrures/f335.swf', 'fourrures/f336.swf', 'fourrures/f338.swf', 'fourrures/f339.swf', 'fourrures/f340.swf', 'fourrures/f341.swf', 'fourrures/f342.swf', 'x_fourrures.swf', 'x_fourrures2.swf', 'x_fourrures3.swf', 'x_fourrures4.swf', 'x_fourrures5.swf', 'x_items_chaman.swf', 'x_macarons.swf', 'x_meli_costumes.swf', 'x_pictos_editeur.swf']
const libraryFiles = ref(librarySwfArray.map(file => ({
  path: `/${file}`,
  selected: false
})));


const isFormValid = computed(() => {
  return downloadPath.value.trim() !== '' &&
    databaseUrl.value.trim() !== '' &&
    imageDownloadServer.value.trim() !== '';
});

// // Toggle selection for files
// const toggleSelection = (item: { selected: boolean }) => {
//   item.selected = !item.selected;
// };

// // Get selected files for a category
// const getSelectedFiles = (type: 'music' | 'libraries') => {
//   if (type === 'music') {
//     return musicFiles.value.filter(file => file.selected);
//   } else {
//     return libraryFiles.value.filter(file => file.selected);
//   }
// };

const setAssetType = (type: string) => {
  currentAssetType.value = type;
  nextTick(() => {
    updateUnderlinePosition();
  });
};

const tabRefs = ref<HTMLElement[]>([]);

const updateUnderlinePosition = () => {
  const activeIndex = tabs.value.findIndex(tab => tab.id === currentAssetType.value);
  const activeTabRef = tabRefs.value[activeIndex];
  if (activeTabRef) {
    const tabLeft = activeTabRef.offsetLeft;
    const tabWidth = activeTabRef.offsetWidth;
    const underlineWidth = 40; // w-10 (40px)
    underlineLeft.value = tabLeft + (tabWidth / 2) - (underlineWidth / 2);
  }
};

nextTick(() => {
  updateUnderlinePosition();
});

const dbError = ref('');
const loadingDatabase = ref(false);

const loadImageDatabase = async () => {
  try {
    loadingDatabase.value = true;
    const data = await invoke<string>("fetch_url_string", { url: databaseUrl.value });
    const json = JSON.parse(data);
    return json;
  } catch (error) {
    dbError.value = "Failed to parse JSON: " + String(error);
  } finally {
    loadingDatabase.value = false;
  }
  return undefined;
}

const downloadAssets = async (type: 'music' | 'libraries') => {
  if (!isFormValid.value) return;

  if (type === 'music') {
    totalFiles.value = soundFiles.value.length + musicFiles.value.length;

    const allFiles = [...soundFiles.value, ...musicFiles.value].map(file => file.path);

    filesDownloader(allFiles, downloadPath.value, musicDownloadServer.value, interval.value);

  } else {
    totalFiles.value = libraryFiles.value.length;
    const libraryFilesArray = libraryFiles.value.map(file => file.path);
    filesDownloader(libraryFilesArray, downloadPath.value, libraryDownloadServer.value, interval.value);
  }
};

const downloadError = ref('');

const downloadImages = async () => {
  if (!isFormValid.value) return;

  const imagesFiles = await loadImageDatabase();
  if (!imagesFiles) return;


  const imagesList = Object.values(imagesFiles) as string[];

  totalFiles.value = imagesList.length;

  filesDownloader(imagesList, downloadPath.value, imageDownloadServer.value, interval.value);
}

const filesDownloader = async (files: string[], downloadPath: string, host: string, interval: number) => {
  isDownloading.value = true;
  downloadProgress.value = 0;
  currentFile.value = '';
  downloadedCount.value = 0;

  for (const file of files) {
    const fileUrl = `${host}/${file}?t=${new Date().getTime()}`;
    const filePath = `${downloadPath}\\${file.replace(/\//gi, '\\')}`;
    try {
      await invoke("download_file", { url: fileUrl, path: filePath });
      currentFile.value = file;
      downloadedCount.value++;
      downloadProgress.value = ((downloadedCount.value / totalFiles.value) * 100);
      console.log(isPaused.value, isDownloading.value);
      while (isPaused.value && isDownloading.value) {
        await wait(1000);
      }
      if (!isDownloading.value) break;
      await wait(interval);
    } catch (error) {
      downloadError.value = "Failed to download: " + String(error);
      break;
    }
  }

  setTimeout(() => {
    isDownloading.value = false;
    downloadError.value = '';
    downloadProgress.value = 0;
    currentFile.value = '';
    downloadedCount.value = 0;
    totalFiles.value = 0;
  }, 500);
}

const cancelDownload = () => {
  isDownloading.value = false;
  downloadProgress.value = 0;
  currentFile.value = '';
  downloadedCount.value = 0;
}

const pauseDownload = () => {
  isPaused.value = true;
}

const resumeDownload = () => {
  isPaused.value = false;
}


const wait = (ms: number) => {
  return new Promise(resolve => setTimeout(resolve, ms));
}

</script>

<style scoped>
.scrollbar {
  scrollbar-width: thin;
  scrollbar-color: rgba(156, 163, 175, 0.5) transparent;
}

.scrollbar::-webkit-scrollbar {
  width: 6px;
}

.scrollbar::-webkit-scrollbar-track {
  background: transparent;
}

.scrollbar::-webkit-scrollbar-thumb {
  background-color: rgba(156, 163, 175, 0.5);
  border-radius: 3px;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>