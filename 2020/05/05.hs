data Seat = Seat {row::Int, column::Int, seatId::Int } deriving (Show)

seatBinSearch :: String -> Int
seatBinSearch [] = 0
seatBinSearch (c:cs) = inner c cs
  where
    inner 'F' ""     = 0
    inner 'L' ""     = 0
    inner 'B' ""     = 1
    inner 'R' ""     = 1
    inner 'F' (c:cs) = inner c cs
    inner 'L' (c:cs) = inner c cs
    inner 'B' (c:cs) = (2 ^ length (c:cs)) + inner c cs
    inner 'R' (c:cs) = (2 ^ length (c:cs)) + inner c cs
    inner _ _        = 0

parseSeat :: String -> Seat
parseSeat s =
  let
      rowInfoLength = 7
      columnInfoLength = 3
      columnCount = 8
      row = seatBinSearch (take rowInfoLength s)
      col = seatBinSearch (take columnInfoLength (drop rowInfoLength s))
  in Seat row col ((row*columnCount) + col)

main :: IO ()
main = do
  inputs <- lines <$> readFile "input.txt"
  let seats = map parseSeat inputs
  print $ "Part 1: " ++ show (maximum (map seatId seats))
  let existingIds = map seatId seats
  let missingSeats = filter (`notElem` existingIds) [1..maximum existingIds]
  let mySeat = filter (\id -> id-1 `elem` existingIds && id+1 `elem` existingIds) missingSeats
  print $ "Part 2: " ++ show (head mySeat)
