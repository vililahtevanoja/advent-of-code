
import           Data.Char                    (isAlpha, isDigit)
import           Text.ParserCombinators.ReadP (ReadP, munch, readP_to_S,
                                               satisfy)
type Distance = Int
type Degrees = Int

data Direction = North | East | South | West deriving (Show)
data Location = Location Int Int deriving (Show)
data Status = Status Location Direction deriving (Show)
data Command = Move Direction Distance | Lefty Degrees | Righty Degrees | Forward Distance deriving (Show)
data StatusWaypoint = StatusWaypoint Location Location deriving (Show)

cmdP :: ReadP Command
cmdP = do
  desc <- satisfy isAlpha
  amount <- munch isDigit
  return (cmdStringToCommand desc (read amount :: Int))
    where
      cmdStringToCommand :: Char -> Int -> Command
      cmdStringToCommand 'N' = Move North
      cmdStringToCommand 'E' = Move East
      cmdStringToCommand 'S' = Move South
      cmdStringToCommand 'W' = Move West
      cmdStringToCommand 'L' = Lefty
      cmdStringToCommand 'R' = Righty
      cmdStringToCommand 'F' = Forward
      cmdStringToCommand c   = error ("wtf: " ++ show c)

dirToAngle :: Direction -> Int
dirToAngle North = 0
dirToAngle East  = 90
dirToAngle South = 180
dirToAngle West  = 270

angleToDir :: Int -> Direction
angleToDir 0      = North
angleToDir 360    = North
angleToDir 720    = North
angleToDir 90     = East
angleToDir 450    = East
angleToDir (-270) = East
angleToDir 180    = South
angleToDir (-180) = South
angleToDir 540    = South
angleToDir 270    = West
angleToDir (-90)  = West
angleToDir 630    = West
angleToDir n      = error ("wtf: " ++ show n)

directionChange :: Command -> Direction -> Direction
directionChange (Lefty d) dir  = angleToDir (dirToAngle dir - d)
directionChange (Righty d) dir = angleToDir ((dirToAngle dir + d) `mod` 360)

performCommandP1 :: Status -> Command -> Status
performCommandP1 (Status (Location x y) currentDirection) = performCommandP1'
  where
    performCommandP1' :: Command -> Status
    performCommandP1' (Move North dist) = Status (Location x (y+dist)) currentDirection
    performCommandP1' (Move East dist) = Status (Location (x+dist) y) currentDirection
    performCommandP1' (Move South dist) = Status (Location x (y-dist)) currentDirection
    performCommandP1' (Move West dist) = Status (Location (x-dist) y) currentDirection
    performCommandP1' (Forward dist) = performCommandP1' (Move currentDirection dist)
    performCommandP1' (Lefty deg) = Status (Location x y) (directionChange (Lefty deg) currentDirection)
    performCommandP1' (Righty deg) = Status (Location x y) (directionChange (Righty deg) currentDirection)

rotateWaypoint :: Location -> Command -> Location
rotateWaypoint (Location wpX wpY) (Lefty deg)
  | deg > 0 = rotateWaypoint (Location (-wpY) wpX) (Lefty (deg-90))
  | otherwise = Location wpX wpY
rotateWaypoint (Location wpX wpY) (Righty deg)
  | deg > 0 = rotateWaypoint (Location wpY (-wpX)) (Righty (deg-90))
  | otherwise = Location wpX wpY
rotateWaypoint (Location wpX wpY) _ = Location wpX wpY

getCoordinates :: Status -> (Int, Int)
getCoordinates (Status (Location x y) _) = (x, y)

getCoordinatesSwp :: StatusWaypoint -> (Int, Int)
getCoordinatesSwp (StatusWaypoint (Location x y) _) = (x, y)

performCommandP2 :: StatusWaypoint -> Command -> StatusWaypoint
performCommandP2 (StatusWaypoint (Location sX sY) (Location wpX wpY)) = performCommandP2'
   where
    performCommandP2' :: Command -> StatusWaypoint
    performCommandP2' (Move North dist) = StatusWaypoint (Location sX sY ) (Location wpX (wpY+dist))
    performCommandP2' (Move East dist) = StatusWaypoint (Location sX sY) (Location (wpX+dist) wpY)
    performCommandP2' (Move South dist) = StatusWaypoint (Location sX sY) (Location wpX (wpY-dist))
    performCommandP2' (Move West dist) = StatusWaypoint (Location sX sY) (Location (wpX-dist) wpY)
    performCommandP2' (Forward dist) = StatusWaypoint (Location (sX+(wpX*dist)) (sY+(wpY*dist))) (Location wpX wpY)
    performCommandP2' (Lefty deg) = StatusWaypoint (Location sX sY) (rotateWaypoint (Location wpX wpY) (Lefty deg))
    performCommandP2' (Righty deg) = StatusWaypoint (Location sX sY) (rotateWaypoint (Location wpX wpY) (Righty deg))

main :: IO ()
main = do
  inputs <- lines <$> readFile "input.txt"
  let cmds = map fst $ concatMap (readP_to_S cmdP) inputs
  let resultP1 = foldl performCommandP1 (Status (Location 0 0) East)  cmds
  let (x1, y1) = getCoordinates resultP1
  let p1 = abs x1 + abs y1
  putStrLn $ "Part 1: " ++ show p1 ++ " (" ++ show x1 ++ ", " ++ show y1 ++ ")"
  let resultP2 = foldl performCommandP2 (StatusWaypoint (Location 0 0) (Location 10 1)) cmds
  let (x2, y2) = getCoordinatesSwp resultP2
  let p2 = abs x2 + abs y2
  putStrLn $ "Part 2: " ++ show p2 ++ " (" ++ show x2 ++ ", " ++ show y2 ++ ")"
