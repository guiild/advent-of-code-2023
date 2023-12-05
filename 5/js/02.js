const fs = require('fs')

const [seedsStr, ...maps] = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n\n')

// transforms a string in the format "start1 nb1 start2 nb2" into a list of ranges like [[start1, start1 + nb1], [start2, start2 + nb2]]
const seeds = seedsStr.split(':')[1].split(' ').filter(it => it !== '').map(it => +it).filter(it => !isNaN(it))
  .map((s, idx, arr) => {
    if (idx % 2 == 0) return [s, s + arr[idx + 1]]
  })
  .filter(it => it != null)

const parseMap = (str) => str.split('\n').slice(1).map(it => it.split(' ').map(it => +it).filter(it => !isNaN(it)))

const [seedToSoil, soilToFertilizer, fertilizerToWater, waterToLight, lightToTemperature, temperatureToHumidity, humidityToLocation] = maps.map(parseMap)

const mapper = (element, map) => map.filter(it => it[0] <= element && element < it[0] + it[2]).map(it => it[1] + (element - it[0]))[0] ?? element

for (let location = 0; ; location++) {
  const s = [location]
    .map(it => mapper(it, humidityToLocation))
    .map(it => mapper(it, temperatureToHumidity))
    .map(it => mapper(it, lightToTemperature))
    .map(it => mapper(it, waterToLight))
    .map(it => mapper(it, fertilizerToWater))
    .map(it => mapper(it, soilToFertilizer))
    .map(it => mapper(it, seedToSoil))
    .filter(it => seeds.some(s => s[0] <= it && it <= s[1]))
  if (s.length != 0) {
    console.log(location)
    break
  }
}
