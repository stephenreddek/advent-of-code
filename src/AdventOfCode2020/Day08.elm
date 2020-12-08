module AdventOfCode2020.Day08 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Array exposing (Array)
import Basics.Extra as Basics
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Set exposing (Set)
import Util.Parser

-- Types -----------------------------------------------------------------------
type alias Instruction =
    ( Op, Int )


type Op
    = NoOp
    | Acc
    | Jmp


type alias State =
    { accumulator : Int
    , instructionNumber : Int
    }

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (Array Instruction)
parseInput input =
  Parser.run inputParser input
      |> Result.map (Array.fromList)
      |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (List Instruction)
inputParser =
  Parser.loop [] (\xs ->
      Parser.oneOf
        [ Parser.succeed (\op operand -> Loop ((op, operand) :: xs))
            |= (Parser.getChompedString (Parser.chompUntil " ") |> Parser.andThen opFromString)
            |. Parser.spaces
            |= intWithSign
            |. Parser.token "\n"
        , Parser.succeed (\_ -> Done (List.reverse xs))
            |= Parser.end
        ]
   )


intWithSign : Parser Int
intWithSign =
    Parser.succeed (\signMultiplier value -> signMultiplier * value)
        |= Parser.oneOf
            [ Parser.succeed (-1)
                |. Parser.token "-"
            , Parser.succeed 1
                |. Parser.token "+"
            ]
        |= Parser.int

opFromString : String -> Parser Op
opFromString string =
    case string of
        "nop" ->
            Parser.succeed NoOp

        "acc" ->
            Parser.succeed Acc

        "jmp" ->
            Parser.succeed Jmp

        op ->
            Parser.problem (op ++ " is not a valid op code")

-- Functions -------------------------------------------------------------------
accValueBeforeLoop : Array Instruction -> Int
accValueBeforeLoop instructions =
    runUntilLoop instructions Set.empty { accumulator = 0, instructionNumber = 0 }
        |> .accumulator


runUntilLoop : Array Instruction -> Set Int -> State -> State
runUntilLoop instructions seenInstructions state =
    let
        newState =
            run instructions state
    in
    if Set.member newState.instructionNumber seenInstructions then
        state
    else
        runUntilLoop instructions (Set.insert newState.instructionNumber seenInstructions) newState

run : Array Instruction -> State -> State
run instructions { accumulator, instructionNumber } =
    case Array.get instructionNumber instructions of
        Just (Acc, change) ->
            { accumulator = accumulator + change, instructionNumber = instructionNumber + 1 }

        Just (Jmp, change) ->
            { accumulator = accumulator, instructionNumber = instructionNumber + change }

        Just (NoOp, _) ->
            { accumulator = accumulator, instructionNumber = instructionNumber + 1 }

        Nothing ->
            { accumulator = accumulator, instructionNumber = instructionNumber }


solvePart2 : Array Instruction -> Int
solvePart2 instructions =
    let
        initialState =
            { accumulator = 0, instructionNumber = 0 }

        possiblePrograms =
            List.Extra.indexedFoldl (\index (operation, operand) acc ->
                    case operation of
                        Acc ->
                            acc

                        Jmp ->
                            (Array.set index (NoOp, operand) instructions) :: acc

                        NoOp ->
                            (Array.set index (Jmp, operand) instructions) :: acc
                )
                []
                (Array.toList instructions)

        terminatesNormally program =
            runUntilLoopOrTermination program Set.empty initialState
                |> Tuple.second

    in
    List.Extra.find terminatesNormally possiblePrograms
        |> Maybe.map (\program -> runUntilLoopOrTermination program Set.empty initialState)
        |> Maybe.map (\(state, _) -> state.accumulator)
        |> Maybe.withDefault -1

runUntilLoopOrTermination :  Array Instruction -> Set Int -> State -> (State, Bool)
runUntilLoopOrTermination instructions seenInstructions state =
    let
        newState =
            run instructions state
    in
    if Set.member newState.instructionNumber seenInstructions then
        (state, False)
    else if newState.instructionNumber == Array.length instructions then
        (newState, True)
    else
        runUntilLoopOrTermination instructions (Set.insert newState.instructionNumber seenInstructions) newState


-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (accValueBeforeLoop)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map solvePart2
