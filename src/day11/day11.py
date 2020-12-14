from copy import deepcopy

def get_next_chair(start, array, j_d, i_d, max):    
    spot = (start[0] + j_d, start[1] + i_d)
    while spot[0] <= max[0] and spot[0] >= 0 and spot[1] <= max[1] and spot[1] >= 0 and array[spot[0]][spot[1]] == '.':
        spot = (spot[0] + j_d, spot[1] + i_d)
    if (spot[0] <= max[0] and spot[0] >= 0 and spot[1] <= max[1] and spot[1] >= 0): 
        if start == (9,7): print(spot, array[spot[0]][spot[1]])
        return array[spot[0]][spot[1]]
    return '.'



def calc_next_state(array, j, i):
    occupied = 0
    state = array[j][i]
    if state == '.': return '.'
    for j_d in range(-1, 2):
        for i_d in range(-1, 2):
            if j_d == 0 and i_d == 0: continue
            if get_next_chair((j,i), array, j_d, i_d, (len(array)-1, len(array[0])-1)) == '#': occupied += 1
    if state == 'L' and occupied == 0:  return '#' # Open
    elif state == '#' and occupied >= 5: return 'L' # Occupied
    return state


with open('./input.txt') as file:
    lines = list(map(lambda x: x.strip(), file.readlines()))


array = [[None for j in range(len(lines[0]))] for i in range(len(lines))]
for (j, line) in enumerate(lines):
    for (i, char) in enumerate(line):
        array[j][i] = char

changed = True
k = 0

while changed:
    occupied = 0
    changed = False
    next_array = deepcopy(array)
    for (j, line) in enumerate(array):
        for (i, char) in enumerate(line):
            next_state = calc_next_state(array, j, i)
            if next_state != array[j][i]: changed = True
            next_array[j][i] = next_state
            if next_state == '#': occupied += 1
    array = next_array 
    k += 1
    # if (k == 3): break

print(occupied)


