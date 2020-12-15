module AdventOfCode2020.Day14 exposing (..)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import Binary exposing (Bits)
import Dict exposing (Dict)
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Util.Parser

-- Types -----------------------------------------------------------------------
type alias State =
    { mask : Mask
    , memory : Dict Int Int
    }


type alias Mask =
    { floating : List Int
    , ones : Bits
    }


type alias AddressMask =
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
    { floating = string
        |> String.reverse
        |> String.indexes "X"
    , ones = Binary.ensureSize 36 <| Binary.fromIntegers (List.map (\char -> if char == '1' then 1 else 0) (String.toList string))
    }


-- Functions -------------------------------------------------------------------
initialState : State
initialState =
    State { floating = [], ones = Binary.empty } Dict.empty

runProgram : State -> List Operation -> State
runProgram memory operations =
    List.foldl doOperation memory operations

doOperation : Operation -> State -> State
doOperation operation state =
    case operation of
        SetMask mask ->
            { state | mask = mask }

        SetValueAt address value ->
            let
                enumeratedAddresses =
                    addressesFromFloating state address
            in
            List.foldl (\addressOption -> setValue { address = addressOption, value = value } ) state enumeratedAddresses


addressesFromFloating : State -> Int -> List Int
addressesFromFloating state address =
    state.mask.floating
        |> generateFloatingAddressMasks
        |> List.map (applyAddressMask address state.mask)


generateFloatingAddressMasks : List Int -> List AddressMask
generateFloatingAddressMasks floatingIndexes =
    floatingIndexes
        |> List.Extra.subsequences
        |> List.map (\indexesThatShouldBeOnes -> createAddressMask { indexesThatShouldBeOnes = indexesThatShouldBeOnes, allFloatingAddresses = floatingIndexes })


createAddressMask : { indexesThatShouldBeOnes : List Int, allFloatingAddresses : List Int } -> AddressMask
createAddressMask { indexesThatShouldBeOnes, allFloatingAddresses } =
    let
        indexesThatShouldBeZeros =
            allFloatingAddresses
                |> List.filter (\index -> List.member index indexesThatShouldBeOnes |> not)
    in
    { ones =
        List.foldl (\index bitMask -> List.Extra.setAt index 1 bitMask ) (List.repeat 36 0) indexesThatShouldBeOnes
            |> List.reverse
            |> Binary.fromIntegers
            |> Binary.ensureSize 36
    , zeros =
        List.foldl (\index bitMask -> List.Extra.setAt index 0 bitMask ) (List.repeat 36 1) indexesThatShouldBeZeros
            |> List.reverse
            |> Binary.fromIntegers
            |> Binary.ensureSize 36
    }


setValue : { value : Int, address : Int} -> State -> State
setValue { value, address } state =
    { state | memory = Dict.insert address value state.memory }


applyAddressMask : Int -> Mask -> AddressMask -> Int
applyAddressMask int mask addressMask  =
    int
        |> Binary.fromDecimal
        |> Binary.ensureSize 36
        |> Binary.and addressMask.zeros
        |> Binary.or addressMask.ones
        |> Binary.or mask.ones
        |> Binary.toDecimal


applyOnesMask : Mask -> Int -> Int
applyOnesMask mask address =
    address
        |> Binary.fromDecimal
        |> Binary.ensureSize 36
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
        |> Result.map (runProgram initialState >> sumValues)


-- 2792825529595
-- 2710825158441
-- 3816594901962 -- right answer