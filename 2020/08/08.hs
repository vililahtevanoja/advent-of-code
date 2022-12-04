import           Data.Char                    (isAlpha, isDigit)
import           Data.Maybe                   (isJust)
import           Text.ParserCombinators.ReadP (ReadP, count, munch, readP_to_S,
                                               satisfy, string)

data Operation = Acc | Jmp | Nop | Unknown deriving (Eq, Show)

toOp :: String -> Operation
toOp "acc" = Acc
toOp "jmp" = Jmp
toOp "nop" = Nop
toOp _     = Unknown

data Instruction = Instruction {
  op  :: Operation,
  arg :: Int
} deriving (Show)

instructionP :: ReadP Instruction
instructionP = do
  operator <- munch isAlpha
  _ <- string " "
  sign <- count 1 $ satisfy (\c -> c == '-' || c == '+')
  argument <- munch isDigit
  return (Instruction (toOp operator) (read (minusOrEmpty sign ++ argument) :: Int))
  where minusOrEmpty c = if c == "-" then c else ""

flipNopJmp :: (Int, Instruction) -> (Int, Instruction)
flipNopJmp (i, ins) = case op ins of
                        Acc -> (i, ins)
                        Nop -> (i, Instruction{op=Jmp, arg=arg ins})
                        Jmp -> (i, Instruction{op=Nop, arg=arg ins})
                        _   -> error "wtf"

getNopJmpPositions :: [(Int, Instruction)] -> [Int]
getNopJmpPositions inss = map fst $ filter (\(_,ins) -> op ins == Nop || op ins == Jmp) inss

transformNthElem :: (a -> a) -> Int -> [a] -> [a]
transformNthElem f n xs =
  if n < length xs
    then take n xs ++ replacement ++ drop (n+1) xs
    else xs
  where replacement = case drop n xs of
          x:_ -> [f x]
          []  -> []

getInstructionFlipPermutations :: [Int] -> [(Int, Instruction)] -> [[(Int, Instruction)]]
getInstructionFlipPermutations ps inss = map (\p -> transformNthElem flipNopJmp p inss) ps

getAccValue :: [(Int, Instruction)] -> Int
getAccValue instructions = getAccValue' [] 0 0
  where
    getAccValue' :: [Int] -> Int -> Int -> Int
    getAccValue' visits acc idx
      | idx `notElem` visits =
        case lookup idx instructions of
          Just (Instruction Acc a) -> getAccValue' (idx:visits) (acc + a) (idx+1)
          Just (Instruction Jmp a) -> getAccValue' (idx:visits) acc (idx+a)
          Just (Instruction Nop _) -> getAccValue' (idx:visits) acc (idx+1)
          Just (Instruction _ _) -> acc
          Nothing -> acc
      | otherwise = acc -- second visit of item

getAccValueStrict :: [(Int, Instruction)] -> Maybe Int
getAccValueStrict instructions = getAccValueStrict' [] 0 0
  where
    getAccValueStrict' :: [Int] -> Int -> Int -> Maybe Int
    getAccValueStrict' visits acc idx
      | idx `notElem` visits =
        case lookup idx instructions of
          Just ins | op ins == Acc -> getAccValueStrict' (idx:visits) (acc + arg ins) (idx+1)
          Just ins | op ins == Jmp -> getAccValueStrict' (idx:visits) acc (idx+arg ins)
          Just ins | op ins == Nop -> getAccValueStrict' (idx:visits) acc (idx+1)
          Nothing -> Just acc
      | otherwise = Nothing -- second visit of item

main :: IO ()
main = do
  inputs <- lines <$> readFile "input.txt"
  let instructions = map fst $ concatMap (readP_to_S instructionP) inputs
  let instructionsWithPositions = zip [0..] instructions
  let p1 = getAccValue instructionsWithPositions
  putStrLn $ "Part 1: " ++ show p1

  let nopJmpPositions = getNopJmpPositions instructionsWithPositions
  let candidates = getInstructionFlipPermutations nopJmpPositions instructionsWithPositions
  let p2 = case head $ filter isJust $ map getAccValueStrict candidates of
            Just n  -> n
            Nothing -> error "wtfp2"

  putStrLn $ "Part 2: " ++ show p2
