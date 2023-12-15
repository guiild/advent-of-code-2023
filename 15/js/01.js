const fs = require('fs')

const hash = str => str
  .split('')
  .map(it => it.charCodeAt(0))
  .reduce((h, code) => (h + code) * 17 % 256, 0)

console.log(fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  .trim()
  .split(',')
  .map(hash)
  .reduce((a, b) => a + b))
