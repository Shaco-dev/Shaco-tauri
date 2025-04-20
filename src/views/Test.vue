<script setup lang="ts">
import ShacoInput from '../components/ShacoInput.vue'
import { ref } from 'vue'

const searchText = ref<string>('');
</script>

<template>
<ShacoInput v-model="searchText" variant="sm" />
<ShacoInput v-model="searchText" variant="md" />
<ShacoInput v-model="searchText" variant="lg" />
</template>



<style scoped>

</style>
