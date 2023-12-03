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
                    numbers.push(+currentNumber)
                }
                currentNumber = ''
            }
        }
        if (currentNumber != '') {
            if (isAdjacentToSymbol(i, width - currentNumber.length, currentNumber.length)) {
                numbers.push(+currentNumber)
            }
            currentNumber = ''
        }

    }
    return numbers;
}

console.log(
    getPartNumbers()
    .reduce((a, b) => a + b, 0)
)