const fs = require('fs')

const [pathStr, graphStr] = input = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n\n')

const path = pathStr.split('')

const graph = graphStr.split('\n')
  .map(it => it.split(" = "))
  .reduce((acc, [node, neighbors]) => {
    const groups = [...neighbors.matchAll(/([\dA-Z]+), ([\dA-Z]+)/g)]
    acc[node] = { L: groups[0][1], R: groups[0][2] }
    return acc
  }, {})

let curr = Object.keys(graph).filter(k => k.endsWith('A'))
const lengths = curr.map(it => {
  let steps = 0
  while (!it.endsWith('Z')) {
    it = graph[it][path[steps++ % path.length]]
  }
  return steps;
})

const pgcd = (a, b) => b ? pgcd(b, a % b) : Math.abs(a)

const ppcm = (x, y) => (!x || !y) ? 0 : Math.abs((x * y) / pgcd(x, y))

console.log(lengths.reduce((a, b) => ppcm(a, b), 1))
