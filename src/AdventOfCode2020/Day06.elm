module AdventOfCode2020.Day06 exposing (part1, part2)

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
