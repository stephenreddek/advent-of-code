module AdventOfCode2020.Day04 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Regex exposing (Regex)

-- Types -----------------------------------------------------------------------
type alias Passport = List (String, String)

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List Passport)
parseInput input =
  Parser.run inputParser input
      |> Result.mapError Parser.deadEndsToString

--
inputParser : Parser (List Passport)
inputParser =
  Parser.loop [] <| (\xs ->
      Parser.oneOf
        [ Parser.succeed (\_ -> Done (List.reverse xs))
            |= Parser.end
        , Parser.succeed (\x -> Loop (x :: xs))
            |= passportParser
        ]
   )


isWhiteSpace : Char -> Bool
isWhiteSpace char =
    case char of
        '\t' ->
            True

        ' ' ->
            True

        '\n' ->
            True

        _ ->
            False


pairParser : Parser (String, String)
pairParser =
    Parser.succeed Tuple.pair
        |= (Parser.getChompedString (Parser.chompUntil ":")) -- |> Parser.andThen (\field -> if String.isEmpty field then Parser.problem "field must be at least one character long." else Parser.succeed field)
        |. Parser.token ":"
        |= Parser.getChompedString (Parser.chompWhile (\char -> not (isWhiteSpace char)))
        |. Parser.chompIf isWhiteSpace


passportParser : Parser (Passport)
passportParser =
    Parser.loop [] <| (\xs ->
          Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
                 |. Parser.token "\n"
            , Parser.succeed (\_ -> Done (List.reverse xs))
                |= Parser.end
            , Parser.succeed (\x -> Loop (x :: xs))
                |= pairParser
            ]
       )

-- Functions -------------------------------------------------------------------
isValidHeight : String -> Bool
isValidHeight heightString =
    case (String.endsWith "cm" heightString, String.endsWith "in" heightString) of
        (True, _) ->
            String.split "cm" heightString
                |> List.head
                |> Maybe.andThen String.toInt
                |> Maybe.map (\height -> 150 <= height && height <= 193)
                |> Maybe.withDefault False

        (_,True) ->
            String.split "in" heightString
                |> List.head
                |> Maybe.andThen String.toInt
                |> Maybe.map (\height -> 59 <= height && height <= 76)
                |> Maybe.withDefault False
        _ ->
            False


colorRegex : Regex
colorRegex =
    Regex.fromString "^#[0-9a-f]{6}$"
        |> Maybe.withDefault Regex.never


requiredFields : List (String, String -> Bool)
requiredFields =
    [ ("byr", \value -> String.toInt value |> Maybe.map (\year -> year >= 1920 && year <= 2002 && (String.length value == 4)) |> Maybe.withDefault False)
    , ("iyr", \value -> String.toInt value |> Maybe.map (\year -> year >= 2010 && year <= 2020 && (String.length value == 4)) |> Maybe.withDefault False)
    , ("eyr", \value -> String.toInt value |> Maybe.map (\year -> year >= 2020 && year <= 2030 && (String.length value == 4)) |> Maybe.withDefault False)
    , ("hgt", isValidHeight)
    , ("hcl", Regex.contains colorRegex)
    , ("ecl", \value -> List.member value ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"] )
    , ("pid", \value -> String.length value == 9 && (String.toInt value  |> Maybe.map (\_ -> True) |> Maybe.withDefault False))
    ]


hasRequiredFields : Passport -> Bool
hasRequiredFields passport =
    List.map (\(fieldName, _) -> List.any (Tuple.first >> (==) fieldName) passport) requiredFields
        |> List.foldl ((&&)) True


isValidPassport : Passport -> Bool
isValidPassport passport =
    List.map (\(fieldName, fieldValidator) -> List.any (\(actualName, actualValue) -> actualName == fieldName && fieldValidator actualValue) passport) requiredFields
            |> List.foldl ((&&)) True


-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
  parseInput input
    |> Result.map (List.Extra.count hasRequiredFields)

part2 : String -> Result String Int
part2 input =
  parseInput input
    |> Result.map (List.Extra.count isValidPassport)
