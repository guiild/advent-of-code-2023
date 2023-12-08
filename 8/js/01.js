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
    const groups = [...neighbors.matchAll(/([A-Z]+), ([A-Z]+)/g)]
    acc[node] = { L: groups[0][1], R: groups[0][2] }
    return acc
  }, {})

let steps = 0
let curr = 'AAA'
while (curr !== 'ZZZ') {
  curr = graph[curr][path[steps++ % path.length]]
}
console.log(steps)
