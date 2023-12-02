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

const maxValues = {red: 12, green: 13, blue: 14}

const isValidSet = set => Object.keys(set).every(k => set[k] <= maxValues[k])

console.log(
    fs.readFileSync('./input', {
        encoding: 'utf8',
        flag: 'r'
    })
    // out of habit, to take care of unwanted new lines in the end of file and stuff like that
    .trim()
    .split('\n')
    .map(parseGame)
    .filter(it => it.sets.every(isValidSet))
    .map(it => it.gameId)
    .reduce((a, b) => a + b, 0)
)
