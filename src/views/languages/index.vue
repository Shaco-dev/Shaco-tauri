<script setup lang="ts">
// Import components
import { computed, reactive, ref, watch, watchEffect } from 'vue'
import BaseButton from '../../components/BaseButton.vue'
import Dropdown from '../../components/Dropdown.vue'
import Modal from '../../components/Modal.vue'

import LanguesLogo from '../../assets/langues.png'

import flagMap from './flagLoader';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

// Import icons
import {
    DocumentArrowDownIcon,
    DocumentArrowUpIcon,
    FolderIcon,
    CloudArrowDownIcon,
    PaintBrushIcon,
    PencilSquareIcon
} from '@heroicons/vue/24/outline'
import { readFile } from '@tauri-apps/plugin-fs'


const currentView = ref('edit')
const showDownloadModal = ref(false)
const showExportModal = ref(false)
const exportFormat = ref('gzip')
const selectedLanguages = ref<string[]>([])
const searchQuery = ref('')

const languages = reactive([
    { code: 'af', flagCode: 'za', community: 'Afrikaans', language: 'Afrikaans', loaded: false },
    { code: 'az', flagCode: 'az', community: 'Azərbaycan dili', language: 'Azerbaijani', loaded: false },
    { code: 'id', flagCode: 'my', community: 'Bahasa Indonesia', language: 'Indonesian', loaded: false },
    { code: 'ms', flagCode: 'my', community: 'Bahasa Melayu', language: 'Malay', loaded: false },
    { code: 'bi', flagCode: 'vu', community: 'Bislama', language: 'Bislama', loaded: false },
    { code: 'bs', flagCode: 'ba', community: 'Bosanski jezik', language: 'Bosnian', loaded: false },
    { code: 'ca', flagCode: 'ad', community: 'Català', language: 'Catalan', loaded: false },
    { code: 'ny', flagCode: 'mw', community: 'ChiCheŵa', language: 'Chichewa', loaded: false },
    { code: 'da', flagCode: 'dk', community: 'Dansk', language: 'Danish', loaded: false },
    { code: 'de', flagCode: 'de', community: 'Deutsch', language: 'German', loaded: false },
    { code: 'et', flagCode: 'ee', community: 'Eesti keel', language: 'Estonian', loaded: false },
    { code: 'na', flagCode: 'nr', community: 'Ekakairũ Naoero', language: 'Nauruan', loaded: false },
    { code: 'en', flagCode: 'gb', community: 'English', language: 'English', loaded: false },
    { code: 'es', flagCode: 'es', community: 'Español', language: 'Spanish', loaded: false },
    { code: 'to', flagCode: 'to', community: 'Faka Tonga', language: 'Tongan', loaded: false },
    { code: 'mg', flagCode: 'mg', community: 'Fiteny malagasy', language: 'Malagasy', loaded: false },
    { code: 'fr', flagCode: 'fr', community: 'Français', language: 'French', loaded: false },
    { code: 'sm', flagCode: 'ws', community: "Gagana fa'a Samoa", language: 'Samoan', loaded: false },
    { code: 'hr', flagCode: 'hr', community: 'Hrvatski', language: 'Croatian', loaded: false },
    { code: 'it', flagCode: 'it', community: 'Italiano', language: 'Italian', loaded: false },
    { code: 'mh', flagCode: 'mh', community: 'Kajin M̧ajeļ', language: 'Marshallese', loaded: false },
    { code: 'kl', flagCode: 'gl', community: 'Kalaallisut', language: 'Greenlandic', loaded: false },
    { code: 'rn', flagCode: 'bi', community: 'KiRundi', language: 'Kirundi', loaded: false },
    { code: 'rw', flagCode: 'rw', community: 'Kinyarwanda', language: 'Kinyarwanda', loaded: false },
    { code: 'sw', flagCode: 'ke', community: 'Kiswahili', language: 'Swahili', loaded: false },
    { code: 'ht', flagCode: 'ht', community: 'Kreyòl ayisyen', language: 'Haitian Creole', loaded: false },
    { code: 'lv', flagCode: 'lv', community: 'Latviešu valoda', language: 'Latvian', loaded: false },
    { code: 'lt', flagCode: 'lt', community: 'Lietuvių kalba', language: 'Lithuanian', loaded: false },
    { code: 'lb', flagCode: 'lu', community: 'Lëtzebuergesch', language: 'Luxembourgish', loaded: false },
    { code: 'hu', flagCode: 'hu', community: 'Magyar', language: 'Hungarian', loaded: false },
    { code: 'mt', flagCode: 'mt', community: 'Malti', language: 'Maltese', loaded: false },
    { code: 'nl', flagCode: 'nl', community: 'Nederlands', language: 'Dutch', loaded: false },
    { code: 'no', flagCode: 'no', community: 'Norsk', language: 'Norwegian', loaded: false },
    { code: 'uz', flagCode: 'uz', community: "O'zbek", language: 'Uzbek', loaded: false },
    { code: 'pl', flagCode: 'pl', community: 'Polski', language: 'Polish', loaded: false },
    { code: 'pt', flagCode: 'pt', community: 'Português', language: 'Portuguese', loaded: false },
    { code: 'br', flagCode: 'br', community: 'Português brasileiro', language: 'Brazilian Portuguese', loaded: false },
    { code: 'ro', flagCode: 'ro', community: 'Română', language: 'Romanian', loaded: false },
    { code: 'qu', flagCode: 'bo', community: 'Runa Simi', language: 'Quechua', loaded: false },
    { code: 'st', flagCode: 'ls', community: 'SeSotho', language: 'Southern Sotho', loaded: false },
    { code: 'tn', flagCode: 'bw', community: 'SeTswana', language: 'Tswana', loaded: false },
    { code: 'sq', flagCode: 'al', community: 'Shqip', language: 'Albanian', loaded: false },
    { code: 'ss', flagCode: 'sz', community: 'SiSwati', language: 'Swazi', loaded: false },
    { code: 'sk', flagCode: 'sk', community: 'Slovenčina', language: 'Slovak', loaded: false },
    { code: 'sl', flagCode: 'si', community: 'Slovenščina', language: 'Slovene', loaded: false },
    { code: 'so', flagCode: 'so', community: 'Soomaaliga', language: 'Somali', loaded: false },
    { code: 'fi', flagCode: 'fi', community: 'Suomen kieli', language: 'Finnish', loaded: false },
    { code: 'sv', flagCode: 'se', community: 'Svenska', language: 'Swedish', loaded: false },
    { code: 'tl', flagCode: 'ph', community: 'Tagalog', language: 'Tagalog', loaded: false },
    { code: 'vi', flagCode: 'vn', community: 'Tiếng Việt', language: 'Vietnamese', loaded: false },
    { code: 'tk', flagCode: 'tm', community: 'Türkmen', language: 'Turkmen', loaded: false },
    { code: 'tr', flagCode: 'tr', community: 'Türkçe', language: 'Turkish', loaded: false },
    { code: 'fj', flagCode: 'fj', community: 'Vosa Vakaviti', language: 'Fijian', loaded: false },
    { code: 'wo', flagCode: 'sn', community: 'Wollof', language: 'Wolof', loaded: false },
    { code: 'yo', flagCode: 'ng', community: 'Yorùbá', language: 'Yoruba', loaded: false },
    { code: 'is', flagCode: 'is', community: 'Íslenska', language: 'Icelandic', loaded: false },
    { code: 'cs', flagCode: 'cz', community: 'Česky', language: 'Czech', loaded: false },
    { code: 'el', flagCode: 'gr', community: 'Ελληνικά', language: 'Greek', loaded: false },
    { code: 'be', flagCode: 'by', community: 'Беларуская', language: 'Belarusian', loaded: false },
    { code: 'ky', flagCode: 'kg', community: 'Кыргыз тили', language: 'Kyrgyz', loaded: false },
    { code: 'mk', flagCode: 'mk', community: 'Македонски јазик', language: 'Macedonian', loaded: false },
    { code: 'mn', flagCode: 'mn', community: 'монгол', language: 'Mongolian', loaded: false },
    { code: 'ru', flagCode: 'ru', community: 'Русский язык', language: 'Russian', loaded: false },
    { code: 'sr', flagCode: 'rs', community: 'Српски језик', language: 'Serbian', loaded: false },
    { code: 'tg', flagCode: 'tj', community: 'тоҷикӣ', language: 'Tajik', loaded: false },
    { code: 'uk', flagCode: 'ua', community: 'Українська мова', language: 'Ukrainian', loaded: false },
    { code: 'bg', flagCode: 'bg', community: 'български език', language: 'Bulgarian', loaded: false },
    { code: 'kk', flagCode: 'kz', community: 'Қазақ тілі', language: 'Kazakh', loaded: false },
    { code: 'hy', flagCode: 'am', community: 'Հայերեն', language: 'Armenian', loaded: false },
    { code: 'he', flagCode: 'il', community: 'עברית', language: 'Hebrew', loaded: false },
    { code: 'ur', flagCode: 'pk', community: 'اُردُو', language: 'Urdu', loaded: false },
    { code: 'ar', flagCode: 'iar', community: 'العربية', language: 'Arabic', loaded: false },
    { code: 'fa', flagCode: 'ir', community: 'فارسی', language: 'Persian', loaded: false },
    { code: 'dv', flagCode: 'mv', community: 'ދިވެހި', language: 'Maldivian', loaded: false },
    { code: 'ne', flagCode: 'np', community: 'नेपाली', language: 'Nepali', loaded: false },
    { code: 'hi', flagCode: 'in', community: 'हिन्दी', language: 'Hindi', loaded: false },
    { code: 'bn', flagCode: 'bd', community: 'বাংলা', language: 'Bengali', loaded: false },
    { code: 'ta', flagCode: 'lk', community: 'தமிழ்', language: 'Tamil', loaded: false },
    { code: 'th', flagCode: 'th', community: 'ไทย', language: 'Thai', loaded: false },
    { code: 'lo', flagCode: 'la', community: 'ພາສາລາວ', language: 'Lao', loaded: false },
    { code: 'dz', flagCode: 'bt', community: 'རྫོང་ཁ་', language: 'Dzongkha', loaded: false },
    { code: 'my', flagCode: 'mm', community: 'ဗမာစာ', language: 'Burmese', loaded: false },
    { code: 'ka', flagCode: 'ge', community: 'ქართული', language: 'Georgian', loaded: false },
    { code: 'ti', flagCode: 'er', community: 'ትግርኛ', language: 'Tigrinya', loaded: false },
    { code: 'am', flagCode: 'et', community: 'አማርኛ', language: 'Amharic', loaded: false },
    { code: 'km', flagCode: 'kh', community: 'ភាសាខ្មែរ', language: 'Khmer', loaded: false },
    { code: 'cn', flagCode: 'cn', community: '中国简体', language: 'Simplified Chinese', loaded: false },
    { code: 'zh', flagCode: 'hk', community: '中國繁體', language: 'Traditional Chinese', loaded: false },
    { code: 'ja', flagCode: 'jp', community: '日本語', language: 'Japanese', loaded: false },
    { code: 'ko', flagCode: 'kr', community: '한국어', language: 'Korean', loaded: false },
]);

