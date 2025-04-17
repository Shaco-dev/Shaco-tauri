<script setup lang="ts">
import { computed, ref } from 'vue';
import SearchInput from '../../../components/SearchInput.vue';
import BaseButton from '../../../components/BaseButton.vue';
import ProgressBar from '../../../components/ProgressBar.vue';
import { ArrowDownTrayIcon, CheckIcon, DocumentMagnifyingGlassIcon } from '@heroicons/vue/24/outline';

const props = defineProps<{
  selectedLanguages: string[];
  languages: any[];
  flagMap: Record<string, string>;
  isDownloading: boolean;
  downloadProgress: number;
  downloadedFiles: number;
  downloadingFilename: string;
  searchQuery: string;
}>();

const emit = defineEmits<{
  (e: 'update:searchQuery', value: string): void;
  (e: 'update:selectedLanguages', value: string[]): void;
  (e: 'download'): void;
  (e: 'cancel'): void;
  (e: 'toggleSelectAll'): void;
  (e: 'toggleLanguage', code: string): void;
}>();

const updateSearch = (value: string) => {
  emit('update:searchQuery', value);
};

const filteredLanguages = computed(() => {
  if (!props.searchQuery) return props.languages;
  
  const query = props.searchQuery.toLowerCase();
  return props.languages.filter(lang =>
    lang.language.toLowerCase().includes(query) ||
    lang.code.toLowerCase().includes(query)
  );
});

const loadAfterDownload = ref(true);
</script>

