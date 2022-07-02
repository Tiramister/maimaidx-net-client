<template>
  <v-app>
    <CommonHeader />
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
import CommonHeader from "./components/CommonHeader.vue";

export default defineComponent({
  name: "App",
  components: {
    HelloWorld,
    CommonHeader,
  },
});
</script>
