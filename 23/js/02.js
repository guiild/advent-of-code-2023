const fs = require('fs')

const map = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  .trim()
  .split('\n')
  .map(it => it.split('')
  )

const label = (x, y) => x + ',' + y
let start = null
let end = null
const graph = {}

for (let y = 0; y < map.length; y++) {
  for (let x = 0; x < map[0].length; x++) {
    if (map[y][x] == '#') continue
    if (y == 0 && map[y][x] == '.') start = [x, y]
    if (y == map.length - 1 && map[y][x] == '.') end = [x, y]
    const l = label(x, y)
    graph[l] = []
    if ((map?.[y - 1]?.[x] ?? '#') != '#') graph[l].push([x, y - 1])
    if ((map?.[y + 1]?.[x] ?? '#') != '#') graph[l].push([x, y + 1])
    if ((map?.[y]?.[x - 1] ?? '#') != '#') graph[l].push([x - 1, y])
    if ((map?.[y]?.[x + 1] ?? '#') != '#') graph[l].push([x + 1, y])
  }
}

const visited = new Set()
let currentPathLength = 0
let maxPathLength = 0

const traverse = node => {
  const l = label(...node)
  visited.add(l)
  currentPathLength++
  if (node[0] == end[0] && node[1] == end[1] && currentPathLength > maxPathLength) {
    maxPathLength = currentPathLength
  }
  else graph[l].filter(n => !visited.has(label(...n))).forEach(n => traverse(n))
  visited.delete(l)
  currentPathLength--
}

traverse(start)
console.log(maxPathLength - 1)
