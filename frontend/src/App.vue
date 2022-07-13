<template>
  <v-app>
    <common-header />
    <v-main>
      <main-container />
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import CommonHeader from "./components/CommonHeader.vue";
import MainContainer from "./components/MainContainer.vue";

export default defineComponent({
  name: "App",
  components: {
    MainContainer,
    CommonHeader,
  },
});
</script>

<style>
:root {
  font-size: 20px;
}
</style>
