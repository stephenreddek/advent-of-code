module AdventOfCode2019.Day02 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------

import Array exposing (Array)
import Basics.Extra as Basics
import List.Extra
import Parser exposing ((|.), (|=), Parser, Step(..), Trailing(..))
import Util.Parser



-- Types -----------------------------------------------------------------------
-- Input -----------------------------------------------------------------------
--


parseInput : String -> Result String (Array Int)
parseInput input =
    Parser.run inputParser input
        |> Result.map identity
        |> Result.map Array.fromList
        |> Result.mapError Util.Parser.deadEndsToString



--


inputParser : Parser (List Int)
inputParser =
    Parser.loop []
        (\xs ->
            Parser.int
                |> Parser.andThen
                    (\x ->
                        Parser.oneOf
                            [ Parser.succeed (Loop (x :: xs))
                                |. Parser.token ","
                            , Parser.succeed (Done (List.reverse (x :: xs)))
                                |. Parser.token "\n"
                            ]
                    )
        )



-- Functions -------------------------------------------------------------------


get3Parameters : Int -> Array Int -> Maybe ( Int, Int, Int )
get3Parameters instructionAddress program =
    Maybe.map3 (\a b c -> ( a, b, c ))
        (Array.get (instructionAddress + 1) program)
        (Array.get (instructionAddress + 2) program)
        (Array.get (instructionAddress + 3) program)


store : Int -> Int -> Array Int -> Array Int
store address value program =
    Array.set address value program


read : Int -> Array Int -> Maybe Int
read address program =
    Array.get address program


run : Int -> Array Int -> Result String ( Int, Array Int )
run instructionPointer program =
    case Array.get instructionPointer program of
        Just 1 ->
            case get3Parameters instructionPointer program of
                Just ( value1At, value2At, storeAt ) ->
                    case Maybe.map2 Tuple.pair (read value1At program) (read value2At program) of
                        Just ( value1, value2 ) ->
                            Ok ( instructionPointer + 4, store storeAt (value1 + value2) program )

                        Nothing ->
                            Err ("Unable to read values from " ++ String.fromInt value1At ++ " or " ++ String.fromInt value2At)

                Nothing ->
                    Err ("Unable to read parameters for instruction at " ++ String.fromInt instructionPointer)

        Just 2 ->
            case get3Parameters instructionPointer program of
                Just ( value1At, value2At, storeAt ) ->
                    case Maybe.map2 Tuple.pair (read value1At program) (read value2At program) of
                        Just ( value1, value2 ) ->
                            Ok ( instructionPointer + 4, store storeAt (value1 * value2) program )

                        Nothing ->
                            Err ("Unable to read values from " ++ String.fromInt value1At ++ " or " ++ String.fromInt value2At)

                Nothing ->
                    Err ("Unable to read parameters for instruction at " ++ String.fromInt instructionPointer)

        Just i ->
            Err ("Invalid OpCode" ++ String.fromInt i)

        Nothing ->
            Err "Invalid instruction address"


runToCompletion : Array Int -> Result String (Array Int)
runToCompletion program =
    runToCompletionHelp 0 program


runToCompletionHelp : Int -> Array Int -> Result String (Array Int)
runToCompletionHelp instructionPointer program =
    case Array.get instructionPointer program of
        Just 99 ->
            Ok program

        Nothing ->
            Err "Invalid instruction address"

        Just _ ->
            case run instructionPointer program of
                Ok ( nextIndex, resultingMemory ) ->
                    runToCompletionHelp nextIndex resultingMemory

                Err e ->
                    Err e


runWithInput : Int -> Int -> Array Int -> Result String Int
runWithInput noun verb program =
    program
        |> Array.set 1 noun
        |> Array.set 2 verb
        |> runToCompletion
        |> Result.andThen (Array.get 0 >> Result.fromMaybe "Unable to get the first position")


findInputToMatch : Int -> Array Int -> Result String ( Int, Int )
findInputToMatch toMatch program =
    let
        possibleNouns =
            List.range 0 100

        possibleVerbs =
            List.range 0 100

        possibleInputs =
            List.concatMap (\noun -> List.map (\verb -> ( noun, verb )) possibleVerbs) possibleNouns
    in
    List.Extra.find (\( noun, verb ) -> runWithInput noun verb program == Ok toMatch) possibleInputs
        |> Result.fromMaybe ("Unable to find an input that results in " ++ String.fromInt toMatch)



-- Solvers ---------------------------------------------------------------------


part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.andThen (runWithInput 12 2)


part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.andThen (findInputToMatch 19690720)
        |> Result.map (\( noun, verb ) -> noun * 100 + verb)
