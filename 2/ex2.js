import { input } from './input.js';

const toArr = input.split('\n');

// turn "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
// into {id: 1, set: [{b: 3, r: 4, g: 0}, {b: 6, r: 1, g: 2}, {b: 0, r: 0, g: 2}]}
const games = toArr.map((e) => {
    const [id, sets] = e.split(': ');
    const set = sets.split(';').map((e) => {
        const colors = e.split(', ');
        const b = colors.filter((e) => e.includes('blue'));
        const r = colors.filter((e) => e.includes('red'));
        const g = colors.filter((e) => e.includes('green'));
        return {
            b: b.length > 0 ? Number(b[0].trim().split(' ')[0]) : 0,
            r: r.length > 0 ? Number(r[0].trim().split(' ')[0]) : 0,
            g: g.length > 0 ? Number(g[0].trim().split(' ')[0]) : 0
        };
    });
    return { id: Number(id.split(' ')[1]), set };
})

const maxR = 12;
const maxG = 13;
const maxB = 14;

let res = 0;
for (let game of games) {
    res += Math.max(...game.set.map((e) => e.b)) *
        Math.max(...game.set.map((e) => e.r)) *
        Math.max(...game.set.map((e) => e.g));
}

console.log(res);