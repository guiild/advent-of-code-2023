const fs = require('fs')

const hash = str => str
  .split('')
  .map(it => it.charCodeAt(0))
  .reduce((h, code) => (h + code) * 17 % 256, 0)

const toLens = str => str.includes('=') ? ['=', ...str.split('=')] : ['-', ...str.split('-')]

const addToHashmap = (hashmap, lens) => {
  const h = hash(lens[1])
  if (lens[0] == '-') {
    hashmap[h] = hashmap[h].filter(it => it[1] != lens[1])
  } else {
    const l = hashmap[h].map((e, i) => [e, i]).filter(it => it[0][1] == lens[1])
    if (l.length == 0) {
      hashmap[h].push(lens)
    } else {
      hashmap[h].splice(l[0][1], 1, lens)
    }
  }
  return hashmap
}

const score = (bucket, i) => bucket.reduce((s, el, idx) => s + (1 + i) * (idx + 1) * el[2], 0)

console.log(fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  .trim()
  .split(',')
  .map(toLens)
  .reduce((hashmap, e) => addToHashmap(hashmap, e), Array(256).fill().map(() => []))
  .reduce((acc, e, i) => acc + score(e, i), 0)
)
