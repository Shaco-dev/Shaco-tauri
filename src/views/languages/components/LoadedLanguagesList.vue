<script setup lang="ts">
import { ref, computed } from 'vue';
import SearchInput from '../../../components/SearchInput.vue';

interface Language {
  code: string;
  flagCode: string;
  community: string;
  language: string;
  loaded: boolean;
}

const props = defineProps<{
  languages: Language[];
  selectedCode: string | null;
  flagMap: Record<string, string>;
}>();

const searchQuery = ref('');

const filteredLanguages = computed(() => {
  if (!searchQuery.value) return props.languages.filter(lang => lang.loaded);
  
  const query = searchQuery.value.toLowerCase();
  return props.languages.filter(lang => 
    lang.loaded && (
      lang.language.toLowerCase().includes(query) ||
      lang.code.toLowerCase().includes(query) ||
      lang.community.toLowerCase().includes(query)
    )
  );
});
</script>

<template>
  <div class="p-3 bg-white dark:bg-[#0f0f0f98] rounded-lg border border-gray-200 dark:border-gray-700">
    <h3 class="text-sm font-medium mb-2">Loaded Languages ({{ filteredLanguages.length }})</h3>
    
    <SearchInput 
      variant="sm" 
      v-model="searchQuery" 
      placeholder="Search language"
    />
    
    <div class="mt-2 space-y-1 max-h-64 overflow-y-auto scrollbar">
      <div 
        v-for="lang in filteredLanguages" 
        :key="lang.code"
        class="flex items-center text-sm p-2 rounded-md cursor-pointer transition-all" 
        :class="{
          'bg-blue-50 dark:bg-blue-900/30 border border-blue-200 dark:border-blue-800': selectedCode === lang.code,
          'hover:bg-gray-100/50 border border-transparent dark:hover:bg-gray-800': selectedCode !== lang.code
        }" 
        @click="$emit('select', lang.code)"
      >
        <img 
          :src="flagMap[lang.flagCode]" 
          class="h-4 w-4 mr-2" 
          alt="Flag" 
        />
        <span class="flex-1">{{ lang.language }}</span>
        <span class="ml-2 text-xs text-gray-500">tfm-{{ lang.code.toLowerCase() }}.gz</span>
      </div>
      
      <div v-if="filteredLanguages.length === 0" class="text-sm text-gray-500 p-2">
        {{ props.languages.some(lang => lang.loaded) ? 'No languages found' : 'No languages loaded' }}
      </div>
    </div>
  </div>
</template> 