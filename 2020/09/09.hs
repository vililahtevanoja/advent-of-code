import           Data.List  (find)
import           Data.Maybe (fromMaybe)
import           Text.Read  (readMaybe)

findP1 :: Int -> [Int] -> (Int, Int, [Int])
findP1 preamble ns = findP1' preamble
  where
    findP1' :: Int -> (Int, Int, [Int])
    findP1' idx
      | idx >= length ns = error "wtf"
      | notValid (ns!!idx) workSet = (idx, ns!!idx, workSet)
      | otherwise = findP1' (idx+1)
      where
          workSet = precedingFromN idx preamble ns

notValid :: Int -> [Int] -> Bool
notValid n ns = not . any (\p -> sum p == n) $ subsequencesN 2 ns

precedingFromN :: Int -> Int -> [a] -> [a]
precedingFromN n len ns'
            | len <= n = take len $ drop (n-len) ns'
            | otherwise = []

subsequencesN :: Int -> [a] -> [[a]]
subsequencesN n xs = if n > len then [] else subsequencesN' xs !! (len-n)
 where
   len = length xs
   subsequencesN' [] = [[[]]]
   subsequencesN' (x:xs) = zipWith (++) ([]:next) (map (map (x:)) next ++ [[]])
    where next = subsequencesN' xs

windows :: Int -> [Int] -> [[Int]]
windows l xs = map (\i -> take l $ drop i xs) [0..length xs-l]

findP2 :: Int -> Int -> [Int] -> Int
findP2 p1 p1Idx xs = subSeqSum $ reverse [2..p1Idx]
  where
    subSeqSum :: [Int] -> Int
    subSeqSum seqLengths = head $ map (\seq -> minimum seq + maximum seq) $ filter (\seq -> sum seq == p1 ) $ concatMap (`windows` xs) seqLengths

main :: IO ()
main = do
  inputs <- lines <$> readFile "input.txt"
  let preamble = 25
  let values = map (\s -> fromMaybe (-1) (readMaybe s :: Maybe Int)) inputs
  let (p1Idx, p1, _) = findP1 preamble values
  putStrLn $ "Part 1: " ++ show p1

  let p2 = findP2 p1 p1Idx values
  putStrLn $ "Part 2: " ++ show p2
