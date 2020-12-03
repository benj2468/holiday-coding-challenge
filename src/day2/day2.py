import re

with open('./input.txt') as file:
    lines = map(lambda x: re.findall(r'(\d+)-(\d+) (\w): (.*)', x)[0], file.read().strip().split('\n'))

def checkPasswordPartOne(line):
    count = 0
    for c in line[3]:
        if c == line[2]: count += 1
        if count > int(line[1]): return False
    
    if int(line[0]) <= count <= int(line[1]): return True


def checkPasswordPartTwo(line):
    char = line[2]
    return (line[3][int(line[0])-1] == char) ^ (line[3][int(line[1])-1] == char)

goodPasswordsPartOne = 0
goodPasswordsPartTwo = 0

for line in lines:
    if checkPasswordPartOne(line):
        goodPasswordsPartOne += 1
    if checkPasswordPartTwo(line):
        goodPasswordsPartTwo += 1

print(f'Part 1: {goodPasswordsPartOne}')
print(f'Part 2: {goodPasswordsPartTwo}')


