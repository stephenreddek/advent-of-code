module AdventOfCode2020.Day23 exposing (..)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Util.Parser

-- Types -----------------------------------------------------------------------
type alias RollingList a =
    { before : List a
    , current : a
    , after : List a
    }

roll : RollingList a -> RollingList a
roll rollingList =
    case rollingList.after of
        [] ->
            case List.reverse rollingList.before of
                head :: tail ->
                    { before = [rollingList.current], current = head, after = tail }

                [] ->
                    rollingList

        element :: tail ->
            { before = rollingList.current :: rollingList.before, current = element, after = tail }


rollTo : Int -> RollingList Int -> RollingList Int
rollTo int rollingList =
    if rollingList.current == int then
        rollingList
    else
        rollTo int (roll rollingList)


unroll : RollingList a -> RollingList a
unroll rollingList =
    unroll rollingList


insertAfter : List a -> RollingList a -> RollingList a
insertAfter list rollingList =
    { before = rollingList.before, current = rollingList.current, after = list ++ rollingList.after }


push : a -> RollingList a -> RollingList a
push item rollingList =
    { before = rollingList.before, current = item, after = rollingList.current :: rollingList.after }


pop : RollingList a -> (a, Maybe (RollingList a))
pop rollingList =
    case rollingList.after of
        [] ->
            case List.reverse rollingList.before of
                head :: tail ->
                    (rollingList.current, Just { before = [], current = head, after = tail })

                [] ->
                    (rollingList.current, Nothing)

        element :: tail ->
            (rollingList.current, Just { before = rollingList.before, current = element, after = tail })


pop3 : RollingList a -> Maybe ((a, a, a), (RollingList a))
pop3 rollingList =
    case pop rollingList of
        (item1, Just rest1) ->
            case pop rest1 of
                (item2, Just rest2) ->
                    case pop rest2 of
                        (item3, Just rest3) ->
                            Just ((item1, item2, item3), rest3)

                        _ ->
                            Nothing

                _ ->
                    Nothing
        _ ->
            Nothing


toList : RollingList a -> List a
toList rollingList =
    rollingList.current :: rollingList.after ++ List.reverse rollingList.before

fromList : List a -> Maybe (RollingList a)
fromList list =
    case list of
        current :: after ->
            Just { before = [], current = current, after = after }

        _ ->
            Nothing


-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (RollingList Int)
parseInput input =
    Parser.run inputParser input
        |> Result.map (identity)
        |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (RollingList Int)
inputParser =
    Parser.getChompedString (Parser.chompUntilEndOr "\n")
        |> Parser.andThen (String.toList >> List.filterMap (String.fromChar >> String.toInt) >> fromList >> Maybe.map Parser.succeed >> Maybe.withDefault (Parser.problem "need at least one"))

-- Functions -------------------------------------------------------------------
simulateRounds : Int -> RollingList Int -> Result String (RollingList Int)
simulateRounds n rollingList =
    List.range 1 n
        |> List.foldl (\_ state -> Result.andThen simulateRound state) (Ok rollingList)


simulateRound : RollingList Int -> Result String (RollingList Int)
simulateRound rollingList =
    case pop rollingList of
        (current, Just rest )->
            case pop3 rest of
                Just ((cup1, cup2, cup3), remaining) ->
                    let
                        remAsList =
                            toList remaining

                        destinationNumber : Int
                        destinationNumber =
                            List.sortBy (\val -> -val) remAsList
                                |> List.Extra.find (\val -> val < current)
                                |> Maybe.withDefault (List.maximum remAsList |> Maybe.withDefault current)
                    in
                    remaining
                        |> push current
                        |> rollTo destinationNumber
                        |> insertAfter (cup1 :: cup2 :: cup3 :: [])
                        |> rollTo current
                        |> roll
                        |> Ok


                Nothing ->
                    Err "Could not pop 3"

        _ ->
            Err "Not enough cups to play"
    -- remove first 3 cups after the current
    --

cupsAfterLabel1 : RollingList Int -> String
cupsAfterLabel1 rollingList =
    rollingList
        |> rollTo 1
        |> toList
        |> List.tail
        |> Maybe.withDefault []
        |> List.map (String.fromInt)
        |> String.join ""

part2Answer : RollingList Int -> Result String Int
part2Answer rollingList =
    case pop3 rollingList of
        Just ((first, second, _), _) ->
            Ok (first * second)

        Nothing ->
            Err "Couldn't pop 2"


fillOutToMillion : RollingList Int -> RollingList Int
fillOutToMillion rollingList =
    fromList (toList rollingList ++ List.range 10 1000000)
        |> Maybe.withDefault rollingList

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String String
part1 input =
    parseInput input
        |> Result.andThen (simulateRounds 100 >> Result.map cupsAfterLabel1)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.andThen (fillOutToMillion >> simulateRounds 1 >> Result.andThen part2Answer)
        -- 10000000
