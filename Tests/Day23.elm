module Day23 exposing (..)


import Expect exposing (Expectation)
import Fuzz exposing (Fuzzer, int, list, string)
import Parser
import Tagged
import Tagged.Dict exposing (TaggedDict)
import Test exposing (..)
import AdventOfCode2020.Day23 exposing (..)
import Util.Parser


rollingListTest : Test
rollingListTest =
    describe "RollingList"
        [ describe "roll"
            [ test "that it moves to the next" <|
                \_ ->
                    Expect.equal (Just 3) ((fromList [1,2,3,4,5] |> Maybe.map (roll >> roll)) |> Maybe.map .current)
            , test "that it can wrap" <|
                \_ ->
                    Expect.equal (Just 2) ((fromList [1,2,3,4,5] |> Maybe.map (roll >> roll >> roll >> roll >> roll >> roll)) |> Maybe.map .current)
            , test "that it can really wrap" <|
                \_ ->
                    Expect.equal (Just 3) ((fromList [1,2,3,4,5] |> Maybe.map (roll >> roll >> roll >> roll >> roll >> roll >> roll >> roll >> roll >> roll >> roll >> roll)) |> Maybe.map .current)
            ]
        ]