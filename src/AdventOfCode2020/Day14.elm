module AdventOfCode2020.Day14 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import Binary exposing (Bits)
import Dict exposing (Dict)
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Util.Parser

-- Types -----------------------------------------------------------------------
type alias State =
    { mask : Mask
    , memory : Dict Int Int
    }


type alias Mask =
    { zeros : Bits
    , ones : Bits
    }


type Operation
    = SetMask Mask
    | SetValueAt Int Int

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List Operation)
parseInput input =
    Parser.run inputParser input
        |> Result.map (identity)
        |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (List Operation)
inputParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
              |. Parser.end
            , Parser.succeed (\op -> Loop (op :: xs))
                |= Parser.oneOf
                    [ Parser.succeed (\mask -> SetMask (toMask mask))
                        |. Parser.token "mask = "
                        |= Parser.getChompedString (Parser.chompUntil "\n")
                    , Parser.succeed (\address value -> SetValueAt address value)
                        |. Parser.token "mem["
                        |= Parser.int
                        |. Parser.token "] = "
                        |= Parser.int
                    ]
                |. Parser.token "\n"
            ]
   )


toMask : String -> Mask
toMask string =
    { ones = Binary.fromIntegers (List.map (\char -> if char == '1' then 1 else 0) (String.toList string))
    , zeros = Binary.fromIntegers (List.map (\char -> if char == '0' then 0 else 1) (String.toList string))
    }


-- Functions -------------------------------------------------------------------
initialState : State
initialState =
    State { zeros = Binary.empty, ones = Binary.empty } Dict.empty

runProgram : State -> List Operation -> State
runProgram memory operations =
    List.foldl doOperation memory operations

doOperation : Operation -> State -> State
doOperation operation state =
    case operation of
        SetMask mask ->
            { state | mask = mask }


        SetValueAt address value ->
            { state | memory = Dict.insert address (applyMask state.mask value) state.memory }


applyMask : Mask -> Int -> Int
applyMask mask value =
    (Binary.fromDecimal value)
        |> Binary.and mask.zeros
        |> Binary.or mask.ones
        |> Binary.toDecimal


sumValues : State -> Int
sumValues =
    .memory
        >> Dict.values
        >> List.sum

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (runProgram initialState >> sumValues)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map (always 0)
