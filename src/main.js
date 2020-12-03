// External Imports ------------------------------------------------------------
const commandLineArgs = require('command-line-args')
const fetch = require('node-fetch')
const fs = require('fs')

let startTime = process.hrtime()
let { day, part, year } = commandLineArgs([
  { name: 'day', alias: 'd', type: Number },
  { name: 'part', alias: 'p', type: Number },
  { name: 'year', alias: 'y', type: Number }
])

// Elm App ---------------------------------------------------------------------
const { Elm } = require('./Main.elm')
const app = Elm.Main.init()

app.solve = input => app.ports.fromJs.send({ day, part, year, input })
app.ports.fromElm.subscribe(solution => {
  const endTime = process.hrtime(startTime)

  switch (solution.status) {
    case 'Ok':
      console.log(`The result for Advent of Code ${year} day ${day} part ${part} is:\n  ${solution.result}`)
      break
    case 'Error':
      console.log(`Error while solving Advent of Code ${year} day ${day} part ${part}:\n  ${solution.notice}`)
      break
  }

  console.info('\nExecution time: %ds %dms', endTime[0], endTime[1] / 1000000)
})

// Get Input Locally -----------------------------------------------------------
try {
  const input = require(`./data/${year}/${day}.json`)

  startTime = process.hrtime()
  app.solve(input)

// Fetch input from server -----------------------------------------------------
} catch (e) {
  const session = require('./data/session.json')
  const requestParams = {
    method: 'GET',
    headers: { Cookie: `session=${session}` }
  }

  fetch(`https://adventofcode.com/${year}/day/${day}/input`, requestParams)
    .then(res => res.text())
    .then(input => {
      fs.writeFile(`./data/${year}/${day}.json`, JSON.stringify(input), err => {
        if (err) console.log(err)
      })

      startTime = process.hrtime()
      app.solve(input)
    })
    .catch(console.log)
}
