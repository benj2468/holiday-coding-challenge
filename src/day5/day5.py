import heapq

def calc_binary(s, true_str):
    if len(s) == 0:
        return 0
    if s[0] == true_str:
        return 2**(len(s)-1) + calc_binary(s[1:], true_str)
    else:
        return calc_binary(s[1:], true_str)


heap_ids = []
with open('./input.txt') as file:
    line = file.readline().strip()
    while line:
        row = calc_binary(line[0:7], 'B')
        col = calc_binary(line[7:], 'R')
        seat_id = row * 8 + col
        heapq.heappush(heap_ids, seat_id)
        line = file.readline().strip()
    
ids = [heapq.heappop(heap_ids) for i in range(len(heap_ids))]

i = 0
while i < len(ids) - 1:
    if ids[i] == ids[i-1] + 2:
        print(ids[i], ids[i-1])
        print(ids[i]-1)
        break
    else:
        i += 1
