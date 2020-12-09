module AdventOfCode2020.Day09 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import List.Extra
import Maybe.Extra
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


solvePart2 : Int -> List Int -> Result String Int
solvePart2 sumToFind numbers =
    mapFind (\startingIndex -> contiguousNumbersThatSumTo sumToFind (List.drop startingIndex numbers)) (List.range 0 (List.length numbers - 1))
        |> Result.fromMaybe "cannot find contiguous numbers"
        |> Result.andThen sumSmallestAndLargest


mapFind : (a -> Maybe b) -> List a -> Maybe b
mapFind mapper list =
    case list of
        [] ->
            Nothing

        first :: rest ->
            let
                mapped =
                    mapper first
            in
            case mapped of
                Just value ->
                    Just value

                Nothing ->
                    mapFind mapper rest


foldUntilResult : (a -> b -> (b, Maybe c)) -> b -> List a -> Maybe c
foldUntilResult func acc list =
    case list of
        [] ->
            Nothing

        x :: xs ->
            case func x acc of
                ( _, Just value ) ->
                    Just value

                ( newAcc, Nothing ) ->
                    foldUntilResult func newAcc xs



contiguousNumbersThatSumTo : Int -> List Int -> Maybe (List Int)
contiguousNumbersThatSumTo sumToFind allNumbers =
    foldUntilResult (\nextNum previousNumbers ->
            let
                currentSum =
                    List.sum (nextNum :: previousNumbers)
            in
            if currentSum > sumToFind then
                (previousNumbers, Just Nothing)

            else if currentSum == sumToFind then
                (nextNum :: previousNumbers, Just (Just (nextNum :: previousNumbers)))

            else
                (nextNum :: previousNumbers, Nothing)
        )
        []
        allNumbers
            |> Maybe.andThen identity


rangeSumTo : Int -> List Int -> Bool
rangeSumTo sumToCheck ints =
    List.sum ints == sumToCheck


sumSmallestAndLargest : List Int -> Result String Int
sumSmallestAndLargest ints =
    let
        smallest =
            List.minimum ints

        largest =
            List.maximum ints
    in
    Maybe.map2 (+) smallest largest
        |> Result.fromMaybe "Cannot sum smallest and largest in empty list"

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (solvePart1)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.andThen (solvePart2 257342611)
