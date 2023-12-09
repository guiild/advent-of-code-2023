import { input } from "./input.js";
const times = input.split('\n')[0].split(':')[1].split(' ').filter(e => e !== '').map((num) => Number(num));
const distances = input.split('\n')[1].split(':')[1].split(' ').filter(e => e !== '').map((num) => Number(num));

let res = 1;
for (let i = 0; i < times.length; i++) {
    const time = times[i];
    const distanceToBeat = distances[i];
    let betterOption = 0
    for (let holdingTime = 1; holdingTime < time; holdingTime++) {
        const distance = holdingTime * (time - holdingTime);
        betterOption = distance > distanceToBeat ? betterOption + 1 : betterOption;
    }
    res *= betterOption;
}
console.log(res)