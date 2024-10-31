<script setup lang="ts">
import Main from "./components/Main.vue";
</script>

<template>
  <div class="container">
    <Main></Main>
  </div>
</template>

<style scoped>
:root {
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  margin: 0;
  background-color: yellow;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}
</style>
