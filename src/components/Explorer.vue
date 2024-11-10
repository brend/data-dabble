<template>
  <Tree
    :value="nodes"
    @node-expand="onNodeExpand"
    :loading="loading"
    class="w-full md:w-[30rem] explorer"
  ></Tree>
</template>

<script setup>
import { ref } from "vue";
import Tree from "primevue/tree";
import ExplorerService from "../services/ExplorerService";

const emit = defineEmits(["error"]);
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
  explorerService
    .getExplorerTree(event)
    .then((data) => {
      event.children = data;
      loading.value = false;
    })
    .catch((e) => {
      loading.value = false;
      emit("error", e);
    });
};
</script>
