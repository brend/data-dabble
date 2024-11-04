<script setup lang="ts">
import Main from "./components/Main.vue";
</script>

<template>
  <div class="container">
    <Main class="main-view"></Main>
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
  width: 100vw;
  height: 100vh;
  position: relative;
  top: 0;
  left: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
}

.main-view {
  width: 100%;
  height: 100%;
}
</style>
