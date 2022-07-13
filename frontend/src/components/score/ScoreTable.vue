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
      <tr v-for="record in records" :key="record.id">
        <td>{{ record.title }}</td>
        <td>{{ record.type }}</td>
        <td>{{ record.difficulty }}</td>
        <td>{{ record.level }}</td>
        <td>{{ record.scorePercent }} %</td>
        <td>{{ record.rank }}</td>
        <td>{{ record.badge }}</td>
      </tr>
    </tbody>
  </v-table>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ScoreRecord from "./scoreRecord";

export default defineComponent({
  name: "ScoreTable",
  data() {
    return {
      records: <Array<ScoreRecord>>[],
    };
  },
  created() {
    this.records.push(
      new ScoreRecord(
        "星空パーティーチューン",
        "DX",
        "MAS",
        "13+",
        1006837,
        "FC+"
      )
    );
    this.records.push(
      new ScoreRecord(
        "大輪の魂 (feat. AO, 司芭扶)",
        "DX",
        "MAS",
        "13",
        988293,
        ""
      )
    );
  },
});
</script>
