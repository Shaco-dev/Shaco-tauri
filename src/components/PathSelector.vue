<script setup lang="ts">
import { FolderIcon } from '@heroicons/vue/24/outline';
import BaseButton from './BaseButton.vue';
import { open } from '@tauri-apps/plugin-dialog';

const props = defineProps<{
  modelValue: string;
  placeholder?: string;
  disabled?: boolean;
  buttonLabel?: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();

function truncatePath(path: string): string {
  if (!path) return props.placeholder || 'Select directory...';
  
  const segments = path.split(/[\/\\]/);
  if (segments.length <= 3) {
    return path;
  }
  
  return segments[0] + '/...' + '/' + segments[segments.length - 2] + '/' + segments[segments.length - 1];
}

async function handleSelectPath() {
  try {
    const selected = await open({
      directory: true,
      title: 'Select Directory',
      defaultPath: props.modelValue || undefined
    });

    if (selected) {
      emit('update:modelValue', selected as string);
    }
  } catch (error) {
    console.error('Error selecting directory:', error);
  }
}
</script>

<template>
  <div>
    <div class="flex items-center gap-2">
      <div class="flex-1 relative">
        <input 
          :value="truncatePath(modelValue)" 
          type="text" 
          readonly 
          :disabled="disabled"
          class="s-input select-none w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#0f0f0f69] truncate placeholder-gray-400"
          :class="{ 
            'text-gray-600 dark:text-gray-300': modelValue, 
            'text-gray-400 dark:text-gray-500': !modelValue,
            'opacity-50 cursor-not-allowed': disabled
          }" 
        />
      </div>
      <BaseButton 
        variant="secondary" 
        class="!px-3 !py-2 h-10" 
        @click="handleSelectPath"
        :disabled="disabled"
      >
        <FolderIcon class="w-5 h-5" />
        <span v-if="buttonLabel" class="ml-1">{{ buttonLabel }}</span>
      </BaseButton>
    </div>
  </div>
</template> 