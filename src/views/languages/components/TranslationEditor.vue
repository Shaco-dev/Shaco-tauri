<script setup lang="ts">
import { computed, watch, ref } from 'vue';
import ShacoInput from '../../../components/ShacoInput.vue';
import { PlusIcon, TrashIcon } from '@heroicons/vue/24/outline';

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
const renderedChunks = ref(1);
const chunkSize = 50;
const newTranslationKey = ref('');
const newTranslationValue = ref('');
const showAddForm = ref(false);
const editingKey = ref<{originalKey: string, newKey: string} | null>(null);


const translationEntries = computed(() => {
  if (!props.selectedCode) return [];
  
  const translation = Object.entries(props.translations[props.selectedCode] || {});
  if (!searchQuery.value) return translation;
  
  const query = searchQuery.value.toLowerCase();
  return translation.filter(([key, value]: [string, any]) => {
    return key.toLowerCase().includes(query) || 
           value.toLowerCase().includes(query);
  });
});

const visibleEntries = computed(() => {
  return translationEntries.value.slice(0, chunkSize * renderedChunks.value);
});

watch(translationEntries, () => {
  renderedChunks.value = 1;
}, { immediate: true });

watch(visibleEntries, () => {
  if (visibleEntries.value.length < translationEntries.value.length) {
    requestIdleCallback(() => {
      renderedChunks.value++;
    });
  }
});

const handleInput = (e: Event, key: string) => {
  const target = e.target as HTMLTextAreaElement;

  target.style.height = 'auto';
  target.style.height = target.scrollHeight + 'px';
  
  updateTranslation(key, target.value);
};

const updateTranslation = (key: string, value: string) => {
  if (!props.selectedCode) return;
  
  const updatedTranslations = { 
    ...props.translations[props.selectedCode],
    [key]: value 
  };
  
  emit('update:translations', props.selectedCode, updatedTranslations);
};

const addNewTranslation = () => {
  if (!props.selectedCode || !newTranslationKey.value.trim()) return;
  
  // Check if key already exists
  if (props.translations[props.selectedCode]?.[newTranslationKey.value]) {
    alert('This key already exists!');
    return;
  }
  
  const updatedTranslations = { 
    ...props.translations[props.selectedCode],
    [newTranslationKey.value]: newTranslationValue.value 
  };
  
  emit('update:translations', props.selectedCode, updatedTranslations);
  
  // Reset form
  newTranslationKey.value = '';
  newTranslationValue.value = '';
  showAddForm.value = false;
};

const startEditKey = (key: string) => {
  editingKey.value = {
    originalKey: key,
    newKey: key
  };
};

const saveKeyEdit = () => {
  if (!editingKey.value || !props.selectedCode) return;
  
  const originalKey = editingKey.value.originalKey;
  const newKey = editingKey.value.newKey;
  
  // If key hasn't changed or is empty
  if (originalKey === newKey || !newKey.trim()) {
    editingKey.value = null;
    return;
  }
  
  // Check if new key already exists
  if (props.translations[props.selectedCode]?.[newKey]) {
    alert('This key already exists!');
    return;
  }
  
  // Create new translations object with updated key
  const updatedTranslations = { ...props.translations[props.selectedCode] };
  updatedTranslations[newKey] = updatedTranslations[originalKey];
  delete updatedTranslations[originalKey];
  
  emit('update:translations', props.selectedCode, updatedTranslations);
  editingKey.value = null;
};

const cancelKeyEdit = () => {
  editingKey.value = null;
};

