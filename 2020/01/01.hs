main :: IO ()
main = do
  inputs <- lines <$> readFile "input.txt"
  let ns = map read inputs :: [Int]
  let result1 = head [a * b | a <- ns, b <- ns, a + b == 2020]
  let result2 = head [a * b * c | a <- ns, b <- ns, c <- ns, a + b + c == 2020]
  putStrLn $ "Part 1: " ++ show result1
  putStrLn $ "Part 2: " ++ show result2