const loadedLanguages = computed(() => languages.filter(l => l.loaded))
const searchLoadedQuery = ref('')
const selectedCode = ref<string | null>(null)
const selectedLanguage = ref<any>(null)

const selectLanguage = (code: string) => {
    const selected = languages.find(lang => lang.code === code)
    if (selected && code !== selectedCode.value) {
        selectedLanguage.value = selected
        selectedCode.value = selected.code
    } else {
        selectedLanguage.value = null
        selectedCode.value = null
    }
}

const filteredLoadedLanguages = computed(() => {
    if (!searchLoadedQuery.value) return languages.filter(lang => lang.loaded)
    const query = searchLoadedQuery.value.toLowerCase()
    return languages.filter(lang =>
        lang.loaded &&
        (lang.language.toLowerCase().includes(query) ||
            lang.code.toLowerCase().includes(query))
    )
})

// Sample translation data structure
const translations = reactive<Record<string, any>>({})

const translationEntries = computed(() => {
    if (!selectedCode.value) return []
    const translation = Object.entries(translations[selectedCode.value] || {})
    return translation.filter(([key, value]: [string, any]) => {
        return key.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
            value.toLowerCase().includes(searchQuery.value.toLowerCase())
    })
})

function toggleLanguage(langCode: string) {
    const index = selectedLanguages.value.indexOf(langCode)
    if (index === -1) {
        selectedLanguages.value.push(langCode)
    } else {
        selectedLanguages.value.splice(index, 1)
    }
}

