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

let instructionIndex = 0;
let res = 0;
let currentEntry = "AAA";
while (currentEntry !== "ZZZ") {
    if (instructionIndex >= instr.length) instructionIndex = 0;
    currentEntry = map[currentEntry][instr[instructionIndex]];
    instructionIndex++;
    res++;
}

console.log(res);