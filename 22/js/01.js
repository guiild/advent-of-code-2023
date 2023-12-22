const fs = require('fs')

class Brick {
  constructor(s, e) {
    this.s = { x: s[0], y: s[1], z: s[2] }
    this.e = { x: e[0], y: e[1], z: e[2] }
    this.supports = null
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
    return this.bottom() > 1 && bricks.every(b => b == this || !b.isJustBelow(this))
  }

  isSupportedBy() {
    if (this.supports == null)
      this.supports = bricks.filter(b => b != this && b.isJustBelow(this))
    return this.supports
  }

  canBeRemoved() {
    return bricks.filter(b => b != this && b.isSupportedBy().includes(this)).every(b => b.isSupportedBy().length > 1)
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


while (bricks.some(e => e.drop())) { }

console.log(bricks.filter(b => b.canBeRemoved()).length)
