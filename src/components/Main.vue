<template>
  <Splitter class="mb-8">
    <SplitterPanel
      :size="25"
      :minSize="25"
      class="flex items-center justify-center"
    >
      <Tree
        :value="nodes"
        @node-expand="onNodeExpand"
        :loading="loading"
        class="w-full md:w-[30rem] explorer"
      ></Tree>
    </SplitterPanel>
    <SplitterPanel :size="75">
      <Tabs class="full-height" value="0">
        <TabList>
          <Tab value="0">Query 1</Tab>
          <Tab value="1">Query 2</Tab>
          <Tab value="2">Query 3</Tab>
        </TabList>
        <TabPanels class="full-height">
          <TabPanel class="full-height" value="0">
            <Textarea
              autoResize
              class="code-editor full-height"
              v-model="text1"
            />
          </TabPanel>
          <TabPanel value="1">
            <Textarea autoResize class="code-editor" v-model="text2" />
          </TabPanel>
          <TabPanel value="2">
            <Textarea autoResize class="code-editor" v-model="text3" />
          </TabPanel>
        </TabPanels>
      </Tabs>
    </SplitterPanel>
  </Splitter>
</template>

<script setup>
import { ref } from "vue";
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";

import Tabs from "primevue/tabs";
import TabList from "primevue/tablist";
import Tab from "primevue/tab";
import TabPanels from "primevue/tabpanels";
import TabPanel from "primevue/tabpanel";

import Textarea from "primevue/textarea";

import Tree from "primevue/tree";

import ExplorerService from "../services/ExplorerService";

const text1 = ref(
  "SELECT PP_ID, PP_ARTIKEL_NR, PP_BEZEICHNUNG_1 FROM PP_PRODUKT;"
);
const text2 = ref(
  "CREATE TABLE T_Shlorp (ID NUMBER NOT NULL, Name NVARCHAR(255));"
);
const text3 = ref(
  "DECLARE\n  n NUMBER;\nBEGIN\n  SELECT COUNT(*) INTO n FROM PP_PRODUKT;\n  DBMS_OUTPUT.PUT_LINE('Anzahl vorhandener Produkte: ' || n);\nEND;"
);

const explorerService = new ExplorerService();
const nodes = ref([]);
const loading = ref(true);
explorerService.getExplorerTree().then((data) => {
  nodes.value = data;
  loading.value = false;
});
const onNodeExpand = (event) => {
  console.log("event", event);
  loading.value = true;
  explorerService.getExplorerTree(event).then((data) => {
    event.children = data;
    loading.value = false;
  });
};
</script>

<style scoped>
.full-height {
  height: 100%;
}

.code-editor {
  font-family: monospace;
  width: 100%;
}

.explorer {
  font-size: 0.8rem;
  cursor: pointer;
}
</style>
