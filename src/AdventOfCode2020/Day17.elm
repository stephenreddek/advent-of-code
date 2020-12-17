module AdventOfCode2020.Day17 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import Dict exposing (Dict)
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Util.Parser

-- Types -----------------------------------------------------------------------
type PocketDimension a =
    PocketDimension DimensionInfo (Dict Int (Dict Int (Dict Int (Dict Int a))))


type alias DimensionInfo =
    { x : (Int, Int)
    , y : (Int, Int)
    , z : (Int, Int)
    , w : (Int, Int)
    }


type Cube
    = Active
    | Inactive

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (PocketDimension Cube)
parseInput input =
    Parser.run inputParser input
        |> Result.map (pocketDimensionFromSlice)
        |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (List (List Cube))
inputParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
              |. Parser.end
            , Parser.succeed (\x -> Loop (x :: xs))
                |= rowParser
            ]
   )


rowParser : Parser (List Cube)
rowParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
              |. Parser.token "\n"
            , Parser.succeed (\x -> Loop (x :: xs))
                |= Parser.oneOf
                    [ Parser.succeed Active
                        |. Parser.token "#"
                    , Parser.succeed Inactive
                        |. Parser.token "."
                    ]
            ]
   )


pocketDimensionFromSlice : List (List Cube) -> PocketDimension Cube
pocketDimensionFromSlice rows =
    let
        sizeOfX =
            List.length rows

        sizeOfY =
            rows
                |> List.head
                |> Maybe.map List.length
                |> Maybe.withDefault 0
    in
    List.Extra.indexedFoldl (\x cols xs ->
        Dict.insert x (List.Extra.indexedFoldl (\y cube ys ->
                Dict.insert y (Dict.singleton 0 (Dict.singleton 0 cube)) ys) Dict.empty cols
            ) xs
        ) Dict.empty rows
        |> PocketDimension
            { x = (0, sizeOfX - 1)
            , y = (0, sizeOfY - 1)
            , z = (0, 0)
            , w = (0, 0)
            }


-- Functions -------------------------------------------------------------------
indexedMap : (((Int, Int), (Int, Int)) -> a -> b) -> PocketDimension a -> PocketDimension b
indexedMap func (PocketDimension sizes xs) =
    Dict.map (\x ys ->
            Dict.map (\y zs ->
                    Dict.map (\z ws ->
                            Dict.map (\w val -> func ((x, y), (z, w)) val) ws
                    ) zs
                ) ys
        ) xs
        |> PocketDimension sizes


foldl : (((Int, Int), (Int, Int)) -> a -> b -> b) -> b -> PocketDimension a -> b
foldl func acc (PocketDimension sizes pocketDimension) =
    Dict.foldl (\x ys xAcc ->
            Dict.foldl (\y zs yAcc ->
                    Dict.foldl (\z ws zAcc ->
                            Dict.foldl (\w val wAcc -> func ((x, y), (z, w)) val wAcc)
                                zAcc ws
                        ) yAcc zs
                ) xAcc ys
        ) acc pocketDimension


getAt : ((Int, Int), (Int, Int)) -> PocketDimension a -> Maybe a
getAt ((x, y), (z, w)) (PocketDimension sizes pocketDimension) =
    Dict.get x pocketDimension
        |> Maybe.andThen (\ys -> Dict.get y ys)
        |> Maybe.andThen (\zs -> Dict.get z zs)
        |> Maybe.andThen (\ws -> Dict.get w ws)


wrapInEmpty : a -> PocketDimension a -> PocketDimension a
wrapInEmpty empty (PocketDimension sizes pocketDimension) =
    let
        emptyWDimension : Dict Int a
        emptyWDimension =
            List.range (Tuple.first sizes.w |> subtract1) (Tuple.second sizes.w |> add1)
                |> List.foldl (\w -> Dict.insert w empty) Dict.empty


        emptyZDimension : Dict Int (Dict Int a)
        emptyZDimension =
            List.range (Tuple.first sizes.z |> subtract1) (Tuple.second sizes.z |> add1)
                |> List.foldl (\z -> Dict.insert z emptyWDimension) Dict.empty

        emptyYDimension : Dict Int (Dict Int (Dict Int a))
        emptyYDimension =
            List.range (Tuple.first sizes.y |> subtract1) (Tuple.second sizes.y |> add1)
                |> List.foldl (\y -> Dict.insert y emptyZDimension) Dict.empty

        roundOutZDimension : Dict Int (Dict Int a) -> Dict Int (Dict Int a)
        roundOutZDimension zDimension =
            zDimension
                |> Dict.map (\_ -> roundOutWDimension)
                |> Dict.insert (Tuple.first sizes.z |> subtract1) emptyWDimension
                |> Dict.insert (Tuple.second sizes.z |> add1) emptyWDimension

        roundOutWDimension : Dict Int a -> Dict Int a
        roundOutWDimension wDimension =
            wDimension
                |> Dict.insert (Tuple.first sizes.w |> subtract1) empty
                |> Dict.insert (Tuple.second sizes.w |> add1) empty


        roundOutYDimension : Dict Int (Dict Int (Dict Int a)) -> Dict Int (Dict Int (Dict Int a))
        roundOutYDimension yDimension =
            yDimension
                |> Dict.map (\_ -> roundOutZDimension)
                |> Dict.insert (Tuple.first sizes.y |> subtract1) emptyZDimension
                |> Dict.insert (Tuple.second sizes.y |> add1) emptyZDimension


        subtract1 : Int -> Int
        subtract1 int =
            int - 1

        add1 : Int -> Int
        add1 int =
            int + 1
    in
    pocketDimension
        |> Dict.map (\_ -> roundOutYDimension)
        |> Dict.insert (Tuple.first sizes.x |> subtract1) emptyYDimension
        |> Dict.insert (Tuple.second sizes.x |> add1) emptyYDimension
        |> PocketDimension
            { x = Tuple.mapBoth subtract1 add1 sizes.x
            , y = Tuple.mapBoth subtract1 add1 sizes.y
            , z = Tuple.mapBoth subtract1 add1 sizes.z
            , w = Tuple.mapBoth subtract1 add1 sizes.w
            }

