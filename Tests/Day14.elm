module Day14 exposing (..)

import Binary
import Dict
import Expect exposing (Expectation)
import Fuzz exposing (Fuzzer, int, list, string)
import Test exposing (..)
import AdventOfCode2020.Day14 exposing (..)


suite : Test
suite =
    describe "part 2"
        [ describe "generateFloatingAddressMasks"
            [ test "that it generates the expected masks" <|
                \_ ->
                    let
                        expected =
                            [ { zeros = Binary.fromIntegers [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,0,0]
                              , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
                              }
                            , { zeros = Binary.fromIntegers [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,0,1]
                              , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]
                              }
                            , { zeros = Binary.fromIntegers [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,0]
                              , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0]
                              }
                            , { zeros = Binary.fromIntegers [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1]
                              , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1]
                              }
                            , { zeros = Binary.fromIntegers [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0]
                              , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0]
                              }
                            , { zeros = Binary.fromIntegers [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1]
                              , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,1]
                              }
                            , { zeros = Binary.fromIntegers [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0]
                              , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0]
                              }
                            , { zeros = Binary.fromIntegers [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
                              , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,1]
                              }
                            ]

                        floatingIndex =
                            [0, 1, 3]

                        actual =
                            generateFloatingAddressMasks floatingIndex
                    in
                    Expect.equal expected actual
            ]
        , describe "toMask"
            [ test "the example" <|
                \_ ->
                let
                    expected =
                        { floating = [0, 5]
                        , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,1,0]
                        }

                    actual =
                        toMask "000000000000000000000000000000X1001X"
                in
                Expect.equal expected actual
            , test "something made up" <|
              \_ ->
              let
                  expected =
                      { floating = [0, 5, 20]
                      , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0,1,0,0,1,0]
                      }

                  actual =
                      toMask "000000000000000X00010000010000X1001X"
              in
              Expect.equal expected actual
            ]
        , describe "createFloatingAddressMask"
            [ test "the example" <|
                \_ ->
                let
                    expected =
                        { zeros = Binary.fromIntegers [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0]
                        , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0]
                        }

                    actual =
                        createFloatingAddressMask { indexesThatShouldBeOnes = [1,3], allFloatingAddresses = [0,1,3] }
                in
                Expect.equal expected actual
            , test "when there are not floating" <|
                  \_ ->
                  let
                      expected =
                          { zeros = Binary.fromIntegers [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
                          , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
                          }

                      actual =
                          createFloatingAddressMask { indexesThatShouldBeOnes = [], allFloatingAddresses = [] }
                  in
                  Expect.equal expected actual
            ]
        , describe "applyFloatingAddressMask"
            [ test "the example [26]" <|
                \_ ->
                let
                    addressMask =
                        { zeros = Binary.fromIntegers [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0]
                        , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0]
                        }

                    expected =
                        26

                    actual =
                        applyFloatingAddressMask 26 { floating = [0, 1, 3], ones = Binary.empty } addressMask
                in
                Expect.equal expected actual
            , test "the example [19]" <|
                  \_ ->
                  let
                      addressMask =
                          { zeros = Binary.fromIntegers [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1]
                          , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1]
                          }

                      expected =
                          19

                      actual =
                          applyFloatingAddressMask 26 { floating = [0, 1, 3], ones = Binary.empty } addressMask
                  in
                  Expect.equal expected actual
            ]
        , describe "addressesFromFloating"
            [ test "the example" <|
                \_ ->
                let
                    --testState : State
                    --testState =
                    --    { mask =
                    --        { floating = [ 0, 1, 3 ]
                    --        , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
                    --        }
                    --    , memory = Dict.empty
                    --    }

                    testState : State
                    testState =
                        { mask = toMask "00000000000000000000000000000000X0XX"
                        , memory = Dict.empty
                        }

                    expected =
                        [16, 17, 18, 19, 24, 25, 26, 27]

                    actual =
                        addressesFromFloating testState 26
                in
                Expect.equal expected actual
            , test "the other example" <|
              \_ ->
              let
                  testState : State
                  testState =
                      { mask =
                          { floating = [ 0, 5 ]
                          , ones = Binary.fromIntegers [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,1,0]
                          }
                      , memory = Dict.empty
                      }

                  expected =
                      [26, 27, 58, 59]

                  actual =
                      addressesFromFloating testState 42
              in
              Expect.equal expected actual
            ]
        ]
