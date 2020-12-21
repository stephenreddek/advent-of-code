module Day20 exposing (..)

import Binary
import Dict
import Expect exposing (Expectation)
import Fuzz exposing (Fuzzer, int, list, string)
import Parser
import Tagged
import Tagged.Dict exposing (TaggedDict)
import Test exposing (..)
import AdventOfCode2020.Day20 exposing (..)
import Util.Parser

part1 : Test
part1 =
    skip <| describe "part 1"
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
        , describe "flip number"
            [ test "back through" <|
                \_ -> Expect.equal 823 (Tagged.untag (flipNumber (flipNumber (Tagged.tag 27))))
            ]
        ]


image2797 : Image
image2797 =
    """Tile 2797:
.#...#...#
##........
#...#.....
#.#..#...#
##..#.....
#.....#..#
..#......#
..........
#......#..
...######.

"""
    |> Parser.run imageParser
    |> \image ->
        case image of
            Ok i ->
                i
            Err err ->
                Debug.todo (Util.Parser.deadEndsToString err)


part2 : Test
part2 =
    skip <| describe "part 2"
        [
        --describe "parser"
        --    [ test "with 2797" <|
        --        \_ ->
        --        let
        --            expected : Image
        --            expected =
        --                { id = Tagged.tag 2797
        --                --, pixels : Array (Array Bool)
        --                , numericRepresentation =
        --                    ({ top : SideValue
        --                     , bottom : SideValue
        --                     , left : SideValue
        --                     , right : SideValue
        --                     , asSet : TaggedSet SideValueTag Int
        --                     }
        --                    ,{ top : SideValue
        --                     , bottom : SideValue
        --                     , left : SideValue
        --                     , right : SideValue
        --                     , asSet : TaggedSet SideValueTag Int
        --                     })
        --                }
        --
        --        in
        --        Expect.equal expected image2797
        --    ]
        --,
        describe "makeIntoStartingImage"
            [ test "with 2797" <|
                \_ ->
                let
                    expected : Result String Matched
                    expected =
                        Ok { image = image2797
                        , side = Tuple.first image2797.numericRepresentation
                        --, oriented = image2797.pixels
                        }

                    matchingSquares : TaggedDict SideValueTag Int ImageId
                    matchingSquares =
                        Tagged.Dict.empty
                            |> Tagged.Dict.insert (Tuple.first image2797.numericRepresentation).bottom (Tagged.tag 1)
                            |> Tagged.Dict.insert (Tuple.first image2797.numericRepresentation).left (Tagged.tag 2)


                    actual : Result String Matched
                    actual =
                        makeIntoStartingImage image2797 matchingSquares
                in
                Expect.equal expected actual
            ]
        ]
