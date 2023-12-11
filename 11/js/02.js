const fs = require('fs')

const factor = 1_000_000
const map = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n')
  .map(it => it.split(''))

const galaxies = []
map.forEach((line, y) => {
  line.forEach((e, x) => {
    if (e !== '.') galaxies.push([x, y])
  })
})

const rowIsEmptyCache = {}
const rowIsEmpty = row => rowIsEmptyCache[row] != null ? rowIsEmptyCache[row] : rowIsEmptyCache[row] = map[row].every(it => it === '.')
const colIsEmptyCache = {}
const colIsEmpty = col => colIsEmptyCache[col] != null ? colIsEmptyCache[col] : colIsEmptyCache[col] = map.map(it => it.filter((c, i) => i == col)[0]).every(it => it === '.')

const dist = (a, b) => {
  let d = 0;
  for (let x = Math.min(a[0], b[0]); x < Math.max(a[0], b[0]); x++) {
    d += colIsEmpty(x) ? factor : 1
  }
  for (let y = Math.min(a[1], b[1]); y < Math.max(a[1], b[1]); y++) {
    d += rowIsEmpty(y) ? factor : 1
  }
  return d
}

let sum = 0
for (let i = 0; i < galaxies.length; i++) {
  for (let j = i + 1; j < galaxies.length; j++) {
    const s = dist(galaxies[i], galaxies[j])
    sum += s
  }
}

console.log(sum)
