const commandLineArgs = require('command-line-args')
const fs = require('fs')
const options = commandLineArgs([
  { name: 'day', alias: 'd', type: Number },
  { name: 'year', alias: 'y', type: Number }
])

const day = String(options.day).padStart(2, '0')
const year = options.year || 2019

const template = `module AdventOfCode${year}.Day${day} exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))

-- Types -----------------------------------------------------------------------


-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String Int
parseInput input =
  Err "I don't parse any input right now!"

--
inputParser : Parser ()
inputParser =
  Parser.succeed ()

-- Functions -------------------------------------------------------------------


-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
  parseInput input

part2 : String -> Result String Int
part2 input =
  parseInput input
`

// -----------------------------------------------------------------------------
fs.writeFile(`src/AdventOfCode${year}/Day${day}.elm`, template, err => {
  if (err) throw err
})

fs.readFile('src/Main.elm', (err, data) => {
  if (err) throw err

  const code = data.toString().split('\n')

  const injectionImportIndex = code.findIndex(line => line.includes(`<< INJECT ${year} IMPORT >>`))
  const injectionImport = [
    `import AdventOfCode${year}.Day${day}`
  ]

  const injectionCodeIndex = code.findIndex(line => line.includes(`<< INJECT ${year} SOLUTION >>`))
  const injectionCode = [
    `    , Tuple.pair (${options.day}, 1, ${year}) AdventOfCode${year}.Day${day}.part1`,
    `    , Tuple.pair (${options.day}, 2, ${year}) AdventOfCode${year}.Day${day}.part2`
  ]

  code.splice(injectionImportIndex, 0, ...injectionImport)
  code.splice(injectionCodeIndex + 1, 0, ...injectionCode)

  fs.writeFile('src/Main.elm', code.join('\n'), err => {
    if (err) throw err
  })
})
