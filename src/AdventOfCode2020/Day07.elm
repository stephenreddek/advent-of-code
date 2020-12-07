module AdventOfCode2020.Day07 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Set exposing (Set)
import Util.Parser

-- Types -----------------------------------------------------------------------
type alias Rule =
    { color : String
    , contains : List { quantity : Int, color : String }
    }


-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List Rule)
parseInput input =
  Parser.run inputParser input
      |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (List Rule)
inputParser =
  Parser.loop [] (\xs ->
      Parser.oneOf
        [ Parser.succeed (Done xs)
            |. Parser.end
        , Parser.succeed (\color contains -> Loop ({ color = color, contains = contains } :: xs))
            |= Parser.getChompedString (Parser.chompUntil " bags contain ")
            |. Parser.token " bags contain "
            |= containsParser
            |. Parser.token "\n"
        ]
   )


containsParser : Parser (List { quantity : Int, color : String })
containsParser =
    Parser.oneOf
        [ Parser.succeed []
            |. Parser.token "no other bags."
        , Parser.loop [] (\xs ->
            Parser.oneOf
              [ Parser.succeed (Done xs)
                  |. Parser.token "."
              , Parser.succeed (\quantity color -> Loop ({ quantity = quantity, color = color } :: xs))
                  |= Parser.int
                  |. Parser.spaces
                  |= Parser.getChompedString (Parser.chompUntil " bag")
                  |. Parser.token " bag"
                  |. Parser.oneOf
                      [ Parser.token "s, "
                      , Parser.token ", "
                      , Parser.token "s"
                      , Parser.succeed ()
                      ]
              ]
         )
        ]


-- Functions -------------------------------------------------------------------
solvePart1 : String -> List Rule -> Int
solvePart1 colorWeCareAbout rules =
    let
        derivedRules =
            deriveRules (Set.insert colorWeCareAbout Set.empty) rules
    in
    Set.size derivedRules - 1


deriveRules : Set String -> List Rule -> Set String
deriveRules thingsThatCanContainWhatWeWant allRules =
    let
        newSetOfThingsThatCanContainWhatWeWant : Set String
        newSetOfThingsThatCanContainWhatWeWant =
            List.foldl (\rule result ->
                    if List.any (\contained -> Set.member contained.color thingsThatCanContainWhatWeWant) rule.contains then
                        Set.insert rule.color result
                    else
                        result
                )
                thingsThatCanContainWhatWeWant
                allRules
    in
    if Set.size newSetOfThingsThatCanContainWhatWeWant == Set.size thingsThatCanContainWhatWeWant then
        thingsThatCanContainWhatWeWant
    else
        deriveRules newSetOfThingsThatCanContainWhatWeWant allRules


-- bags = n * bagsinside + n * bagsinside foreach bag inside


bagsInsideColor : String -> List Rule -> Int
bagsInsideColor color rules =
    let
        ruleAboutColor : Maybe Rule
        ruleAboutColor =
            List.Extra.find (\rule -> rule.color == color) rules
    in
    case ruleAboutColor of
        Just rule ->
            rule.contains
                |> List.map (\contained -> contained.quantity * (1 + bagsInsideColor contained.color rules))
                |> List.sum

        Nothing ->
            0


-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (solvePart1 "shiny gold")

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map (bagsInsideColor "shiny gold")
