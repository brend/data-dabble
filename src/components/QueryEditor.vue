<template>
  <Toolbar>
    <template #start>
      <Button icon="pi pi-play-circle" class="mr-2" severity="primary" text @click="executeQuery" />
    </template>
  </Toolbar>
  <Splitter style="height: 100%" layout="vertical">
    <SplitterPanel class="flex items-center justify-center">
      <Textarea autoResize class="code-editor" v-model="text" />
    </SplitterPanel>
    <SplitterPanel class="flex items-center justify-center">
      <DataTable :value="products" tableStyle="min-width: 50rem">
        <Column field="code" header="Code"></Column>
        <Column field="name" header="Name"></Column>
        <Column field="category" header="Category"></Column>
        <Column field="quantity" header="Quantity"></Column>
      </DataTable>
    </SplitterPanel>
  </Splitter>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';
import Textarea from 'primevue/textarea';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';

import createQueryService from '../services/QueryService';

const props = defineProps<{
  dataProviderId: string;
}>();
const text = ref("SELECT * FROM users;");
const products = ref([] as any[]);
const queryService = createQueryService();
const executeQuery = () => {
  queryService.executeQuery(props.dataProviderId, text.value).then((data) => {
    products.value = data;
  });
}
</script>

<style scoped>
.code-editor {
  font-family: monospace;
  width: 100%;
}
</style>