module AdventOfCode2020.Day16 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Array exposing (Array)
import Basics.Extra as Basics
import Dict exposing (Dict)
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Set exposing (Set)
import Util.Parser

-- Types -----------------------------------------------------------------------
type alias Input =
    { fieldRules : List FieldRule
    , myTicket : Array Int
    , nearbyTickets : List (Array Int)
    }

type alias FieldRule =
    { fieldName : String
    , ranges : List (Int, Int)
    }

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String Input
parseInput input =
    Parser.run inputParser input
        |> Result.map (identity)
        |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser Input
inputParser =
    Parser.succeed Input
        |= fieldRulesParser
        |. Parser.token "your ticket:\n"
        |= ticketParser
        |. Parser.token "\n"
        |. Parser.token "nearby tickets:\n"
        |= nearbyTicketsParser


fieldRulesParser : Parser (List FieldRule)
fieldRulesParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
              |. Parser.token "\n"
            , Parser.succeed (\fieldName ranges -> Loop ({ fieldName = fieldName, ranges = ranges } :: xs))
                |= Parser.getChompedString (Parser.chompUntil ":")
                |. Parser.token ": "
                |= fieldRangeParser
            ]
   )


fieldRangeParser : Parser (List (Int, Int))
fieldRangeParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
              |. Parser.token "\n"
            , Parser.succeed (\start stop -> Loop ((start, stop) :: xs))
                |= Parser.int
                |. Parser.token "-"
                |= Parser.int
                |. Parser.oneOf
                    [ Parser.token " or "
                    , Parser.succeed ()
                    ]
            ]
   )


nearbyTicketsParser : Parser (List (Array Int))
nearbyTicketsParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
              |. Parser.end
            , Parser.succeed (\x -> Loop (x :: xs))
                |= ticketParser
            ]
   )


ticketParser : Parser (Array Int)
ticketParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (Array.fromList (List.reverse xs)))
              |. Parser.oneOf
                [ Parser.end
                , Parser.token "\n"
                ]
            , Parser.succeed (\x -> Loop (x :: xs))
                |= Parser.int
                |. Parser.oneOf
                    [ Parser.token ","
                    , Parser.succeed ()
                    ]
            ]
   )


-- Functions -------------------------------------------------------------------
valuesInvalidForAnyField : Input -> Array Int -> List Int
valuesInvalidForAnyField input ticket =
    List.filter (isValidForAnyField input >> not) (Array.toList ticket)


allValuesValidForAnyField : Input -> Array Int -> Bool
allValuesValidForAnyField input ticket =
    List.all (isValidForAnyField input) (Array.toList ticket)


isValidForAnyField : Input -> Int -> Bool
isValidForAnyField input value =
    List.any (\rule -> isValidForRule rule value) input.fieldRules


isValidForRule : FieldRule -> Int -> Bool
isValidForRule fieldRule int =
    List.any (\(min, max) -> int >= min && int <= max) fieldRule.ranges


validTickets : Input -> List (Array Int)
validTickets input =
    List.filter (allValuesValidForAnyField input) input.nearbyTickets


solveForFieldOrder : Input -> List (Array Int) -> Result String (List FieldRule)
solveForFieldOrder input foundValidTickets =
    let
        expectedLength =
            List.length input.fieldRules

        possibleIndexes =
            List.range 0 (expectedLength - 1)

        fieldsThatCanWorkAtIndex : Int -> List String
        fieldsThatCanWorkAtIndex indexOfValue =
            List.filterMap (\rule ->
                    if List.all (ticketIsValidForRuleAtIndex indexOfValue rule) foundValidTickets then
                        Just rule.fieldName
                    else
                        Nothing
                ) input.fieldRules

        rulesThatWorkForAnIndex : Dict Int (List String)
        rulesThatWorkForAnIndex =
            possibleIndexes
                |> List.map (\index -> (index, fieldsThatCanWorkAtIndex index))
                |> Dict.fromList
    in
    --Err (
    --    rulesThatWorkForAnIndex
    --        |> Dict.values
    --        |> List.map (List.length >> String.fromInt)
    --        |> String.join ","
    --        |> (++) ("valid tickets: " ++ String.fromInt (List.length foundValidTickets) ++ ". valid rules per index: ")
    --)
    validOrdering rulesThatWorkForAnIndex 0 (Set.fromList (List.map .fieldName input.fieldRules))
        |> Maybe.map (List.filterMap (\ruleName -> List.Extra.find (.fieldName >> (==) ruleName) input.fieldRules))
        |> Result.fromMaybe "Cannot find a valid ordering"

validOrdering : Dict Int (List String) -> Int -> Set String -> Maybe (List String)
validOrdering rulesThatWorkForAnIndex index unusedFields =
    case Dict.get index rulesThatWorkForAnIndex of
        Just indexesThatWorkHere ->
            indexesThatWorkHere
                |> List.filter (\fieldName -> Set.member fieldName unusedFields)
                |> mapFind
                    (\fieldName ->
                        let
                            updatedUnused =
                                Set.remove fieldName unusedFields
                        in
                        if Set.isEmpty updatedUnused then
                            Just [fieldName]
                        else
                            case (validOrdering rulesThatWorkForAnIndex (index + 1) updatedUnused) of
                                Just rest ->
                                    Just (fieldName :: rest)

                                Nothing ->
                                    Nothing
                    )

        Nothing ->
            Nothing


mapFind : (a -> Maybe b) -> List a -> Maybe b
mapFind mapper list =
    case list of
        [] ->
            Nothing

        first :: rest ->
            let
                mapped =
                    mapper first
            in
            case mapped of
                Just value ->
                    Just value

                Nothing ->
                    mapFind mapper rest


validOrderingHelp : Dict Int (Set Int) -> Int -> Set Int -> Maybe (List Int)
validOrderingHelp fieldsThatCanWorkAtIndexAnIndex index unusedIndexes =
    Nothing

isValidFieldOrdering : List (Array Int) -> List FieldRule -> Bool
isValidFieldOrdering foundValidTickets fieldRules =
    List.all (validatesWithFieldRuleOrdering fieldRules) foundValidTickets


ticketIsValidForRuleAtIndex : Int -> FieldRule -> Array Int -> Bool
ticketIsValidForRuleAtIndex valueIndex fieldRule ticket =
    case Array.get valueIndex ticket of
        Just value ->
            isValidForRule fieldRule value

        Nothing ->
            False


validatesWithFieldRuleOrdering : List FieldRule -> Array Int -> Bool
validatesWithFieldRuleOrdering fieldRules ticket =
    List.map2 isValidForRule fieldRules (Array.toList ticket)
        |> List.all (identity)


part2Answer : Input -> List FieldRule -> Int
part2Answer input orderedFieldRules =
    orderedFieldRules
        |> List.indexedMap (\index rule ->
            if String.startsWith "departure" rule.fieldName then
                Array.get index input.myTicket
            else
                Nothing
        )
        |> List.foldl (\val acc -> acc * (Maybe.withDefault 1 val)) 1


-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 inputString =
    parseInput inputString
        |> Result.map (\input -> List.concatMap (valuesInvalidForAnyField input) input.nearbyTickets |> List.sum)

part2 : String -> Result String Int
part2 inputString =
    parseInput inputString
        |> Result.andThen (\input -> validTickets input |> solveForFieldOrder input |> Result.map (part2Answer input))
