from collections import namedtuple


Instruction = namedtuple('Instruction', ['name', 'argument'])

lines = []
with open('input.txt', 'r') as f:
    lines = f.readlines()

instructions = []
for i, line in enumerate(lines):
    iv = line.strip().split(' ')
    ins = iv[0]
    argument = iv[1]
    instructions.append(Instruction(ins, int(argument)))

def rec_travel_instructions(instructions, visits,  idx, acc):
    if idx >= len(instructions):
        return (acc, True)
    elif idx not in visits:
        visits[idx] = {}
    else:
        return (acc, False)
    
    instruction = instructions[idx]
    if instruction.name == 'nop':
        return rec_travel_instructions(instructions, visits, idx+1, acc)
    elif instruction.name == 'acc':
        return rec_travel_instructions(instructions, visits, idx+1, acc+instruction.argument)
    elif instruction.name == 'jmp':
        return rec_travel_instructions(instructions, visits, idx+instruction.argument, acc)
    else:
        raise ValueError("Should not be here")

p1, _ = rec_travel_instructions(instructions, {}, 0, 0)
print("Part 1: ",p1)

nopJmpPositions = [i for i,instruction in enumerate(instructions) if instruction.name == 'jmp' or instruction.name == 'nop']
candidates = []

def flipNopJmp(instruction):
    if instruction.name == 'jmp':
        return Instruction('nop', instruction.argument)
    elif instruction.name == 'nop':
        return Instruction('nop', instruction.argument)
    else:
        raise ValueError("Should not be here")

for i in nopJmpPositions:
    candidate = instructions[:]
    candidate[i] = flipNopJmp(candidate[i])
    candidates.append(candidate)

p2 = None
for candidate in candidates:
    accSum, natural = rec_travel_instructions(candidate, {}, 0, 0)
    if not natural:
        continue
    else:
        p2 = accSum
if not p2:
    print ("Not found")
else:
    print ("Part 2: ", p2)
