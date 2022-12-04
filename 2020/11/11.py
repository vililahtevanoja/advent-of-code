empty = 'L'
occupied = '#'
floor = '.'

lines = []
with open('input.txt', 'r') as f:
    lines = f.readlines()
lines = [line.strip() for line in lines]

def getOccupiedAdjacent(ls, y, x):
    x_min = max(x-1, 0)
    x_max = min(x+1, len(ls[0])-1)
    y_min = max(y-1, 0)
    y_max = min(y+1, len(ls)-1)
    occupieds = 0
    for xcurr in range(x_min, x_max+1):
        for ycurr in range(y_min, y_max+1):
            if xcurr == x and ycurr == y:
                continue
            if ls[ycurr][xcurr] == occupied:
                occupieds+=1
    return occupieds

def countOccupied(ls):
    return sum([row.count(occupied) for row in ls])

def maybeSwapP1(ls, y, x):
    seat = ls[y][x]
    occupiedAdjacent = getOccupiedAdjacent(ls, y, x)
    if seat == empty and occupiedAdjacent == 0:
        return occupied
    elif seat == occupied and occupiedAdjacent >= 4:
        return empty
    else:
        return seat

def roundP1(ls):
    swaps = 0
    newMap = []
    for row in range(len(ls)):
        newRow = ""
        for col in range(len(ls[0])):
            old = ls[row][col]
            maybeNewSeat = maybeSwapP1(ls, row, col)
            if maybeNewSeat != old:
                swaps += 1
            newRow = newRow + maybeNewSeat
        newMap.append(newRow)
    return (swaps, newMap)

p1Map = lines
swaps = 1
while swaps > 0:
    swaps, p1Map = roundP1(p1Map)
print("Part 1: ",countOccupied(p1Map))


def getOccupiedVisual(ls, y, x):
    direction_deltas = [(1,0), (1,1), (0,1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]
    x_max = len(ls[0])-1
    y_max = len(ls)-1
    occupieds = 0
    for y_d, x_d in direction_deltas:
        x_curr = x+x_d
        y_curr = y+y_d
        while 0 <= x_curr <= x_max and 0 <= y_curr <= y_max:
            currSeat = ls[y_curr][x_curr]
            if currSeat == occupied:
                occupieds += 1
                break
            elif currSeat == empty:
                break
            x_curr = x_curr+x_d
            y_curr = y_curr+y_d
    return occupieds


def maybeSwapP2(ls, y, x):
    seat = ls[y][x]
    occupiedVisual = getOccupiedVisual(ls, y, x)
    if seat == empty and occupiedVisual == 0:
        return occupied
    elif seat == occupied and occupiedVisual >= 5:
        return empty
    else:
        return seat

def roundP2(ls):
    swaps = 0
    newMap = []
    for row in range(len(ls)):
        newRow = ""
        for col in range(len(ls[0])):
            old = ls[row][col]
            maybeNewSeat = maybeSwapP2(ls, row, col)
            if maybeNewSeat != old:
                swaps += 1
            newRow = newRow + maybeNewSeat
        newMap.append(newRow)
    return (swaps, newMap)

p2Map = lines
swaps = 1
while swaps > 0:
    swaps, p2Map = roundP2(p2Map)
print("Part 1: ",countOccupied(p2Map))
