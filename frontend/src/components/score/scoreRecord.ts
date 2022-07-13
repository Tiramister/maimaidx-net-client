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

class ScoreRecord {
  constructor(
    public title: string,
    public type: "ST" | "DX",
    public difficulty: "BSC" | "ADV" | "EXP" | "MAS" | "ReMAS",
    public level: string,
    public score: number,
    public badge: "" | "FC" | "FC+" | "AP" | "AP+"
  ) {}

  get id(): string {
    return this.title + "-" + this.type + "-" + this.difficulty;
  }

  get rank(): string {
    for (const border of rankBorders) {
      if (this.score >= border.score) {
        return border.rank;
      }
    }
    return "~AAA";
  }

  get scorePercent(): string {
    return (this.score / 10000).toFixed(4);
  }
}

export default ScoreRecord;
