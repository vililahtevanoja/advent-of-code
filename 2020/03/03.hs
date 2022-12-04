
tree = '#'

data Slope = Slope { right::Int, down::Int } deriving (Show)

main :: IO ()
main = do
  inputs <- lines <$> readFile "input.txt"
  let rowLen = length (head inputs)
  let depth = length inputs
  let treeIndices = getTreeIndices inputs
  let slopes = [Slope 3 1, Slope 1 1, Slope 5 1, Slope 7 1, Slope 1 2]
  let steps = generateSteps (depth-1) (head slopes)
  let treesHit = length (filter (uncurry (hitTree treeIndices rowLen)) steps)
  putStrLn ("Part 1 trees hit: " ++ show treesHit)

  let stepss = map (generateSteps (depth-1)) slopes
  let treesHitPerSlope = map (length . filter (uncurry (hitTree treeIndices rowLen))) stepss
  let treesHitProduct = product treesHitPerSlope
  putStrLn ("Part 2 product of trees hit: " ++ show treesHitProduct)


generateSteps :: Int -> Slope -> [(Int, Int)]
generateSteps depth slope = do
  let rows = [0, (down slope) .. depth]
  let columns = [0,(right slope)..depth * right slope]
  zip rows columns

getTreeIndices :: [String] -> [[Int]]
getTreeIndices pattern = do
  let patternWithIndices = map (\line -> zip line [0..(length line)]) pattern
  let treesWithIndices = map (filter (\(c,_) -> c == tree)) patternWithIndices
  let treeIndices = map (map snd) treesWithIndices
  treeIndices

hitTree :: [[Int]] -> Int -> Int -> Int -> Bool
hitTree treeIndices rowLen row col = do
  let moduloCol = col `mod` rowLen
  moduloCol `elem` (treeIndices!!row)
