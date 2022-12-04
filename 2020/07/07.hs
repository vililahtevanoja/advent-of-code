import           Data.Either        (rights)
import qualified Data.Map.Strict    as M
import           Data.Maybe         (fromMaybe)
import           Text.Parsec
import           Text.Parsec.String (Parser)
import           Text.Read          (readMaybe)

type Bag = String

type BagDef = (Bag, [(Bag, Int)])

bagDefP :: Parser BagDef
bagDefP = do
  desc <- many1 letter
  _ <- space
  color <- many1 letter
  let topBag = unwords [desc, color]
  _ <- space
  _ <- string "bags"
  _ <- space
  _ <- string "contain"
  _ <- space
  contained <- try $ many1 noBags <|> many1 innerBagP
  return (topBag, contained)

innerBagP :: Parser (Bag, Int)
innerBagP = do
  bCount <- digit
  _ <- space
  desc <- many1 letter
  _ <- space
  color <- many1 letter
  let name = unwords [desc, color]
  _ <- space
  _ <- optional $ try $ string "bags"
  _ <- optional $ try $ string "bag"
  _ <- try (char ',' <|> char '.')
  _ <- optional $ try space
  return (name, fromMaybe (-1) (readMaybe [bCount] :: Maybe Int))

noBags :: Parser (Bag, Int)
noBags = do
  _ <- string "no other bags."
  return ("", 0)

numBagsCanContain :: Bag -> M.Map Bag [Bag] -> Int
numBagsCanContain name bagMap = length $ M.filterWithKey canCarryBag $ M.delete name bagMap
  where
    canCarryBag :: Bag -> [Bag] -> Bool
    canCarryBag b [] = b == name
    canCarryBag b bags = (b == name) || any (\ib -> canCarryBag ib (M.findWithDefault [] ib bagMap)) bags

innerBagCount :: Bag -> M.Map Bag [(Bag, Int)] -> Int
innerBagCount name bagCountMap = innerBagCount' name
  where
    innerBagCount' :: Bag -> Int
    innerBagCount' name =sum(map (\(b,c) -> c + c * innerBagCount' b) bagCount)
      where
        bagCount = M.findWithDefault [] name bagCountMap

main :: IO ()
main = do
  inputs <- lines <$> readFile "input.txt"
  let parsed = map (parse bagDefP "input.txt") inputs
  let bags  = filter (not . null . fst) $ rights parsed
  let bagMap = map fst <$> M.fromList bags
  let p1 = numBagsCanContain "shiny gold" bagMap
  putStrLn $ "Part 1: " ++ show p1

  let bagCountMap = M.fromList bags
  let p2 = innerBagCount "shiny gold" bagCountMap
  putStrLn $ "Part 2: " ++ show p2
