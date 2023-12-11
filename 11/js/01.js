const fs = require('fs')

const transpose = matrix => matrix[0].map((col, i) => matrix.map(row => row[i]))
const doubleIfEmptyLines = line => line.every(it => it === '.') ? [line, line] : [line]

let map = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n')
  .map(it => it.split(''))
  .flatMap(doubleIfEmptyLines)

map = transpose(transpose(map).flatMap(doubleIfEmptyLines))

const galaxies = []
map.forEach((line, y) => {
  line.forEach((e, x) => {
    if (e !== '.') galaxies.push([x, y])
  })
})

const dist = (a, b) => {
  return Math.abs(a[0] - b[0]) + Math.abs(a[1] - b[1])
}

let sum = 0
for (let i = 0; i < galaxies.length; i++) {
  for (let j = i + 1; j < galaxies.length; j++) {
    const s = dist(galaxies[i], galaxies[j])
    sum += s
  }
}

console.log(sum)
