module AdventOfCode2020.Day18 exposing (..)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Util.Parser

-- Types -----------------------------------------------------------------------
type Expression
    = Add Expression Expression
    | Multiply Expression Expression
    | Constant Int
    | Parenthetical Expression


-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List Expression)
parseInput input =
    Parser.run inputParser input
        |> Result.map (identity)
        |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (List Expression)
inputParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
              |. Parser.end
            , Parser.succeed (\x -> Loop (x :: xs))
                |= loopExpressionParser "\n"
            ]
   )


baseExpressionParser : Maybe Expression -> Parser Expression
baseExpressionParser lastExpression =
    case lastExpression of
        Just expression ->
            Parser.oneOf
                [ Parser.succeed (\secondOperand -> Add expression secondOperand)
                    |. Parser.token " + "
                    |= baseExpressionParser Nothing
                , Parser.succeed (\secondOperand -> Multiply expression secondOperand)
                    |. Parser.token " * "
                    |= baseExpressionParser Nothing
                ]

        Nothing ->
            Parser.oneOf
                [ Parser.succeed Parenthetical
                    |. Parser.token "("
                    |= Parser.lazy (\_ -> loopExpressionParser ")")
                , Parser.map Constant Parser.int
                ]


loopExpressionParser : String -> Parser Expression
loopExpressionParser endingToken =
    Parser.loop Nothing (\lastExpression ->
        Parser.oneOf
            [ Parser.succeed (Done lastExpression)
                |. Parser.token endingToken
            , Parser.succeed (\expression -> Loop (Just expression))
                |= baseExpressionParser lastExpression
            ]
       )
       |> Parser.andThen (\finalExpression ->
            case finalExpression of
                Just expression ->
                    Parser.succeed expression

                Nothing ->
                    Parser.problem "no expression"
        )
-- Functions -------------------------------------------------------------------
evaluate : Expression -> Int
evaluate expression =
    case expression of
        Add operand1 operand2 ->
            evaluate operand1 + evaluate operand2

        Multiply operand1 operand2 ->
            evaluate operand1 * evaluate operand2

        Constant int ->
            int

        Parenthetical operand ->
            evaluate operand


-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (List.map evaluate >> List.sum)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map (always 0)
