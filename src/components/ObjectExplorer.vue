<template>
    <div class="data">
        <Tree :value="nodes" lazy @node-expand="loadNode" v-slot="{ node }" />
    </div>
</template>

<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import Tree from 'primevue/tree';

const nodes = ref([]);
const loadNode = (node) => {
    if (!node.children) {
        invoke('query_columns', {tableName: node.data}).then((headers) => {
            node.children = headers.map((column) => ({label: column.field, data: `${node.data}.${column.field}`, leaf: true}));
        });
    }
};

nodes.value = [{key: 'PP_PRODUKT', label: 'PP_PRODUKT', data: 'PP_PRODUKT', leaf: false}, {key: 'FS_EXPORT', label: 'FS_EXPORT', data: 'FS_EXPORT', leaf: false}];
</script>

<style scoped>
.data {
    font-family: "Source Code Pro", "Courier New", Courier, monospace;
}
</style>