runCycle : PocketDimension Cube -> PocketDimension Cube
runCycle pocketDimension =
    indexedMap (applyRulesToCube pocketDimension) pocketDimension


runCycleNTimes : Int -> PocketDimension Cube -> PocketDimension Cube
runCycleNTimes n dimension =
    List.range 1 n
        |> List.foldl (\runNumber -> runCycle >> wrapInEmpty Inactive) dimension


applyRulesToCube : PocketDimension Cube -> ((Int, Int), (Int, Int)) -> Cube -> Cube
applyRulesToCube pocketDimension location cube =
    case (cube, activeNeighbors pocketDimension location) of
        (Active, 2) -> Active
        (Active, 3) -> Active
        (Inactive, 3) -> Active
        _ -> Inactive



neighborsCoordinatesOffsets : List ((Int, Int), (Int, Int))
neighborsCoordinatesOffsets =
    let
        coordinateModifications =
            [-1, 0, 1]
    in
    List.concatMap (\xOffset ->
            List.concatMap (\yOffset ->
                    List.concatMap (\zOffset ->
                        List.map (\wOffset -> ((xOffset, yOffset), (zOffset, wOffset))) coordinateModifications
                    ) coordinateModifications
                ) coordinateModifications
        ) coordinateModifications
        |> List.filter ((/=) ((0,0),(0,0)))
    -- [(-1,-1,-1),(-1,-1,0),(-1,-1,1),(-1,0,-1),(-1,0,0),(-1,0,1),(-1,1,-1),(-1,1,0),(-1,1,1),(0,-1,-1),(0,-1,0),(0,-1,1),(0,0,-1),(0,0,1),(0,1,-1),(0,1,0),(0,1,1),(1,-1,-1),(1,-1,0),(1,-1,1),(1,0,-1),(1,0,0),(1,0,1),(1,1,-1),(1,1,0),(1,1,1)]
    -- [(-1,-1,-1,-1),(-1,-1,-1,0),(-1,-1,-1,1),(-1,-1,0,-1),(-1,-1,0,0),(-1,-1,0,1),(-1,-1,1,-1),(-1,-1,1,0),(-1,-1,1,1),(-1,0,-1,-1),(-1,0,-1,0),(-1,0,-1,1),(-1,0,0,-1),(-1,0,0,0),(-1,0,0,1),(-1,0,1,-1),(-1,0,1,0),(-1,0,1,1),(-1,1,-1,-1),(-1,1,-1,0),(-1,1,-1,1),(-1,1,0,-1),(-1,1,0,0),(-1,1,0,1),(-1,1,1,-1),(-1,1,1,0),(-1,1,1,1),(0,-1,-1,-1),(0,-1,-1,0),(0,-1,-1,1),(0,-1,0,-1),(0,-1,0,0),(0,-1,0,1),(0,-1,1,-1),(0,-1,1,0),(0,-1,1,1),(0,0,-1,-1),(0,0,-1,0),(0,0,-1,1),(0,0,0,-1),(0,0,0,1),(0,0,1,-1),(0,0,1,0),(0,0,1,1),(0,1,-1,-1),(0,1,-1,0),(0,1,-1,1),(0,1,0,-1),(0,1,0,0),(0,1,0,1),(0,1,1,-1),(0,1,1,0),(0,1,1,1),(1,-1,-1,-1),(1,-1,-1,0),(1,-1,-1,1),(1,-1,0,-1),(1,-1,0,0),(1,-1,0,1),(1,-1,1,-1),(1,-1,1,0),(1,-1,1,1),(1,0,-1,-1),(1,0,-1,0),(1,0,-1,1),(1,0,0,-1),(1,0,0,0),(1,0,0,1),(1,0,1,-1),(1,0,1,0),(1,0,1,1),(1,1,-1,-1),(1,1,-1,0),(1,1,-1,1),(1,1,0,-1),(1,1,0,0),(1,1,0,1),(1,1,1,-1),(1,1,1,0),(1,1,1,1)]




activeNeighbors : PocketDimension Cube -> ((Int, Int), (Int, Int)) -> Int
activeNeighbors pocketDimension ((x, y), (z, w)) =
    List.Extra.count (\((modX, modY), (modZ, modW)) -> isActive pocketDimension ((x + modX, y + modY), (z + modZ, w + modW))) neighborsCoordinatesOffsets


isActive : PocketDimension Cube -> ((Int, Int), (Int, Int)) -> Bool
isActive pocketDimension location =
    case getAt location pocketDimension of
        Just Active ->
            True

        Just Inactive ->
            False

        Nothing ->
            False


countActive : PocketDimension Cube -> Int
countActive =
    foldl (\_ cube count -> if cube == Active then count + 1 else count) 0


tupleToString : ((Int, Int), (Int, Int)) -> String
tupleToString ((x, y), (z, w)) =
    "(" ++ String.fromInt x ++ "," ++ String.fromInt y ++ "," ++ String.fromInt z ++ "," ++ String.fromInt w ++ ")"



-- Solvers ---------------------------------------------------------------------
-- this actually solves for either one.
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (wrapInEmpty Inactive >> runCycleNTimes 6 >> countActive)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.andThen (\_ -> neighborsCoordinatesOffsets |> List.map tupleToString |> String.join "," |> Err)
