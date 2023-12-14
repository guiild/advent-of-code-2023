const fs = require('fs')

const rollNorth = map => {
  let dy = map[0].map(_ => 0)
  for (let y = 0; y < map.length; y++) {
    for (let x = 0; x < map[0].length; x++) {
      if (map[y][x] == '#') {
        dy[x] = 0
      } else if (map[y][x] == '.') {
        dy[x]++
      } else if (dy[x] > 0) {
        map[y - dy[x]][x] = 'O'
        map[y][x] = '.'
      }
    }
  }
}

const rollWest = map => {
  let dx = map[0].map(_ => 0)
  for (let x = 0; x < map[0].length; x++) {
    for (let y = 0; y < map.length; y++) {
      if (map[y][x] == '#') {
        dx[y] = 0
      } else if (map[y][x] == '.') {
        dx[y]++
      } else if (dx[y] > 0) {
        map[y][x - dx[y]] = 'O'
        map[y][x] = '.'
      }
    }
  }
}

const rollEast = map => {
  let dx = map[0].map(_ => 0)
  for (let x = map[0].length - 1; x >= 0; x--) {
    for (let y = 0; y < map.length; y++) {
      if (map[y][x] == '#') {
        dx[y] = 0
      } else if (map[y][x] == '.') {
        dx[y]++
      } else if (dx[y] > 0) {
        map[y][x + dx[y]] = 'O'
        map[y][x] = '.'
      }
    }
  }
}

const rollSouth = map => {
  let dy = map[0].map(_ => 0)
  for (let y = map.length - 1; y >= 0; y--) {
    for (let x = 0; x < map[0].length; x++) {
      if (map[y][x] == '#') {
        dy[x] = 0
      } else if (map[y][x] == '.') {
        dy[x]++
      } else if (dy[x] > 0) {
        map[y + dy[x]][x] = 'O'
        map[y][x] = '.'
      }
    }
  }
}

const previousStates = []

const cycle = map => {
  const m0 = map.map(it => it.join('')).join('\n')
  if (previousStates.includes(m0)) {
    let idx = previousStates.indexOf(m0)
    idx = idx + ((1000000000 - idx) % (previousStates.length - idx))
    return score(previousStates[idx].split('\n').map(it => it.split('')))
  }
  previousStates.push(m0)
  rollNorth(map)
  rollWest(map)
  rollSouth(map)
  rollEast(map)
  return -1
}

const score = map => {
  let score = 0
  for (let y = 0; y < map.length; y++) {
    for (let x = 0; x < map[0].length; x++) {
      if (map[y][x] == 'O') score += map.length - y
    }
  }
  return score
}

const map = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n')
  .map(it => it.split(''))

let s = -1
for (let i = 0; i < 1000000000 && ((s = cycle(map)) == -1); i++);
console.log(s)
