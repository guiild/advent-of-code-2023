import { input } from './input.js';

const toArr = input.split('\n');

const isDigit = (char) => new RegExp("\\d").test(char);
const isStar = (char) => char === '*';

const numbers = {};
for (let y = 0; y < toArr.length; y++) {
    let currentNum = '';
    let numberIsPart = '';
    for (let x = 0; x < toArr[y].length; x++) {
        if (isDigit(toArr[y][x])) {
            currentNum += toArr[y][x];
            for (let i = y - 1; i <= y + 1; i++) {
                for (let j = x - 1; j <= x + 1; j++) {
                    if (i >= 0 && i < toArr.length && j >= 0 && j < toArr[i].length && isStar(toArr[i][j])) {
                        numberIsPart = '' + i + j;
                    }
                }
            }
        } else if (currentNum && numberIsPart) {
            numbers[numberIsPart] = {
                parts: numbers[numberIsPart] ? numbers[numberIsPart].parts + 1 : 1,
                value: numbers[numberIsPart] ? numbers[numberIsPart].value * Number(currentNum) : Number(currentNum)
            };
            currentNum = '';
            numberIsPart = false;
        } else {
            currentNum = '';
            numberIsPart = false;
        }
    }
    if (currentNum && numberIsPart) {
        numbers[numberIsPart] = {
            parts: numbers[numberIsPart] ? numbers[numberIsPart].parts + 1 : 1,
            value: numbers[numberIsPart] ? numbers[numberIsPart].value * Number(currentNum) : Number(currentNum)
        };
    }
}

console.log(Object.values(numbers).reduce((acc, cur) => cur.parts > 1 ? acc + cur.value : acc, 0));
