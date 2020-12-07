module AdventOfCode2020.Day06 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Set

-- Types -----------------------------------------------------------------------


-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List (List String))
parseInput input =
  Parser.run inputParser input
      |> Result.mapError Parser.deadEndsToString

--
inputParser : Parser (List (List String))
inputParser =
  Parser.loop [] (\xs ->
      Parser.oneOf
        [ Parser.succeed (\_ -> Done (List.reverse xs))
            |= Parser.end
        , Parser.succeed (\x -> Loop (x :: xs))
            |= groupParser
        ]
   )


groupParser : Parser (List String)
groupParser =
    Parser.loop [] (\xs ->
          Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
                 |. Parser.token "\n"
            , Parser.succeed (\_ -> Done (List.reverse xs))
                |= Parser.end
            , Parser.succeed (\x -> Loop (x :: xs))
                |= (Parser.getChompedString (Parser.chompUntil "\n"))
                |. Parser.token "\n"
            ]
       )

-- Functions -------------------------------------------------------------------
numberOfAnyPositives : List String -> Int
numberOfAnyPositives group =
    List.foldl (\memberAnswer groupAnswer -> Set.union groupAnswer (Set.fromList (String.toList memberAnswer))) Set.empty group
        |> Set.size

numberOfAllPositives : List String -> Int
numberOfAllPositives group =
    let
        startingSet =
            List.head group
                |> Maybe.map (String.toList >> Set.fromList)
                |> Maybe.withDefault Set.empty
    in
    List.foldl (\memberAnswer groupAnswer -> Set.intersect groupAnswer (Set.fromList (String.toList memberAnswer))) startingSet group
        |> Set.size

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (List.map numberOfAnyPositives >> List.sum)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map (List.map numberOfAllPositives >> List.sum)