const deleteTranslation = (key: string) => {
  if (!props.selectedCode) return;

  const updatedTranslations = { ...props.translations[props.selectedCode] };
  delete updatedTranslations[key];

  emit('update:translations', props.selectedCode, updatedTranslations);
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
        {{ selectedLanguage ? selectedLanguage.language : 'Select a language' }}
      </span>
      
      <div class="flex space-x-2">
        <button 
          v-if="selectedCode && !showAddForm" 
          @click="showAddForm = true"
          class="px-2 h-6 text-sm bg-[#396cd8] hover:bg-[#2d5ab8] text-white rounded-md flex items-center"
        >
          <PlusIcon class="h-4 w-4 mr-1" />
          New
        </button>
        
        <ShacoInput 
          variant="sm" 
          v-model="searchQuery" 
          placeholder="Search translation"
        />
      </div>
    </div>
    
    <!-- Add new translation form -->
    <div v-if="showAddForm" class="mb-4 p-3 border border-blue-200 dark:border-blue-800 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
      <div class="text-sm font-medium mb-2 text-blue-700 dark:text-blue-300">Add New Translation</div>
      <div class="grid grid-cols-12 gap-2">
        <div class="col-span-4">
          <input 
            v-model="newTranslationKey" 
            placeholder="Translation Key" 
            class="w-full px-2 py-1.5 text-sm border border-gray-300 dark:border-gray-700 dark:bg-[#1f1f1f] rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
          />
        </div>
        <div class="col-span-5">
          <textarea 
            v-model="newTranslationValue" 
            placeholder="Translation Value"
            class="w-full px-2 py-1.5 text-sm border border-gray-300 dark:border-gray-700 dark:bg-[#1f1f1f] rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
            rows="1"
          ></textarea>
        </div>
        <div class="col-span-2 flex space-x-1">
          <button 
            @click="addNewTranslation"
            class="flex-1 h-8 px-2 bg-green-600 hover:bg-green-700 text-white rounded-md"
            style="font-size: 14px;"
          >
            Save
          </button>
          <button 
            @click="showAddForm = false"
            class="flex-1 h-8 px-2 bg-gray-500 hover:bg-gray-600 text-white rounded-md"
            style="font-size: 14px;"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>

    <div class="border rounded-lg border-gray-200 dark:border-gray-700 overflow-hidden">
      <div class="grid grid-cols-12 bg-gray-50 text-gray-600 dark:text-gray-300 dark:bg-[#0f0f0f69] p-2 text-sm font-medium border-b border-gray-200 dark:border-gray-700">
        <div class="col-span-4">Translation Key</div>
        <div class="col-span-8">Translation Value</div>
      </div>

      <div class="divide-y divide-gray-200 dark:divide-gray-700 max-h-76 overflow-y-auto scrollbar">
        <div 
          v-for="[key, value] in visibleEntries" 
          :key="key"
          class="grid grid-cols-12 pl-2 hover:bg-gray-50 dark:hover:bg-[#0f0f0f69] transition-all key-container"
        >
          <div 
            class="col-span-4 font-mono text-sm pr-4 truncate flex items-center"
          >
            <!-- Editing key mode -->
            <div v-if="editingKey && editingKey.originalKey === key" class="flex w-full space-x-1">
              <input 
                v-model="editingKey.newKey" 
                class="flex-1 px-1 w-[100px] py-0.5 text-sm border border-blue-300 dark:border-blue-700 rounded focus:outline-none focus:ring-1 focus:ring-blue-500 bg-white dark:bg-[#1f1f1f]"
              />
              <button @click="saveKeyEdit" class="p-1 text-green-600 hover:text-green-700">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>
              </button>
              <button @click="cancelKeyEdit" class="p-1 text-red-600 hover:text-red-700">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                </svg>
              </button>
            </div>
            
            <!-- Display key with edit button, now using CSS for visibility -->
            <div v-else class="flex w-full items-center key-display">
              <span class="flex-1 truncate" :title="key">{{ key }}</span>
              <button 
                @click="startEditKey(key)" 
                class="ml-1 p-1 text-gray-400 hover:text-blue-500"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z" />
                </svg>
              </button>
              <button @click="deleteTranslation(key)" class="ml-1 p-1 text-gray-400 hover:text-red-500">
                <TrashIcon class="h-3.5 w-3.5" />
              </button>
            </div>
          </div>
          
          <div class="col-span-8">
            <textarea 
              :value="String(value)"
              @input="(e) => handleInput(e, key)"
              class="w-full mt-1 mb-0 bg-transparent dark:bg-[#0f0f0f98] rounded-sm px-2 py-1 text-sm border-b border-gray-200 dark:border-gray-700 hover:border-gray-300 focus:border-[#396cd8] focus:outline-none resize-y min-h-[30px] overflow-y-auto text-gray-700 dark:text-gray-200"
              rows="1"
            ></textarea>
          </div>
        </div>

        <div v-if="!selectedCode" class="p-4 text-center text-gray-500">
          Select a language from the sidebar to view translations
        </div>
        
        <div v-else-if="translationEntries.length === 0" class="p-4 text-center text-gray-500">
          No translations match your search
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Hide edit button by default */
.edit-button {
  opacity: 0;
  visibility: hidden;
  transition: opacity 0.2s, visibility 0.2s;
}

/* Show edit button on hover */
.key-display:hover .edit-button {
  opacity: 1;
  visibility: visible;
}
</style> 