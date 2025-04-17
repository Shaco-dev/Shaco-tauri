<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  modelValue: string;
  formats: Array<{
    value: string;
    label: string;
    description?: string;
    icon?: any;
  }>;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();

const selectedFormat = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
});
</script>

<template>
  <div class="space-y-2">
    <div class="flex">
      <div 
        v-for="format in formats" 
        :key="format.value" 
        class="w-full p-1"
      >
        <label 
          @click="!disabled && (selectedFormat = format.value)"
          class="select-none flex items-start space-x-3 p-3 rounded-lg cursor-pointer transition-colors"
          :class="{
            'bg-blue-50 dark:bg-blue-900/30 border border-blue-200 dark:border-blue-800': selectedFormat === format.value,
            'bg-gray-50 dark:bg-[#0f0f0f98] hover:bg-gray-100/50 border border-transparent dark:hover:bg-gray-800': selectedFormat !== format.value,
            'opacity-50 cursor-not-allowed': disabled
          }"
        >
          <div class="flex-1">
            <div class="flex items-center gap-2">
              <component 
                :is="format.icon" 
                v-if="format.icon" 
                class="w-5 h-5 text-[#ffbd2e]" 
              />
              <span class="text-sm font-medium dark:text-gray-200">{{ format.label }}</span>
            </div>
            <p 
              v-if="format.description" 
              class="mt-1 text-xs text-gray-500 dark:text-gray-400"
              :class="{'pl-7': format.icon}"
            >
              {{ format.description }}
            </p>
          </div>
        </label>
      </div>
    </div>
  </div>
</template> 