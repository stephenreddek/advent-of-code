module AdventOfCode2020.Day11 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Array exposing (Array)
import Basics.Extra as Basics
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Util.Parser

-- Types -----------------------------------------------------------------------
type Spot
    = Empty
    | Occupied
    | Floor

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (Array (Array Spot))
parseInput input =
  Parser.run inputParser input
      |> Result.map (List.map Array.fromList >> Array.fromList)
      |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (List (List Spot))
inputParser =
  Parser.loop [] (\xs ->
      Parser.oneOf
        [ Parser.succeed (\_ -> Done (List.reverse xs))
            |= Parser.end
        , Parser.succeed (\x -> Loop (x :: xs))
            |= lineParser
        ]
   )


lineParser : Parser (List Spot)
lineParser =
    Parser.loop [] (\xs ->
          Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
                |. Parser.token "\n"
            , Parser.succeed (\x -> Loop (x :: xs))
                |= Parser.oneOf
                    [ (Parser.token "." |> Parser.map (\_ -> Floor))
                    , (Parser.token "L" |> Parser.map (\_ -> Empty))
                    , (Parser.token "#" |> Parser.map (\_ -> Occupied))
                    ]
            ]
       )


-- Functions -------------------------------------------------------------------
isSpotOccupied : (Int, Int) -> Array (Array Spot) -> Bool
isSpotOccupied (row, col) layout =
    Array.get row layout
        |> Maybe.andThen (Array.get col)
        |> Maybe.map ((==) Occupied)
        |> Maybe.withDefault False


countAdjacentOccupiedSeats : Int -> Int -> Array (Array Spot) -> Int
countAdjacentOccupiedSeats rowIndex colIndex layout =
    let
        adjacentSpots =
            [( rowIndex - 1, colIndex - 1 )
            , ( rowIndex - 1, colIndex )
            , ( rowIndex - 1, colIndex + 1 )
            , ( rowIndex, colIndex - 1 )
            , ( rowIndex, colIndex + 1 )
            , ( rowIndex + 1, colIndex - 1 )
            , ( rowIndex + 1, colIndex )
            , ( rowIndex + 1, colIndex + 1 )
            ]
    in
    List.Extra.count (\position -> isSpotOccupied position layout) adjacentSpots

applyRules : Array (Array Spot) -> Array (Array Spot)
applyRules rows =
    let
        applyRuleToSpot rowIndex colIndex spot =
            case spot of
                Occupied ->
                    if countAdjacentOccupiedSeats rowIndex colIndex rows >= 4 then
                        Empty
                    else
                        Occupied

                Empty ->
                    if countAdjacentOccupiedSeats rowIndex colIndex rows == 0 then
                        Occupied
                    else
                        Empty

                Floor ->
                    Floor
    in
    -- Array.foldl (\row count -> Array.foldl (\spot rowCount -> if spot == Occupied then rowCount + 1 else rowCount) count row) 0 layout
    Array.indexedMap (\rowIndex columns -> Array.indexedMap (\colIndex spot -> applyRuleToSpot rowIndex colIndex spot) columns) rows


applyRulesUntilStable : Array (Array Spot) -> Array (Array Spot)
applyRulesUntilStable layout =
    let
        afterApplication =
            applyRules layout
    in
    if areLayoutsEqual afterApplication layout then
        layout
    else
        applyRulesUntilStable afterApplication

areLayoutsEqual : Array (Array Spot) -> Array (Array Spot) -> Bool
areLayoutsEqual layout1 layout2 =
    layout1 == layout2


countOccupied : Array (Array Spot) -> Int
countOccupied layout =
    Array.foldl (\row count -> Array.foldl (\spot rowCount -> if spot == Occupied then rowCount + 1 else rowCount) count row) 0 layout


solvePart1 : Array (Array Spot) -> Int
solvePart1 =
    applyRulesUntilStable >> countOccupied

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (solvePart1)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map (always 0)
