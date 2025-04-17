<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'
import Modal from '@/components/Modal.vue'
import LoadedLanguagesList from '@/views/languages/components/LoadedLanguagesList.vue'
import TranslationEditor from '@/views/languages/components/TranslationEditor.vue'
// import CustomTitles from '@/views/languages/components/CustomTitles.vue'
import DownloadModal from '@/views/languages/components/DownloadModal.vue'
import ExportModal from '@/views/languages/components/ExportModal.vue'
import ToolbarActions from '@/views/languages/components/ToolbarActions.vue'

import LanguesLogo from '@/assets/langues.png'
import flagMap from '@/views/languages/flagLoader.ts';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { join } from '@tauri-apps/api/path'
import { readFile, writeFile } from '@tauri-apps/plugin-fs'
import { languagesList  } from '@/data/languages.ts'

import {
    PaintBrushIcon,
    PencilSquareIcon
} from '@heroicons/vue/24/outline'


const currentView = ref('edit')
const showDownloadModal = ref(false)
const showExportModal = ref(false)
const exportFormat = ref('gz')
const selectedLanguages = ref<string[]>([])
const searchQuery = ref('')

const languages = reactive([...languagesList].map(lang => ({
    ...lang,
    loaded: false
})));

const loadedLanguages = computed(() => languages.filter(l => l.loaded))
const selectedCode = ref<string | null>(null)
const selectedLanguage = ref<any>(null)
const exportPath = ref('./');
const translations = reactive<Record<string, any>>({})

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

function toggleLanguage(langCode: string) {
    const index = selectedLanguages.value.indexOf(langCode)
    if (index === -1) {
        selectedLanguages.value.push(langCode)
    } else {
        selectedLanguages.value.splice(index, 1)
    }
}

async function handleImportFiles(extension: 'gz' | 'txt' | 'json') {
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
        console.log('No valid file selected.')
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

// Exporting variables and functions
const isExporting = ref(false)
const exportSuccess = ref(false)
const exportError = ref<string | null>(null)
const openAfterExport = ref(true)

async function handleExport() {
    try {
        isExporting.value = true
        exportSuccess.value = false
        exportError.value = null

        exportSuccess.value = true

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
                const url = `http://transformice.com/langues/tfm-${langCode}.gz?t=${Date.now()}`;
                const fileName = `tfm-${langCode}.gz`;
                const filePath = await join(selectedDir, fileName);

                const data = await invoke<Uint8Array>("fetch_url", { url });

                await writeFile(filePath, new Uint8Array(data));

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

                    <span class="flex items-center bg-white dark:bg-[#2f2f2f]/80 shadow-md rounded-xl px-3 py-1 transition-colors duration-200 border-b border-white/10 hover:bg-[#f9f9f9] dark:hover:bg-[#2f2f2f]/80">
                        <img :src="LanguesLogo" alt="Logo" class="mr-2" />
                        <span class="text-md font-bold text-[#ffbd2e]">Langues</span>
                        </span>

                    <ToolbarActions 
                        :canExport="exportPath !== './'" 
                        @import="handleImportFiles" 
                        @download="showDownloadModal = true" 
                        @export="showExportModal = true"
                    />
                </div>
            </div>
        </nav>

        <!-- Main Content-->
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
                    <LoadedLanguagesList
                        :languages="languages"
                        :selectedCode="selectedCode"
                        :flagMap="flagMap"
                        @select="selectLanguage"
                    />
                </div>

                <!-- Translation Editor Section -->
                <div v-if="currentView === 'edit'" class="flex-1">
                    <TranslationEditor 
                        :selectedLanguage="selectedLanguage"
                        :selectedCode="selectedCode"
                        :translations="translations"
                        :flagMap="flagMap"
                        @update:translations="(code: string, updatedTranslations: Record<string, string>) => translations[code] = updatedTranslations"
                    />
                </div>

                <div v-else-if="currentView === 'title'" class="flex-1">
                </div>

                <div v-else class="flex-1 rounded-xl bg-white dark:bg-[#2f2f2f] p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    <!-- Other views will go here -->
                </div>
            </div>
        </div>

        <!-- Modals -->
        <Modal :show="showDownloadModal" title="Download Languages" @close="showDownloadModal = false" size="xl">
            <DownloadModal
                :selectedLanguages="selectedLanguages"
                :languages="languages"
                :flagMap="flagMap"
                :isDownloading="isDownloading"
                :downloadProgress="downloadProgress"
                :downloadedFiles="downloadedFiles"
                :downloadingFilename="downloadingFilename"
                :searchQuery="searchQuery"
                @update:searchQuery="searchQuery = $event"
                @update:selectedLanguages="selectedLanguages = $event"
                @download="handleDownload"
                @cancel="cancelDownload"
                @toggleSelectAll="toggleSelectAll"
                @toggleLanguage="toggleLanguage"
            />
        </Modal>

        <Modal :show="showExportModal" title="Export Languages" @close="showExportModal = false">
            <ExportModal
                :exportPath="exportPath"
                :exportFormat="exportFormat"
                :isExporting="isExporting"
                :loadedLanguagesCount="loadedLanguages.length"
                @update:exportPath="exportPath = $event"
                @update:exportFormat="exportFormat = $event"
                @export="handleExport"
                @close="showExportModal = false"
            />
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