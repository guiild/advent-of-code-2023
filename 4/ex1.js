import { input } from "./input.js";
const toArr = input.split('\n');

let res = 0;
toArr.map((row) => {
    const winningNumbers = row.split(': ')[1].split(' | ')[0].split(/\s+/).filter((num) => num !== '').map((num) => Number(num));
    const myNumbers = row.split(': ')[1].split(' | ')[1].split(/\s+/).filter((num) => num !== '').map((num) => Number(num));
    const myWinnings = myNumbers.filter((num) => winningNumbers.includes(num));
    res += myWinnings.length > 0 ? Math.pow(2, myWinnings.length - 1) : 0;
});

console.log(res);