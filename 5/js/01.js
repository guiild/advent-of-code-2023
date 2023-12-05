const fs = require('fs')

const [seedsStr, ...mapsStr] = input = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n\n')

const seeds = seedsStr.split(':')[1].split(' ').filter(it => it !== '').map(it => +it)
const maps = mapsStr.map(((str) => str.split('\n').slice(1).map(it => it.split(' ').map(it => +it).filter(it => !isNaN(it)))))

const mapper = (element, map) => map.filter(it => it[1] <= element && element < it[1] + it[2]).map(it => it[0] + (element - it[1]))[0] ?? element

console.log(
  seeds.map(s => maps.reduce((acc, m) => mapper(acc, m), s))
    .reduce((a, b) => a < b ? a : b, Number.POSITIVE_INFINITY)
)
