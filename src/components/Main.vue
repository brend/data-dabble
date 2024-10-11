<template>
    <div class="app-container">
      <!-- Sidebar for Object Explorer -->
      <Drawer v-model:visible="isSidebarVisible" header="Sidebar baby" position="left" :dismissable="false" class="object-explorer-sidebar">
        <ObjectExplorer />
      </Drawer>
  
      <!-- Main Content Area -->
      <div class="main-content">
        <!-- Document Tabs -->
        <TabView>
          <TabPanel v-for="tab in tabs" :key="tab.name" :header="tab.name">
            <component :is="tab.component" />
          </TabPanel>
        </TabView>
      </div>
    </div>
  </template>
  
  <script>
  import { ref } from 'vue';
  import Drawer from 'primevue/drawer';
  import TabView from 'primevue/tabview';
  import TabPanel from 'primevue/tabpanel';
  import ObjectExplorer from './ObjectExplorer.vue';
  
  export default {
    components: {
      Drawer,
      TabView,
      TabPanel,
      ObjectExplorer,
    },
    setup() {
      const isSidebarVisible = ref(true);
      const tabs = ref([
        { name: 'Query Editor', component: 'QueryEditor' },
        { name: 'Results', component: 'ResultsGrid' },
        // Add more tabs as needed
      ]);
  
      return {
        isSidebarVisible,
        tabs,
      };
    },
  };
  </script>
  
  <style scoped>
  .app-container {
    display: flex;
    height: 100vh;
  }
  
  .object-explorer-sidebar {
    width: 300px;
  }
  
  .main-content {
    flex: 1;
    padding: 1rem;
  }
  </style>
  