async function handleImport() {
    try {



    } catch (error) {
        console.error('Import failed:', error);
        // Show error to user
    }
}

async function handleImportFiles(extension: 'gz' | 'txt' | 'json' = 'gz') {
    const filterNames = {
        'gz': 'Archived Language Files',
        'txt': 'Plain Text Language Files',
        'json': 'JSON Language Files'
    }
    const importTitles = {
        'gz': 'Select Archived Language Files (.gz)',
        'txt': 'Select Plain Text Language Files (.txt)',
        'json': 'Select JSON Language Files (.json)'
    }
    const selected = await open({
        multiple: true,
        directory: false,
        title: importTitles[extension],
        filters: [
            { name: filterNames[extension], extensions: [extension] }
        ]
    })

    if (Array.isArray(selected)) {
        for (const filePath of selected) {
            const filename = filePath.split('\\').pop();
            if (!filename) continue
            const languageCode = filename.split('-')[1]?.split('.')[0]
            let content = '';
            if (extension == 'gz')
                content = await loadGzFile(filePath)
            else if (extension)
                content = (await readFile(filePath)).toString()
            addLoadedLanguage(languageCode, content)
        }

    } else {
        console.log('No valid .gz file selected.')
    }
}

async function addLoadedLanguage(languageCode: string, content: string) {
    console.log(languageCode)
    const lang = languages.find(l => l.code === languageCode)
    if (lang) {
        lang.loaded = true
        translations[languageCode] = await textToJson(content)
        console.log(await textToJson(content))
        console.log(`Loaded language ${lang.language} (${lang.code})`)
    } else {
        console.log(`Language code ${languageCode} not found in languages list.`)
    }
}

