<script setup lang="ts">
import { CheckIcon } from '@heroicons/vue/24/outline'

defineProps<{
  code: string;
  flagCode: string;
  language: string;
  flagSrc: string;
  isSelected: boolean;
  isCompact?: boolean;
  isDisabled?: boolean;
}>();

defineEmits<{
  (e: 'select'): void;
}>();
</script>

<template>
  <button 
    @click="$emit('select')" 
    :disabled="isDisabled"
    :title="language" 
    :class="[
      'group relative transition-all',
      isCompact ? 'p-2 rounded-md' : 'p-3 rounded-lg',
      'hover:bg-blue-50/50 dark:hover:bg-blue-900/30',
      isSelected
        ? 'bg-blue-100 dark:bg-blue-700/30 ring-2 ring-[#396cd8] border-1 border-[#396cd8]'
        : 'bg-gray-50 dark:bg-[#1a1a1a]',
      isDisabled ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'
    ]"
  >
    <div class="select-none flex flex-col items-center space-y-0">
      <img 
        :src="flagSrc" 
        :class="[
          'rounded-sm object-cover shadow-sm',
          isCompact ? 'h-6 w-6' : 'h-8 w-8'
        ]" 
        :alt="code" 
      />
      <span
        class="text-center font-mono text-gray-600 dark:text-gray-300 transition-opacity group-hover:opacity-0"
        :class="isCompact ? 'text-xs' : 'text-sm mt-1'"
      >
        {{ code.toUpperCase() }}
      </span>
      <span
        class="absolute bottom-1 font-medium text-[#396cd8] opacity-0 transition-opacity group-hover:opacity-100"
        :class="isCompact ? 'text-xs' : 'text-sm'"
      >
        {{ code.toUpperCase() }}
      </span>
    </div>

    <div 
      :class="[
        'absolute top-1 right-1 flex items-center justify-center rounded-full border',
        'transition-colors text-white',
        isCompact ? 'w-4 h-4 text-[10px]' : 'w-5 h-5 text-xs',
        isSelected
          ? 'bg-[#396cd8] border-[#396cd8]'
          : 'bg-white dark:bg-gray-800 border-gray-300 group-hover:border-[#396cd8]'
      ]"
    >
      <CheckIcon v-if="isSelected" :class="isCompact ? 'w-3 h-3' : 'w-4 h-4'" />
    </div>
  </button>
</template> 