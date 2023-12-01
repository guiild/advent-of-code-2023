const fs = require('fs')

console.log(
    fs.readFileSync('./input', {
        encoding: 'utf8',
        flag: 'r'
    })
    // out of habit, to take care of unwanted new lines in the end of file and stuff like that
    .trim()
    .split('\n')
    .map(it => it
        .split('')
        .filter(char => char >= '0' && char <= '9')
    )
    .map(it => it[0] + it[it.length - 1])
    .map(char => +char)
    .reduce((a, b) => a + b, 0)
)