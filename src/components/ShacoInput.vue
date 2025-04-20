<script setup lang="ts">


const props = withDefaults(defineProps<{
    placeholder?: string
    modelValue?: string | number
    class?: string
    inputClass?: string
    variant?: 'sm' | 'md' | 'lg'
    icon?: 'search' | 'path' | 'database' | 'server' | 'clock'
    type?: 'text' | 'number' 
    disabled?: boolean
}>(), {
    placeholder: 'Search',
    class: '',
    inputClass: '',
    variant: 'sm',
    icon: 'search',
    type: 'text',
    disabled: false
})

const emit = defineEmits<{
    (e: 'update:modelValue', value: string): void
}>()

const updateValue = (e: Event) => {
    const value = (e.target as HTMLInputElement).value
    emit('update:modelValue', value)
}
</script>

<template>
    <div
        :class="['relative transition-colors duration-200 shadow-[0_2px_2px_rgba(0,0,0,0.2)] rounded-lg border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#0f0f0f] ', props.class]">
        <div :class="[
            'flex items-center gap-2 w-full text-gray-300 px-3',
            props.variant === 'sm' ? 'h-6 text-sm' :
                props.variant === 'lg' ? 'h-12 text-lg' :
                    'h-10 text-base',

        ]">
            <svg v-if="props.icon === 'search'" class="h-4 w-4 text-gray-600 dark:text-gray-400 flex-shrink-0" fill="none" stroke="currentColor"
                viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <svg v-if="props.icon === 'path'" class="h-4 w-4 text-gray-600 dark:text-gray-400 flex-shrink-0" fill="none" stroke="currentColor"
                viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 100-2" />
            </svg>
            <svg v-if="props.icon === 'database'" class="h-4 w-4 text-gray-600 dark:text-gray-400 flex-shrink-0" fill="none" stroke="currentColor"
                viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M4 6h16M4 12h16m-7 6h7" />
            </svg>
            <svg v-if="props.icon === 'server'" class="h-4 w-4 text-gray-600 dark:text-gray-400 flex-shrink-0" fill="none" stroke="currentColor"
                viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M5 12l5 5m0 0l5-5m-5 5v-12" />
            </svg>
            <svg v-if="props.icon === 'clock'" class="h-4 w-4 text-gray-600 dark:text-gray-400 flex-shrink-0" fill="none" stroke="currentColor"
                viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>



            <input :type="props.type" :placeholder="props.placeholder" :value="props.modelValue" @input="updateValue" :class="[
                'w-full bg-transparent border-none focus:outline-none h-full',
                'text-gray-700 dark:text-gray-200',
                'placeholder-gray-500 dark:placeholder-gray-400',
                props.inputClass,
                props.variant === 'sm' ? 'h-6 text-sm' :
                    props.variant === 'lg' ? 'h-12 text-lg' :
                        'h-10 text-base',
                props.type === 'number' ? 'outline-none' : '',
                props.disabled ? 'cursor-not-allowed' : ''
                ]" :disabled="props.disabled" />
        </div>
    </div>
</template>
