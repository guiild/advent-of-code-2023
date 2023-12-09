import { input } from "./input.js";
const toArr = input.split('\n');

let res = {};
toArr.map((row, i) => {
    res[i] = res[i] ? res[i] + 1 : 1;
    const winningNumbers = row.split(': ')[1].split(' | ')[0].split(/\s+/).filter((num) => num !== '').map((num) => Number(num));
    const myNumbers = row.split(': ')[1].split(' | ')[1].split(/\s+/).filter((num) => num !== '').map((num) => Number(num));
    const myWinnings = myNumbers.filter((num) => winningNumbers.includes(num));
    for (let j = 1; j <= myWinnings.length; j++) {
        res[i + j] = res[i + j] ? res[j + i] + res[i] : res[i];
    }
});

console.log(Object.values(res).reduce((acc, cur) => acc + cur, 0));