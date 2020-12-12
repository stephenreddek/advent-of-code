module AdventOfCode2020.Day12 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Util.Parser

-- Types -----------------------------------------------------------------------
type Action
    = North Int
    | South Int
    | East Int
    | West Int
    | TurnLeft Angle
    | TurnRight Angle
    | Forward Int


type Angle
    = Angle Int

type Direction
    = Direction Int

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List Action)
parseInput input =
  Parser.run lineParser input
      |> Result.mapError Util.Parser.deadEndsToString

--

lineParser : Parser (List Action)
lineParser =
    Parser.loop [] (\xs ->
          Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
                |. Parser.end
            , Parser.succeed (\x -> Loop (x :: xs))
                |= Parser.oneOf
                    [ Parser.succeed (North)
                        |. Parser.token "N"
                        |= Parser.int
                    , Parser.succeed (South)
                        |. Parser.token "S"
                        |= Parser.int
                    , Parser.succeed (East)
                        |. Parser.token "E"
                        |= Parser.int
                    , Parser.succeed (West)
                        |. Parser.token "W"
                        |= Parser.int
                    , Parser.succeed (TurnLeft)
                        |. Parser.token "L"
                        |= (Parser.int |> Parser.map toAngle)
                    , Parser.succeed (TurnRight)
                        |. Parser.token "R"
                        |= (Parser.int |> Parser.map toAngle)
                    , Parser.succeed (Forward)
                        |. Parser.token "F"
                        |= Parser.int
                    ]
                |. Parser.token "\n"
            ]
       )


toAngle : Int -> Angle
toAngle =
    Angle


-- Functions -------------------------------------------------------------------
processDirections : (Int, Int, Direction) -> List Action -> (Int, Int, Direction)
processDirections (x, y, direction) actions =
    case actions of
        [] ->
            (x, y, direction)

        action :: remaining ->
            processDirections (move (x, y, direction) action) remaining


move : (Int, Int, Direction) -> Action -> (Int, Int, Direction)
move (x, y, direction) action =
    case action of
        North amount ->
            (x, y + amount, direction)

        South amount ->
            (x, y - amount, direction)

        East amount ->
            (x - amount, y, direction)

        West amount ->
            (x + amount, y, direction)

        TurnLeft angle ->
            (x, y, turn angle direction)

        TurnRight angle ->
            (x, y, turn (invert angle) direction)

        Forward amount ->
            case direction of
                Direction 0 ->
                    move (x, y, direction) (East amount)

                Direction 90 ->
                    move (x, y, direction) (North amount)

                Direction 180 ->
                    move (x, y, direction) (West amount)

                _ ->
                    move (x, y, direction) (South amount)

invert : Angle -> Angle
invert (Angle angle) =
    Angle (-angle)


turn : Angle -> Direction -> Direction
turn (Angle angle) (Direction direction) =
    standardizeAngle (Direction (angle + direction))


standardizeAngle : Direction -> Direction
standardizeAngle (Direction direction) =
    if direction > 0 then
        Direction (modBy 360 direction)

    else
        standardizeAngle (Direction (direction + 360))


manhattanDistance : (Int, Int, Direction) -> Int
manhattanDistance (x, y, _) =
    abs x + abs y


facingEast : Direction
facingEast =
    Direction 0

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (processDirections (0, 0, facingEast) >> manhattanDistance)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map (always 0)
