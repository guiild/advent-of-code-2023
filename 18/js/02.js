const fs = require('fs')

let cur = { x: 0, y: 0 }
const dirs = ['R', 'D', 'L', 'U']

let corners = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  .trim()
  .split('\n')
  .map(it => {
    const [_, __, hex] = it.split(' ')
    const values = hex.split('').slice(2, hex.length - 1)
    return { dir: dirs[values[values.length - 1]], nb: parseInt(values.slice(0, values.length - 1).join(''), 16) }
  })
  .map(it => {
    if (it.dir == 'U') cur.y -= it.nb
    if (it.dir == 'D') cur.y += it.nb
    if (it.dir == 'L') cur.x -= it.nb
    if (it.dir == 'R') cur.x += it.nb
    return { x: cur.x, y: cur.y, l: it.nb }
  })

corners.push(corners[0])
let area = 0

for (let i = 0; i < corners.length - 1; i++) {
  const [a, b] = corners.slice(i, i + 2)
  area += a.x * b.y - b.x * a.y
}

console.log(area / 2 + corners.slice(1).map(it => it.l).reduce((a, b) => a + b, 0) / 2 + 1)
