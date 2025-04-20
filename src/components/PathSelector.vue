<script setup lang="ts">
import { FolderIcon } from '@heroicons/vue/24/outline';
import BaseButton from './BaseButton.vue';
import { open } from '@tauri-apps/plugin-dialog';

const props = defineProps<{
  class?: string;
  modelValue: string;
  placeholder?: string;
  buttonLabel?: string;
  lengthToTruncate?: number | undefined;
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();

function truncatePath(path: string): string {
  if (!path) return props.placeholder || 'Select directory...';

  const segments = path.split(/[\/\\]/);
  if (segments.length <= (props.lengthToTruncate || 3)) {
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

// Set default size
const size = props.size || 'md';


const getButtonHeight = () => {
  switch (size) {
    case 'sm': return 'h-7 w-10 !py-1';
    case 'lg': return 'h-12 !py-3';
    default: return 'h-10 !py-2';
  }
};
</script>

<template>
  <div>
    <div class="flex items-center gap-2 w-full">
      <div
        :class="['relative w-full transition-colors duration-200 shadow-[0_2px_2px_rgba(0,0,0,0.2)] rounded-lg border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#0f0f0f] ', props.class]">
        <div :class="[
          'flex items-center gap-2 w-full text-gray-300 px-3',
          props.size === 'sm' ? 'h-6 text-sm' :
            props.size === 'lg' ? 'h-12 text-lg' :
              'h-10 text-base',
              disabled ? 'cursor-not-allowed' : 'cursor-pointer'
        ]">
          <FolderIcon class="h-4 w-4 text-gray-600 dark:text-gray-400 flex-shrink-0" />
          <input :value="truncatePath(modelValue)" type="text" readonly @click="handleSelectPath" :disabled="disabled"
            :class="[
              'w-full bg-transparent border-none focus:outline-none h-full',
              'text-gray-700 dark:text-gray-200',
              'placeholder-gray-500 dark:placeholder-gray-400',
              props.size === 'sm' ? 'h-6 text-sm' :
                props.size === 'lg' ? 'h-12 text-lg' :
                  'h-10 text-base',
              disabled ? 'cursor-not-allowed' : 'cursor-pointer'
            ]" />
        </div>
      </div>
      <BaseButton variant="secondary" :class="['!px-3', getButtonHeight()]" @click="handleSelectPath"
        :disabled="disabled">
        <FolderIcon class="w-4 h-4" />
        <span v-if="buttonLabel" class="ml-1">{{ buttonLabel }}</span>
      </BaseButton>
    </div>
  </div>
</template>