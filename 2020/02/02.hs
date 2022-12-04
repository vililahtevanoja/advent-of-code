import           Data.Char
import           Text.ParserCombinators.ReadP

data PwInfo = PwInfo
  { constraint1 :: Int,
    constraint2 :: Int,
    letter      :: Char,
    pw          :: String
  }
  deriving (Show)

countCharacters :: String -> Char -> Int
countCharacters s c = foldl (\count char -> if char == c then count + 1 else count) 0 s

lineToPwInfo :: ReadP PwInfo
lineToPwInfo = do
  constraint1 <- munch isDigit
  string "-"
  constraint2 <- munch isDigit
  string " "
  letter <- count 1 $ satisfy isAlphaNum
  string ": "
  pw <- munch isAlphaNum
  return (PwInfo (read constraint1 :: Int) (read constraint2 :: Int) (head letter) pw)

isCompliantPart1 :: PwInfo -> Bool
isCompliantPart1 pwi = do
  let c1 = constraint1 pwi
  let c2 = constraint2 pwi
  let letterCount = countCharacters (pw pwi) (letter pwi)
  letterCount >= c1 && letterCount <= c2

isCompliantPart2 :: PwInfo -> Bool
isCompliantPart2 pwi = do
  let c1 = constraint1 pwi - 1
  let c2 = constraint2 pwi - 1
  let c1Fulfilled = pw pwi !! c1 == letter pwi
  let c2Fulfilled = pw pwi !! c2 == letter pwi
  (c1Fulfilled || c2Fulfilled) && not (c1Fulfilled && c2Fulfilled)

main :: IO ()
main = do
  inputs <- lines <$> readFile "input.txt"
  let pwis = map ((fst . last) . readP_to_S lineToPwInfo) inputs
  let compliantsPart1 = filter isCompliantPart1 pwis
  let compliantsPart2 = filter isCompliantPart2 pwis
  putStrLn $ "Part 1: Compliant password count: " ++ show (length compliantsPart1)
  putStrLn $ "Part 2: Compliant password count: " ++ show (length compliantsPart2)
