<template>
  <v-app>
    <v-app-bar color="primary" density="compact">
      <v-app-bar-title>maimaiでらっくすNET Client</v-app-bar-title>
    </v-app-bar>
    <v-main>
      <v-container>
        <HelloWorld />
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import HelloWorld from "./components/HelloWorld.vue";

export default defineComponent({
  name: "App",

  components: {
    HelloWorld,
  },

  data() {
    return {
      //
    };
  },
});
</script>
