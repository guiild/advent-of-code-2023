const fs = require('fs')

let [workflows, ratings] = fs.readFileSync('./input', {
  encoding: 'utf8',
  flag: 'r'
})
  .trim()
  .split('\n\n')

ratings = ratings
  .split('\n')
  .map(it => it.replaceAll('=', '":'))
  .map(it => it.replaceAll('{', '{"'))
  .map(it => it.replaceAll(',', ',"'))
  .map(JSON.parse)
  .map(it => {
    return { ...it, score: it.a + it.m + it.x + it.s }
  })

const toRule = str => {
  if (!str.includes(':')) {
    return { into: str }
  }
  const [test, into] = str.split(':')
  const variable = test[0]
  const op = test[1]
  const val = +test.substring(2)
  const predicate = rating => {
    if (op == '>') return rating[variable] > val
    return rating[variable] < val
  }
  return { predicate, into }
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

const nextOrFinish = (rating, workflowNameOrStatus) => {
  if (workflowsByName[workflowNameOrStatus] == null) {
    return workflowNameOrStatus
  }
  return apply(rating, workflowsByName[workflowNameOrStatus])
}

const apply = (rating, workflow) => {
  for (let rule of workflow.rules) {
    if (rule.predicate != null) {
      if (rule.predicate(rating)) {
        return nextOrFinish(rating, rule.into)
      }
      continue
    }
    return nextOrFinish(rating, rule.into)
  }
}

console.log(
  ratings.map(it => [it, apply(it, workflowsByName['in'])])
    .filter(it => it[1] == 'A')
    .map(it => it[0].score)
    .reduce((a, b) => a + b, 0)
)
