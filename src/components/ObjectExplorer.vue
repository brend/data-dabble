<template>
    <div class="data">
        <Tree :loading="loading" :value="nodes" lazy @node-expand="loadNode" v-slot="{ node }" />
    </div>
</template>

<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import Tree from 'primevue/tree';

const loading = ref(true);
const nodes = ref([]);
const loadNode = (node) => {
    if (!node.children) {
        loading.value = true;
        invoke('query_columns', {tableName: node.data}).then((headers) => {
            node.children = headers.map((column) => ({key: `${node.data}.${column}`, label: column.field, data: `${node.data}.${column.field}`, leaf: true}));
            loading.value = false;
        });
    }
};

//nodes.value = [{key: 'PP_PRODUKT', label: 'PP_PRODUKT', data: 'PP_PRODUKT', leaf: false}, {key: 'FS_EXPORT', label: 'FS_EXPORT', data: 'FS_EXPORT', leaf: false}];
invoke('query_tables').then((tables) => {
    nodes.value = tables.map((table) => ({key: table, label: table, data: table, leaf: false}));
    loading.value = false;
});
</script>

<style scoped>
.data {
    font-family: "Source Code Pro", "Courier New", Courier, monospace;
    cursor: pointer;
}
</style>