module AdventOfCode2020.Day20 exposing (..)

-- Imports ---------------------------------------------------------------------
import Array exposing (Array)
import Basics.Extra as Basics
import Binary
import Dict exposing (Dict)
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Set exposing (Set)
import Util.Parser

-- Types -----------------------------------------------------------------------
type alias Image =
    { id : Int
    , pixels : Array (Array Bool)
    , numericRepresentation : Set Int
    }

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List Image)
parseInput input =
    Parser.run inputParser input
        |> Result.map (identity)
        |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (List Image)
inputParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
              |. Parser.end
            , Parser.succeed (\x -> Loop (x :: xs))
                |= imageParser
            ]
   )


imageParser : Parser Image
imageParser =
    Parser.succeed (\id array -> Image id array (calculateNumber array))
        |. Parser.token "Tile "
        |= Parser.int
        |. Parser.token ":\n"
        |= pixelsParser


calculateNumber : Array (Array Bool) -> Set Int
calculateNumber pixels =
    let
        side1 =
            Array.get 0 pixels
                |> Maybe.map (Array.toList >> Binary.fromBooleans >> Binary.toDecimal)
                |> Maybe.withDefault 0

        side2 =
            Array.map (Array.get 0) pixels
                |> (Array.toList >> List.map (Maybe.withDefault False) >> Binary.fromBooleans >> Binary.toDecimal)

        side3 =
            Array.map (Array.get (sideLength - 1)) pixels
                |> (Array.toList >> List.map (Maybe.withDefault False) >> Binary.fromBooleans >> Binary.toDecimal)

        side4 =
            Array.get (sideLength - 1) pixels
                |> Maybe.map (Array.toList >> Binary.fromBooleans >> Binary.toDecimal)
                |> Maybe.withDefault 0

        flipNumber x =
            x
                |> Binary.fromDecimal
                |> Binary.ensureSize 10
                |> Binary.toBooleans
                |> List.reverse
                |> Binary.fromBooleans
                |> Binary.toDecimal

    in
    Set.empty
        |> Set.insert side1
        |> Set.insert side2
        |> Set.insert side3
        |> Set.insert side4
        |> Set.insert (flipNumber side1)
        |> Set.insert (flipNumber side2)
        |> Set.insert (flipNumber side3)
        |> Set.insert (flipNumber side4)



pixelsParser : Parser (Array (Array Bool))
pixelsParser =
    Parser.loop [] (\xs ->
            Parser.oneOf
                [ Parser.succeed (Done (Array.fromList (List.reverse xs)))
                  |. Parser.token "\n"
                , Parser.succeed (\x -> Loop (x :: xs))
                    |= lineParser
                ]
       )


lineParser : Parser (Array Bool)
lineParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (Array.fromList (List.reverse xs)))
              |. Parser.token "\n"
            , Parser.succeed (\x -> Loop (x :: xs))
                |= Parser.oneOf
                    [ Parser.token "." |> Parser.map (\_ -> False)
                    , Parser.token "#" |> Parser.map (\_ -> True)
                    ]
            ]
   )

-- Functions -------------------------------------------------------------------
sideLength : Int
sideLength =
    10


swapIndex : (Int, Int) -> (Int, Int) -> Image -> Image -> Image
swapIndex sourcePixel destPixel source destination =
    setPixel destPixel (getPixel sourcePixel source) destination


sourceIndexForRotation : (Int, Int) -> (Int, Int)
sourceIndexForRotation (x, y) =
    -- 00 01 02 03 04 05 06 07 08 09
    -- 10 11 12 13 14 15 16 17 18 19
    -- 20 21 22 03 04 05 06 07 08 09
    -- 30 01 32 33 04 05 06 07 08 09
    -- 40 01 42 43 44 05 06 07 08 09
    -- 00 01 02 03 54 05 06 07 08 09
    -- 00 01 02 03 04 05 06 07 08 09
    -- 00 01 02 03 04 05 06 07 08 09
    -- 80 81 82 03 04 05 06 07 88 09
    -- 90 91 02 03 04 05 06 07 08 99
    --
    -- 18 -> 11
    -- 11 -> 81
    -- 21 -> 82
    -- 44 -> 54
    (-y + sideLength - 1, x)


rotateClockwise : Image -> Image
rotateClockwise image =
    let
        destinationValue (x, y) =
            getPixel (sourceIndexForRotation (x, y)) image
    in
    { id = image.id
    , pixels = Array.initialize sideLength (\x -> Array.initialize sideLength (\y -> destinationValue (x, y)))
    , numericRepresentation = image.numericRepresentation
    }



flip : Image -> Image
flip image =
    let
        destinationValue currentRow index =
            Array.get (sideLength - 1 - index) currentRow
                |> Maybe.withDefault False
    in
    { id = image.id
    , pixels = Array.map (\row -> Array.initialize sideLength (destinationValue row)) image.pixels
    , numericRepresentation = image.numericRepresentation
    }


getPixel : (Int, Int) -> Image -> Bool
getPixel (x, y) image =
    Array.get x image.pixels
        |> Maybe.andThen (Array.get y)
        |> Maybe.withDefault False


setPixel : (Int, Int) -> Bool -> Image -> Image
setPixel (x, y) pixel image =
    let
        valueAtX =
            Array.get x image.pixels
    in
    case valueAtX of
        Just array ->
            { image | pixels = Array.set x (Array.set y pixel array) image.pixels }

        Nothing ->
            image


matchingImages : Image -> List Image -> Set Int
matchingImages image images =
    List.foldl (\otherImage acc ->
            if otherImage.id == image.id then
                acc
            else
                if Set.isEmpty (Set.intersect image.numericRepresentation otherImage.numericRepresentation) then
                    acc
                else
                    Set.insert otherImage.id acc
        )
        Set.empty images


findMatchingSquares : List Image -> Dict Int (Set Int)
findMatchingSquares images =
    List.foldl (\image acc -> Dict.insert image.id (matchingImages image images ) acc) Dict.empty images


idsWhereOnlyTwoMatches : Dict Int (Set Int) -> List Int
idsWhereOnlyTwoMatches dict =
    Dict.foldl (\key value acc ->
            if Set.size value == 2 then
                key :: acc
            else
                acc
        ) [] dict


multiply : List Int -> Int
multiply ints =
    List.foldl (\int acc -> acc * int ) 1 ints

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (findMatchingSquares >> idsWhereOnlyTwoMatches >> multiply)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map (always 0)
