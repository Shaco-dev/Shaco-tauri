<script setup lang="ts">

const props = withDefaults(defineProps<{
    placeholder?: string
    modelValue?: string
    class?: string
    inputClass?: string
    variant?: 'sm' | 'md' | 'lg'
}>(), {
    placeholder: 'Search',
    class: '',
    inputClass: '',
    variant: 'sm'
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
                    'h-10 text-base'
        ]">
            <svg class="h-4 w-4 text-gray-600 dark:text-gray-400 flex-shrink-0" fill="none" stroke="currentColor"
                viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>

            <input type="text" :placeholder="props.placeholder" :value="props.modelValue" @input="updateValue" :class="[
                'w-full bg-transparent border-none focus:outline-none h-full',
                'text-gray-700 dark:text-gray-200',
                'placeholder-gray-500 dark:placeholder-gray-400',
                props.inputClass,
                props.variant === 'sm' ? 'h-6 text-sm' :
                    props.variant === 'lg' ? 'h-12 text-lg' :
                        'h-10 text-base'
            ]" />
        </div>
    </div>
</template>
