module AdventOfCode2020.Day01 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))

-- Types -----------------------------------------------------------------------


-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List Int)
parseInput input =
  Parser.run inputParser input
    |> Result.mapError Parser.deadEndsToString

--
inputParser : Parser (List Int)
inputParser =
  Parser.loop [] (\xs ->
    Parser.oneOf
      [ Parser.succeed (\x -> Loop (x :: xs))
          |= Parser.int
          |. Parser.spaces
      , Parser.succeed (\_ -> Done (List.reverse xs))
          |= Parser.end
      ]
 )


-- Functions -------------------------------------------------------------------
sumsTo2020 : (Int, Int) -> Bool
sumsTo2020 (x, y) =
    x + y == 2020

tripleSumsTo2020 : (Int, Int, Int) -> Bool
tripleSumsTo2020 (x, y, z) =
    x + y + z == 2020


multiplyPair : (Int, Int) -> Int
multiplyPair (x, y) =
    x * y

multiplyTriple : (Int, Int, Int) -> Int
multiplyTriple (x, y, z) =
    x * y * z


solvePart2 : Int -> Int
solvePart2 =
    always 0


uniqueTriples : List a -> List ( a, a, a )
uniqueTriples xs =
    case xs of
        [] ->
            []

        x :: xs_ ->
            List.map (\(y, z) -> ( x, y, z )) (List.Extra.uniquePairs xs_) ++ uniqueTriples xs_

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
  parseInput input
    |> Result.andThen (List.Extra.uniquePairs >> List.Extra.find sumsTo2020 >> Maybe.map multiplyPair >> Result.fromMaybe "Cannot find a pair" )

part2 : String -> Result String Int
part2 input =
  parseInput input
    |> Result.andThen (uniqueTriples >> List.Extra.find tripleSumsTo2020 >> Maybe.map multiplyTriple >> Result.fromMaybe "Cannot find a triple" )