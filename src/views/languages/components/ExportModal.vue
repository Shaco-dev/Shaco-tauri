<script setup lang="ts">
import { ref } from 'vue';
import BaseButton from '../../../components/BaseButton.vue';
import PathSelector from '../../../components/PathSelector.vue';
import FileFormatSelector from '../../../components/FileFormatSelector.vue';
import { DocumentArrowDownIcon } from '@heroicons/vue/24/outline';
import { Square2StackIcon, DocumentTextIcon } from '@heroicons/vue/24/outline';

defineProps<{
  exportPath: string;
  exportFormat: string;
  isExporting: boolean;
  loadedLanguagesCount: number;
}>();

const emit = defineEmits<{
  (e: 'update:exportPath', value: string): void;
  (e: 'update:exportFormat', value: string): void;
  (e: 'export'): void;
  (e: 'close'): void;
}>();

// Export formats
const exportFormats = [
  {
    value: 'gz',
    label: 'Compressed zlib Archive (.gz)',
    description: 'Transformice language archive',
    icon: Square2StackIcon
  },
  {
    value: 'txt',
    label: 'Text File (.txt)',
    description: 'Plain text file',
    icon: DocumentTextIcon
  }
];

const openAfterExport = ref(true);

const updateFormat = (value: string) => {
  emit('update:exportFormat', value);
};

const updatePath = (value: string) => {
  emit('update:exportPath', value);
};
</script>

<template>
  <div class="space-y-4">
    <!-- Export Location -->
    <div>
      <label class="block text-sm font-medium mb-1.5 dark:text-gray-200">Export Location</label>
      <PathSelector 
        :modelValue="exportPath" 
        @update:modelValue="updatePath" 
        placeholder="Select export directory..." 
        :disabled="isExporting"
      />
    </div>

    <!-- Export Format -->
    <div>
      <label class="block text-sm font-medium mb-2 dark:text-gray-200">Export Format</label>
      <FileFormatSelector
        :modelValue="exportFormat"
        @update:modelValue="updateFormat"
        :formats="exportFormats"
        :disabled="isExporting"
      />
    </div>

    <!-- Open After Export -->
    <div class="select-none flex items-center space-x-2">
      <input 
        type="checkbox" 
        v-model="openAfterExport" 
        id="openAfterExport"
        :disabled="isExporting"
        class="s-input h-4 w-4 text-[#ffbd2e] border-gray-300 rounded focus:ring-[#ffbd2e] dark:border-[#0f0f0f98] dark:bg-[#0f0f0f98]"
      >
      <label for="openAfterExport" class="text-sm text-[#0f0f0f98] dark:text-gray-300">
        Open folder after export
      </label>
    </div>

    <!-- Loading State -->
    <div v-if="isExporting" class="flex items-center space-x-2 text-sm text-[#ffbd2e] pt-2">
      <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
      <span>Exporting languages...</span>
    </div>

    <!-- Actions -->
    <div class="flex justify-between gap-3 border-t pt-4 border-gray-200 dark:border-gray-700">
      <div>
        <span class="text-xs text-gray-500 dark:text-gray-400">
          {{ loadedLanguagesCount }} languages will be exported
        </span>
      </div>
      
      <div class="flex justify-end gap-2">
        <BaseButton 
          variant="secondary" 
          size="sm" 
          @click="$emit('close')"
          :disabled="isExporting"
        >
          Cancel
        </BaseButton>
        
        <BaseButton 
          variant="primary" 
          size="sm" 
          @click="$emit('export')"
          :disabled="isExporting || !exportPath || loadedLanguagesCount === 0" 
          :loading="isExporting"
        >
          <DocumentArrowDownIcon class="h-5 w-5 mr-2" />
          {{ isExporting ? 'Exporting...' : 'Export' }}
        </BaseButton>
      </div>
    </div>
  </div>
</template> 