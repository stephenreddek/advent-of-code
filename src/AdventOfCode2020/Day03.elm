module AdventOfCode2020.Day03 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Array exposing (Array)
import Basics.Extra as Basics
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))

-- Types -----------------------------------------------------------------------
type Position = Open | Tree

type Slope = Slope Int Int

type alias Map = Array (Array Position)

-- Input -----------------------------------------------------------------------
--
charToPosition : Char -> Position
charToPosition char =
    if char == '#' then
        Tree
    else
        Open


parseInput : String -> Result String Map
parseInput input =
  Parser.run inputParser input
      |> Result.map (List.map (String.toList >> List.map charToPosition >> Array.fromList) >> Array.fromList)
      |> Result.mapError Parser.deadEndsToString

--
inputParser : Parser (List String)
inputParser =
  Parser.loop [] (\xs ->
      Parser.oneOf
        [ Parser.succeed (\x -> Loop (x :: xs))
            |= Parser.getChompedString (Parser.chompUntil "\n")
            |. Parser.token "\n"
        , Parser.succeed (\_ -> Done (List.reverse xs))
            |= Parser.end
        ]
   )

-- Functions -------------------------------------------------------------------
treesInPath : Slope -> Map -> Int
treesInPath slope map =
    treesInPathHelper slope (0,0) map 0



treesInPathHelper : Slope -> (Int, Int) -> Map -> Int -> Int
treesInPathHelper ((Slope deltaX deltaY) as slope) (x, y) map total =
    if y >= Array.length map then
        total
    else
        if isPositionTree (x, y) map then
            treesInPathHelper slope (x + deltaX, y + deltaY) map (total + 1)

        else
            treesInPathHelper slope (x + deltaX, y + deltaY) map total


isPositionTree : (Int, Int) -> Map -> Bool
isPositionTree (x, y) map =
    case Array.get y map of
        Just line ->
            case Array.get (modBy (Array.length line) x) line of
                Just position ->
                    position == Tree

                Nothing ->
                    False

        Nothing ->
            False


multiplyAll : List Int -> Int
multiplyAll list =
    case list of
        [] ->
            1

        x :: xs ->
            x * multiplyAll xs

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
  parseInput input
    |> Result.map (treesInPath (Slope 3 1))

part2 : String -> Result String Int
part2 input =
  parseInput input
      |> Result.map (\ map ->
        [  (Slope 1 1)
        ,  (Slope 3 1)
        ,  (Slope 5 1)
        ,  (Slope 7 1)
        ,  (Slope 1 2)
        ]
            |> List.map (\slope -> treesInPath slope map)
            |> multiplyAll
        )
