import { input } from "./input.js";
const times = input.split('\n')[0].split(':')[1].split(' ').filter(e => e !== '').map((num) => Number(num));
const distances = input.split('\n')[1].split(':')[1].split(' ').filter(e => e !== '').map((num) => Number(num));

const time = times.reduce((acc, cur) => acc + cur, '');
const distanceToBeat = distances.reduce((acc, cur) => acc + cur, '');

let betterOption = 0
for (let holdingTime = 1; holdingTime < time; holdingTime++) {
    const distance = holdingTime * (time - holdingTime);
    betterOption = distance > distanceToBeat ? betterOption + 1 : betterOption;
}

console.log(betterOption)