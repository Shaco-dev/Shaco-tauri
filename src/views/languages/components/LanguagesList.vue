<script setup lang="ts">
import { ref, computed } from 'vue';
import LanguageCard from './LanguageCard.vue';
import SearchInput from '../../../../components/SearchInput.vue';
import { CheckIcon, DocumentMagnifyingGlassIcon } from '@heroicons/vue/24/outline';

interface Language {
  code: string;
  flagCode: string;
  community: string;
  language: string;
  loaded: boolean;
}

const props = defineProps<{
  languages: Language[];
  selectedLanguages: string[];
  flagMap: Record<string, string>;
  isCompact?: boolean;
  isDisabled?: boolean;
  showSelectAll?: boolean;
}>();

defineEmits<{
  (e: 'select', code: string): void;
  (e: 'selectAll'): void;
}>();

const searchQuery = ref('');

const filteredLanguages = computed(() => {
  if (!searchQuery.value) return props.languages;
  
  const query = searchQuery.value.toLowerCase();
  return props.languages.filter(lang =>
    lang.language.toLowerCase().includes(query) ||
    lang.code.toLowerCase().includes(query) ||
    lang.community.toLowerCase().includes(query)
  );
});

const isAllSelected = computed(() => {
  return props.selectedLanguages.length === filteredLanguages.value.length;
});
</script>

<template>
  <div class="space-y-3">
    <div class="flex items-center gap-2">
      <SearchInput 
        variant="sm" 
        v-model="searchQuery" 
        placeholder="Search language" 
        class="flex-grow"
      />
      
      <div 
        v-if="showSelectAll"
        @click="!isDisabled && $emit('selectAll')" 
        :class="[
          'select-none relative pl-2 w-26 rounded-lg border flex items-center h-9 px-2',
          'border-gray-200 dark:border-gray-800 hover:border-[#396bd88a]',
          isDisabled
            ? 'bg-gray-100 dark:bg-gray-800 cursor-not-allowed'
            : 'bg-white dark:bg-[#0f0f0f] hover:cursor-pointer'
        ]"
      >
        <div :class="[
          'flex items-center justify-center w-4 h-4 rounded-full border',
          'transition-colors text-white text-[10px] mr-2',
          isAllSelected
            ? 'bg-[#396cd8] border-[#396cd8]'
            : 'bg-white dark:bg-gray-800 border-gray-300 hover:border-[#396cd8]'
        ]">
          <CheckIcon v-if="isAllSelected" class="w-3 h-3" />
        </div>
        <span class="text-amber-400/80 text-xs">Select All</span>
      </div>
    </div>
    
    <div 
      :class="[
        isCompact ? 'grid grid-cols-8 gap-1.5' : 'grid grid-cols-4 gap-3',
        isDisabled ? 'opacity-50 pointer-events-none' : ''
      ]"
    >
      <LanguageCard
        v-for="lang in filteredLanguages"
        :key="lang.code"
        :code="lang.code"
        :flagCode="lang.flagCode"
        :language="lang.language"
        :flagSrc="flagMap[lang.flagCode]"
        :isSelected="selectedLanguages.includes(lang.code)"
        :isCompact="isCompact"
        :isDisabled="isDisabled"
        @select="$emit('select', lang.code)"
      />
      
      <div 
        v-if="filteredLanguages.length === 0"
        class="col-span-full py-8 text-center text-gray-500 dark:text-gray-400"
      >
        <DocumentMagnifyingGlassIcon class="mx-auto h-8 w-8 mb-2" />
        <p class="text-sm">No matches for "{{ searchQuery }}"</p>
      </div>
    </div>
  </div>
</template> 