<template>
  <v-container>
    <v-table density="comfortable">
      <thead>
        <tr>
          <th>曲名</th>
          <th>譜面</th>
          <th>難易度</th>
          <th>レベル</th>
          <th>スコア</th>
          <th>ランク</th>
          <th>バッジ</th>
        </tr>
      </thead>
      <tbody></tbody>
    </v-table>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "MainContainer",
});
</script>
