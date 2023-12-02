const fs = require('fs')

const parseGame = (str) => {
    const [gameIdStr, setsStr] = str.split(': ')
    const gameId = +(gameIdStr.split(' ')[1])
    const sets = setsStr.split('; ')
        .map(it => it.split(', ')
            .flatMap(it => it.split(' '))
            .toReversed()
            .reduce((acc, e, i, arr) => {
                if (i % 2 == 0) {
                    acc[e] = +arr[i + 1]
                }
                return acc
            }, {})
        )

    return {gameId, sets}
}

const toMin = sets => {
  const ret = {red: Number.NEGATIVE_INFINITY, blue: Number.NEGATIVE_INFINITY, green: Number.NEGATIVE_INFINITY}
  sets.forEach(set => {
    for(const k in set) {
      ret[k] = Math.max(set[k], ret[k])
    }
  })
  return ret
}

console.log(
    fs.readFileSync('./input', {
        encoding: 'utf8',
        flag: 'r'
    })
    // out of habit, to take care of unwanted new lines in the end of file and stuff like that
    .trim()
    .split('\n')
    .map(parseGame)
    .map(it => toMin(it.sets))
    .map(it => it.red * it.blue * it.green)
    .reduce((a, b) => a + b, 0)
)