async function loadGzFile(path: string) {
    const unarchivedText = await invoke<string>('decompress_gz', { path })
    console.log(unarchivedText)
    return unarchivedText;
}

async function textToJson(text: string) {
    const jsonObject: Record<string, string> = {};

    const lines = text.split('\n');

    for (const line of lines) {
        if (line.trim() === '' || line.trim() === '-') continue;

        const [key, value] = line.split('=');

        if (key && value) {
            jsonObject[key.trim()] = value.trim();
        }
    }

    return jsonObject;
}

function handleExport() {
    // Add your export logic here
    showExportModal.value = false
}

const autoResize = (e: Event) => {
    const target = e.target as HTMLTextAreaElement
    target.style.height = 'auto'
    target.style.height = target.scrollHeight + 'px'
}



const chunkSize = 50
const renderedChunks = ref(1)

const visibleEntries = computed(() => {
    return translationEntries.value.slice(0, chunkSize * renderedChunks.value)
})

watchEffect(() => {
    if (visibleEntries.value.length < translationEntries.value.length) {
        requestIdleCallback(() => {
            renderedChunks.value++
        })
    }
})

watch(selectedCode, async () => {
    renderedChunks.value = 1
})

watch(searchQuery, async () => {
    renderedChunks.value = 1
})

</script>


