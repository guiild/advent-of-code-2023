const fs = require('fs')

const memoize = f => {
  const cache = {}
  return (...args) => {
    const k = args.map(it => JSON.stringify(it)).join(",")
    if (k in cache) return cache[k]
    return cache[k] = f(...args)
  }
}

const count = memoize((pattern, solution) => {
  if (pattern == "") return solution.length == 0 ? 1 : 0
  if (solution.length == 0) return pattern.includes('#') ? 0 : 1

  let c = 0

  if (pattern.startsWith('.') || pattern.startsWith('?'))
    c += count(pattern.slice(1), solution)

  if (pattern.startsWith('#') || pattern.startsWith('?'))
    if (pattern.length >= solution[0] && !pattern.slice(0, solution[0]).includes('.') && (solution[0] == pattern.length || pattern[solution[0]] != '#'))
      c += count(pattern.slice(solution[0] + 1), solution.slice(1))

  return c
})

console.log(fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n')
  .map(it => it.split(' '))
  .map(it => [it[0], it[1].split(',').map(that => +that)])
  .map(it => [Array(5).fill(it[0]).join('?'), Array(5).fill(it[1]).flat()])
  .map(it => count(...it))
  .reduce((a, b) => a + b))

