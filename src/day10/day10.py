
with open('./input.txt') as file:
    adapters = list(map(lambda x: int(x), file.readlines()))

adapters.sort()

ones = 1
threes = 1
i = 0
while i < len(adapters) - 1:
    diff = adapters[i+1] - adapters[i]
    if diff == 3: threes += 1
    elif diff == 1: ones += 1
    i += 1


# Adapters must be sorted
memory = {}
def get_arrangements(adapters):
    arrangements = 0
    if len(adapters) == 1: return 1
    i = 1
    while i < len(adapters) and adapters[i] - adapters[0] <= 3:
        if adapters[i] not in memory: memory[adapters[i]] = get_arrangements(adapters[i:])
        arrangements += memory[adapters[i]]
        i += 1
    return arrangements


print(f"1: Solution {ones * threes}")
print(f"2: Solution: {get_arrangements([0, *adapters])}")

