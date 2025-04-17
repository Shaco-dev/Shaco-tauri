<script setup lang="ts">
import {
  DocumentArrowDownIcon,
  DocumentArrowUpIcon,
  CloudArrowDownIcon,
  FolderIcon
} from '@heroicons/vue/24/outline';
import BaseButton from '@/components/BaseButton.vue'
import Dropdown from '@/components/Dropdown.vue';

defineProps<{
  canExport: boolean;
}>();

const emit = defineEmits<{
  (e: 'import', format: 'gz' | 'txt' | 'json'): void;
  (e: 'download'): void;
  (e: 'export'): void;
}>();

const handleImport = (format: 'gz' | 'txt' | 'json') => {
  emit('import', format);
};
</script>

<template>
  <div class="flex items-center space-x-3">
    <!-- Import Dropdown -->
    <Dropdown>
      <template #trigger="{ toggle }">
        <BaseButton variant="secondary" :icon="DocumentArrowUpIcon" @click="toggle">
          Import
        </BaseButton>
      </template>
      
      <template #content>
        <div class="py-1 gap-2">
          <button 
            @click="handleImport('gz')"
            class="flex w-full items-center px-4 py-2.5 text-sm hover:bg-gray-100 dark:hover:bg-[#0f0f0f69] text-gray-700 dark:text-gray-200"
          >
            <FolderIcon class="h-4 w-4 mr-2" />
            Import Archive (.gz)
          </button>
          
          <button 
            @click="handleImport('txt')"
            class="flex w-full items-center px-4 py-2.5 text-sm hover:bg-gray-100 dark:hover:bg-[#0f0f0f69] text-gray-700 dark:text-gray-200"
          >
            <FolderIcon class="h-4 w-4 mr-2" />
            Import Text (.txt)
          </button>
          
          <button 
            :disabled="true" 
            class="flex w-full items-center px-4 py-2.5 text-sm hover:bg-gray-100 dark:hover:bg-[#0f0f0f69] text-gray-700 dark:text-gray-200"
          >
            <FolderIcon class="h-4 w-4 mr-2" />
            Import Json (.json)
            <span class="ml-2 px-2 py-1 text-xs bg-yellow-100 dark:bg-yellow-800 rounded-full">Soon!</span>
          </button>
        </div>
      </template>
    </Dropdown>

    <!-- Download Button -->
    <BaseButton variant="secondary" :icon="CloudArrowDownIcon" @click="$emit('download')">
      Download
    </BaseButton>

    <!-- Export Button -->
    <BaseButton 
      variant="secondary" 
      :icon="DocumentArrowDownIcon"
      @click="$emit('export')"
      :disabled="!canExport"
    >
      Export
    </BaseButton>
  </div>
</template> 