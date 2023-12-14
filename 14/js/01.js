const fs = require('fs')

const roll = map => {
  let score = 0
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
        score += map.length - (y - dy[x])
      } else {
        score += map.length - y
      }
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

console.log(roll(map))
