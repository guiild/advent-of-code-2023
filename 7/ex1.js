import { input } from "./input.js";
const toArr = input.split('\n').map((row) => row.split(' '));

const isFiveOfAKind = (hand) => [...new Set(hand.split(''))].length === 1;
const isFourOfAKind = (hand) => {
    const sorted = hand.split('').sort();
    return sorted[0] === sorted[1] && sorted[1] === sorted[2] && sorted[2] === sorted[3] ||
        sorted[1] === sorted[2] && sorted[2] === sorted[3] && sorted[3] === sorted[4];
};
const isFullHouse = (hand) => {
    const sorted = hand.split('').sort();
    return sorted[0] === sorted[1] && sorted[1] === sorted[2] && sorted[3] === sorted[4] ||
        sorted[0] === sorted[1] && sorted[2] === sorted[3] && sorted[3] === sorted[4];
};
const isThreeOfAKind = (hand) => {
    const sorted = hand.split('').sort();
    return sorted[0] === sorted[1] && sorted[1] === sorted[2] ||
        sorted[1] === sorted[2] && sorted[2] === sorted[3] ||
        sorted[2] === sorted[3] && sorted[3] === sorted[4];
};
const isTwoPairs = (hand) => {
    const sorted = hand.split('').sort();
    return sorted[0] === sorted[1] && sorted[2] === sorted[3] ||
        sorted[0] === sorted[1] && sorted[3] === sorted[4] ||
        sorted[1] === sorted[2] && sorted[3] === sorted[4];
};
const isOnePair = (hand) => {
    const sorted = hand.split('').sort();
    return sorted[0] === sorted[1] ||
        sorted[1] === sorted[2] ||
        sorted[2] === sorted[3] ||
        sorted[3] === sorted[4];
}

const highestType = (hand) => {
    if (isFiveOfAKind(hand)) {
        return 7;
    } else if (isFourOfAKind(hand)) {
        return 6;
    } else if (isFullHouse(hand)) {
        return 5;
    } else if (isThreeOfAKind(hand)) {
        return 4;
    } else if (isTwoPairs(hand)) {
        return 3;
    } else if (isOnePair(hand)) {
        return 2;
    } else {
        return 1;
    }
};

const toNum = (str) => {
    switch (str) {
        case 'A':
            return 14;
        case 'K':
            return 13;
        case 'Q':
            return 12;
        case 'J':
            return 11;
        case 'T':
            return 10;
        default:
            return parseInt(str);
    }
}

const sortInput = (a, b) => {
    const highestTypeA = highestType(a[0]);
    const highestTypeB = highestType(b[0]);
    if (highestTypeA > highestTypeB) {
        return 1;
    } else if (highestTypeA < highestTypeB) {
        return -1;
    } else {
        for (let i = 0; i < a[0].length; i++) {
            const aNum = toNum(a[0][i]);
            const bNum = toNum(b[0][i]);
            if (aNum > bNum) {
                return 1;
            } else if (aNum < bNum) {
                return -1;
            }
        }
    }
    return -1;
};

console.log(toArr.sort(sortInput).reduce((acc, cur, i) => acc + cur[1] * (i + 1), 0));