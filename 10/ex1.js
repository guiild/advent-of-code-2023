import { input } from './input.js';
let lines = input.split('\n').map((line) => line.split(''));

// Find the starting point and put it in an array as x/y coordinates
let currentPositions = [lines.reduce((acc, line, y) => {
    const x = line.indexOf('S');
    return x > -1 ? { x, y } : acc;
}, {})];

const isTopValidMove = ({ x, y }) => {
    try {
        return /[|7F]/.test(lines[y - 1][x]);
    } catch (e) {
        return false;
    }
};

const isBottomValidMove = ({ x, y }) => {
    try {
        return /[|JL]/.test(lines[y + 1][x]);
    } catch (e) {
        return false;
    }
}

const isLeftValidMove = ({ x, y }) => {
    try {
        return /[-FL]/.test(lines[y][x - 1]);
    } catch (e) {
        return false;
    }
}

const isRightValidMove = ({ x, y }) => {
    try {
        return /[-J7]/.test(lines[y][x + 1]);
    } catch (e) {
        return false;
    }
}

// From a position, find the next position and put it in an array as x/y coordinates
// After, replace the position with a dot
const findNextPosition = ({ x, y }) => {
    const nextPositions = [];
    switch (lines[y][x]) {
        case 'S':
            if (isTopValidMove({ x, y })) nextPositions.push({ x, y: y - 1 });
            if (isBottomValidMove({ x, y })) nextPositions.push({ x, y: y + 1 });
            if (isLeftValidMove({ x, y })) nextPositions.push({ x: x - 1, y });
            if (isRightValidMove({ x, y })) nextPositions.push({ x: x + 1, y });
            break;
        case '|':
            if (isTopValidMove({ x, y })) nextPositions.push({ x, y: y - 1 });
            if (isBottomValidMove({ x, y })) nextPositions.push({ x, y: y + 1 });
            break;
        case '-':
            if (isLeftValidMove({ x, y })) nextPositions.push({ x: x - 1, y });
            if (isRightValidMove({ x, y })) nextPositions.push({ x: x + 1, y });
            break;
        case 'L':
            if (isTopValidMove({ x, y })) nextPositions.push({ x, y: y - 1 });
            if (isRightValidMove({ x, y })) nextPositions.push({ x: x + 1, y });
            break;
        case 'J':
            if (isTopValidMove({ x, y })) nextPositions.push({ x, y: y - 1 });
            if (isLeftValidMove({ x, y })) nextPositions.push({ x: x - 1, y });
            break;
        case '7':
            if (isBottomValidMove({ x, y })) nextPositions.push({ x, y: y + 1 });
            if (isLeftValidMove({ x, y })) nextPositions.push({ x: x - 1, y });
            break;
        case 'F':
            if (isBottomValidMove({ x, y })) nextPositions.push({ x, y: y + 1 });
            if (isRightValidMove({ x, y })) nextPositions.push({ x: x + 1, y });
            break;
        default:
            break;
    }
    lines[y][x] = '.';
    return nextPositions;
}

// While there are still positions to check, find the next position
let steps = 0;
while (currentPositions.length) {
    const nextPositions = [];
    currentPositions.forEach((position) => {
        nextPositions.push(...findNextPosition(position));
    });
    steps++;
    currentPositions = nextPositions;
}

console.log(steps - 1);
