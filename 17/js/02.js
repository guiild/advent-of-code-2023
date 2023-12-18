const fs = require('fs')
const { PriorityQueue } = require('@datastructures-js/priority-queue')

const map = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  .trim()
  .split('\n')
  .map(it => it.split('').map(that => +that))

const node = (dist, x, y, dir, nbInDir) => {
  return { dist, x, y, dir, nbInDir, label: x + ',' + y + ',' + dir + ',' + nbInDir }
}

const neighbors = []

for (let y = 0; y < map.length; y++) {
  neighbors[y] = []
  for (let x = 0; x < map[0].length; x++) {
    neighbors[y][x] = []
    if (y > 0) neighbors[y][x].push(node(Number.POSITIVE_INFINITY, x, y - 1, 'N'))
    if (y < map.length - 1) neighbors[y][x].push(node(Number.POSITIVE_INFINITY, x, y + 1, 'S'))
    if (x > 0) neighbors[y][x].push(node(Number.POSITIVE_INFINITY, x - 1, y, 'W'))
    if (x < map[0].length - 1) neighbors[y][x].push(node(Number.POSITIVE_INFINITY, x + 1, y, 'E'))
  }
}

const toVisit = new PriorityQueue((a, b) => a.dist - b.dist)
toVisit.enqueue(node(0, 0, 0, null, 0))
const visited = new Set()

const opposites = { E: 'W', W: 'E', N: 'S', S: 'N' }

while (!toVisit.isEmpty()) {
  const n = toVisit.dequeue()

  if (visited.has(n.label)) continue

  if (n.nbInDir >= 4 && n.x == map[0].length - 1 && n.y == map.length - 1) {
    console.log(n)
    break
  }

  visited.add(n.label)

  neighbors[n.y][n.x].forEach(neighbor => {
    const next = node(n.dist + map[neighbor.y][neighbor.x], neighbor.x, neighbor.y, neighbor.dir, n.dir == neighbor.dir ? n.nbInDir + 1 : 1)
    if (next.nbInDir <= 10 && opposites[n.dir] != next.dir && (n.dir == null || next.dir == n.dir || n.nbInDir >= 4))
      toVisit.enqueue(next)
  })
}
