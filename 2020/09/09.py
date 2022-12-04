from itertools import permutations

lines = []

with open('input.txt', 'r') as f:
    lines = f.readlines()

values = [int(line) for line in lines]

preamble = 25
p1 = None
p1Idx = None
for i, val in enumerate(values[preamble:]):
    relevantValues = values[i:i+preamble]
    perms = permutations(relevantValues, 2)
    isValid = False
    for perm in perms:
        if val == sum(perm) and perm[0] != perm[1]:
            isValid = True
            break
    if not isValid:
        p1 = val
        p1Idx = i+preamble
        break

print("Part 1: ", p1)

p2 = None
for setLen in range(2, p1Idx):
    for i in range(p1Idx):
        ns = values[i:i+setLen]
        if sum(ns) == p1:
            p2 = min(ns) + max(ns)
        if p2:
            break

print("Part 2: ", p2)
