module AdventOfCode2020.Day21 exposing (part1, part2)

-- Imports ---------------------------------------------------------------------
import Basics.Extra as Basics
import Parser exposing (Parser, Trailing (..), Step (..), (|.), (|=))
import Set exposing (Set)
import Util.Parser
import Tagged exposing (Tagged)
import Tagged.Dict exposing (TaggedDict)
import Tagged.Set exposing (TaggedSet)


-- Types -----------------------------------------------------------------------
type alias Food =
    { ingredients : TaggedSet IngredientTag String
    , allergens : TaggedSet AllergenTag String
    }


type IngredientTag = IngredientTag

type alias Ingredient = Tagged IngredientTag String

type AllergenTag = AllergenTag

type alias Allergen = Tagged AllergenTag String

-- Input -----------------------------------------------------------------------
--
parseInput : String -> Result String (List Food)
parseInput input =
    Parser.run inputParser input
        |> Result.map (identity)
        |> Result.mapError Util.Parser.deadEndsToString

--
inputParser : Parser (List Food)
inputParser =
    Parser.loop [] (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done (List.reverse xs))
              |. Parser.end
            , Parser.succeed (\x -> Loop (x :: xs))
                |= foodParser
            ]
   )


foodParser : Parser Food
foodParser =
    Parser.succeed Food
        |= ingredientsParser
        |= allergensParser
        |. Parser.token "\n"


ingredientsParser : Parser (TaggedSet IngredientTag String)
ingredientsParser =
    Parser.loop Tagged.Set.empty (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done xs)
                |. Parser.token "(contains "
            , Parser.succeed (\ingredient -> Loop (Tagged.Set.insert (Tagged.tag ingredient) xs))
                |= Parser.getChompedString (Parser.chompUntil " ")
                |. Parser.spaces
            ]
    )


allergensParser : Parser (TaggedSet AllergenTag String)
allergensParser =
    Parser.loop Tagged.Set.empty (\xs ->
        Parser.oneOf
            [ Parser.succeed (Done xs)
                |. Parser.token ")"
            , Parser.succeed (\ingredient -> Loop (Tagged.Set.insert (Tagged.tag ingredient) xs))
                |= Parser.getChompedString (Parser.chompWhile Char.isAlpha)
                |. Parser.oneOf
                    [ Parser.token ", "
                    , Parser.succeed ()
                    ]
            ]
    )

-- Functions -------------------------------------------------------------------
safeIngredients : List Food -> TaggedSet IngredientTag String
safeIngredients foods =
    let
        allIngredients =
            List.foldl (\food acc -> Tagged.Set.union acc food.ingredients ) Tagged.Set.empty foods

        updateAllergenAcc : TaggedSet IngredientTag String -> Maybe (TaggedSet IngredientTag String) -> Maybe (TaggedSet IngredientTag String)
        updateAllergenAcc ingredients maybeIngredients =
            maybeIngredients
                |> Maybe.withDefault Tagged.Set.empty
                |> Tagged.Set.union ingredients
                |> Just


        ingredientsThatCouldContainAllergen =
            List.foldl (\food acc -> Tagged.Set.foldl (\allergen allergenAcc -> Tagged.Dict.update allergen (updateAllergenAcc food.ingredients) allergenAcc) acc food.allergens ) Tagged.Dict.empty foods

        unsafeIngredients =
            findIngredientsThatContainAllergen allIngredients foods ingredientsThatCouldContainAllergen
                |> Tagged.Dict.values
                |> List.foldl (\ingredientSet acc -> Tagged.Set.union ingredientSet acc) Tagged.Set.empty
    in
    Tagged.Set.diff allIngredients unsafeIngredients


ingredientCannotContainAllergen : Allergen -> Ingredient -> TaggedDict AllergenTag String (TaggedSet IngredientTag String) -> TaggedDict AllergenTag String (TaggedSet IngredientTag String)
ingredientCannotContainAllergen allergen ingredient ingredientsThatCouldContainAllergen =
    Tagged.Dict.update allergen (Maybe.map (Tagged.Set.remove ingredient)) ingredientsThatCouldContainAllergen


findIngredientsThatContainAllergen : TaggedSet IngredientTag String -> List Food -> TaggedDict AllergenTag String (TaggedSet IngredientTag String) -> TaggedDict AllergenTag String (TaggedSet IngredientTag String)
findIngredientsThatContainAllergen allIngredients foods ingredientsThatCouldContainAllergen =
    if isSolved ingredientsThatCouldContainAllergen then
        ingredientsThatCouldContainAllergen
    else
        let
            iteration =
                List.foldl searchForAllergenCausesInFood ingredientsThatCouldContainAllergen foods

            afterRemovingIngredientsThatAreAlreadySolved =
                iteration
                    |> removeIngredientsAsPossibleCausesIfTheyAreSolved
        in
        findIngredientsThatContainAllergen allIngredients foods afterRemovingIngredientsThatAreAlreadySolved
        -- go through the foods
        -- if an allergen is present and one of the ingredients that can possibly cause it is not present, then remove it as a possibility
        -- once only one ingredient is left for an allergen, it is the cause
                    -- remove this ingredient from the possible causes of other allergens
        -- keep iterating


