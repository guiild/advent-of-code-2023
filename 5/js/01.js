const fs = require('fs')

const input = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  // out of habit, to take care of unwanted new lines in the end of file and stuff like that
  .trim()
  .split('\n\n')

const seeds = input[0].split(':')[1].split(' ').filter(it => it !== '').map(it => +it)

const parseMap = (str) => str.split('\n').slice(1).map(it => it.split(' ').map(it => +it).filter(it => !isNaN(it)))

const seedToSoil = parseMap(input[1])
const soilToFertilizer = parseMap(input[2])
const fertilizerToWater = parseMap(input[3])
const waterToLight = parseMap(input[4])
const lightToTemperature = parseMap(input[5])
const temperatureToHumidity = parseMap(input[6])
const humidityToLocation = parseMap(input[7])

const mapper = (element, map) => map.filter(it => it[1] <= element && element < it[1] + it[2]).map(it => it[0] + (element - it[1]))[0] ?? element

console.log(
  seeds
    .map(it => mapper(it, seedToSoil))
    .map(it => mapper(it, soilToFertilizer))
    .map(it => mapper(it, fertilizerToWater))
    .map(it => mapper(it, waterToLight))
    .map(it => mapper(it, lightToTemperature))
    .map(it => mapper(it, temperatureToHumidity))
    .map(it => mapper(it, humidityToLocation))
    .reduce((a, b) => a < b ? a : b, Number.POSITIVE_INFINITY)
)
