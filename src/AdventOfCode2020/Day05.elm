module AdventOfCode2020.Day05 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))

-- Types -----------------------------------------------------------------------


-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List String)
parseInput input =
  Parser.run inputParser input
      |> Result.mapError Parser.deadEndsToString

--
inputParser : Parser (List String)
inputParser =
  Parser.loop [] (\xs ->
      Parser.oneOf
        [ Parser.succeed (\_ -> Done (List.reverse xs))
            |= Parser.end
        , Parser.succeed (\x -> Loop (x :: xs))
            |= Parser.getChompedString (Parser.chompUntil "\n")
            |. Parser.spaces
        ]
   )

-- Functions -------------------------------------------------------------------
calculateSeat : String -> Int
calculateSeat positionText =
    let
        rowPart =
            String.left 7 positionText

        columnPart =
            String.dropLeft 7 positionText
    in
    (calculateRow rowPart) * 8 + (calculateColumn columnPart)

calculateRow : String -> Int
calculateRow rowText =
    calculateRowHelp 0 128 rowText

calculateRowHelp : Int -> Int -> String -> Int
calculateRowHelp acc step rowText =
    case String.uncons rowText of
        Just ('F', rest) ->
            calculateRowHelp acc (step // 2) rest

        Just ('B', rest) ->
            calculateRowHelp (acc + (step // 2)) (step // 2) rest

        _ ->
            acc


calculateColumn : String -> Int
calculateColumn columnText =
    calculateColumnHelp 0 8 columnText


calculateColumnHelp : Int -> Int -> String -> Int
calculateColumnHelp acc step columnText =
    case String.uncons columnText of
        Just ('L', rest) ->
            calculateColumnHelp acc (step // 2) rest

        Just ('R', rest) ->
            calculateColumnHelp (acc + (step // 2)) (step // 2) rest

        _ ->
            acc


findHole : List Int -> Maybe Int
findHole spots =
    List.foldl (\position (prev, found) -> if position == prev + 2 then (position, Just (position - 1)) else (position, found) ) (1000,Nothing) spots
        |> Tuple.second

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (List.map calculateSeat >> List.maximum >> Maybe.withDefault 0)

part2 : String -> Result String Int
part2 input =
  parseInput input
    |> Result.map (List.map calculateSeat >> List.sort >> findHole >> Maybe.withDefault 0)
