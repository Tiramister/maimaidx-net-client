<template>
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
        <td>{{ getRank(record.score) }}</td>
        <td>{{ record.badge }}</td>
      </tr>
    </tbody>
  </v-table>
</template>

<script lang="ts">
import { defineComponent } from "vue";

const rankBorders = [
  {
    score: 1005000,
    rank: "SSS+",
  },
  {
    score: 1000000,
    rank: "SSS",
  },
  {
    score: 995000,
    rank: "SS+",
  },
  {
    score: 990000,
    rank: "SS",
  },
  {
    score: 980000,
    rank: "S+",
  },
  {
    score: 970000,
    rank: "S",
  },
];

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
    getRank(score: number) {
      for (const border of rankBorders) {
        if (score >= border.score) {
          return border.rank;
        }
      }
      return "~AAA";
    },
  },
});
</script>
