port module Main exposing (main)

-- Imports ---------------------------------------------------------------------
import AdventOfCode2020.Day01
import AdventOfCode2020.Day02
import AdventOfCode2020.Day03
import AdventOfCode2020.Day03
import AdventOfCode2020.Day04
import AdventOfCode2020.Day05
import AdventOfCode2020.Day06
import AdventOfCode2020.Day07
import AdventOfCode2020.Day08
import AdventOfCode2020.Day09
import AdventOfCode2020.Day10
import AdventOfCode2020.Day11
import AdventOfCode2020.Day12
import AdventOfCode2020.Day13
import AdventOfCode2020.Day14
import AdventOfCode2020.Day15
import AdventOfCode2020.Day16
import AdventOfCode2020.Day17
import AdventOfCode2020.Day18
import AdventOfCode2020.Day19
import AdventOfCode2020.Day20
import AdventOfCode2020.Day21
import AdventOfCode2020.Day22
import AdventOfCode2020.Day23
import AdventOfCode2019.Day01
-- << INJECT IMPORT >>

import Dict exposing (Dict)
import Json.Encode

-- Ports -----------------------------------------------------------------------
port fromJs : (Question -> msg) -> Sub msg
port fromElm : Solution -> Cmd msg

-- JavaScript solution ---------------------------------------------------------
type alias Solution
  = Json.Encode.Value

toSolution : Result String String -> Solution
toSolution result =
  case result of
    Ok n ->
      Json.Encode.object
        [ ("status", Json.Encode.string "Ok")
        , ("result", Json.Encode.string n)
        ]

    Err notice ->
      Json.Encode.object
        [ ("status", Json.Encode.string "Error")
        , ("notice", Json.Encode.string notice)
        ]

-- Main ------------------------------------------------------------------------
main : Program () () Question
main =
  Platform.worker
    { init = always ((), Cmd.none)
    , update = \q _ -> update q |> Cmd.map never |> Tuple.pair ()
    , subscriptions = subscriptions
    }

type alias Question =
  { day : Int
  , part : Int
  , year : Int
  , input : String
  }

answerMap : Dict (Int, Int, Int) (String -> Result String String)
answerMap =
  Dict.fromList
    [ Tuple.pair (1, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day01.part1)
    , Tuple.pair (1, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day01.part2)
    , Tuple.pair (2, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day02.part1)
    , Tuple.pair (2, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day02.part2)
    , Tuple.pair (3, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day03.part1)
    , Tuple.pair (3, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day03.part2)
    , Tuple.pair (4, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day04.part1)
    , Tuple.pair (4, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day04.part2)
    , Tuple.pair (5, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day05.part1)
    , Tuple.pair (5, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day05.part2)
    , Tuple.pair (6, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day06.part1)
    , Tuple.pair (6, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day06.part2)
    , Tuple.pair (7, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day07.part1)
    , Tuple.pair (7, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day07.part2)
    , Tuple.pair (8, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day08.part1)
    , Tuple.pair (8, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day08.part2)
    , Tuple.pair (9, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day09.part1)
    , Tuple.pair (9, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day09.part2)
    , Tuple.pair (10, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day10.part1)
    , Tuple.pair (10, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day10.part2)
    , Tuple.pair (11, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day11.part1)
    , Tuple.pair (11, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day11.part2)
    , Tuple.pair (12, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day12.part1)
    , Tuple.pair (12, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day12.part2)
    , Tuple.pair (13, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day13.part1)
    , Tuple.pair (13, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day13.part2)
    , Tuple.pair (14, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day14.part1)
    , Tuple.pair (14, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day14.part2)
    , Tuple.pair (15, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day15.part1)
    , Tuple.pair (15, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day15.part2)
    , Tuple.pair (16, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day16.part1)
    , Tuple.pair (16, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day16.part2)
    , Tuple.pair (17, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day17.part1)
    , Tuple.pair (17, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day17.part2)
    , Tuple.pair (18, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day18.part1)
    , Tuple.pair (18, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day18.part2)
    , Tuple.pair (19, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day19.part1)
    , Tuple.pair (19, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day19.part2)
    , Tuple.pair (20, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day20.part1)
    , Tuple.pair (20, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day20.part2)
    , Tuple.pair (21, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day21.part1)
    , Tuple.pair (21, 2, 2020) AdventOfCode2020.Day21.part2
    , Tuple.pair (22, 1, 2020) (Result.map String.fromInt << AdventOfCode2020.Day22.part1)
    , Tuple.pair (22, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day22.part2)
    , Tuple.pair (23, 1, 2020) AdventOfCode2020.Day23.part1
    , Tuple.pair (23, 2, 2020) (Result.map String.fromInt << AdventOfCode2020.Day23.part2)
    , Tuple.pair (1, 1, 2019) (Result.map String.fromInt << AdventOfCode2019.Day01.part1)
    , Tuple.pair (1, 2, 2019) (Result.map String.fromInt << AdventOfCode2019.Day01.part2)
    ] -- << INJECT SOLUTION >>

-- Update ----------------------------------------------------------------------
update : Question -> Cmd Never
update { day, part, year, input } =
  let
    errorMessage = 
      "I don't have a solution for day " ++ String.fromInt day
        ++ " part " ++ String.fromInt part
        ++ " year " ++ String.fromInt year
        ++ "!"
  in
  Dict.get (day, part, year) answerMap
    |> Result.fromMaybe errorMessage
    |> Result.andThen ((|>) input)
    |> toSolution
    |> fromElm

-- Subscriptions ---------------------------------------------------------------
subscriptions : () -> Sub Question
subscriptions _ =
  fromJs identity
