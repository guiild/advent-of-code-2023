import { input } from './input.js';
const instr = input.split('\n')[0];

const map = input.split('\n\n')[1].split('\n').reduce((acc, cur) => {
    const it = cur.split(' = ');
    const LR = it[1].replace('(', '').replace(')', '').split(', ')
    const L = LR[0];
    const R = LR[1];
    acc[it[0]] = { L, R };
    return acc;
}, {});

let currentEntries = Object.keys(map).filter((entry) => entry[2] === 'A');

const lengths = currentEntries.map(it => {
    let instructionIndex = 0;
    let steps = 0
    while (it[2] !== 'Z') {
        if (instructionIndex >= instr.length) instructionIndex = 0;
        it = map[it][instr[instructionIndex]];
        instructionIndex++;
        steps++;
    }
    return steps;
})
console.log(lengths)

const pgcd = (a, b) => b ? pgcd(b, a % b) : Math.abs(a)

const ppcm = (x, y) => (!x || !y) ? 0 : Math.abs((x * y) / pgcd(x, y))

console.log(lengths.reduce((a, b) => ppcm(a, b), 1))
