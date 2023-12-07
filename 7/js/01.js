const fs = require('fs')

const hands = input = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n')
  .map(it => it.split(' '))

const type = hand => {
  const groups = hand.split('')
    .reduce((acc, e) => {
      if (acc[e] == null) {
        acc[e] = 0
      }
      acc[e]++
      return acc
    }, {})
  if (Object.values(groups).includes(5)) return 6
  if (Object.values(groups).includes(4)) return 5
  if (Object.values(groups).includes(3) && Object.values(groups).includes(2)) return 4
  if (Object.values(groups).includes(3)) return 3
  if (Object.values(groups).filter(it => it == 2).length == 2) return 2
  if (Object.values(groups).includes(2)) return 1
  return 0
}

const order = 'AKQJT98765432'

const comparator = (a, b) => {
  const ta = type(a)
  const tb = type(b)
  if (ta > tb) return -1
  if (ta < tb) return 1
  for (let i = 0; ; i++) {
    const oa = order.indexOf(a[i])
    const ob = order.indexOf(b[i])
    if (oa == ob) continue
    return oa - ob
  }
}

console.log(hands.sort((a, b) => comparator(a[0], b[0])).toReversed()
  .map((card, e) => (e + 1) * card[1])
  .reduce((a, b) => a + b, 0))
