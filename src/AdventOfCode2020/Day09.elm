module AdventOfCode2020.Day09 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Util.Parser

-- Types -----------------------------------------------------------------------


-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List Int)
parseInput input =
  Parser.run inputParser input
      |> Result.map (identity)
      |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (List Int)
inputParser =
  Parser.loop [] (\xs ->
      Parser.oneOf
        [ Parser.succeed (\x -> Loop (x :: xs))
            |= Parser.int
            |. Parser.token "\n"
        , Parser.succeed (\_ -> Done (List.reverse xs))
            |= Parser.end
        ]
   )

-- Functions -------------------------------------------------------------------
solvePart1 : List Int -> Int
solvePart1 allNumbers =
    let
        (initialPreamble, remaining) =
            List.Extra.splitAt 25 allNumbers
    in
    List.foldl (\toEvaluate (preamble, found) ->
            let
                nextPreamble =
                     (List.drop 1 preamble) ++ [toEvaluate]
            in
            if failsTest preamble toEvaluate then
                ( nextPreamble, Just (Maybe.withDefault toEvaluate found) )
            else
                ( nextPreamble, found )
        )
        (initialPreamble, Nothing)
        remaining
        |> Tuple.second
        |> Maybe.withDefault 0


failsTest : List Int -> Int -> Bool
failsTest preamble toEvaluate =
    List.Extra.uniquePairs preamble
        |> List.any (\(x, y) -> x + y == toEvaluate)
        |> not

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (solvePart1)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map (always 0)
