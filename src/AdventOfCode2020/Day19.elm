module AdventOfCode2020.Day19 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import Dict exposing (Dict)
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Util.Parser

-- Types -----------------------------------------------------------------------
type Rule
    = Matches String
    | Sequence (List Int)
    | OneOf (List Int) (List Int)


type alias Input =
    { rules : Dict Int Rule
    , messages : List String
    }


-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (Input)
parseInput input =
    Parser.run inputParser input
        |> Result.map (identity)
        |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (Input)
inputParser =
    Parser.succeed Input
        |= rulesParser
        |= messagesParser


rulesParser : Parser (Dict Int Rule)
rulesParser =
    Parser.loop Dict.empty (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done xs)
              |. Parser.token "\n"
            , Parser.succeed (\ruleNumber rule -> Loop (Dict.insert ruleNumber rule xs))
                |= Parser.int
                |. Parser.token ": "
                |= Parser.oneOf
                    [ Parser.succeed (Matches)
                        |. Parser.token "\""
                        |= Parser.getChompedString (Parser.chompIf (\_ -> True))
                        |. Parser.token "\""
                    , ruleIDsParser
                        |> Parser.andThen (\firstSet ->
                            Parser.oneOf
                                [ Parser.succeed (OneOf firstSet)
                                    |. Parser.token "| "
                                    |= ruleIDsParser
                                , Parser.succeed (Sequence firstSet)
                                ]
                        )
                    ]
                |. Parser.token "\n"
            ]
   )


ruleIDsParser : Parser (List Int)
ruleIDsParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (\x -> Loop (x :: xs))
                |= Parser.int
                |. Parser.oneOf
                    [ Parser.chompIf (\x -> x == ' ')
                    , Parser.succeed ()
                    ]
            , Parser.succeed (Done (List.reverse xs))
            ]
    )


messagesParser : Parser (List String)
messagesParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
              |. Parser.end
            , Parser.succeed (\x -> Loop (x :: xs))
                |= Parser.getChompedString (Parser.chompUntil "\n")
                |. Parser.token "\n"
            ]
   )

-- Functions -------------------------------------------------------------------
matchesRule : Dict Int Rule -> Int -> String -> Bool
matchesRule rules ruleNumber message =
    case matchesRuleHelp rules ruleNumber message of
        [] ->
            False

        timesItWorked ->
            List.any String.isEmpty timesItWorked


matchesRuleHelp : Dict Int Rule -> Int -> String -> List String
matchesRuleHelp rules ruleNumber message =
    case Dict.get ruleNumber rules of
        Just (Matches substring) ->
            if String.startsWith substring message then
                [String.dropLeft 1 message]
            else
                []

        Just (Sequence ruleNumbers) ->
            matchesSequence rules ruleNumbers message

        Just (OneOf ruleSet1 ruleSet2) ->
            matchesOneOf rules (ruleSet1, ruleSet2) message

        Nothing ->
            []


matchesSequence : Dict Int Rule -> List Int -> String -> List String
matchesSequence rules rulesToMatch message =
    case rulesToMatch of
        [] ->
            [message]

        ruleNumber :: numbersRest ->
            case matchesRuleHelp rules ruleNumber message of
                [] ->
                    []

                casesWhereItWorked ->
                    List.concatMap (\remaining -> matchesSequence rules numbersRest remaining) casesWhereItWorked


matchesOneOf : Dict Int Rule -> (List Int, List Int) -> String -> List String
matchesOneOf rules (set1, set2) message =
    matchesSequence rules set1 message ++ matchesSequence rules set2 message


-- Solvers ---------------------------------------------------------------------
-- 102
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (\aocInput -> List.Extra.count (matchesRule aocInput.rules 0) aocInput.messages)


-- X 145 (not greedy)
part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map (\aocInput -> List.Extra.count (matchesRule aocInput.rules 0) aocInput.messages)
