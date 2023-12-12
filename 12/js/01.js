const fs = require('fs')

const isValid = (test, solution) => {
  const t = test.split('.').filter(it => it != '').map(it => it.length)
  return solution.length == t.length && solution.every((nb, idx) => nb == t[idx])
}

const count = (pattern, solution) => {
  pattern = pattern.split('')
  const pos = pattern.map((e, i) => [e, i]).filter(it => it[0] == '?').map(it => it[1])
  const nb = pos.length
  // I never know how to generate permutations, so I use the permutations of the
  // binary representation of a number ^^'
  let i = Math.pow(2, nb)
  let nbSol = 0
  while (i-- > 0) {
    let test = pattern.map((e, idx) => {
      if (!pos.includes(idx)) return e
      return i & Math.pow(2, pos.indexOf(idx)) ? '#' : '.'
    }).join('')
    if (isValid(test, solution)) {
      nbSol++
    }
  }
  return nbSol
}

console.log(fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n')
  .map(it => it.split(' '))
  .map(it => [it[0], it[1].split(',').map(that => +that)])
  .map(it => count(...it))
  .reduce((a, b) => a + b))

