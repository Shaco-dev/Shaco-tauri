<script setup lang="ts">
import SearchInput from '../components/SearchInput.vue'
import { ref } from 'vue'

const searchText = ref<string>('');
</script>

<template>
<SearchInput v-model="searchText" variant="sm" />
<SearchInput v-model="searchText" variant="md" />
<SearchInput v-model="searchText" variant="lg" />
</template>



<style scoped>

</style>
