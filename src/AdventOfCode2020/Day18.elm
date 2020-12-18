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


type Part2Expression
    = MultiplyExpr Term Part2Expression
    | JustTerm Term

type Term
    = AddTerm Factor Term
    | FactorTerm Factor

type Factor
    = Number Int
    | ParentheticalFactor Part2Expression

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Parser a -> Result String (List a)
parseInput input baseParser =
    Parser.run (inputParser baseParser) input
        |> Result.map (identity)
        |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser a -> Parser (List a)
inputParser baseParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
              |. Parser.end
            , Parser.succeed (\x -> Loop (x :: xs))
                |= baseParser
            ]
   )


part1BaseExpressionParser : Maybe Expression -> Parser Expression
part1BaseExpressionParser lastExpression =
    case lastExpression of
        Just expression ->
            Parser.oneOf
                [ Parser.succeed (\secondOperand -> Add expression secondOperand)
                    |. Parser.token " + "
                    |= part1BaseExpressionParser Nothing
                , Parser.succeed (\secondOperand -> Multiply expression secondOperand)
                    |. Parser.token " * "
                    |= part1BaseExpressionParser Nothing
                ]

        Nothing ->
            Parser.oneOf
                [ Parser.succeed Parenthetical
                    |. Parser.token "("
                    |= Parser.lazy (\_ -> part1LoopExpressionParser ")")
                , Parser.map Constant Parser.int
                ]


part1LoopExpressionParser : String -> Parser Expression
part1LoopExpressionParser endingToken =
    Parser.loop Nothing (\lastExpression ->
        Parser.oneOf
            [ Parser.succeed (Done lastExpression)
                |. Parser.token endingToken
            , Parser.succeed (\expression -> Loop (Just expression))
                |= part1BaseExpressionParser lastExpression
            ]
       )
       |> Parser.andThen (\finalExpression ->
            case finalExpression of
                Just expression ->
                    Parser.succeed expression

                Nothing ->
                    Parser.problem "no expression"
        )


part2LoopExpressionParser : Parser Part2Expression
part2LoopExpressionParser =
    Parser.oneOf
        [ Parser.backtrackable
          (Parser.succeed (MultiplyExpr)
              |= termParser
              |. Parser.token " * "
              |. Parser.commit ()
              |= Parser.lazy (\_ -> part2LoopExpressionParser)
          )
        , Parser.map JustTerm termParser
        ]


termParser : Parser Term
termParser =
    Parser.oneOf
        [ Parser.backtrackable
            (Parser.succeed (\operand1 operand2 -> AddTerm operand1 operand2)
                |= factorParser
                |. Parser.token  " + "
                |. Parser.commit ()
                |= Parser.lazy (\_ -> termParser)
            )
        , Parser.map FactorTerm factorParser
        ]

factorParser : Parser Factor
factorParser =
    Parser.oneOf
        [ Parser.map Number Parser.int
        , Parser.succeed ParentheticalFactor
            |. Parser.token "("
            |= Parser.lazy (\_ -> part2LoopExpressionParser)
            |. Parser.token ")"
        ]


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


part2Evaluate : Part2Expression -> Int
part2Evaluate expression =
    case expression of
        MultiplyExpr operand1 operand2 ->
            (evaluateTerm operand1) * (part2Evaluate operand2)

        JustTerm term ->
            evaluateTerm term


evaluateTerm : Term -> Int
evaluateTerm term =
    case term of
        AddTerm (Number term1) term2 ->
            term1 + evaluateTerm term2

        AddTerm (ParentheticalFactor term1) term2 ->
            part2Evaluate term1 + evaluateTerm term2

        FactorTerm (Number int) ->
            int

        FactorTerm (ParentheticalFactor part2Expression) ->
            part2Evaluate part2Expression


-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input (part1LoopExpressionParser "\n")
        |> Result.map (List.map evaluate >> List.sum)

part2 : String -> Result String Int
part2 input =
    parseInput input (part2LoopExpressionParser |. Parser.token "\n")
        |> Result.map (List.map part2Evaluate >> List.sum)
