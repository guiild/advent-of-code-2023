const fs = require('fs')

console.log(
    fs.readFileSync('./input', {
        encoding: 'utf8',
        flag: 'r'
    })
    // out of habit, to take care of unwanted new lines in the end of file and stuff like that
    .trim()
    .split('\n')
    .map(it => it.split(': ')[1].split(' | ').map(that => that.split(' ').filter(c => c != '').map(c => +c)))
    .map(it => Math.floor(Math.pow(2, it[1].filter(that => it[0].includes(that)).length - 1)))
    .reduce((a, b) => a + b, 0)
)