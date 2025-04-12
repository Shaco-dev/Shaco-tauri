<!-- components/Modal.vue -->
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
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-2"
      @click.self="emit('close')"
    >
      <div class="bg-white dark:bg-[#2f2f2f] rounded-xl w-full max-w-2xl p-6 mt-24">
        <div class="flex justify-between items-center mb-4">
          <h3 class="text-lg font-semibold">{{ title }}</h3>
        </div>
        <slot />
      </div>
    </div>
  </Transition>
</template>

<style>
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>