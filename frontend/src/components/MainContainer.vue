<template>
  <v-container>
    <score-table />
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ScoreTable from "./score/ScoreTable.vue";

export default defineComponent({
  name: "MainContainer",
  components: {
    ScoreTable,
  },
});
</script>
