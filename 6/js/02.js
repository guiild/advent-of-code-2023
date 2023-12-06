const fs = require('fs')

const [time, record] = input = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n')
  .map(it => +it.split(':')[1].split(' ').filter(it => it != '').join(''))

const d = time * time - 4 * record
const x1 = Math.floor((-time - Math.sqrt(d)) / -2)
const x2 = Math.floor((-time + Math.sqrt(d)) / -2)

console.log(Math.abs(x1 - x2))
