<script setup lang="ts">
import { ref } from 'vue';
import LanguagesList from './LanguagesList.vue';
import Modal from './Modal.vue';
import BaseButton from './BaseButton.vue';
import ProgressBar from './ProgressBar.vue';
import { ArrowDownTrayIcon } from '@heroicons/vue/24/outline';

interface Language {
  code: string;
  flagCode: string;
  community: string;
  language: string;
  loaded: boolean;
}

const props = defineProps<{
  show: boolean;
  languages: Language[];
  selectedLanguages: string[];
  flagMap: Record<string, string>;
  isDownloading: boolean;
  downloadProgress: number;
  downloadedFiles: number;
  downloadingFilename: string;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'cancel'): void;
  (e: 'download'): void;
  (e: 'update:selectedLanguages', languages: string[]): void;
  (e: 'update:loadAfterDownload', value: boolean): void;
}>();

const loadAfterDownload = ref(true);

const toggleSelectAll = () => {
  emit('update:selectedLanguages', props.selectedLanguages.length === props.languages.length ? [] : props.languages.map(lang => lang.code));
};

const handleSelect = (code: string) => {
  const newSelected = [...props.selectedLanguages];
  const index = newSelected.indexOf(code);
  
  if (index === -1) {
    newSelected.push(code);
  } else {
    newSelected.splice(index, 1);
  }
  
  emit('update:selectedLanguages', newSelected);
};

const updateLoadAfterDownload = () => {
  emit('update:loadAfterDownload', loadAfterDownload.value);
};
</script>

<template>
  <Modal :show="show" title="Download Languages" @close="$emit('close')" size="xl">
    <div class="space-y-4 relative">
      <div v-if="isDownloading"
        class="absolute h-[255px] rounded-lg border border-transparent bottom-20 rouned-md inset-0 bg-black/50 z-10 flex items-center justify-center">
      </div>

      <LanguagesList
        :languages="languages"
        :selectedLanguages="selectedLanguages"
        :flagMap="flagMap"
        :isCompact="true"
        :isDisabled="isDownloading"
        :showSelectAll="true"
        @select="handleSelect"
        @selectAll="toggleSelectAll"
      />
      
      <div class="select-none flex items-center space-x-2">
        <input 
          type="checkbox" 
          v-model="loadAfterDownload" 
          id="loadAfterDownload"
          :disabled="isDownloading"
          @change="updateLoadAfterDownload"
          class="h-4 w-4 text-[#ffbd2e] border-gray-300 rounded focus:ring-[#ffbd2e] dark:border-[#0f0f0f98] dark:bg-[#0f0f0f98]"
        >
        <label for="loadAfterDownload" class="text-sm text-[#0f0f0f98] dark:text-gray-300">
          Load after download
        </label>
      </div>
      
      <div class="border-t pt-3 border-gray-200 dark:border-gray-700">
        <div class="flex items-center justify-between text-sm">
          <div v-if="!isDownloading" class="text-gray-600 dark:text-gray-300">
            {{ selectedLanguages.length }} selected
          </div>
          
          <div v-else class="w-full">
            <p class="text-sm mb-1 font-medium text-gray-700 dark:text-gray-200">
              Downloading {{ selectedLanguages.length }} files...
            </p>
            
            <ProgressBar 
              :progress="downloadProgress" 
              color="amber" 
              :animated="true"
            />
            
            <p class="mt-2 text-xs text-gray-500 dark:text-gray-400">
              <span class="font-mono">{{ downloadingFilename }}</span> - 
              {{ downloadedFiles }} of {{ selectedLanguages.length }} files processed
            </p>
          </div>
          
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
  </Modal>
</template> 