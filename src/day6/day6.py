from functools import reduce


with open('./input.txt') as f:
    groups = [[set(e) for e in v.split('\n')] for v in f.read().split('\n\n') ]

group_overlap = [reduce(lambda x,y : x & y, group) for group in groups]

sum = reduce(lambda x,y: x + y,[len(s) for s in group_overlap])
print(sum)