isSolved : TaggedDict AllergenTag String (TaggedSet IngredientTag String) -> Bool
isSolved ingredientsThatCouldContainAnAllergen =
    Tagged.Dict.toList ingredientsThatCouldContainAnAllergen
        |> List.all (Tuple.second >> Tagged.Set.size >> (==) 1)


removeIngredientsAsPossibleCausesIfTheyAreSolved : TaggedDict AllergenTag String (TaggedSet IngredientTag String) ->  TaggedDict AllergenTag String (TaggedSet IngredientTag String)
removeIngredientsAsPossibleCausesIfTheyAreSolved ingredientsThatCouldContainAnAllergen =
    Tagged.Dict.foldl (\allergen ingredients acc ->
        if Tagged.Set.size ingredients == 1 then
            Tagged.Dict.map (\otherAllergen otherIngredientSet ->
                if allergen /= otherAllergen then
                    Tagged.Set.diff otherIngredientSet ingredients
                else
                    ingredients
            ) acc
        else
            acc
    ) ingredientsThatCouldContainAnAllergen ingredientsThatCouldContainAnAllergen


searchForAllergenCausesInFood : Food -> TaggedDict AllergenTag String (TaggedSet IngredientTag String) ->  TaggedDict AllergenTag String (TaggedSet IngredientTag String)
searchForAllergenCausesInFood food ingredientsThatCouldContainAnAllergen =
    Tagged.Set.foldl (\allergen acc ->
        case Tagged.Dict.get allergen ingredientsThatCouldContainAnAllergen of
            Just possibleCauses ->
                if Tagged.Set.size possibleCauses <= 1 then
                    -- Already solved
                    acc
                else
                    let
                        ingredientsThatMustNotBeTheCause =
                            Tagged.Set.diff possibleCauses food.ingredients

                        remainingPossibleCauses =
                            Tagged.Set.diff possibleCauses ingredientsThatMustNotBeTheCause
                    in
                    Tagged.Dict.insert allergen remainingPossibleCauses acc

            Nothing ->
                acc
    ) ingredientsThatCouldContainAnAllergen food.allergens


solvePart1 : List Food -> Int
solvePart1 foods =
    safeIngredients foods
        |> Tagged.Set.toList
        |> List.map (countIngredientOccurrence foods)
        |> List.sum

solvePart2 : List Food -> String
solvePart2 foods =
    let
            allIngredients =
                List.foldl (\food acc -> Tagged.Set.union acc food.ingredients ) Tagged.Set.empty foods

            updateAllergenAcc : TaggedSet IngredientTag String -> Maybe (TaggedSet IngredientTag String) -> Maybe (TaggedSet IngredientTag String)
            updateAllergenAcc ingredients maybeIngredients =
                maybeIngredients
                    |> Maybe.withDefault Tagged.Set.empty
                    |> Tagged.Set.union ingredients
                    |> Just


            ingredientsThatCouldContainAllergen =
                List.foldl (\food acc -> Tagged.Set.foldl (\allergen allergenAcc -> Tagged.Dict.update allergen (updateAllergenAcc food.ingredients) allergenAcc) acc food.allergens ) Tagged.Dict.empty foods

            unsafeIngredients =
                findIngredientsThatContainAllergen allIngredients foods ingredientsThatCouldContainAllergen
        in
        unsafeIngredients
            |> Tagged.Dict.toList
            |> List.sortBy (Tuple.first >> Tagged.untag)
            |> List.map (\(_, ingredients) -> Tagged.Set.toList ingredients |> List.map (Tagged.untag) |> String.join "-") --hacky because we know there's only one
            |> String.join ","

countIngredientOccurrence : List Food -> Ingredient -> Int
countIngredientOccurrence foods ingredient =
    List.foldl (\food tally ->
        if Tagged.Set.member ingredient food.ingredients then
            tally + 1
        else
            tally
    ) 0 foods

-- Solvers ---------------------------------------------------------------------
part1 : String -> Result String Int
part1 input =
    parseInput input
        |> Result.map (solvePart1)

part2 : String -> Result String String
part2 input =
    parseInput input
        |> Result.map (solvePart2)
