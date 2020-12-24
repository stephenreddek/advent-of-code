module AdventOfCode2020.Day22 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import Deque exposing (Deque)
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Set exposing (Set)
import Util.Parser

-- Types -----------------------------------------------------------------------


-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (Deque Int, Deque Int)
parseInput input =
    Parser.run inputParser input
        |> Result.map (identity)
        |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (Deque Int, Deque Int)
inputParser =
    Parser.succeed (Tuple.pair)
        |. Parser.token "Player 1:\n"
        |= parsePlayerDeck
        |. Parser.token "Player 2:\n"
        |= parsePlayerDeck


parsePlayerDeck : Parser (Deque Int)
parsePlayerDeck =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (Deque.fromList (List.reverse xs)))
                |. Parser.token "\n"
            , Parser.succeed (Done (Deque.fromList (List.reverse xs)))
                |. Parser.end
            , Parser.succeed (\x -> Loop (x :: xs))
                |= Parser.int
                |. Parser.token "\n"
            ]
   )

-- Functions -------------------------------------------------------------------
playRound : (Deque Int, Deque Int) -> (Deque Int, Deque Int)
playRound (player1, player2) =
    let
        playCards : (Int, Deque Int) -> (Int, Deque Int) -> (Deque Int, Deque Int)
        playCards (player1Card, player1Remaining) (player2Card, player2Remaining) =
            if player1Card >= player2Card then
                (player1Remaining |> Deque.pushBack player1Card |> Deque.pushBack player2Card, player2Remaining)
            else
                (player1Remaining, player2Remaining |> Deque.pushBack player2Card |> Deque.pushBack player1Card)

    in
    Maybe.map2 playCards (drawTopCard player1) (drawTopCard player2)
        |> Maybe.withDefault (player1, player2)


drawTopCard : Deque Int -> Maybe (Int, Deque Int)
drawTopCard deque =
    case Deque.popFront deque of
        (Just top, rest) ->
            Just (top, rest)

        (Nothing, _) ->
            Nothing


playUntilFinished : (Deque Int, Deque Int) -> Deque Int
playUntilFinished hands =
    let
        (player1After, player2After) =
            playRound hands
    in
    if Deque.isEmpty player1After then
        player2After
    else if Deque.isEmpty player2After then
        player1After
    else
        playUntilFinished (player1After, player2After)


scoreWinningHand : Deque Int -> Int
scoreWinningHand hand =
    Deque.toList hand
        |> List.reverse
        |> List.indexedMap (\index card -> card * (index + 1))
        |> List.sum

type PlayerNumber
    = Player1
    | Player2

type GameResult
    = GameVictory PlayerNumber (Deque Int)
    | GameFailedToDraw

type RoundResult
    = Victory PlayerNumber (Deque Int)
    | Inconclusive (Deque Int, Deque Int)
    | FailedToDraw


type alias History = ( Set String, Set String )


handToString : Deque Int -> String
handToString hand =
   Deque.toList hand
    |> List.map String.fromInt
    |> String.join ","


playersHandsToState : (Deque Int, Deque Int) -> String
playersHandsToState (player1, player2) =
    handToString player1 ++ "-" ++ handToString player2

playRecursiveRound : History -> (Deque Int, Deque Int) -> RoundResult
playRecursiveRound (historyPart1, historyPart2) (player1, player2) =
    if Set.member (handToString player1) historyPart1 || Set.member (handToString player2) historyPart2 then
        Victory Player1 player1
    else
        let
            playCards : (Int, Deque Int) -> (Int, Deque Int) -> RoundResult
            playCards (player1Card, player1Remaining) (player2Card, player2Remaining) =
                let
                    higherWins _ =
                        if player1Card > player2Card then
                            Inconclusive (player1Remaining |> Deque.pushBack player1Card |> Deque.pushBack player2Card, player2Remaining)
                        else
                            Inconclusive (player1Remaining, player2Remaining |> Deque.pushBack player2Card |> Deque.pushBack player1Card)

                    playNestedGame _ =
                        case playRecursiveUntilFinished (Set.empty, Set.empty) (Deque.left player1Card player1Remaining, Deque.left player2Card player2Remaining) of
                            GameVictory Player1 deque ->
                                Inconclusive  (player1Remaining |> Deque.pushBack player1Card |> Deque.pushBack player2Card, player2Remaining)

                            GameVictory Player2 deque ->
                                Inconclusive (player1Remaining, player2Remaining |> Deque.pushBack player2Card |> Deque.pushBack player1Card)

                            GameFailedToDraw ->
                                FailedToDraw
                in
                if player1Card > Deque.length player1Remaining then
                    higherWins ()
                else if player2Card > Deque.length player2Remaining then
                    higherWins ()
                else
                    playNestedGame ()

        in
        Maybe.map2 playCards (drawTopCard player1) (drawTopCard player2)
            |> Maybe.withDefault FailedToDraw


playRecursiveUntilFinished : History -> (Deque Int, Deque Int) -> GameResult
playRecursiveUntilFinished (player1History, player2History) hands =
    case playRecursiveRound (player1History, player2History) hands of
        Victory playerNumber winningHand ->
            GameVictory playerNumber winningHand

        Inconclusive (player1After, player2After) ->
            if Deque.isEmpty player1After then
                GameVictory Player2 player2After
            else if Deque.isEmpty player2After then
                GameVictory Player1 player1After
            else
                playRecursiveUntilFinished ((Set.insert (handToString (Tuple.first hands)) player1History), (Set.insert (handToString (Tuple.second hands)) player2History)) (player1After, player2After)

        FailedToDraw ->
            GameFailedToDraw


scoreGameResult : GameResult -> Result String Int
scoreGameResult gameResult =
    case gameResult of
        GameVictory playerNumber deque ->
            --case playerNumber of
            --    Player1 ->
            --        Err ("victory for 1: " ++ handToString deque)
            --
            --    Player2 ->
            --        Err ("victory for 2: " ++ handToString deque)
            Ok (scoreWinningHand (deque))

        GameFailedToDraw ->
            Err "failed to draw"


-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (playUntilFinished >> scoreWinningHand)

part2 : String -> Result String Int
part2 input =
    parseInput input
        |> Result.andThen (playRecursiveUntilFinished (Set.empty, Set.empty) >> scoreGameResult)
