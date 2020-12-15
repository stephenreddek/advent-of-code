module AdventOfCode2020.Day15 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import Dict exposing (Dict)
import List.Extra
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Util.Parser

-- Types -----------------------------------------------------------------------
type alias GameState =
    { lastRoundPlayed : Int
    , lastNumberSpoken : Int
    , memory : Dict Int Int
    }

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List Int)
parseInput input =
    Ok [16,1,0,18,12,14,19]

-- Functions -------------------------------------------------------------------
playGame : Int -> List Int -> Int
playGame roundsToPlay startingList =
    numberSpokenForRound roundsToPlay (setupGameState startingList)


setupGameState : List Int -> GameState
setupGameState startingList =
    { lastRoundPlayed = List.length startingList
    , lastNumberSpoken = Maybe.withDefault 0 (List.Extra.last startingList)
    , memory = List.Extra.indexedFoldl (\index num -> Dict.insert num (index + 1)) Dict.empty startingList
    }


numberSpokenForRound : Int -> GameState -> Int
numberSpokenForRound roundsToPlay ({ lastRoundPlayed, lastNumberSpoken, memory } as gameState) =
    if roundsToPlay == lastRoundPlayed then
        lastNumberSpoken
    else
        numberSpokenForRound roundsToPlay (playRound gameState)


playRound : GameState -> GameState
playRound gameState =
    let
        thisRound =
            gameState.lastRoundPlayed + 1

        thisNumber =
            case Dict.get gameState.lastNumberSpoken gameState.memory of
                Just roundItWasSpoken ->
                    gameState.lastRoundPlayed - roundItWasSpoken

                Nothing ->
                    0
    in
    { lastRoundPlayed = thisRound
    , lastNumberSpoken = thisNumber
    , memory = Dict.insert gameState.lastNumberSpoken gameState.lastRoundPlayed gameState.memory
    }


-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (playGame 2020)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.map (playGame 30000000)
