module AdventOfCode2019.Day01 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------

import Basics.Extra as Basics
import Parser exposing ((|.), (|=), Parser, Step(..), Trailing(..))
import Util.Parser



-- Types -----------------------------------------------------------------------
-- Input -----------------------------------------------------------------------
--


parseInput : String -> Result String (List Int)
parseInput input =
    Parser.run inputParser input
        |> Result.map identity
        |> Result.mapError Util.Parser.deadEndsToString



--


inputParser : Parser (List Int)
inputParser =
    Parser.loop []
        (\xs ->
            Parser.oneOf
                [ Parser.succeed (Done (List.reverse xs))
                    |. Parser.end
                , Parser.succeed (\x -> Loop (x :: xs))
                    |= Parser.int
                    |. Parser.token "\n"
                ]
        )



-- Functions -------------------------------------------------------------------


calculateFuelCost : Int -> Int
calculateFuelCost mass =
    mass // 3 - 2


recursiveFuelCost : Int -> Int
recursiveFuelCost mass =
    recursiveFuelCostHelp 0 mass


recursiveFuelCostHelp : Int -> Int -> Int
recursiveFuelCostHelp acc mass =
    let
        cost =
            calculateFuelCost mass
    in
    if cost <= 0 then
        acc

    else
        recursiveFuelCostHelp (acc + cost) cost



-- Solvers ---------------------------------------------------------------------


part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (List.map calculateFuelCost >> List.sum)


part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map (List.map recursiveFuelCost >> List.sum)
