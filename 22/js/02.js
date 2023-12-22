const fs = require('fs')

class Brick {
  constructor(s, e) {
    this.s = { x: s[0], y: s[1], z: s[2] }
    this.e = { x: e[0], y: e[1], z: e[2] }
    this.supports = null
  }

  clone() {
    return new Brick([this.s.x, this.s.y, this.s.z], [this.e.x, this.e.y, this.e.z])
  }

  top() {
    return Math.max(this.s.z, this.e.z)
  }

  bottom() {
    return Math.min(this.s.z, this.e.z)
  }

  isJustBelow(other) {
    return this.top() == other.bottom() - 1 &&
      this.s.x <= other.e.x && this.e.x >= other.s.x && this.s.y <= other.e.y && this.e.y >= other.s.y
  }

  drop() {
    let dropped = false
    while (this.canDrop()) {
      this.s.z--
      this.e.z--
      dropped = true
    }
    return dropped
  }

  canDrop() {
    return this.bottom() > 1 && newBricks.every(b => b == this || !b.isJustBelow(this))
  }

  isSupportedBy() {
    if (this.supports == null)
      this.supports = newBricks.filter(b => b != this && b.isJustBelow(this))
    return this.supports
  }

  canBeRemoved() {
    return newBricks.filter(b => b != this && b.isSupportedBy().includes(this)).every(b => b.isSupportedBy().length > 1)
  }
}

let bricks = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  .trim()
  .split('\n')
  .map(it =>
    new Brick(...it.split('~').map(that => that.split(',').map(e => +e)))
  )

let newBricks = bricks

while (newBricks.some(e => e.drop())) { }

let count = 0
for (let i = 0; i < bricks.length; i++) {
  newBricks = bricks.filter((_, idx) => idx != i).map(it => it.clone())
  const nb = new Set()
  let someDropped = true
  while (someDropped) {
    const droppers = newBricks.filter(it => it.drop())
    droppers.forEach(it => nb.add(it))
    if (droppers.length == 0) someDropped = false
  }
  count += nb.size
}

console.log(count)

