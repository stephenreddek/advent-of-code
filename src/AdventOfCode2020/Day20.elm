module AdventOfCode2020.Day20 exposing (..)

-- Imports ---------------------------------------------------------------------
import Array exposing (Array)
import Basics.Extra as Basics
import Binary
import Dict exposing (Dict)
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Set exposing (Set)
import Tagged exposing (Tagged)
import Tagged.Dict exposing (TaggedDict)
import Tagged.Set exposing (TaggedSet)
import Util.Parser

-- Types -----------------------------------------------------------------------
type alias Image =
    { id : ImageId
    --, pixels : Array (Array Bool)
    , numericRepresentation : (NumericRepresentation, NumericRepresentation)
    }

type alias Matched =
    { image : Image
    , side : NumericRepresentation
    --, oriented : Array (Array Bool)
    }

type ImageIdTag = ImageIdTag

type alias ImageId = Tagged ImageIdTag Int

type SideValueTag = SideValueTag

type alias SideValue = Tagged SideValueTag Int


type alias NumericRepresentation =
    { top : SideValue
    , bottom : SideValue
    , left : SideValue
    , right : SideValue
    , asSet : TaggedSet SideValueTag Int
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
    Parser.succeed (\id array -> Image id (calculateNumericRepresentation array))
        |. Parser.token "Tile "
        |= (Parser.int |> Parser.map Tagged.tag)
        |. Parser.token ":\n"
        |= pixelsParser


calculateNumericRepresentation : Array (Array Bool) -> (NumericRepresentation, NumericRepresentation)
calculateNumericRepresentation pixels =
    let
        side1 =
            Array.get 0 pixels
                |> Maybe.map (Array.toList >> Binary.fromBooleans >> Binary.toDecimal)
                |> Maybe.withDefault 0
                |> Tagged.tag

        side2 =
            Array.map (Array.get 0) pixels
                |> (Array.toList >> List.reverse >> List.map (Maybe.withDefault False) >> Binary.fromBooleans >> Binary.toDecimal)
                |> Tagged.tag

        side3 =
            Array.map (Array.get (sideLength - 1)) pixels
                |> (Array.toList >> List.map (Maybe.withDefault False) >> Binary.fromBooleans >> Binary.toDecimal)
                |> Tagged.tag

        side4 =
            Array.get (sideLength - 1) pixels
                |> Maybe.map (Array.toList >> List.reverse >> Binary.fromBooleans >> Binary.toDecimal)
                |> Maybe.withDefault 0
                |> Tagged.tag
    in
    -- flip left and bottom or no?
    ({ top = ( side1)
     , left = ( side2)
     , right = ( side3)
     , bottom = ( side4)
     , asSet =
        Tagged.Set.empty
        |> Tagged.Set.insert ( side1)
        |> Tagged.Set.insert ( side2)
        |> Tagged.Set.insert ( side3)
        |> Tagged.Set.insert ( side4)
    }
    , { top = (flipNumber side1)
       , left = (flipNumber side3)
       , right = (flipNumber side2)
       , bottom = (flipNumber side4)
       , asSet =
          Tagged.Set.empty
              |> Tagged.Set.insert (flipNumber side1)
              |> Tagged.Set.insert (flipNumber side2)
              |> Tagged.Set.insert (flipNumber side3)
              |> Tagged.Set.insert (flipNumber side4)

   })


flipNumber : SideValue -> SideValue
flipNumber x =
    x
        |> Tagged.untag
        |> Binary.fromDecimal
        |> Binary.ensureSize 10
        |> Binary.toBooleans
        |> List.reverse
        |> Binary.fromBooleans
        |> Binary.toDecimal
        |> Tagged.tag


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


swapIndex : (Int, Int) -> (Int, Int) -> Matched -> Matched -> Matched
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


rotateClockwise : Matched -> Matched
rotateClockwise matched =
    let
        destinationValue (x, y) =
            getPixel (sourceIndexForRotation (x, y)) matched

        rotatedSide =
            { top = matched.side.left
            , right = matched.side.top
            , bottom = matched.side.right
            , left = matched.side.bottom
            , asSet = matched.side.asSet
            }
    in
    { image = matched.image
    , side = rotatedSide
    --, oriented = Array.initialize sideLength (\x -> Array.initialize sideLength (\y -> destinationValue (x, y)))
    --, oriented = matched.image.pixels
    }



flip : Image -> Image
flip image =
    let
        destinationValue currentRow index =
            Array.get (sideLength - 1 - index) currentRow
                |> Maybe.withDefault False
    in
    { id = image.id
    --, pixels = image.pixels
      --, pixels = Array.map (\row -> Array.initialize sideLength (destinationValue row)) image.pixels
    , numericRepresentation = image.numericRepresentation
    }


getPixel : (Int, Int) -> Matched -> Bool
getPixel (x, y) image =
    --Array.get x image.oriented
    --    |> Maybe.andThen (Array.get y)
    --    |> Maybe.withDefault False
    False


setPixel : (Int, Int) -> Bool -> Matched -> Matched
setPixel (x, y) pixel image =
    --let
    --    valueAtX =
    --        Array.get x image.oriented
    --in
    --case valueAtX of
    --    Just array ->
    --        { image | oriented = Array.set x (Array.set y pixel array) image.oriented }
    --
    --    Nothing ->
    --        image
    image


overlappingElement : TaggedSet a comparable -> TaggedSet a comparable -> List (Tagged a comparable)
overlappingElement set1 set2 =
    Tagged.Set.intersect set1 set2
            |> Tagged.Set.toList


imageOverlap : Image -> Image -> List SideValue
imageOverlap image1 image2 =
    overlappingElement (bothSides image1) (bothSides image2)


bothSides : Image -> TaggedSet SideValueTag Int
bothSides image =
    Tagged.Set.union (Tuple.first image.numericRepresentation).asSet (Tuple.second image.numericRepresentation).asSet


matchingImages : Image -> List Image -> TaggedDict SideValueTag Int ImageId
matchingImages image images =
    List.foldl (\otherImage acc ->
            if otherImage.id == image.id then
                acc
            else
                List.foldl (\sideValue -> Tagged.Dict.insert sideValue otherImage.id) acc (imageOverlap image otherImage)
        )
        Tagged.Dict.empty images


findMatchingSquares : List Image -> TaggedDict ImageIdTag Int (TaggedDict SideValueTag Int ImageId)
findMatchingSquares images =
    List.foldl (\image acc -> Tagged.Dict.insert image.id (matchingImages image images ) acc) Tagged.Dict.empty images


idsWhereOnlyTwoMatches : TaggedDict ImageIdTag Int (TaggedDict SideValueTag Int ImageId) -> List ImageId
idsWhereOnlyTwoMatches dict =
    Tagged.Dict.foldl (\key value acc ->
            if Tagged.Dict.size value == 2 then
                key :: acc
            else
                acc
        ) [] dict

idsWhereOnlyFourMatches : TaggedDict ImageIdTag Int (TaggedDict SideValueTag Int ImageId) -> List ImageId
idsWhereOnlyFourMatches dict =
    Tagged.Dict.foldl (\key value acc ->
            if Tagged.Dict.size value == 4 then
                key :: acc
            else
                acc
        ) [] dict


multiply : List ImageId -> Int
multiply ints =
    List.foldl (\int acc -> acc * Tagged.untag int ) 1 ints


assembleGrid : List Image -> Result String (Dict (Int, Int) Matched)
assembleGrid images =
    let
        imagesById =
            images
                |> List.map (\image -> (image.id, image))
                |> Tagged.Dict.fromList

        matchingSquares =
            findMatchingSquares images

        starting =
            idsWhereOnlyFourMatches matchingSquares
                --|> List.Extra.last
                |> List.filterMap (\id -> Maybe.map2 makeIntoStartingImage (Tagged.Dict.get id imagesById) (Tagged.Dict.get id matchingSquares) |> Maybe.andThen Result.toMaybe)
                --|> (List.map (\beginning -> assembleGridHelp imagesById (Tagged.Dict.remove beginning.image.id imagesById) (1,0) beginning (Dict.insert (0,0) beginning Dict.empty)))
                --|> List.foldl (\res acc ->
                --    case res of
                --        Ok solution ->
                --            res
                --
                --        Err e ->
                --            acc
                --    ) (Err "no solutions found")
                --|> List.foldl (\res acc ->
                --    case res of
                --        Ok _ ->
                --            acc
                --
                --        Err err ->
                --            case acc of
                --                Ok _ ->
                --                    Err [err]
                --
                --                Err errAcc ->
                --                    Err (err  :: errAcc )
                --) (Err [])

    in
    case starting of
        beginning :: otherOptions ->
            --Ok (Dict.insert (0,0) beginning Dict.empty)
            assembleGridHelp imagesById (Tagged.Dict.remove beginning.image.id imagesById) (1,0) beginning (Dict.insert (0,0) beginning Dict.empty)

        [] ->
            Err "No images with only 2 matches"

        --Ok multipleOptions ->
        --    Err (List.length multipleOptions |> String.fromInt |> (++) "number of starting options: ")

        --Ok _ ->
        --    starting

        --Err err ->
        --    Err err


makeIntoStartingImage : Image -> (TaggedDict SideValueTag Int ImageId) -> Result String Matched
makeIntoStartingImage image matchingSquaresResult =
    let
        matchingSideValues =
            Tagged.Dict.keys matchingSquaresResult
                |> Tagged.Set.fromList
    in
    case (overlappingElement (Tuple.first image.numericRepresentation).asSet matchingSideValues, overlappingElement (Tuple.second image.numericRepresentation).asSet matchingSideValues) of
        (x :: _, _) ->
            { image = image
            , side = Tuple.first image.numericRepresentation
            --, oriented = image.pixels
            }
                |> rotateIntoTopLeft 0 matchingSideValues

        ([], x :: _) ->
            { image = image
            , side = Tuple.second image.numericRepresentation
            --, oriented = (flip image).pixels
            }
                |> rotateIntoTopLeft 0 matchingSideValues

        ([], []) ->
            Err "didn't overlap on any side"


rotateIntoTopLeft : Int -> TaggedSet SideValueTag Int -> Matched -> Result String Matched
rotateIntoTopLeft attempts theNumbersThatMatch matched =
    if attempts > 3 then
        Err ("couldn't rotate\nlooking for (r,b): " ++ (String.fromInt (Tagged.untag matched.side.right)) ++ ", " ++ (String.fromInt (Tagged.untag matched.side.bottom)) ++ "\n and checking against" ++ String.join ", " (List.map (Tagged.untag >> String.fromInt) <| Tagged.Set.toList theNumbersThatMatch) ++ " but got\n" ++ displayImageSides matched.image)
    else if Tagged.Set.member matched.side.right theNumbersThatMatch && Tagged.Set.member matched.side.bottom theNumbersThatMatch then
        Ok matched
    else
        rotateIntoTopLeft (attempts + 1) theNumbersThatMatch (rotateClockwise matched)


imageLength : Int
imageLength =
    12


getMatchingImageForSide : TaggedDict ImageIdTag Int Image ->  TaggedDict SideValueTag Int ImageId -> SideValue -> Maybe Image
getMatchingImageForSide imagesById matchingSquares sideValue =
    Tagged.Dict.get sideValue matchingSquares
        |> Maybe.andThen (\imageId -> Tagged.Dict.get imageId imagesById)


assembleGridHelp : TaggedDict ImageIdTag Int Image ->   TaggedDict ImageIdTag Int Image -> (Int, Int) -> Matched -> Dict (Int, Int) Matched -> Result String (Dict (Int, Int) Matched)
assembleGridHelp allImages imagesById (x, y) prev acc =
    --if (x == 1) then
     if Tagged.Dict.isEmpty imagesById then
        Ok acc
     else if x + 1 == imageLength then
        -- the previous and all that are messed up...
        -- insert one that matches to the right number of prev
        --case Maybe.andThen (\matching -> getMatchingImageForSide imagesById matching prev.side.right) (Tagged.Dict.get prev.image.id matchingSquares) of
        case (Tagged.Dict.values (Tagged.Dict.filter (\_ image -> matchesOnOneSide (flipNumber prev.side.right) image) imagesById)) of
            image :: [] ->
                let
                    thisMatch =
                        orientToSideWithValueOf .left (flipNumber prev.side.right) image

                    prevMatchForNewLine =
                        Dict.get (0,y) acc
                            |> Result.fromMaybe "could not find the previous row match"
                in
                prevMatchForNewLine
                    |> Result.andThen (\nextPrev -> assembleGridHelp allImages (Tagged.Dict.remove image.id imagesById) (0, y + 1) nextPrev (Dict.insert (x, y) thisMatch acc))

            _ :: xs ->
                Err "1. found too many!!!"
            [] ->
                Err ("1. Could not find a match to go at position (" ++ String.fromInt x ++ ", " ++ String.fromInt y ++ ") and after image " ++ (displayImageSides prev.image)
                        ++ ". trying to find: " ++ String.fromInt (Tagged.untag (flipNumber prev.side.right))
                    ++ "\nfound so far:\n" ++ printOutImageIdsInSolution acc)
     else if x == 0 then
        --insert one that matches to the bottom of prev
        case (Tagged.Dict.values (Tagged.Dict.filter (\_ image -> matchesOnOneSide (flipNumber prev.side.bottom) image) imagesById)) of
        --case Maybe.andThen (\matching -> getMatchingImageForSide imagesById matching (flipNumber prev.side.bottom)) (Tagged.Dict.get prev.image.id matchingSquares) of
        --case List.Extra.find (matchesOnOneSide (flipNumber prev.side.bottom)) (getMatchingImages matchingSquares imagesById prev) of
            image :: [] ->
                let
                    thisMatch =
                        orientToSideWithValueOf .top (flipNumber prev.side.bottom) image
                in
                assembleGridHelp allImages (Tagged.Dict.remove image.id imagesById) (x + 1, y) thisMatch (Dict.insert (x, y) thisMatch acc)


            _ :: xs ->
                Err "2. found too many!!!"

            [] ->
                Err ("2. Could not find a match to go at position (" ++ String.fromInt x ++ ", " ++ String.fromInt y ++ ") and after image " ++ (displayImageSides prev.image)
                    ++ ". trying to find: " ++ String.fromInt (Tagged.untag (flipNumber prev.side.bottom))
                    ++ "\nfound so far:\n" ++ printOutImageIdsInSolution acc)
     else
        case (Tagged.Dict.values (Tagged.Dict.filter (\_ image -> matchesOnOneSide (flipNumber prev.side.right) image) imagesById)) of
        --case Maybe.andThen (\matching -> getMatchingImageForSide imagesById matching (flipNumber prev.side.right)) (Tagged.Dict.get prev.image.id matchingSquares) of
            image :: [] ->
                let
                    thisMatch =
                        orientToSideWithValueOf .left (flipNumber prev.side.right) image
                in
                    assembleGridHelp allImages (Tagged.Dict.remove image.id imagesById) (x + 1, y) thisMatch (Dict.insert (x, y) thisMatch acc)


            _ :: xs ->
                Err "3. found too many!!!"

            [] ->
                Err ("3. Could not find a match to go at position (" ++ String.fromInt x ++ ", " ++ String.fromInt y ++ ") and after image " ++ (displayImageSides prev.image)
                    ++ "\ntrying to find: " ++ String.fromInt (Tagged.untag (flipNumber prev.side.right))
                    ++ "\nfound so far:\n" ++ printOutImageIdsInSolution acc
                    ++ "\nImages with that:\n" ++ (Tagged.Dict.values allImages |> List.filter (\image -> Tagged.Set.member prev.side.right (bothSides image)) |> List.map displayImageSides |> String.join "\n")
                    ++ "\nImages with any overlap:\n" ++ (Tagged.Dict.values imagesById |> List.filter (\image -> overlappingElement prev.side.asSet (bothSides image) |> List.isEmpty |> not) |> List.map displayImageSides |> String.join "\n")
                    )


isJust : Maybe a -> Bool
isJust maybe =
    case maybe of
        Just _ ->
            True

        Nothing ->
            False


printOutImageIdsInSolution : Dict (Int, Int) Matched -> String
printOutImageIdsInSolution gridSoFar =
    Dict.toList gridSoFar
        |> List.map (\(location, value) -> (String.fromInt (Tuple.first location)) ++ ", " ++ (String.fromInt (Tuple.second location)) ++ ": " ++ (String.fromInt (Tagged.untag value.image.id)))
        |> String.join "\n"


sideDisplay : NumericRepresentation -> String
sideDisplay numericRepresentation =
    "t:" ++ String.fromInt (Tagged.untag numericRepresentation.top)
        ++ "r:" ++ String.fromInt (Tagged.untag numericRepresentation.right)
        ++ "b:" ++ String.fromInt (Tagged.untag numericRepresentation.bottom)
        ++ "l:" ++ String.fromInt (Tagged.untag numericRepresentation.left)


displayImageSides : Image -> String
displayImageSides image =
    "image:" ++ String.fromInt (Tagged.untag image.id) ++ "\nside1\n" ++ sideDisplay (Tuple.first image.numericRepresentation)
        ++ "\nside2\n" ++ sideDisplay (Tuple.second image.numericRepresentation)


matchesOnOneSide : SideValue -> Image -> Bool
matchesOnOneSide value image =
    let
        (front, back) =
            image.numericRepresentation
    in
    Tagged.Set.member value front.asSet || Tagged.Set.member value back.asSet


orientToSideWithValueOf : (NumericRepresentation -> SideValue) -> SideValue -> Image -> Matched
orientToSideWithValueOf toSide toValueOf image =
    if Tagged.Set.member toValueOf (Tuple.first image.numericRepresentation).asSet then
        { image = image
        , side = Tuple.first image.numericRepresentation
        --, oriented = image.pixels
        }
            |> rotateUntilValueOnSide toSide toValueOf
    else
        { image = image
        , side = Tuple.second image.numericRepresentation
        --, oriented = (flip image).pixels
        }
            |> rotateUntilValueOnSide toSide toValueOf


rotateUntilValueOnSide : (NumericRepresentation -> SideValue) -> SideValue -> Matched -> Matched
rotateUntilValueOnSide toSide value matched =
    if toSide matched.side == value then
        matched
    else
        rotateUntilValueOnSide toSide value (rotateClockwise matched)


getMatchingImages : TaggedDict ImageIdTag Int (TaggedDict SideValueTag Int ImageId) -> TaggedDict ImageIdTag Int Image -> Matched -> List Image
getMatchingImages matchingSquares imagesById matched =
    Tagged.Dict.get matched.image.id matchingSquares
        |> Maybe.map (Tagged.Dict.toList >> List.filterMap (\(value, id) -> Tagged.Dict.get id imagesById))
        |> Maybe.withDefault []


-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (findMatchingSquares >> idsWhereOnlyFourMatches >> multiply)

countNumbers : (Int, List Int) -> String
countNumbers (int, ints) =
    String.fromInt int ++ ": " ++ String.fromInt (1 + List.length ints)


displayValues : Image -> String
displayValues image =
    let
        (front, back) =
            image.numericRepresentation
    in
    [ front.top
    , front.right
    , front.bottom
    , front.left
    , back.top
    , back.right
    , back.bottom
    , back.left
    ]
        |> List.map (Tagged.untag >> String.fromInt)
        |> String.join ","

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.andThen (assembleGrid >> Result.map (\grid -> Dict.size grid))
        --|> Result.andThen (findMatchingSquares >> Tagged.Dict.map (\key value -> String.fromInt (Tagged.untag key) ++ ": " ++ String.fromInt (Tagged.Dict.size value) ) >> Tagged.Dict.values >> String.join "\n" >> Err)
        --|> Result.andThen (List.take 5 >> List.map displayValues >> String.join "\n" >> Err)
        --|> Result.andThen (List.map (\image -> Tagged.Set.size (bothSides image) |> String.fromInt) >> String.join "," >> Err)
        --|> Result.andThen (List.length >> Ok)
