<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import ShacoInput from '../../../components/ShacoInput.vue';

const props = defineProps<{
  selectedLanguage: any | null;
  selectedCode: string | null;
  translations: Record<string, any>;
  flagMap: Record<string, string>;
}>();

const emit = defineEmits<{
  (e: 'update:translations', code: string, translations: Record<string, string>): void;
}>();

const searchQuery = ref('');
const activeTitleKey = ref<string | null>(null);
const renderedChunks = ref(1);
const chunkSize = 50;

const titlePatterns = [
  '^TITRE_', 
  '^TITLE_',
  '^T_',
  '^nom_'
];

const titleEntries = computed(() => {
  if (!props.selectedCode) return [];
  
  const translation = Object.entries(props.translations[props.selectedCode] || {})
    .filter(([key]) => {
      return titlePatterns.some(pattern => {
        const regex = new RegExp(pattern);
        return regex.test(key);
      });
    });
  
  if (!searchQuery.value) return translation;
  
  const query = searchQuery.value.toLowerCase();
  return translation.filter(([key, value]: [string, any]) => {
    return key.toLowerCase().includes(query) || 
           String(value).toLowerCase().includes(query);
  });
});

const visibleTitles = computed(() => {
  return titleEntries.value.slice(0, chunkSize * renderedChunks.value);
});

watch(titleEntries, () => {
  renderedChunks.value = 1;
}, { immediate: true });

watch(visibleTitles, () => {
  if (visibleTitles.value.length < titleEntries.value.length) {
    requestIdleCallback(() => {
      renderedChunks.value++;
    });
  }
});

const titleColors = [
  'text-blue-500',
  'text-purple-500',
  'text-red-500',
  'text-green-500',
  'text-amber-500',
  'text-teal-500'
];

const randomColorIndex = (key: string) => {
  const hashCode = key.split('').reduce((acc, char) => {
    return char.charCodeAt(0) + ((acc << 5) - acc);
  }, 0);
  return Math.abs(hashCode) % titleColors.length;
};

const getColorClass = (key: string) => {
  return titleColors[randomColorIndex(key)];
};

const setActiveTitle = (key: string) => {
  activeTitleKey.value = key === activeTitleKey.value ? null : key;
};

const updateTitle = (key: string, value: string) => {
  if (!props.selectedCode) return;
  
  const updatedTranslations = { 
    ...props.translations[props.selectedCode],
    [key]: value 
  };
  
  emit('update:translations', props.selectedCode, updatedTranslations);
};

const handleInput = (e: Event, key: string) => {
  const target = e.target as HTMLTextAreaElement;

  target.style.height = 'auto';
  target.style.height = target.scrollHeight + 'px';
  
  updateTitle(key, target.value);
};
</script>

<template>
  <div class="flex-1 rounded-xl bg-white dark:bg-[#2f2f2f] p-6 shadow-sm border border-gray-200 dark:border-gray-700">
    <div class="flex items-center justify-between mb-4">
      <span class="flex items-center text-sm font-semibold text-gray-700 dark:text-[#f6f6f6]">
        <img 
          v-if="selectedLanguage" 
          :src="flagMap[selectedLanguage?.flagCode]"
          class="h-5 w-5 mr-2" 
          alt="Flag" 
        />
        {{ selectedLanguage ? `${selectedLanguage.language} Titles` : 'Select a language' }}
      </span>
      
      <ShacoInput 
        variant="md" 
        v-model="searchQuery" 
        placeholder="Search titles"
      />
    </div>

    <div class="border rounded-lg border-gray-200 dark:border-gray-700 overflow-hidden mb-4">
      <div class="bg-gray-50 text-gray-600 dark:text-gray-300 dark:bg-[#0f0f0f69] p-2 text-sm font-medium border-b border-gray-200 dark:border-gray-700">
        <div class="flex justify-between items-center">
          <span>Title Keys ({{ titleEntries.length }})</span>
          <span class="text-xs text-gray-500">Click on a title to edit</span>
        </div>
      </div>

      <div class="divide-y divide-gray-200 dark:divide-gray-700 max-h-[500px] overflow-y-auto scrollbar">
        <div 
          v-for="[key, value] in visibleTitles" 
          :key="key"
          @click="setActiveTitle(key)"
          class="p-3 hover:bg-gray-50 dark:hover:bg-[#0f0f0f69] transition-all cursor-pointer"
          :class="{'bg-blue-50 dark:bg-blue-900/20': activeTitleKey === key}"
        >
          <div class="flex justify-between items-start">
            <div class="font-mono text-xs text-gray-500 dark:text-gray-400 mb-1">{{ key }}</div>
            <div v-if="activeTitleKey !== key" 
                 :class="['font-medium text-sm', getColorClass(key)]">
              {{ String(value).substring(0, 120) }}{{ String(value).length > 120 ? '...' : '' }}
            </div>
          </div>
          
          <div v-if="activeTitleKey === key" class="mt-2 border-t pt-2 border-gray-200 dark:border-gray-700">
            <textarea 
              :value="String(value)"
              @input="(e) => handleInput(e, key)"
              class="w-full bg-transparent dark:bg-[#0f0f0f98] rounded-sm px-2 py-1 text-sm border border-gray-200 dark:border-gray-700 hover:border-gray-300 focus:border-[#396cd8] focus:outline-none resize-y min-h-[60px] text-gray-700 dark:text-gray-200"
              rows="3"
            ></textarea>
            
            <div class="mt-2 text-xs text-gray-500 dark:text-gray-400 italic">
              Preview: <span :class="['font-medium', getColorClass(key)]">{{ String(value) }}</span>
            </div>
          </div>
        </div>

        <div v-if="!selectedCode" class="p-4 text-center text-gray-500">
          Select a language from the sidebar to view and edit titles
        </div>
        
        <div v-else-if="titleEntries.length === 0" class="p-4 text-center text-gray-500">
          No title translations found or match your search
        </div>
      </div>
    </div>
    
    <div class="text-xs text-gray-500 dark:text-gray-400">
      <p>Title keys are automatically identified by patterns like: TITRE_, TITLE_, T_, nom_</p>
    </div>
  </div>
</template>

<style scoped>
.scrollbar {
  scrollbar-width: thin;
  scrollbar-color: rgba(156, 163, 175, 0.5) transparent;
}

.scrollbar::-webkit-scrollbar {
  width: 6px;
}

.scrollbar::-webkit-scrollbar-track {
  background: transparent;
}

.scrollbar::-webkit-scrollbar-thumb {
  background-color: rgba(156, 163, 175, 0.5);
  border-radius: 3px;
}
</style> 