<template>
  <div class="space-y-4 relative">
    <div 
      v-if="isDownloading"
      class="absolute h-[255px] rounded-lg border border-transparent bottom-20 rouned-md inset-0 bg-black/50 z-10 flex items-center justify-center"
    >
    </div>

    <div class="grid grid-cols-3 w-full p-1">
      <SearchInput 
        variant="sm" 
        :modelValue="searchQuery" 
        @update:modelValue="updateSearch" 
        placeholder="Search language"
      />
      
      <div 
        @click="!isDownloading && $emit('toggleSelectAll')" 
        :class="[
          'select-none relative ml-2 pl-2 w-26 rounded-lg border',
          'border-gray-200 dark:border-gray-800 hover:border-[#396bd88a]',
          isDownloading
            ? 'bg-gray-100 dark:bg-gray-800 cursor-not-allowed'
            : 'bg-white dark:bg-[#0f0f0f] hover:cursor-pointer'
        ]" 
        style="font-size: 12px"
      >
        <div 
          :class="[
            'absolute top-1 left-1 flex items-center justify-center w-4 h-4 rounded-full border',
            'transition-colors text-white text-[10px]',
            selectedLanguages.length === languages.length
              ? 'bg-[#396cd8] border-[#396cd8]'
              : 'bg-white dark:bg-gray-800 border-gray-300 group-hover:border-[#396cd8]'
          ]"
        >
          <CheckIcon v-if="selectedLanguages.length === languages.length" class="w-3 h-3" />
        </div>
        <span class="ml-5 text-amber-400/80">Select All</span>
      </div>

      <!-- Open After Export -->
      <div class="select-none flex items-center space-x-2">
        <input 
          type="checkbox" 
          v-model="loadAfterDownload" 
          id="loadAfterDownload"
          :disabled="isDownloading"
          class="s-input h-4 w-4 text-[#ffbd2e] border-gray-300 rounded focus:ring-[#ffbd2e] dark:border-[#0f0f0f98] dark:bg-[#0f0f0f98]"
        >
        <label for="loadAfterDownload" class="text-sm text-[#0f0f0f98] dark:text-gray-300">
          Load after download
        </label>
      </div>
    </div>

    <!-- Language Grid -->
    <div class="h-[200px] overflow-y-auto scrollbar">
      <div 
        :class="[
          'grid grid-cols-8 gap-1.5 pr-2',
          isDownloading ? 'opacity-50 pointer-events-none' : ''
        ]"
      >
        <button 
          v-for="lang in filteredLanguages" 
          :key="lang.code"
          @click="!isDownloading && $emit('toggleLanguage', lang.code)" 
          :disabled="isDownloading"
          :title="lang.language" 
          :class="[
            'group relative p-2 rounded-md transition-all',
            'hover:bg-blue-50/50 dark:hover:bg-blue-900/30',
            selectedLanguages.includes(lang.code)
              ? 'bg-blue-100 dark:bg-blue-700/30 ring-2 ring-[#396cd8] border-1 border-[#396cd8]'
              : 'bg-gray-50 dark:bg-[#1a1a1a]',
            isDownloading ? 'cursor-not-allowed' : ''
          ]"
        >
          <div class="select-none flex flex-col items-center space-y-0">
            <img 
              :src="flagMap[lang.flagCode]" 
              class="h-6 w-6 rounded-sm object-cover shadow-sm"
              :alt="lang.code" 
            />
            <span
              class="text-center text-xs font-mono text-gray-600 dark:text-gray-300 transition-opacity group-hover:opacity-0"
            >
              {{ lang.code.toUpperCase() }}
            </span>
            <span
              class="absolute bottom-1 text-xs font-medium text-[#396cd8] opacity-0 transition-opacity group-hover:opacity-100"
            >
              {{ lang.code.toUpperCase() }}
            </span>
          </div>

          <!-- Selection Check -->
          <div 
            :class="[
              'absolute top-1 right-1 flex items-center justify-center w-4 h-4 rounded-full border',
              'transition-colors text-white text-[10px]',
              selectedLanguages.includes(lang.code)
                ? 'bg-[#396cd8] border-[#396cd8]'
                : 'bg-white dark:bg-gray-800 border-gray-300 group-hover:border-[#396cd8]'
            ]"
          >
            <CheckIcon v-if="selectedLanguages.includes(lang.code)" class="w-3 h-3" />
          </div>
        </button>
      </div>

      <div 
        v-if="filteredLanguages.length === 0"
        class="py-6 text-center text-gray-500 dark:text-gray-400"
      >
        <DocumentMagnifyingGlassIcon class="mx-auto h-6 w-6 mb-2" />
        <p class="text-xs">No matches for "{{ searchQuery }}"</p>
      </div>
    </div>

    <!-- Footer -->
    <div class="border-t pt-3 border-gray-200 dark:border-gray-700">
      <div class="flex items-center justify-between text-sm">
        <div class="text-gray-600 dark:text-gray-300 w-full">
          <span v-if="!isDownloading">
            {{ selectedLanguages.length }} selected
          </span>
          
          <!-- Download Progress -->
          <div v-else-if="isDownloading" class="text-center w-full max-w-xs px-4">
            <p class="text-sm mb-1 font-medium text-gray-700 dark:text-gray-200">
              Downloading {{ selectedLanguages.length }} files...
            </p>
            
            <ProgressBar 
              :progress="downloadProgress" 
              height="h-2.5"
              color="bg-[#f0b22d] dark:bg-[#f0b22d]"
            />

            <!-- Progress Text -->
            <div class="space-y-1.5 mt-2">
              <p class="text-xs text-gray-500 dark:text-gray-400">
                <span class="font-mono">{{ downloadingFilename }}</span> - 
                {{ downloadedFiles }} of {{ selectedLanguages.length }} files processed
              </p>
            </div>
          </div>
        </div>
        
        <!-- Action Buttons -->
        <div class="flex space-x-2">
          <BaseButton 
            variant="secondary" 
            size="sm" 
            @click="$emit('cancel')" 
            :disabled="isDownloading"
          >
            Cancel
          </BaseButton>
          
          <BaseButton 
            variant="primary" 
            size="sm"
            :disabled="selectedLanguages.length === 0 || isDownloading" 
            @click="$emit('download')"
            :loading="isDownloading"
          >
            <ArrowDownTrayIcon class="w-4 h-4 mr-1.5" />
            Download
          </BaseButton>
        </div>
      </div>
    </div>
  </div>
</template>

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
</style> 