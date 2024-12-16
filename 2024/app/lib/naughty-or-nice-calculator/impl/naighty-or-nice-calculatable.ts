export class Score {
    name: string;
    score: number;

    constructor(name: string, score: number) {
        this.name = name;
        this.score = score;
    }
}

export  const naughtyOrNiceCalculatorable = {
    calculate(name: string): Score {
        const s = Math.floor(Math.random() * 100);
        const score = new Score(name, s);
        return score;
    }
}