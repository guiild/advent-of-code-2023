const fs = require('fs')

const map = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n')
  .map(it => it.split(''))

const L = map[0].length
const H = map.length

let sx = null
let sy = null
for (let y = 0; y < H; y++) {
  for (let x = 0; x < L; x++) {
    if (map[y][x] === 'S') {
      sx = x
      sy = y
      break
    }
  }
  if (sx != null) break
}

const pipeStartPos = (x, y) => {
  let connections = ''
  if (y - 1 >= 0 && ['|', 'F', '7'].includes(map[y - 1][x])) {
    connections += 'N'
  }
  if (y + 1 < H && ['|', 'J', 'L'].includes(map[y + 1][x])) {
    connections += 'S'
  }
  if (x - 1 >= 0 && ['-', 'L', 'F'].includes(map[y][x - 1])) {
    connections += 'W'
  }
  if (x + 1 < L && ['-', '7', 'J'].includes(map[y][x + 1])) {
    connections += 'E'
  }
  if (connections == 'NS') return '|'
  if (connections == 'NE') return 'L'
  if (connections == 'NW') return 'J'
  if (connections == 'SE') return 'F'
  if (connections == 'SW') return '7'
  if (connections == 'WE') return '-'
}

map[sy][sx] = pipeStartPos(sx, sy)

const neighbors = (x, y) => {
  const ret = []
  if (['J', '-', '7'].includes(map[y][x]))
    ret.push([x - 1, y])
  if (['F', '-', 'L'].includes(map[y][x]))
    ret.push([x + 1, y])
  if (['J', '|', 'L'].includes(map[y][x]))
    ret.push([x, y - 1])
  if (['F', '|', '7'].includes(map[y][x]))
    ret.push([x, y + 1])
  return ret
}

const visited = {}
const toVisit = [[sx, sy]]

while (toVisit.length > 0) {
  const node = toVisit.pop()
  visited[node.join(',')] = true
  toVisit.push(...neighbors(...node).filter(it => !visited[it.join(',')]))
}

console.log(Object.keys(visited).length / 2)
