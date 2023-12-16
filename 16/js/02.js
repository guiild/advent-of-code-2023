const fs = require('fs')

const map = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  .trim()
  .split('\n')
  .map(it => it.split(''))

let beams = []
let visited = {}

const key = beam => `${beam.x},${beam.y}`

const visit = beam => {
  const k = key(beam)
  if (visited[k] == null) {
    visited[k] = {}
  }
  visited[k][beam.dir] = true
}

const hasBeenVisited = beam => {
  const k = key(beam)
  return visited[k] != null && visited[k][beam.dir]
}

const travelThrough = beam => {
  if (beam.dir == 'E') {
    return beam.x < map[0].length - 1 ? [{ ...beam, x: beam.x + 1 }] : []
  }
  if (beam.dir == 'S') {
    return beam.y < map.length - 1 ? [{ ...beam, y: beam.y + 1 }] : []
  }
  if (beam.dir == 'W') {
    return beam.x > 0 ? [{ ...beam, x: beam.x - 1 }] : []
  }
  if (beam.dir == 'N') {
    return beam.y > 0 ? [{ ...beam, y: beam.y - 1 }] : []
  }
}

const turn90DegClockwise = beam => {
  if (beam.dir == 'E') {
    return beam.y < map.length - 1 ? [{ ...beam, y: beam.y + 1, dir: 'S' }] : []
  }
  if (beam.dir == 'S') {
    return beam.x > 0 ? [{ ...beam, x: beam.x - 1, dir: 'W' }] : []
  }
  if (beam.dir == 'W') {
    return beam.y > 0 ? [{ ...beam, y: beam.y - 1, dir: 'N' }] : []
  }
  if (beam.dir == 'N') {
    return beam.x < map[0].length - 1 ? [{ ...beam, x: beam.x + 1, dir: 'E' }] : []
  }
}

const turn90DegAntiClockwise = beam => {
  if (beam.dir == 'W') {
    return beam.y < map.length - 1 ? [{ ...beam, y: beam.y + 1, dir: 'S' }] : []
  }
  if (beam.dir == 'N') {
    return beam.x > 0 ? [{ ...beam, x: beam.x - 1, dir: 'W' }] : []
  }
  if (beam.dir == 'E') {
    return beam.y > 0 ? [{ ...beam, y: beam.y - 1, dir: 'N' }] : []
  }
  if (beam.dir == 'S') {
    return beam.x < map[0].length - 1 ? [{ ...beam, x: beam.x + 1, dir: 'E' }] : []
  }
}

const travel = beam => {
  const cell = map[beam.y][beam.x]
  if (cell == '.') {
    return travelThrough(beam)
  } else if (cell == '-') {
    if (beam.dir == 'E' || beam.dir == 'W') {
      return travelThrough(beam)
    } else {
      return [...turn90DegAntiClockwise(beam), ...turn90DegClockwise(beam)]
    }
  } else if (cell == '|') {
    if (beam.dir == 'N' || beam.dir == 'S') {
      return travelThrough(beam)
    } else {
      return [...turn90DegAntiClockwise(beam), ...turn90DegClockwise(beam)]
    }
  } else if (cell == '/') {
    return beam.dir == 'N' || beam.dir == 'S' ? turn90DegClockwise(beam) : turn90DegAntiClockwise(beam)
  } else if (cell == '\\') {
    return beam.dir == 'E' || beam.dir == 'W' ? turn90DegClockwise(beam) : turn90DegAntiClockwise(beam)
  }
}

const run = () => {
  while (beams.length > 0) {
    const beam = beams.pop()
    if (hasBeenVisited(beam)) continue
    visit(beam)
    beams.push(...travel(beam))
  }
}

let theMax = Number.NEGATIVE_INFINITY
for (let x = 0; x < map[0].length; x++) {
  beams.push({ x, y: 0, dir: 'S' })
  run()
  theMax = Math.max(theMax, Object.keys(visited).length)
  visited = {}
  beams = []

  beams.push({ x, y: map.length - 1, dir: 'N' })
  run()
  theMax = Math.max(theMax, Object.keys(visited).length)
  visited = {}
  beams = []
}
for (let y = 0; y < map.length; y++) {
  beams.push({ x: 0, y, dir: 'E' })
  run()
  theMax = Math.max(theMax, Object.keys(visited).length)
  visited = {}
  beams = []

  beams.push({ x: map[0].length - 1, y, dir: 'W' })
  run()
  theMax = Math.max(theMax, Object.keys(visited).length)
  visited = {}
  beams = []
}

console.log(theMax)
