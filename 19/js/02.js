const fs = require('fs')

let [workflows] = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  .trim()
  .split('\n\n')

const toRule = str => {
  if (!str.includes(':')) {
    return { into: str }
  }
  const [test, into] = str.split(':')
  const variable = test[0]
  const op = test[1]
  const val = +test.substring(2)
  return { variable, op, val, into }
}

const workflowsByName = {}

workflows = workflows
  .split('\n')
  .map(it => {
    let [name, rules] = it.split('{')
    rules = rules
      .substring(0, rules.length - 1)
      .split(',')
      .map(toRule)
    const w = { name, rules }
    workflowsByName[name] = w
    return w
  })

const reduce = (rule, rating) => {
  const ret = cloneRating(rating)
  if (rule.op == null) return ret
  if (ret[rule.variable].length == 0) {
    return ret
  }
  if (rule.op == '>') {
    if (ret[rule.variable][0] < rule.val) {
      ret[rule.variable][0] = rule.val + 1
    } else {
      ret[rule.variable] = []
    }
  } else {
    if (ret[rule.variable][1] > rule.val) {
      ret[rule.variable][1] = rule.val - 1
    } else {
      ret[rule.variable] = []
    }
  }
  if (ret[rule.variable][0] >= ret[rule.variable][1]) {
    ret[rule.variable] = []
  }
  return ret
}

const cloneRating = rating => {
  const ret = {}
  ret.x = rating.x.map(it => it)
  ret.a = rating.a.map(it => it)
  ret.m = rating.m.map(it => it)
  ret.s = rating.s.map(it => it)
  return ret
}

const reverse = (rating, rule) => {
  if (rule.op == '>') return reduce({ op: '<', variable: rule.variable, val: rule.val + 1 }, rating)
  return reduce({ op: '>', variable: rule.variable, val: rule.val - 1 }, rating)
}

const apply = (workflow, rating) => {
  const ret = []
  let currentRating = cloneRating(rating)
  for (let rule of workflow.rules) {
    if (rule.op == null) {
      if (rule.into == 'A') {
        ret.push(currentRating)
        continue
      }
      if (rule.into == 'R') continue
      ret.push(...apply(workflowsByName[rule.into], currentRating))
      continue
    }
    const r = reduce(rule, currentRating)
    if (rule.into == 'A') {
      ret.push(r)
      currentRating = reverse(currentRating, rule)
      continue
    }
    if (rule.into == 'R') {
      currentRating = reverse(currentRating, rule)
      continue
    }
    ret.push(...apply(workflowsByName[rule.into], r))
    currentRating = reverse(currentRating, rule)
  }
  return ret
}

const initialRating = { x: [1, 4000], m: [1, 4000], a: [1, 4000], s: [1, 4000] }

const score = rating => {
  return (rating.x[1] - rating.x[0] + 1) * (rating.a[1] - rating.a[0] + 1) * (rating.m[1] - rating.m[0] + 1) * (rating.s[1] - rating.s[0] + 1)
}

console.log(apply(workflowsByName.in, initialRating).map(score).reduce((a, b) => a + b, 0))
