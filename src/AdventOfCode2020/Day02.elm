module AdventOfCode2020.Day02 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Array
import Basics.Extra as Basics
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))

-- Types -----------------------------------------------------------------------
type alias Policy =
    { char : String
    , min : Int
    , max : Int
    , password : String
    }

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List Policy)
parseInput input =
  Parser.run inputParser input
      |> Result.mapError Parser.deadEndsToString

--
inputParser : Parser (List Policy)
inputParser =
  Parser.loop [] <| (\xs ->
      Parser.oneOf
        [ Parser.succeed (\min max char password -> Loop ({ char = char, min = min, max = max, password = password } :: xs))
            |= Parser.int
            |. Parser.token "-"
            |= Parser.int
            |. Parser.token " "
            |= Parser.getChompedString (Parser.chompIf (always True))
            |. Parser.token ":"
            |. Parser.spaces
            |= Parser.getChompedString (Parser.chompUntil "\n")
            |. Parser.spaces
        , Parser.succeed (\_ -> Done (List.reverse xs))
            |= Parser.end
        ]
   )

-- Functions -------------------------------------------------------------------
inRange : Int -> (Int, Int) -> Bool
inRange count (min, max) =
    count >= min && count <= max


countOccurrences : String -> String -> Int
countOccurrences sub value =
    String.indexes sub value
        |> List.length



isValid : Policy -> Bool
isValid policy =
    inRange (countOccurrences policy.char policy.password) (policy.min, policy.max)


isValidPart2 : Policy -> Bool
isValidPart2 policy =
    let
        passwordChars =
            Array.fromList (String.toList policy.password)

        firstValueMatches =
            Maybe.map String.fromChar (Array.get (policy.min - 1) passwordChars) == Just policy.char

        secondValueMatches =
            Maybe.map String.fromChar (Array.get (policy.max - 1) passwordChars) == Just policy.char
    in
    firstValueMatches /= secondValueMatches


-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
  parseInput input
    |> Result.map (List.Extra.count isValid)

part2 : String -> Result String Int
part2 input =
  parseInput input
    |> Result.map (List.Extra.count isValidPart2)