module AdventOfCode2020.Day10 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Array exposing (Array)
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
solvePart1 adapters =
    let
        adaptersInOrder =
            List.sort adapters

        jumps =
            List.foldl (\adapter (prev, acc) -> (adapter, (adapter - prev) :: acc)) (0, [3]) adaptersInOrder
                |> Tuple.second

        jumpsOf1 =
            List.Extra.count ((==) 1) jumps

        jumpsOf3 =
            List.Extra.count ((==) 3) jumps
    in
    jumpsOf1 * jumpsOf3


solvePart2 : List Int -> Result String Int
solvePart2 adapters =
    let
        adaptersInOrder =
            List.sort adapters

        finalValue =
            Maybe.withDefault 0 (List.Extra.last adaptersInOrder)
    in
    combinations finalValue adaptersInOrder


combinations : Int -> List Int -> Result String Int
combinations finalValue adaptersInOrder =
    let
        iter : Int -> Array Int -> Array Int
        iter value solutionsSoFar =
            case getCombinationForValue finalValue solutionsSoFar value of
                0 ->
                    solutionsSoFar

                x ->
                    Array.set value x solutionsSoFar

    in
    List.foldr iter (Array.initialize (finalValue + 1) (always 0)) (0 :: adaptersInOrder)
        |> Array.get 0
        |> Result.fromMaybe "No value at index 0"


getCombinationForValue : Int -> Array Int -> Int -> Int
getCombinationForValue finalValue solutions value =
    if value == finalValue then
        1
    else
        Maybe.withDefault 0 (Array.get (value + 1) solutions)
            +  Maybe.withDefault 0 (Array.get (value + 2) solutions)
            + Maybe.withDefault 0 (Array.get (value + 3) solutions)


-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (solvePart1)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.andThen (solvePart2)
