import { input } from './input.js';

const toArr = input.split('\n');

const sanitizer = (str) => {
    let newStr = '';
    for (let i = 0; i < str.length; i++) {
        if (str.substring(i, i + 3) === 'one') {
            newStr += '1';
        } else if (str.substring(i, i + 3) === 'two') {
            newStr += '2';
        } else if (str.substring(i, i + 5) === 'three') {
            newStr += '3';
        } else if (str.substring(i, i + 4) === 'four') {
            newStr += '4';
        } else if (str.substring(i, i + 4) === 'five') {
            newStr += '5';
        } else if (str.substring(i, i + 3) === 'six') {
            newStr += '6';
        } else if (str.substring(i, i + 5) === 'seven') {
            newStr += '7';
        } else if (str.substring(i, i + 5) === 'eight') {
            newStr += '8';
        } else if (str.substring(i, i + 4) === 'nine') {
            newStr += '9';
        } else {
            newStr += str[i];
        }
    }
    return newStr;
}

let sum = 0;

for (let i = 0; i < toArr.length; i++) {
    let sanitizedSring = sanitizer(toArr[i]);
    const keepOnlyNumbers = sanitizedSring.replace(/\D/g, '');
    sum += Number(`${keepOnlyNumbers[0]}${keepOnlyNumbers[keepOnlyNumbers.length - 1]}`);
}
console.log(sum);
