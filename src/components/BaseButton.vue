<!-- components/BaseButton.vue -->
<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps({
  variant: {
    type: String,
    default: 'secondary',
    validator: (value: string) => ['primary', 'secondary', 'ghost'].includes(value)
  },
  size: {
    type: String,
    default: 'md',
    validator: (value: string) => ['sm', 'md', 'lg'].includes(value)
  },
  icon: {
    type: Object,  // Changed from String to Object
    default: null
  },
  disabled: Boolean
})

const buttonClasses = computed(() => {
  return [
    'inline-flex items-center rounded-lg font-medium transition-all border',
    props.variant === 'primary' 
      ? 'bg-[#396cd8] text-white hover:bg-[#2e5ac7] border-transparent' 
      : 'bg-white dark:bg-[#0f0f0f98] border-gray-200 dark:border-gray-700 hover:border-[#396cd8] dark:hover:border-[#24c8db]',
    props.size === 'sm' ? 'px-2.5 py-1.5 text-xs' : 'px-4 py-2 text-sm',
    props.disabled ? 'opacity-50 cursor-not-allowed' : ''
  ]
})
</script>

<template>
  <button :class="buttonClasses" :disabled="disabled">
    <component 
      :is="icon" 
      v-if="icon" 
      class="h-4 w-4 mr-1.5" 
    />
    <slot />
  </button>
</template>