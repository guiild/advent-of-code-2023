const fs = require('fs')

const computePrev = seq => {
  const seqs = [seq]
  while (seqs[seqs.length - 1].some(it => it != 0)) {
    seqs.push(seqs[seqs.length - 1]
      .toReversed()
      .flatMap((e, i, arr) => i == arr.length - 1 ? [] : e - arr[i + 1])
      .toReversed())
  }
  return seqs.toReversed().map(it => it[0]).reduce((acc, e) => {
    const r = e - acc[acc.length - 1]
    acc.push(r)
    return acc
  }, [0]).toReversed()[0]
}

console.log(fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n')
  .map(it => it.split(' ').map(that => +that))
  .map(computePrev)
  .reduce((a, b) => a + b))
