export class Score {
    name;
    score;
    constructor(name, score) {
        this.name = name;
        this.score = score;
    }
}
export const naughtyOrNiceCalculatorable = {
    calculate(name) {
        const s = Math.floor(Math.random() * 100);
        const score = new Score(name, s);
        return score;
    }
};
