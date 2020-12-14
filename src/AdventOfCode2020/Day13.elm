module AdventOfCode2020.Day13 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Util.Parser

-- Types -----------------------------------------------------------------------
type alias BusAndOffset =
    { busID : Int
    , offset : Int
    }

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (Int, List Int)
parseInput input =
  Parser.run inputParser input
      |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (Int, List Int)
inputParser =
    Parser.succeed Tuple.pair
        |= Parser.int
        |. Parser.token "\n"
        |=  Parser.loop [] (\xs ->
              Parser.oneOf
                [ Parser.succeed (Done (List.reverse xs))
                    |. Parser.end
                , Parser.succeed (Loop xs)
                    |. Parser.token "x"
                    |. Parser.oneOf
                        [ Parser.token ","
                        , Parser.token "\n"
                        ]
                , Parser.succeed (\x -> Loop (x :: xs))
                    |= Parser.int
                    |. Parser.oneOf
                        [ Parser.token ","
                        , Parser.token "\n"
                        ]
                ]
           )


parseInputForPart2 : String -> Result String (List BusAndOffset)
parseInputForPart2 =
    Parser.run inputParserForPart2
          >> Result.mapError Util.Parser.deadEndsToString


inputParserForPart2 : Parser (List BusAndOffset)
inputParserForPart2 =
    Parser.succeed identity
        |. Parser.chompUntil "\n"
        |. Parser.token "\n"
        |= parseBusAndOffset

parseBusAndOffset : Parser (List BusAndOffset)
parseBusAndOffset =
    Parser.loop (0, []) (\(index, xs) ->
      Parser.oneOf
        [ Parser.succeed (Done (List.reverse xs))
            |. Parser.end
        , Parser.succeed (Loop (index + 1, xs))
            |. Parser.token "x"
            |. Parser.oneOf
                [ Parser.token ","
                , Parser.token "\n"
                ]
        , Parser.succeed (\x -> Loop (index + 1, { offset = index, busID = x } :: xs))
            |= Parser.int
            |. Parser.oneOf
                [ Parser.token ","
                , Parser.token "\n"
                ]
        ]
    )

-- Functions -------------------------------------------------------------------
solvePart1 : (Int, List Int) -> Result String Int
solvePart1 (currentTimestamp, busIDs) =
    busIDs
        |> List.map (\busID -> nextStopForBus { now = currentTimestamp, busID = busID })
        |> List.sortBy .time
        |> List.head
        |> Maybe.map (\{time, busID} -> (time - currentTimestamp) * busID)
        |> Result.fromMaybe "No busses"


nextStopForBus : { now : Int, busID : Int } -> { time : Int, busID : Int }
nextStopForBus { now, busID } =
    { time = Basics.ceiling (toFloat now / toFloat busID) * busID, busID = busID }


solvePart2 : List BusAndOffset -> Int
solvePart2 busAndOffsets =
    progressivelyFindSolution busAndOffsets [] 0


progressivelyFindSolution : List BusAndOffset -> List BusAndOffset -> Int -> Int
progressivelyFindSolution stillToUse usedInCalculation timestamp =
    case stillToUse of
        [] ->
            timestamp

        x :: xs ->
            progressivelyFindSolution xs (x :: usedInCalculation) (nextWorkingTimestamp x usedInCalculation timestamp)

nextWorkingTimestamp : BusAndOffset -> List BusAndOffset -> Int -> Int
nextWorkingTimestamp current usedSoFar currentTimestamp =
    let
        step =
            (calculateStep usedSoFar)
    in
    iterateAndSolve step (current :: usedSoFar) currentTimestamp


calculateStep : List BusAndOffset -> Int
calculateStep busAndOffsets =
    List.foldl (\{ busID } acc -> acc * busID) 1 busAndOffsets


iterateAndSolve : Int -> List BusAndOffset -> Int -> Int
iterateAndSolve step busAndOffsets timestamp =
    if doesTimeWork busAndOffsets timestamp then
        timestamp
    else
        iterateAndSolve step busAndOffsets (timestamp + step)


doesTimeWork : List BusAndOffset -> Int -> Bool
doesTimeWork busAndOffsets timestamp =
    List.all (\{ busID, offset } -> modBy busID (timestamp + offset) == 0 ) busAndOffsets

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.andThen (solvePart1)

part2 : String -> Result String Int
part2 input =
    parseInputForPart2 input
        |> Result.map (solvePart2)
