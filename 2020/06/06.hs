
import           Data.List (intersect, union)

isPrefixOf :: (Eq a) => [a] -> [a] -> Bool
isPrefixOf [] _          = True
isPrefixOf _ []          = False
isPrefixOf (x:xs) (y:ys) = (x == y) && xs `isPrefixOf` ys

-- split at sub-sequence, excluding the sub-sequence from split results
splitAtSubSeqExcl :: (Eq a) => [a] -> [a] -> [[a]]
splitAtSubSeqExcl _ [] = []
splitAtSubSeqExcl sub seq = splitAtSubSeqExcl' sub seq [] []
  where
    splitAtSubSeqExcl' _ [] subacc acc = reverse $ reverse subacc:acc
    splitAtSubSeqExcl' sub (x:xs) subacc acc
      | sub `isPrefixOf` (x:xs) = splitAtSubSeqExcl' sub (drop (length sub - 1) xs) [] (reverse subacc:acc)
      | otherwise               = splitAtSubSeqExcl' sub xs (x:subacc) acc

-- split at element, exluding the element from split results
splitAtExcl :: (Eq a) => a -> [a] -> [[a]]
splitAtExcl el = splitAtSubSeqExcl [el]

solve1 :: String -> Int
solve1 inputs =
  let
    groups =  map (splitAtExcl '\n') $ splitAtSubSeqExcl "\n\n" inputs
    groupUnions = map (foldl1 union) groups
  in sum $ map length groupUnions

solve1AltForm :: String -> Int
solve1AltForm inputs = sum $ map length groupUnions
  where
    groupUnions = map (foldl1 union) groups
      where
        groups = map (splitAtExcl '\n') $ splitAtSubSeqExcl "\n\n" inputs


solve2 :: String -> Int
solve2 inputs = do
  let groups = map (splitAtExcl '\n') $ splitAtSubSeqExcl "\n\n" inputs
  let groupIntersections = map (foldl1 intersect) groups
  sum $ map length groupIntersections

main :: IO ()
main = do
  inputs <- readFile "input.txt"
  let p1 = solve1 inputs
  putStrLn $ "Part 1: " ++ show p1
  let p2 = solve2 inputs
  putStrLn $ "Part 2: " ++ show p2
