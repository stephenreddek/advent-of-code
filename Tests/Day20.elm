module Day20 exposing (..)

import Binary
import Dict
import Expect exposing (Expectation)
import Fuzz exposing (Fuzzer, int, list, string)
import Test exposing (..)
import AdventOfCode2020.Day20 exposing (..)

part1 : Test
part1 =
    describe "part 1"
        [ describe "swapIndex"
            ([ ((1,8), (1, 1))
             , ((1,1), (8, 1))
             , ((2,1), (8, 2))
             , ((9,9), (0, 9))
             , ((4,4), (5, 4))
            ]
                |> List.map (\(input, expectation) ->
                    test ("works for " ++ String.fromInt (Tuple.first input) ++ ", " ++ String.fromInt (Tuple.second input)) <|
                        \_ -> Expect.equal (sourceIndexForRotation input) expectation
                )
            )
        ]
