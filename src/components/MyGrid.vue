<template>
    <div class="card data">
        <DataTable :value="products" tableStyle="min-width: 50rem">
            <Column v-for="col of columns" :key="col.field" :field="col.field" :header="col.header"></Column>
        </DataTable>
    </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
/*
import { ProductService } from '@/service/ProductService';

onMounted(() => {
    ProductService.getProductsMini().then((data) => (products.value = data));
});
*/

onMounted(() => {
    const tableName = 'PP_PRODUKT';
    invoke('query_columns', {tableName}).then((headers) => {
        columns.value = headers;
        invoke('query_rows', {tableName}).then((data) => (products.value = data));
    });
});

const products = ref([]);
const columns = ref([]);

</script>

<style scoped>
.data {
    font-family: 'Courier New', Courier, monospace;
}
</style>