<template>
    <div class="bg-[#f6f6f6] dark:bg-[#2f2f2f]">
        <!-- Top Navigation -->
        <nav
            class="sticky top-0 z-50 border-b border-gray-200 dark:border-[#ffbd2e] backdrop-blur bg-white/80 dark:bg-[#2f2f2f]/80">
            <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
                <div class="flex h-16 items-center justify-between text-sm">

                    <span class="flex text-xl font-bold text-[#646cff] dark:text-[#ffbd2e]">
                        <img :src="LanguesLogo" alt="Logo" class="mr-4" />
                        <span class="mt-1">Langues</span>
                    </span>

                    <div class="flex items-center space-x-3">
                        <BaseButton variant="secondary" :icon="DocumentArrowDownIcon" @click="showExportModal = true">
                            Export
                        </BaseButton>
                        <BaseButton variant="secondary" :icon="CloudArrowDownIcon">
                            Download
                        </BaseButton>

                        <Dropdown>
                            <template #trigger="{ toggle }">
                                <BaseButton variant="secondary" :icon="DocumentArrowUpIcon" @click="toggle">
                                    Import
                                </BaseButton>
                            </template>

                            <template #content>
                                <div class="py-1 gap-2">
                                    <button @click="handleImportFiles('gz')"
                                        class="flex w-full items-center px-4 py-2.5 text-sm hover:bg-gray-100 dark:hover:bg-[#0f0f0f69] text-gray-700 dark:text-gray-200">
                                        <FolderIcon class="h-4 w-4 mr-2" />
                                        Import Archive (.gz)
                                    </button>
                                    <button @click="handleImportFiles('txt')"
                                        class="flex w-full items-center px-4 py-2.5 text-sm hover:bg-gray-100 dark:hover:bg-[#0f0f0f69] text-gray-700 dark:text-gray-200">
                                        <FolderIcon class="h-4 w-4 mr-2" />
                                        Import Text (.txt)
                                    </button>
                                    <button :disabled="true" @click="showDownloadModal = true"
                                        class="flex w-full items-center px-4 py-2.5 text-sm hover:bg-gray-100 dark:hover:bg-[#0f0f0f69] text-gray-700 dark:text-gray-200">
                                        <FolderIcon class="h-4 w-4 mr-2" />
                                        Import Json (.json)
                                        <span
                                            class="ml-2 px-2 py-1 text-xs bg-yellow-100 dark:bg-yellow-800 rounded-full">Soon!</span>
                                    </button>
                                </div>
                            </template>
                        </Dropdown>
                    </div>
                </div>
            </div>
        </nav>

        <!-- Main Content (Add back your main content section) -->
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 pt-6">
            <div class="flex gap-6">
                <!-- Sidebar -->
                <div class="w-64 flex flex-col gap-3">
                    <div class="space-y-1.5">
                        <button @click="() => { currentView = 'edit' }" :class="[
                            'flex items-center w-full px-3 py-2 rounded-md transition-all h-10',
                            currentView === 'edit'
                                ? 'bg-[#e8e8e8] dark:bg-[#0f0f0f69] text-[#396cd8] dark:text-[#24c8db] border border-[#396cd8] dark:border-[#24c8db]'
                                : 'hover:bg-gray-100 dark:hover:bg-[#0f0f0f69] text-gray-600 dark:text-gray-300 bg-white dark:bg-[#0f0f0f98]'
                        ]">
                            <PencilSquareIcon class="h-4 w-4 mr-2" />
                            Translation Editor
                        </button>

                        <button @click="currentView = 'title'" :disabled="true" :class="[
                            'flex items-center w-full px-3 py-2 rounded-md transition-all opacity-50 cursor-not-allowed h-10',
                            currentView === 'title'
                                ? 'bg-[#e8e8e8] dark:bg-[#0f0f0f69] text-[#396cd8] dark:text-[#24c8db] border border-[#396cd8] dark:border-[#24c8db]'
                                : ' text-gray-600 dark:text-gray-300 bg-white dark:bg-[#0f0f0f98]'
                        ]">
                            <PaintBrushIcon class="h-4 w-4 mr-2" />
                            Custom Titles
                            <span
                                class="ml-2 px-2 py-1 text-xs bg-yellow-100 dark:bg-yellow-800 rounded-full">Soon!</span>
                        </button>
                    </div>

                    <!-- Loaded Languages -->
                    <div
                        class="p-3 bg-white dark:bg-[#0f0f0f98] rounded-lg border border-gray-200 dark:border-gray-700">
                        <h3 class="text-sm font-medium mb-2">Loaded Languages ({{ loadedLanguages.length }})</h3>
                        <div class="relative">
                            <div
                                class="relative w-full pl-3 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-[#0f0f0f] text-gray-300 placeholder-gray-400">
                                <input type="text" placeholder="Search language" v-model="searchLoadedQuery"
                                    class="w-full pl-10 pr-4 bg-transparent border-none focus:outline-none h-6" />
                                <div class="absolute left-2 top-1/2 -translate-y-1/2 text-gray-400">
                                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                                    </svg>
                                </div>
                            </div>
                        </div>
                        <div class="mt-2 space-y-1 max-h-64 overflow-y-auto scrollbar">
                            <div v-for="lang in filteredLoadedLanguages" :key="lang.code"
                                class="flex items-center text-sm p-2 rounded-md cursor-pointer transition-all" :class="{
                                    'bg-blue-50 dark:bg-blue-900/30 border border-blue-200 dark:border-blue-800': selectedCode === lang.code,
                                    'hover:bg-gray-100/50 border border-transparent dark:hover:bg-gray-800': selectedCode !== lang.code
                                }" @click="selectLanguage(lang.code)">
                                <img :src="flagMap[lang.flagCode]" class="h-4 w-4 mr-2" alt="Flag" />
                                <span class="flex-1">{{ lang.language }}</span>
                                <span class="ml-2 text-xs text-gray-500">tfm-{{ lang.code.toLowerCase() }}.gz</span>
                            </div>
                            <div v-if="loadedLanguages.length === 0" class="text-sm text-gray-500 p-2">
                                No languages loaded
                            </div>
                            <div v-else-if="filteredLoadedLanguages.length === 0" class="text-sm text-gray-500 p-2">
                                No languages found
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Translation Editor Section -->
                <div
                    class="flex-1 rounded-xl bg-white dark:bg-[#2f2f2f] p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    <div v-if="currentView === 'edit'">
                        <div class="flex items-center justify-between mb-2">
                            <span class="flex text-sm font-semibold text-[#0f0f0f] dark:text-[#f6f6f6]">
                                <img v-if="selectedLanguage" :src="flagMap[selectedLanguage?.flagCode]"
                                    class="h-5 w-5 mr-2" alt="Flag" />
                                {{ selectedLanguage ? selectedLanguage.language :
                                    'Select a language' }}
                            </span>
                            <div class="relative">
                                <div
                                    class="relative w-full pl-3 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-[#0f0f0f] text-gray-300 placeholder-gray-400">
                                    <input type="text" placeholder="Search translation" v-model="searchQuery"
                                        class="w-full pl-10 pr-4 py-2 bg-transparent border-none focus:outline-none" />
                                    <div class="absolute left-2 top-1/2 -translate-y-1/2 text-gray-400">
                                        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                                        </svg>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="border rounded-lg border-gray-200 dark:border-gray-700 overflow-hidden">
                            <div
                                class="grid grid-cols-12 bg-gray-50 dark:bg-[#0f0f0f69] p-2 text-sm font-medium border-b border-gray-200 dark:border-gray-700">
                                <div class="col-span-4">Translation Key</div>
                                <div class="col-span-8">Translation Value</div>
                            </div>

                            <div
                                class="divide-y divide-gray-200 dark:divide-gray-700 max-h-76 overflow-y-auto scrollbar">
                                <div v-for="[key] in visibleEntries" :key="key"
                                    class="grid grid-cols-12 p-1 hover:bg-gray-50 dark:hover:bg-[#0f0f0f69] transition-all">
                                    <div class="col-span-4 font-mono text-sm pr-4 truncate">{{ key }}</div>
                                    <div class="col-span-8">
                                        <textarea v-model="translations[selectedCode as string][key]"
                                            class="w-full bg-[#0f0f0f98] rounded-sm px-2 py-1 text-sm border-b border-gray-200 dark:border-gray-700 hover:border-gray-300 focus:border-[#396cd8] focus:outline-none resize-y min-h-[30px] overflow-y-auto"
                                            rows="1" @input="autoResize"></textarea>
                                    </div>
                                </div>

                                <div v-if="!selectedCode" class="p-4 text-center text-gray-500">
                                    Select a language from the sidebar to view translations
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Modals with proper content -->
        <Modal :show="showDownloadModal" title="Select Languages to Download" @close="showDownloadModal = false">
            <div class="mb-4">
                <input v-model="searchQuery" type="text" placeholder="Search languages..."
                    class="w-full px-4 py-2 rounded-lg border border-gray-200 dark:border-gray-700 bg-transparent">
            </div>

            <div class="grid grid-cols-3 gap-3 max-h-96 overflow-y-auto">
                <div v-for="lang in languages" :key="lang.code" @click="toggleLanguage(lang.code)" :class="[
                    'p-3 rounded-lg border cursor-pointer transition-all',
                    selectedLanguages.includes(lang.code)
                        ? 'border-[#396cd8] bg-[#396cd8]/10 dark:bg-[#396cd8]/20'
                        : 'border-gray-200 dark:border-gray-700 hover:border-[#396cd8]'
                ]">
                    <div class="flex items-center">
                        <span class="fi mr-2" :class="`fi-${lang.code === 'en' ? 'us' : lang.code}`"></span>
                        {{ lang.language }}
                        <span class="ml-auto text-sm text-gray-500">{{ lang.code }}</span>
                    </div>
                </div>
            </div>

            <div class="mt-6 flex justify-end gap-3">
                <BaseButton variant="secondary" @click="showDownloadModal = false">
                    Cancel
                </BaseButton>
                <BaseButton variant="primary" @click="handleImport()">
                    Download Selected ({{ selectedLanguages.length }})
                </BaseButton>
            </div>
        </Modal>

        <Modal :show="showExportModal" title="Export Languages" @close="showExportModal = false">
            <div class="space-y-4">
                <div>
                    <label class="block text-sm font-medium mb-2">Export Format</label>
                    <select v-model="exportFormat"
                        class="w-full px-3 py-2 rounded-lg border border-gray-200 dark:border-gray-700 bg-transparent">
                        <option value="gzip">GZIP Compressed (.gz)</option>
                        <option value="plain">Plain Text (.json)</option>
                    </select>
                </div>

                <div class="mt-6 flex justify-end gap-3">
                    <BaseButton variant="primary" @click="handleExport" class="h-10 font-normal" :icon="DocumentArrowDownIcon">
                        Export Now
                    </BaseButton>
                    <BaseButton variant="secondary" @click="showExportModal = false"  class="h-10 font-normal">
                        Cancel
                    </BaseButton>
                </div>
            </div>
        </Modal>
    </div>
</template>

<style>
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