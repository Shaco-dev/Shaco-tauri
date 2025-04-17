<script setup lang="ts">
defineProps({
  title: String,
  show: Boolean
})

const emit = defineEmits(['close'])
</script>

<template>
  <Transition name="modal">
    <div 
      v-if="show"
      class="fixed inset-0 z-400 bg-black/50 backdrop-blur-sm flex items-center justify-center px-4 sm:px-6"
      @click.self="emit('close')"
    >
      <div class="bg-white dark:bg-[#2f2f2f] text-[#1f1f1f] dark:text-[#f6f6f6] rounded-2xl w-full max-w-2xl p-6 shadow-lg border border-[#e5e7eb] dark:border-[#3a3a3a]">
        <div class="flex justify-between items-center mb-4">
          <h3 class="text-lg font-semibold">{{ title }}</h3>
          <button @click="emit('close')" class="text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 transition">
            ✕
          </button>
        </div>
        <slot />
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}
.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>
