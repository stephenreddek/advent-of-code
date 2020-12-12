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


type alias State =
    { shipPosition : (Int, Int)
    , waypointOffset : (Int, Int)
    }

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
    standardizeDirection (Direction (angle + direction))


standardizeDirection : Direction -> Direction
standardizeDirection (Direction direction) =
    if direction > 0 then
        Direction (modBy 360 direction)

    else
        standardizeDirection (Direction (direction + 360))


manhattanDistance : (Int, Int, Direction) -> Int
manhattanDistance (x, y, _) =
    abs x + abs y


facingEast : Direction
facingEast =
    Direction 0

-- Part2 Functions -------------------------------------------------------------
followPart2Directions : State -> List Action -> State
followPart2Directions state actions =
    case actions of
        [] ->
            state

        action :: remaining ->
            followPart2Directions (processPart2Action state action) remaining


processPart2Action : State -> Action -> State
processPart2Action state action =
    case action of
        North amount ->
            { shipPosition = state.shipPosition
            , waypointOffset = (Tuple.first state.waypointOffset, Tuple.second state.waypointOffset + amount)
            }

        South amount ->
            { shipPosition = state.shipPosition
            , waypointOffset = (Tuple.first state.waypointOffset, Tuple.second state.waypointOffset - amount)
            }

        East amount ->
            { shipPosition = state.shipPosition
            , waypointOffset = (Tuple.first state.waypointOffset + amount, Tuple.second state.waypointOffset)
            }

        West amount ->
            { shipPosition = state.shipPosition
            , waypointOffset = (Tuple.first state.waypointOffset - amount, Tuple.second state.waypointOffset)
            }

        TurnLeft angle ->
            rotateWaypoint angle state

        TurnRight angle ->
            rotateWaypoint (invert angle) state

        Forward amount ->
            let
                (shipX, shipY) =
                    state.shipPosition

                (waypointX, waypointY) =
                    state.waypointOffset
            in
            { shipPosition = (shipX + (amount * waypointX), shipY + (amount * waypointY))
            , waypointOffset = state.waypointOffset
            }


rotateWaypoint : Angle -> State -> State
rotateWaypoint angle state =
    let
        (x, y) =
            state.waypointOffset
    in
    case standardizeAngle angle of
        Angle 90 ->
            { shipPosition = state.shipPosition
            , waypointOffset = (-y, x)
            }

        Angle 180 ->
            { shipPosition = state.shipPosition
            , waypointOffset = (-x, -y)
            }

        Angle 270 ->
            { shipPosition = state.shipPosition
            , waypointOffset = (y, -x)
            }

        _ ->
            state


standardizeAngle : Angle -> Angle
standardizeAngle (Angle angle) =
    if angle > 0 then
        Angle (modBy 360 angle)

    else
        standardizeAngle (Angle (angle + 360))


manhattanDistanceForState : State -> Int
manhattanDistanceForState { shipPosition } =
    let
        (x, y) =
            shipPosition
    in
    abs x + abs y

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (processDirections (0, 0, facingEast) >> manhattanDistance)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map (followPart2Directions { shipPosition = (0,0), waypointOffset = (10, 1) } >> manhattanDistanceForState)
