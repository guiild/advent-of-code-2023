import { input } from './input.js';

const toArr = input.split('\n');

const isDigit = (char) => new RegExp("\\d").test(char);
const isDigitOrPoint = (char) => new RegExp("[0-9.]").test(char);

let res = 0;
const numbers = [];
for (let y = 0; y < toArr.length; y++) {
    let currentNum = '';
    let numberIsPart = false;
    for (let x = 0; x < toArr[y].length; x++) {
        if (isDigit(toArr[y][x])) {
            currentNum += toArr[y][x];
            for (let i = y - 1; i <= y + 1; i++) {
                for (let j = x - 1; j <= x + 1; j++) {
                    if (i >= 0 && i < toArr.length && j >= 0 && j < toArr[i].length && !isDigitOrPoint(toArr[i][j])) {
                        numberIsPart = true;
                    }
                }
            }
        } else if (currentNum && numberIsPart) {
            numbers.push(Number(currentNum));
            res += Number(currentNum);
            currentNum = '';
            numberIsPart = false;
        } else {
            currentNum = '';
            numberIsPart = false;
        }
    }
    if (currentNum && numberIsPart) {
        numbers.push(Number(currentNum));
        res += Number(currentNum);
    }
}

console.log(res);
