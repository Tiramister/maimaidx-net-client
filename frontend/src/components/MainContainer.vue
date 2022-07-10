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
      <tbody>
        <tr v-for="record in records" :key="getId(record)">
          <td>{{ record.title }}</td>
          <td>{{ record.type }}</td>
          <td>{{ record.difficulty }}</td>
          <td>{{ record.level }}</td>
          <td>{{ record.score / 10000 }} %</td>
          <td></td>
          <td>{{ record.badge }}</td>
        </tr>
      </tbody>
    </v-table>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "MainContainer",
  data() {
    return {
      records: [
        {
          title: "星空パーティーチューン",
          type: "DX",
          difficulty: "MAS",
          level: "13+",
          score: 1006837,
          badge: "FC+",
        },
        {
          title: "大輪の魂 (feat. AO, 司芭扶)",
          type: "DX",
          difficulty: "MAS",
          level: "13",
          score: 988293,
          badge: "",
        },
      ],
    };
  },
  methods: {
    getId(record: any) {
      return record.title + "-" + record.type + "-" + record.difficulty;
    },
  },
});
</script>
