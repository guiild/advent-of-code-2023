const fs = require('fs')

const [time, record] = input = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n')
  .map(it => it.split(':')[1].split(' ').filter(it => it != '').map(it => +it))

console.log(
  time.map((it, race) => {
    const d = it * it - 4 * record[race]
    const x1 = Math.floor((-it - Math.sqrt(d)) / -2)
    const x2 = Math.floor((-it + Math.sqrt(d)) / -2)

    return Math.abs(x1 - x2)
  }).reduce((a, b) => a * b, 1))
