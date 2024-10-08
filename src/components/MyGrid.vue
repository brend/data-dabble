<template>
    <div class="card">
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
    invoke('query_columns', {tableName: 'MY_TABLE'}).then((headers) => {
        columns.value = headers;
        invoke('query_rows', {tableName: 'MY_TABLE'}).then((data) => (products.value = data));
    });
})

const products = ref([]);
const columns = ref([]);

</script>
