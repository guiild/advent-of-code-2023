const fs = require('fs')

const startsTheSame = (a, b) => a.length < b.length ? startsTheSame(b, a) : a.startsWith(b)
const reverse = s => s.split('').toReversed().join('')
const isSymBy = (line, idx) => startsTheSame(line.slice(idx), reverse(line.slice(0, idx)))
const transpose = array => array[0].map((_, colIndex) => array.map(row => row[colIndex]))

const score = map => {
  for (let i = 1; i < map[0].length; i++) {
    if (map.filter(it => !isSymBy(it, i)).length == 1) return i
  }
  const t = transpose(map.map(it => it.split(''))).map(it => it.join(''))
  for (let i = 1; i < t[0].length; i++) {
    if (t.filter(it => !isSymBy(it, i)).length == 1) return i * 100
  }
  return 0
}

console.log(fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n\n')
  .map(it => it.split('\n'))
  .map(score)
  .reduce((a, b) => a + b))
