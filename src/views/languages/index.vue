<script setup lang="ts">
// Import components
import { computed, reactive, ref, watch, watchEffect } from 'vue'
import BaseButton from '../../components/BaseButton.vue'
import Dropdown from '../../components/Dropdown.vue'
import Modal from '../../components/Modal.vue'
import SearchInput from '../../components/SearchInput.vue'

import LanguesLogo from '../../assets/langues.png'

import flagMap from './flagLoader';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
// import { Command } from '@tauri-apps/plugin-shell';

// Import icons
import {
    DocumentArrowDownIcon,
    DocumentArrowUpIcon,
    FolderIcon,
    CloudArrowDownIcon,
    PaintBrushIcon,
    PencilSquareIcon,
    DocumentTextIcon,
    Square2StackIcon,
    CheckIcon,
    DocumentMagnifyingGlassIcon,
    ArrowDownTrayIcon,
} from '@heroicons/vue/24/outline'

import { readFile, writeFile, } from '@tauri-apps/plugin-fs'
import { join } from '@tauri-apps/api/path'


const currentView = ref('edit')
const showDownloadModal = ref(false)
const showExportModal = ref(false)
const exportFormat = ref('gz')
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
    // { code: 'zh-cn', flagCode: 'cn', community: '中国简体', language: 'Simplified Chinese', loaded: false },
]);

const loadedLanguages = computed(() => languages.filter(l => l.loaded))
const searchLoadedQuery = ref('')
const selectedCode = ref<string | null>(null)
const selectedLanguage = ref<any>(null)
const exportPath = ref('./');

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
        let loadedFrom = '';
        for (const filePath of selected) {
            const filename = filePath.split('\\').pop();
            loadedFrom = filePath.split('\\').slice(0, -1).join('\\')
            if (!filename) continue
            const languageCode = filename.split('-')[1]?.split('.')[0]
            let content = '';
            if (extension == 'gz')
                content = await loadGzFile(filePath)
            else if (extension)
                content = (await readFile(filePath)).toString()
            addLoadedLanguage(languageCode, content)
        }
        exportPath.value = loadedFrom;

    } else {
        console.log('No valid .gz file selected.')
    }
}

async function addLoadedLanguage(languageCode: string, content: string) {
    const lang = languages.find(l => l.code === languageCode)
    if (lang) {
        lang.loaded = true
        translations[languageCode] = await textToJson(content)
    } else {
        console.log(`Language code ${languageCode} not found in languages list.`)
    }
}

async function loadGzFile(path: string) {
    const unarchivedText = await invoke<string>('decompress_gz', { path })
    return unarchivedText;
}

async function textToJson(text: string) {
    const jsonObject: Record<string, string> = {};

    const blocks = text.split('\n-\n');

    for (const block of blocks) {

        const equalIndex = block.indexOf('=');
        if (equalIndex === -1) continue;

        const key = block.substring(0, equalIndex);
        const value = block.substring(equalIndex + 1);

        jsonObject[key] = value;
    }

    return jsonObject;
}

async function jsonToText(json: Record<string, string>) {
    const entries = Object.entries(json)
        .map(([key, value]) => `${key}=${value}`)
        .join('\n-\n');

    return entries + '\n-\n';
}

async function saveAsGz(content: string, path: string) {
    const gzFile = await invoke<string>('compress_gz', { content, path })
    return gzFile
}

