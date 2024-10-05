<script setup>
import { ref } from 'vue';

const items = ref([
    { id: 1, name: 'Item 1' },
    { id: 2, name: 'Item 2' },
    { id: 3, name: 'Item 3' },
]);
</script>

<template>
    <div class="my-grid">
        <div v-for="item in items" :key="item.id" class="grid-item">
            {{ item.name }}
        </div>
    </div>
</template>

<style scoped>
.my-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
    gap: 10px;
}

.grid-item {
    background-color: #f0f0f0;
    padding: 10px;
    text-align: center;
    border: 1px solid #ccc;
}
</style>