const fs = require('fs')

const cards = fs.readFileSync('./input', {
        encoding: 'utf8',
        flag: 'r'
    })
    // out of habit, to take care of unwanted new lines in the end of file and stuff like that
    .trim()
    .split('\n')
    .map(it => it.split(': ')[1].split(' | ').map(that => that.split(' ').filter(c => c != '').map(c => +c)))
    .map((card, idx) => {
        return {
            idx,
            wins: card[1].filter(it => card[0].includes(it)).length
        }
    })


const deck = cards.map((e, i) => i)

for (let i = 0; i < deck.length; i++) {
    const card = cards[deck[i]]
    const wins = card.wins
    deck.push(...cards.slice(card.idx + 1, card.idx + 1 + wins).map(it => it.idx))
}


console.log(deck.length)