async function saveContentToFile(content: string, filePath: string) {
    try {
        const encoder = new TextEncoder();
        await writeFile(filePath, encoder.encode(content))
    } catch (error) {
        console.error('Error saving file:', error);
    }
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

const exportFormats = [
    {
        value: 'gz',
        label: 'Compressed zlib Archive (.gz)',
        description: 'Transformice language archive',
        icon: Square2StackIcon
    },
    {
        value: 'txt',
        label: 'Text File (.txt)',
        description: 'Plain text file',
        icon: DocumentTextIcon
    }
]


// Handle directory selection
async function handleSelectExportPath() {
    try {
        const selected = await open({
            directory: true,
            title: 'Select Export Directory',
            defaultPath: exportPath.value
        });

        if (selected) {
            exportPath.value = selected;
        }
    } catch (error) {
        console.error('Error selecting directory:', error);
    }
}

// Utility to truncate long paths
function truncatePath(path: string): string {
    const segments = path.split('/');
    if (segments.length <= 2) {
        return path;
    }
    return segments[0] + '/...' + '/' + segments[segments.length - 2] + '/' + segments[segments.length - 1];
}

// New state variables
const isExporting = ref(false)
const exportSuccess = ref(false)
const exportError = ref<string | null>(null)
const openAfterExport = ref(true)

// Modified handleExport function
async function handleExport() {
    try {
        isExporting.value = true
        exportSuccess.value = false
        exportError.value = null

        // Show success
        exportSuccess.value = true

        // Export each selected language
        for (const lang of loadedLanguages.value) {
            if (lang.loaded) {
                const content = await jsonToText(translations[lang.code])
                const fileName = `tfm-${lang.code.toLowerCase()}.${exportFormat.value}`
                const filePath = `${exportPath.value}/${fileName}`
                if (exportFormat.value === 'gz') {
                    await saveAsGz(content, filePath)
                } else if (exportFormat.value === 'txt') {
                    await saveContentToFile(content, filePath)
                }
            }
        }

        // Open folder if enabled
        if (openAfterExport.value) {
            // todo: dosn't work, need a rust command to open the folder
            // const command = Command.create('explorer', [exportPath.value])
            // await command.spawn();
        }

        showExportModal.value = false;
        exportSuccess.value = false;
    } catch (error) {
        exportError.value = error instanceof Error ? error.message : 'Unknown error occurred'
        console.error(error)
    } finally {
        isExporting.value = false
    }
}


const filteredLanguages = computed(() => {
    if (!searchQuery.value) return languages
    const query = searchQuery.value.toLowerCase()
    return languages.filter(lang =>
        lang.language.toLowerCase().includes(query) ||
        lang.code.toLowerCase().includes(query)
    )
})

const toggleSelectAll = () => {
    if (selectedLanguages.value.length === filteredLanguages.value.length) {
        selectedLanguages.value = []
    } else {
        selectedLanguages.value = filteredLanguages.value.map(lang => lang.code)
    }
}

const downloadedFiles = ref(0);
const downloadingFilename = ref('');
const downloadProgress = ref(0);
const isDownloading = ref(false);
const loadAfterDownload = ref(true);

async function cancelDownload() {
    try {
        isDownloading.value = false
        downloadProgress.value = 0
        downloadedFiles.value = 0
        downloadingFilename.value = ''
    } catch (error) {
        console.error('Error cancelling download:', error)
    }
    finally {
        showDownloadModal.value = false;
    }
}

async function handleDownload() {
    let loadPath = ''
    try {
        // Get save directory from user
        const selectedDir = await open({
            directory: true,
            multiple: false,
            title: 'Select Download Directory'
        });

        if (!selectedDir)
            return;

        loadPath = selectedDir as string;

        isDownloading.value = true;
        downloadProgress.value = 0;
        downloadedFiles.value = 0;
        downloadingFilename.value = '';

        const totalFiles = selectedLanguages.value.length;

        for (const [index, langCode] of selectedLanguages.value.entries()) {
            try {
                // Construct download URL and path
                const url = `http://transformice.com/langues/tfm-${langCode}.gz?t=${Date.now()}`;
                const fileName = `tfm-${langCode}.gz`;
                const filePath = await join(selectedDir, fileName);

                // Download the file using Tauri command 
                const data = await invoke<Uint8Array>("fetch_url", { url });

                // Save the file
                await writeFile(filePath, new Uint8Array(data));

                // Update progress
                downloadedFiles.value = index + 1;
                downloadProgress.value = ((index + 1) / totalFiles) * 100;
                downloadingFilename.value = fileName;
            } catch (error) {
                console.error(`Failed to download ${langCode}:`, error);
                continue;
            }
        }

        showDownloadModal.value = false;
    } catch (error) {
        console.error('Download failed:', error);
    } finally {
        isDownloading.value = false;
        downloadProgress.value = 0;
        downloadingFilename.value = '';
        downloadedFiles.value = 0;
    }

    if (loadAfterDownload.value) {
        for (const lang of languages) {
            lang.loaded = false
        }
        for (const langCode of selectedLanguages.value) {
            const lang = languages.find(l => l.code === langCode)
            if (lang) {
                lang.loaded = true
                const textData = await loadGzFile(`${loadPath}/tfm-${langCode}.gz`)
                translations[langCode] = await textToJson(textData)
            }
        }
        selectedLanguages.value = [];
        exportPath.value = loadPath;
    }
}

</script>


<template>
    <div class="bg-[#f6f6f6] dark:bg-[#2f2f2f]">
        <!-- Top Navigation -->
        <nav
            class="sticky top-0 z-50 border-b border-gray-300 dark:border-[#ffbd2e] backdrop-blur bg-white/70 dark:bg-[#2f2f2f]/80">
            <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
                <div class="flex h-16 items-center justify-between text-sm">

                    <span class="flex items-center bg-white dark:bg-[#2f2f2f]/80 shadow-md rounded-xl px-3 py-1 transition-colors duration-200 border-b border-white/10 hover:bg-[#f9f9f9]">
                        <img :src="LanguesLogo" alt="Logo" class="mr-2" />
                        <span class="text-md font-bold text-[#ffbd2e]">Langues</span>
                        </span>

                    <div class="flex items-center space-x-3">

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

                        <BaseButton variant="secondary" :icon="CloudArrowDownIcon" @click="showDownloadModal = true">
                            Download
                        </BaseButton>

                        <BaseButton :disabled="exportPath == './'" variant="secondary" :icon="DocumentArrowDownIcon"
                            @click="showExportModal = true">
                            Export
                        </BaseButton>

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
                        <SearchInput variant="sm" v-model="searchLoadedQuery" placeholder="Search language"/>
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
                            <span class="flex text-sm font-semibold text-gray-700 dark:text-[#f6f6f6]">
                                <img v-if="selectedLanguage" :src="flagMap[selectedLanguage?.flagCode]"
                                    class="h-5 w-5 mr-2" alt="Flag" />
                                {{ selectedLanguage ? selectedLanguage.language :
                                    'Select a language' }}
                            </span>
                            <SearchInput variant="md" v-model="searchQuery" placeholder="Search translation"/>
                        </div>

                        <div class="border rounded-lg border-gray-200 dark:border-gray-700 overflow-hidden">
                            <div
                                class="grid grid-cols-12 bg-gray-50 text-gray-600 dark:text-gray-300 dark:bg-[#0f0f0f69] p-2 text-sm font-medium border-b border-gray-200 dark:border-gray-700">
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
        <Modal :show="showDownloadModal" title="Download Languages" @close="showDownloadModal = false" size="xl">

            <div class="space-y-4 relative">
                <!-- Overlay with spinner -->
                <div v-if="isDownloading"
                    class="absolute h-[255px] rounded-lg border border-transparent bottom-20 rouned-md inset-0 bg-black/50 z-10 flex items-center justify-center">
                </div>

                <!-- Search Bar -->
                <div class="grid grid-cols-3 w-full p-1">
                    <SearchInput variant="sm" v-model="searchQuery" placeholder="Search language"/>
                    <div @click="!isDownloading && toggleSelectAll()" :class="[
                        'select-none relative ml-2 pl-2 w-26 rounded-lg border',
                        'border-gray-200 dark:border-gray-800 hover:border-[#396bd88a]',
                        isDownloading
                            ? 'bg-gray-100 dark:bg-gray-800 cursor-not-allowed'
                            : 'bg-white dark:bg-[#0f0f0f] hover:cursor-pointer'
                    ]" style="font-size: 12px">
                        <div :class="[
                            'absolute top-1 left-1 flex items-center justify-center w-4 h-4 rounded-full border',
                            'transition-colors text-white text-[10px]',
                            selectedLanguages.length === languages.length
                                ? 'bg-[#396cd8] border-[#396cd8]'
                                : 'bg-white dark:bg-gray-800 border-gray-300 group-hover:border-[#396cd8]'
                        ]">
                            <CheckIcon v-if="selectedLanguages.length === languages.length" class="w-3 h-3" />
                        </div>
                        <span class="ml-5 text-amber-400/80">Select All</span>
                    </div>

                    <!-- Open After Export -->
                    <div class="select-none flex items-center space-x-2">
                        <input type="checkbox" v-model="loadAfterDownload" id="openAfterExport"
                            :disabled="isDownloading"
                            class="s-input h-4 w-4 text-[#ffbd2e] border-gray-300 rounded focus:ring-[#ffbd2e] dark:border-[#0f0f0f98] dark:bg-[#0f0f0f98]">
                        <label for="openAfterExport" class="text-sm text-[#0f0f0f98] dark:text-gray-300">
                            Load after download
                        </label>
                    </div>
                </div>

                <!-- Compact Language Grid -->
                <div class="h-[200px] overflow-y-auto scrollbar">
                    <div :class="[
                        'grid grid-cols-8 gap-1.5 pr-2',
                        isDownloading ? 'opacity-50 pointer-events-none' : ''
                    ]">
                        <button v-for="lang in filteredLanguages" :key="lang.code"
                            @click="!isDownloading && toggleLanguage(lang.code)" :disabled="isDownloading"
                            :title="lang.language" :class="[
                                'group relative p-2 rounded-md transition-all',
                                'hover:bg-blue-50/50 dark:hover:bg-blue-900/30',
                                selectedLanguages.includes(lang.code)
                                    ? 'bg-blue-100 dark:bg-blue-700/30 ring-2 ring-[#396cd8] border-1 border-[#396cd8]'
                                    : 'bg-gray-50 dark:bg-[#1a1a1a]',
                                isDownloading ? 'cursor-not-allowed' : ''
                            ]">
                            <div class="select-none flex flex-col items-center space-y-0">
                                <img :src="flagMap[lang.flagCode]" class="h-6 w-6 rounded-sm object-cover shadow-sm"
                                    :alt="lang.code" />
                                <span
                                    class="text-center text-xs font-mono text-gray-600 dark:text-gray-300 transition-opacity group-hover:opacity-0">
                                    {{ lang.code.toUpperCase() }}
                                </span>
                                <span
                                    class="absolute bottom-1 text-xs font-medium text-[#396cd8] opacity-0 transition-opacity group-hover:opacity-100">
                                    {{ lang.code.toUpperCase() }}
                                </span>
                            </div>

                            <!-- Selection Check -->
                            <div :class="[
                                'absolute top-1 right-1 flex items-center justify-center w-4 h-4 rounded-full border',
                                'transition-colors text-white text-[10px]',
                                selectedLanguages.includes(lang.code)
                                    ? 'bg-[#396cd8] border-[#396cd8]'
                                    : 'bg-white dark:bg-gray-800 border-gray-300 group-hover:border-[#396cd8]'
                            ]">
                                <CheckIcon v-if="selectedLanguages.includes(lang.code)" class="w-3 h-3" />
                            </div>
                        </button>
                    </div>

                    <div v-if="filteredLanguages.length === 0"
                        class="py-6 text-center text-gray-500 dark:text-gray-400">
                        <DocumentMagnifyingGlassIcon class="mx-auto h-6 w-6 mb-2" />
                        <p class="text-xs">No matches for "{{ searchQuery }}"</p>
                    </div>
                </div>

                <!-- Footer -->
                <div class="border-t pt-3 border-gray-200 dark:border-gray-700">
                    <div class="flex items-center justify-between text-sm">
                        <div class="text-gray-600 dark:text-gray-300 w-full">
                            <span v-if="!isDownloading">
                                {{ selectedLanguages.length }} selected
                            </span>
                            <!-- Downloading -->
                            <div v-else-if="isDownloading" class="text-center w-full max-w-xs px-4">
                                <p class="text-sm mb-1 font-medium text-gray-700 dark:text-gray-200">
                                    Downloading {{ selectedLanguages.length }} files...
                                </p>
                                <div
                                    class="relative w-full h-2.5 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden mb-2">
                                    <!-- Progress Bar -->
                                    <div class="absolute left-0 top-0 h-full bg-[#f0b22d] dark:bg-[#f0b22d] transition-all duration-300 ease-out"
                                        :style="{ width: `${downloadProgress}%` }"></div>
                                </div>

                                <!-- Progress Text -->
                                <div class="space-y-1.5">
                                    <p class="text-xs text-gray-500 dark:text-gray-400">
                                        <span class="font-mono">{{ downloadingFilename }}</span> - {{
                                            downloadedFiles
                                        }} of {{ selectedLanguages.length }} files processed
                                    </p>
                                </div>
                            </div>
                        </div>
                        <div class="flex space-x-2">
                            <BaseButton variant="secondary" size="sm" @click="cancelDownload" :disabled="isDownloading">
                                Cancel
                            </BaseButton>
                            <BaseButton variant="primary" size="sm"
                                :disabled="selectedLanguages.length === 0 || isDownloading" @click="handleDownload"
                                :loading="isDownloading">
                                <ArrowDownTrayIcon class="w-4 h-4 mr-1.5" />
                                Download
                            </BaseButton>
                        </div>
                    </div>
                </div>
            </div>
        </Modal>

        <Modal :show="showExportModal" title="Export Languages" @close="showExportModal = false">
            <div class="space-y-2">
                <!-- Export Location -->
                <div>
                    <label class="block text-sm font-medium mb-1.5 dark:text-gray-200">Export Location</label>
                    <div class="flex items-center gap-2">
                        <div class="flex-1 relative">
                            <input :value="exportPath ? truncatePath(exportPath) : 'Select export directory...'"
                                type="text" readonly :disabled="true" style="font-size: 0.875rem;"
                                class="s-input select-none w-full px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#0f0f0f69] dark:text-gray-500 truncate placeholder-gray-400"
                                :class="{ 'text-gray-600 dark:text-gray-500': exportPath, 'text-gray-400': !exportPath }" />
                        </div>
                        <BaseButton variant="secondary" class="!px-3 !py-2 h-10" @click="handleSelectExportPath">
                            <FolderIcon class="w-5 h-5" />
                        </BaseButton>
                    </div>
                </div>

                <!-- Export Format -->
                <div>
                    <label class="block text-sm font-medium mb-2 dark:text-gray-200">Export Format</label>
                    <div class="space-y-2 flex">
                        <div v-for="format in exportFormats" :key="format.value" class="w-full p-1">
                            <label @click="exportFormat = format.value"
                                class="select-none flex items-start space-x-3 p-3 bg-[#0f0f0f98] rounded-lg cursor-pointer transition-colors"
                                :class="{
                                    'bg-blue-50 dark:bg-blue-900/30 border border-blue-200 dark:border-blue-800': exportFormat === format.value,
                                    'hover:bg-gray-100/50 border border-transparent dark:hover:bg-gray-800': exportFormat !== format.value
                                }">
                                <div class="flex-1">
                                    <div class="flex items-center gap-2">
                                        <component :is="format.icon" class="w-5 h-5 text-[#ffbd2e]" />
                                        <span class="text-sm font-medium dark:text-gray-200">{{ format.label }}</span>
                                    </div>
                                    <p class="mt-1 text-xs text-gray-500 dark:text-gray-400 pl-7">{{ format.description
                                    }}
                                    </p>
                                </div>
                            </label>
                        </div>
                    </div>
                </div>

                <!-- Open After Export -->
                <div class="select-none flex items-center space-x-2">
                    <input type="checkbox" v-model="openAfterExport" id="openAfterExport"
                        class="s-input h-4 w-4 text-[#ffbd2e] border-gray-300 rounded focus:ring-[#ffbd2e] dark:border-[#0f0f0f98] dark:bg-[#0f0f0f98]">
                    <label for="openAfterExport" class="text-sm text-[#0f0f0f98] dark:text-gray-300">
                        Open folder after export
                    </label>
                </div>

                <!-- Loading State -->


                <!-- Actions -->
                <div class="flex justify-between gap-3 border-t pt-4 border-gray-200 dark:border-gray-700">

                    <div>
                        <div v-if="isExporting" class="flex items-center space-x-2 text-sm text-[#ffbd2e] pt-2">
                            <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none"
                                viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor"
                                    stroke-width="4">
                                </circle>
                                <path class="opacity-75" fill="currentColor"
                                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                                </path>
                            </svg>
                            <span>Exporting languages...</span>
                        </div>
                    </div>
                    <div class="flex justify-end gap-2">
                        <BaseButton variant="secondary" size="sm" @click="showExportModal = false"
                            :disabled="isExporting">
                            Cancel
                        </BaseButton>
                        <BaseButton variant="primary" size="sm" @click="handleExport"
                            :disabled="isExporting || !exportPath" :loading="isExporting">
                            <DocumentArrowDownIcon class="h-5 w-5 mr-2" />
                            {{ isExporting ? 'Exporting...' : 'Export' }}
                        </BaseButton>
                    </div>
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