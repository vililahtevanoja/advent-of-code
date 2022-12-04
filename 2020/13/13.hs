import           Data.List (foldl', minimumBy)
import           Data.Ord  (comparing)
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


distToNext :: Int -> Int -> Int
distToNext n freq = if freqMod == 0 then 0 else freq - freqMod
  where freqMod = n `mod` freq

iterFind :: (a -> Bool) -> (a -> a) -> a -> a
iterFind pred func = iterFind'
  where
    iterFind' x
      | pred x    = x
      | otherwise = iterFind' (func x)

p2Find :: [(Int, Int)] -> Int
p2Find = fst . foldl' p2Find' (0, 1)
  where
    p2Find' (base, step) (offset, i) = (newBase, step * i)
      where newBase = iterFind (\n -> (n + offset) `mod` i == 0) (+step) base

main :: IO ()
main = do
  inputs <- lines <$> readFile "input.txt"
  let t = read (head inputs) :: Int
  let ss = splitAtExcl ',' $ head (tail inputs )
  let busDepartures =
        map (\bl -> (bl, [0,bl..])) busLines
          where busLines = map (\l -> read l :: Int) $ filter (/= "x") ss
  let firstBusDeparturesAfterTimestamp = map (\(b,ds) -> (b, head (filter (> t) ds))) busDepartures
  let (firstBusLine, firstBusDeparture) = minimumBy (comparing snd) firstBusDeparturesAfterTimestamp
  let p1 = firstBusLine * (firstBusDeparture - t)
  putStrLn $ "Part 1: " ++ show p1

  let busDeparturesP2 = map (\(i,s) -> (i, read s :: Int)) $ filter (\(_,s) -> s /= "x") $ zip [0..] ss
  let p2 = p2Find busDeparturesP2
  putStrLn $ "Part 2: " ++ show p2
