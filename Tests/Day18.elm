module Day18 exposing (..)



import Expect exposing (Expectation)
import Fuzz exposing (Fuzzer, int, list, string)
import Parser
import Test exposing (..)
import AdventOfCode2020.Day18 exposing (..)


part1 : Test
part1 =
    describe "part 1"
        [ describe "parsing"
            [ test "Just addition" <|
              \_ ->
                  let
                      expected =
                          Ok (Add (Constant 2) (Constant 3))

                      actual =
                          Parser.run (part1LoopExpressionParser "\n") "2 + 3\n"
                  in
                  Expect.equal expected actual

            , test "Addition and multiplication" <|
               \_ ->
                   let
                       expected =
                           Ok (Add (Multiply (Constant 2) (Constant 3)) (Constant 4))

                       actual =
                           Parser.run (part1LoopExpressionParser "\n") "2 * 3 + 4\n"
                   in
                   Expect.equal expected actual

            , test "Addition and multiplication and parentheses" <|
               \_ ->
                   let
                       expected =
                           Ok (Add (Multiply (Constant 2) (Constant 3)) (Parenthetical (Multiply (Constant 4) (Constant 5))))

                       actual =
                           Parser.run (part1LoopExpressionParser "\n") "2 * 3 + (4 * 5)\n"
                   in
                   Expect.equal expected actual
            ]
        , describe "evaluating"
            [ test "evaluating from an expression" <|
                \_ ->
                Expect.equal (26) (evaluate (Add (Multiply (Constant 2) (Constant 3)) (Parenthetical (Multiply (Constant 4) (Constant 5)))))

            , describe "evaluating from unparsed string"
                ([ ("2 * 3 + (4 * 5)\n", 26)
                 , ("5 + (8 * 3 + 9 + 3 * 4 * 3)\n", 437)
                 , ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))\n", 12240)
                 , ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2\n", 13632)
                 ]
                    |> List.map (\(expression, expectedResult) ->
                        test ("that it can evaluate: " ++ expression) <| (\_ -> Expect.equal (Ok expectedResult) (Result.map evaluate (Parser.run (part1LoopExpressionParser "\n") expression)))
                    )
                )

            ]

        ]


part2 : Test
part2 =
    describe "Part 2"
        [ describe "parsing"
            ([ ("1 + 1", JustTerm (AddTerm (Number 1) (FactorTerm (Number 1))) )
             , ("(1 + 1) * 2", MultiplyExpr (FactorTerm (ParentheticalFactor (JustTerm (AddTerm (Number 1) (FactorTerm (Number 1)))))) (JustTerm (FactorTerm (Number 2))))
            ]  |> List.map (\(expression, expectedResult) ->
                 test ("that it can parse: " ++ expression) <| (\_ -> Expect.equal (Ok expectedResult) (Parser.run part2LoopExpressionParser expression))
                )
            )
        , describe "evaluating from unparsed string"
           ([ ("1 + 1", 2)
            , ("(1 + 1)", 2)
            , ("(1 + 1) * 2", 4)
            , ("1 + 1 * 2", 4)
            , ("1 + (2 * 3) + (4 * (5 + 6))", 51)
            , ("2 * 3 + (4 * 5)", 46)
            , ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445)
            , ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060)
            , ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340)
            ]
               |> List.map (\(expression, expectedResult) ->
                   test ("that it can evaluate: " ++ expression) <| (\_ -> Expect.equal (Ok expectedResult) (Result.map part2Evaluate (Parser.run part2LoopExpressionParser expression)))
               )
           )
        ]