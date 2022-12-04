import           Data.Char (isDigit, isHexDigit)
import           Data.List (sort)
import           Text.Read (readMaybe)

required :: [String]
required =  sort ["byr", "iyr", "eyr", "hgt","hcl","ecl","pid"]
notRequired = "cid"

checkCompliance :: String -> String -> Bool
checkCompliance "byr" s = case (readMaybe s :: Maybe Int) of
                            Just n  -> n >= 1920 && n <= 2002
                            Nothing -> False
checkCompliance "iyr" s = case (readMaybe s :: Maybe Int) of
                            Just n  -> n >= 2010 && n <= 2020
                            Nothing -> False
checkCompliance "eyr" s = case (readMaybe s :: Maybe Int) of
                            Just n  -> n >= 2020 && n <= 2030
                            Nothing -> False
checkCompliance "hgt" s | unit == "cm"  = value >= 150 && value <= 193
                        | unit == "in" = value >= 59 && value <= 76
                        | otherwise = False
                        where unit = reverse $ take 2 (reverse s)
                              value = let
                                  valStr = if unit == "cm" then take 3 s else take 2 s
                                in if all isDigit valStr then (read valStr :: Int) else 0
checkCompliance "hcl" (c:cs) = c == '#' && length cs == 6 && all isHexDigit cs
checkCompliance "ecl" s = s `elem` ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
checkCompliance "pid" s = length ( filter isDigit s) == 9
checkCompliance "cid" _ = True -- ignore
checkCompliance k s = error ("Weird values: " ++ k ++ ":" ++ s)

isPrefixOf :: (Eq a) => [a] -> [a] -> Bool
isPrefixOf [] _              = True
isPrefixOf _ []              = False
isPrefixOf (c1:cs1) (c2:cs2) = (c1 == c2) && cs1 `isPrefixOf` cs2

hasPrefix :: (Eq a) => [a] -> [a] -> Bool
hasPrefix [] _              = False
hasPrefix _ []              = True
hasPrefix (c1:cs1) (c2:cs2) = (c1 == c2) && cs2 `hasPrefix` cs1

-- split at sub-sequence, excluding the sub-sequence from split results
splitAtSubSeqExcl :: (Eq a) => [a] -> [a] -> [[a]]
splitAtSubSeqExcl _ [] = []
splitAtSubSeqExcl sub seq = splitAtSubSeqExcl' sub seq [] []
  where
    splitAtSubSeqExcl' _ [] subacc acc = reverse $ reverse subacc:acc
    splitAtSubSeqExcl' sub (x:xs) subacc acc
      | sub `isPrefixOf` (x:xs) = splitAtSubSeqExcl' sub (drop (length sub - 1) xs) [] (reverse subacc:acc)
      | otherwise               = splitAtSubSeqExcl' sub xs (x:subacc) acc

orderAndFilterEntries :: [String] -> [String]
orderAndFilterEntries = map (unwords  . sort . filter (not . hasPrefix notRequired) . splitAtSubSeqExcl " ")

parseEntryKeyValues :: String -> [(String, String)]
parseEntryKeyValues s = do
  let entries =  splitAtSubSeqExcl " " s
  map (\e -> let [k,v] = splitAtSubSeqExcl ":" e in (k,v)) entries

main :: IO ()
main = do
    input <- readFile "input.txt"
    let splitInputs = splitAtSubSeqExcl "\n\n" input
    let cleanedInputs = map (map (\c -> if c == '\n' then ' ' else c)) splitInputs
    let orderedAndFilteredEntries = orderAndFilterEntries cleanedInputs
    let entriesKeyValues = map parseEntryKeyValues orderedAndFilteredEntries
    let sortedByKeys = map sort entriesKeyValues
    let p1Compliant = filter (\kvs -> map fst kvs ==required) sortedByKeys
    putStrLn $ "Part 1 compliant count: " ++ show (length p1Compliant) ++ " "
    let p2Compliant = filter (all (uncurry checkCompliance)) p1Compliant
    putStrLn $ "Part 2 compliant count: " ++ show (length p2Compliant)
