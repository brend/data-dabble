<template>
  <Toolbar>
    <template #start>
        <Button icon="pi pi-plus" class="mr-2" severity="secondary" text />
        <Button icon="pi pi-print" class="mr-2" severity="secondary" text />
        <Button icon="pi pi-upload" severity="secondary" text />
    </template>

    <template #center>
        <IconField>
            <InputIcon>
                <i class="pi pi-search" />
            </InputIcon>
            <InputText placeholder="Search" />
        </IconField>
    </template>

    <template #end> <SplitButton label="Save" :model="items"></SplitButton></template>
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

const text = ref("SELECT * FROM users;");
const products = ref([
    { code: 'a1', name: 'Apple', category: 'Fruit', quantity: 5 },
    { code: 'b1', name: 'Banana', category: 'Fruit', quantity: 10 },
    { code: 'o1', name: 'Orange', category: 'Fruit', quantity: 5 },
    { code: 'c1', name: 'Carrot', category: 'Vegetable', quantity: 5 },
    { code: 't1', name: 'Tomato', category: 'Vegetable', quantity: 10 },
    { code: 'p1', name: 'Potato', category: 'Vegetable', quantity: 15 }
]);
const items = ref([
    {
        label: 'Update',
        icon: 'pi pi-refresh'
    },
    {
        label: 'Delete',
        icon: 'pi pi-times'
    }
])
</script>

<style scoped>
.code-editor {
  font-family: monospace;
  width: 100%;
}
</style>