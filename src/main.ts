import App from "./App.vue";
import { createApp } from "vue";
import PrimeVue from "primevue/config";
import Aura from "@primevue/themes/aura";

import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Tree from "primevue/tree";

const app = createApp(App);
app.use(PrimeVue, {
  theme: {
    preset: Aura,
  },
});
app.component("DataTable", DataTable);
app.component("Column", Column);
app.component("Tree", Tree);
app.mount("#app");
