import { input } from './input.js';

const toArr = input.split('\n');
let sum = 0;
for (let i = 0; i < toArr.length; i++) {
    const keepOnlyNumbers = toArr[i].replace(/\D/g, '');
    sum += Number(`${keepOnlyNumbers[0]}${keepOnlyNumbers[keepOnlyNumbers.length - 1]}`);
}
console.log(sum);
