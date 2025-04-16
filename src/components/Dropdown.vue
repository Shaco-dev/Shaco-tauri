<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'

const isOpen = ref(false)
const dropdownRef = ref<HTMLElement | null>(null)

function toggle() {
  isOpen.value = !isOpen.value
}

function close() {
  isOpen.value = false
}

function handleClickOutside(event: MouseEvent) {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    close()
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})
onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<template>
  <div ref="dropdownRef" class="relative inline-block text-left">
    <slot name="trigger" :isOpen="isOpen" :toggle="toggle" />

    <transition
      enter-active-class="transition ease-out duration-150"
      enter-from-class="opacity-0 translate-y-1 scale-95"
      enter-to-class="opacity-100 translate-y-0 scale-100"
      leave-active-class="transition ease-in duration-100"
      leave-from-class="opacity-100 translate-y-0 scale-100"
      leave-to-class="opacity-0 translate-y-1 scale-95"
    >
      <div
        v-show="isOpen"
        @click="close"
        class="absolute right-0 mt-2 w-64 origin-top-right rounded-xl dark:border bg-white text-[#1f1f1f] dark:bg-[#1e1e1e] dark:text-white border-gray-200 border-2 dark:border-gray-700 shadow-lg z-50"
      >
      <div class="grid gap-x-8 gap-row-4">
        <slot name="content" :toggle="toggle" />
      </div>
      </div>
    </transition>
  </div>
</template>
