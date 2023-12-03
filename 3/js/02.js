const fs = require('fs')

const schematics =
    fs.readFileSync('./input', {
        encoding: 'utf8',
        flag: 'r'
    })
    // out of habit, to take care of unwanted new lines in the end of file and stuff like that
    .trim()
    .split('\n')
    .map(it => it.split(''))

const height = schematics.length
const width = schematics[0].length


const isDigit = (char) => char >= '0' && char <= '9'

const isAdjacentToSymbol = (h, w, l) => {
    for (let i = Math.max(0, h - 1); i < Math.min(h + 2, height); i++) {
        for (let j = Math.max(w - 1, 0); j < Math.min(w + 1 + l, width); j++) {
            if (schematics[i][j] != '.' && !isDigit(schematics[i][j])) {
                return true
            }
        }
    }
    return false
}

const getPartNumbers = () => {
    const numbers = []
    let currentNumber = ''
    for (let i = 0; i < height; i++) {
        for (let j = 0; j < width; j++) {
            const c = schematics[i][j]
            if (isDigit(c)) {
                currentNumber += c
            } else if (currentNumber != '') {
                if (isAdjacentToSymbol(i, j - currentNumber.length, currentNumber.length)) {
                    numbers.push({
                        value: +currentNumber,
                        i,
                        j: j - currentNumber.length
                    })
                }
                currentNumber = ''
            }
        }
        if (currentNumber != '') {
            if (isAdjacentToSymbol(i, width - currentNumber.length, currentNumber.length)) {
                numbers.push({
                    value: +currentNumber,
                    i,
                    j: width - currentNumber.length
                })
            }
            currentNumber = ''
        }

    }
    return numbers;
}

const partNumbers = getPartNumbers()

const getAdjacentParts = (h, w) => {
    return partNumbers
        .filter(it => h >= it.i - 1 && h <= it.i + 1 && w >= it.j - 1 && w <= it.j + ("" + it.value).length)
}

const gearRatios = () => {
    const ratios = []
    for (let i = 0; i < height; i++) {
        for (let j = 0; j < width; j++) {
            if (schematics[i][j] === '*') {
                const adjacentParts = getAdjacentParts(i, j)
                if (adjacentParts.length == 2) {
                    ratios.push(adjacentParts[0].value * adjacentParts[1].value)
                }
            }
        }
    }
    return ratios
}

console.log(
    gearRatios()
    .reduce((a, b) => a + b, 0)
)