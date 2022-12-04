
def parse_mask(line):
    return line.split(' ')[-1]

def parse_mem_assignment(line):
    val = int(line.split(' ')[-1])
    addr = int(line.split('[')[1].split(']')[0])
    return addr, val

def to_bin(n):
    return '{0:036b}'.format(n)

def from_bin(s):
    return int(s,2)

def apply_mask_p1(mask_s, num_s):
    assert len(mask_s) == len(num_s)
    new_num = ""
    for i in range(len(mask_s)):
        if mask_s[i] == 'X':
            new_num += num_s[i]
        else:
            new_num += mask_s[i]
    assert len(new_num) == len(num_s)
    return new_num
        
lines = []
with open('input.txt', 'r') as f:
    lines = f.readlines()
lines = [line.strip() for line in lines]

resultsP1 = {}

curr_mask = None
for line in lines:
    if line.startswith("mask"):
        curr_mask = parse_mask(line)
        continue
    k, num = parse_mem_assignment(line)
    new_num = apply_mask_p1(curr_mask, to_bin(num))
    resultsP1[k] = from_bin(new_num)

p1Sum = 0
for v in resultsP1.values():
    p1Sum += v
print("Part 1:", p1Sum)

def apply_mask_p2(mask_s, num_s):
    assert len(mask_s) == len(num_s)
    new_num = ""
    for i in range(len(mask_s)):
        if mask_s[i] == '0':
            new_num += num_s[i]
        else:
            new_num += mask_s[i]
    assert len(new_num) == len(num_s)
    return new_num

def memory_addr_decode(mask, addr):
    addrs = []
    masked_addr = apply_mask_p2(mask, addr)
    xs = masked_addr.count('X')
    val_count = xs**2
    vals = []
    for i in range(val_count):
        s = '{:b}'.format(i).zfill(xs)
        vals.append(s)

    for val in vals:
        val_i = 0
        curr_addr = ""
        for j in range(len(masked_addr)):
            if masked_addr[j] == "X":
                curr_addr += val[val_i]
                val_i += 1
            else:
                curr_addr += masked_addr[j]
        assert len(curr_addr) == len(masked_addr)
        addrs.append(curr_addr)
    addr_numbers = [from_bin(x) for x in addrs]
    assert len(addr_numbers) == val_count
    return addr_numbers

curr_mask = None
resultsP2 = {}
for line in lines:
    if line.startswith("mask"):
        curr_mask = parse_mask(line)
        continue
    addr, num = parse_mem_assignment(line)
    addrs = memory_addr_decode(curr_mask, to_bin(addr))
    for a in addrs:
        resultsP2[a] = num

p2Sum = 0
for v in resultsP2.values():
    p2Sum += v
print("Part 2:", p2Sum)
