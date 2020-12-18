module Day18 exposing (..)



import Expect exposing (Expectation)
import Fuzz exposing (Fuzzer, int, list, string)
import Parser
import Test exposing (..)
import AdventOfCode2020.Day18 exposing (..)


suite : Test
suite =
    describe "part 1"
        [ describe "parsing"
            [ test "Just addition" <|
              \_ ->
                  let
                      expected =
                          Ok (Add (Constant 2) (Constant 3))

                      actual =
                          Parser.run (loopExpressionParser "\n") "2 + 3\n"
                  in
                  Expect.equal expected actual

            , test "Addition and multiplication" <|
               \_ ->
                   let
                       expected =
                           Ok (Add (Multiply (Constant 2) (Constant 3)) (Constant 4))

                       actual =
                           Parser.run (loopExpressionParser "\n") "2 * 3 + 4\n"
                   in
                   Expect.equal expected actual

            , test "Addition and multiplication and parentheses" <|
               \_ ->
                   let
                       expected =
                           Ok (Add (Multiply (Constant 2) (Constant 3)) (Parenthetical (Multiply (Constant 4) (Constant 5))))

                       actual =
                           Parser.run (loopExpressionParser "\n") "2 * 3 + (4 * 5)\n"
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
                        test ("that it can evaluate: " ++ expression) <| (\_ -> Expect.equal (Ok expectedResult) (Result.map evaluate (Parser.run (loopExpressionParser "\n") expression)))
                    )
                )

            ]

        ]
