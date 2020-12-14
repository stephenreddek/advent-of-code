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
        |> String.toList
        |> List.reverse
        |> List.Extra.indexedFoldr (\index char acc ->
                if char == 'X' then
                    index :: acc
                else
                    acc
            ) []
    , ones = Binary.fromIntegers (List.map (\char -> if char == '1' then 1 else 0) (String.toList string))
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
    let
        addressWithOnesApplied =
            applyOnesMask state.mask address
    in
    state.mask.floating
        |> generateFloatingAddressMasks
        |> List.map (applyAddressMask addressWithOnesApplied)


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
        List.repeat 36 0
            |> List.indexedMap (\index _ -> if List.member index indexesThatShouldBeOnes then 1 else 0)
            |> List.reverse
            |> Binary.fromIntegers
    , zeros =
        List.repeat 36 0
            |> List.indexedMap (\index _ -> if List.member index indexesThatShouldBeZeros then 0 else 1)
            |> List.reverse
            |> Binary.fromIntegers
    }
-- 2710825158441

setValue : { value : Int, address : Int} -> State -> State
setValue { value, address } state =
    { state | memory = Dict.insert address value state.memory }


applyAddressMask : Int -> AddressMask -> Int
applyAddressMask int addressMask =
    int
        |> Binary.fromDecimal
        |> Binary.ensureSize 36
        |> Binary.or addressMask.ones
        |> Binary.and addressMask.zeros
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
