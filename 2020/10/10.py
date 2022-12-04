lines = []

with open('input.txt', 'r') as f:
    lines = f.readlines()

values = [int(line.strip()) for line in lines]
values = sorted(values)

ds = []
for i, v in enumerate(values):
    if i == 0:
        ds.append(values[i])
        continue
    ds.append(values[i] - values[i-1])
ds.append(3)

ones = 0
threes = 0
for v in ds:
    if v == 1:
        ones += 1
    elif v == 3:
        threes += 1
    else:
        print("wtf:", v)

print("Part 1: ", ones*threes,"(", ones, "*", threes,")")

endgame = values[-1] + 3

perms = []
curr = []
for d in ds:
    if d == 1:
        curr.append(1)
    if d != 1 and len(curr) > 0:
        perms.append(curr)
        curr = []
for i, perm in enumerate(perms):
    perm.pop(0)
perms = [perm for perm in perms if perm]

perm_lengths = [len(l) for l in perms]

twoFactors = [n for n in perm_lengths if n <= 2]
sevenFactors = [n for n in perm_lengths if n == 3]

twoFactorsPart = 2 ** sum(twoFactors)
sevenFactorsPart = 7 ** len(sevenFactors)
result = twoFactorsPart*sevenFactorsPart
print("Part 2: ", result, "(",twoFactorsPart, "*", sevenFactorsPart, ")")
