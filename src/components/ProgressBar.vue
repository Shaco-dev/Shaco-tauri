<script setup lang="ts">
import { computed } from 'vue';

const { 
  progress, 
  color = 'bg-[#f0b22d]', 
  height = 'h-2.5', 
  showText = false, 
  animate = true 
} = defineProps<{
  progress: number;
  color?: string;
  height?: string;
  showText?: boolean;
  animate?: boolean;
}>();

const formattedProgress = computed(() => {
  return Math.round(progress);
});
</script>

<template>
  <div class="w-full space-y-1">
    <div 
      :class="[
        'relative w-full bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden',
        height
      ]"
    >
      <div 
        :class="[
          'absolute left-0 top-0 h-full transition-all duration-300 ease-out',
          color,
          animate ? 'transition-all duration-300 ease-out' : ''
        ]" 
        :style="{ width: `${progress}%` }"
      ></div>
    </div>

    <div v-if="showText" class="text-right text-xs text-gray-500 dark:text-gray-400">
      {{ formattedProgress }}%
    </div>
  </div>
</template> 