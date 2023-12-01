const fs = require('fs')

const lettersToNumber = {
    one: 1,
    two: 2,
    three: 3,
    four: 4,
    five: 5,
    six: 6,
    seven: 7,
    eight: 8,
    nine: 9,
}

// the start of the regex is a lookahead to manage spelled digits sharing a letter (e.g. "eightwo")
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Regular_expressions/Lookahead_assertion
const digitRegex = new RegExp("(?=(" + Object.keys(lettersToNumber).join("|") + "|\\d))", "g")

console.log(
    fs.readFileSync('./input', {
        encoding: 'utf8',
        flag: 'r'
    })
    // out of habit, to take care of unwanted new lines in the end of file and stuff like that
    .trim()
    .split('\n')
    .map(it => [...it.matchAll(digitRegex)]
        .map(it => lettersToNumber[it[1]] ?? +it[1])
    )
    .map(it => 10 * it[0] + it[it.length - 1])
    .reduce((a, b) => a + b, 0)
)